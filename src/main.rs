use crate::{
    compiler::{
        compile,
        linking::{build_libstd, define_libstd, link},
        CompileMetadata, Compiler, Scope,
    },
    errors::syntax_error,
    project::ProjectConfiguration,
};
use clap::{Parser, Subcommand};
use colored::Colorize;
use git::generate;
use inkwell::{
    context::Context,
    module::Module,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};
use parser::{parse, Tree};
use pest::Parser as PestParser;
use pest_derive::Parser as PestParser;
use project::Project;
use semver::Version;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
    hash::Hash,
    path::{Path, PathBuf},
    process::Command,
};
use utils::Result;

pub mod compiler;
pub mod errors;
pub mod git;
pub mod logger;
pub mod parser;
pub mod project;
pub mod utils;

#[derive(PestParser)]
#[grammar = "../grammar.pest"]
struct RLParser;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Builds a program
    Cook {
        /// Enables release mode, longer build but more optimizations.
        #[arg(short, long)]
        release: bool,

        /// Compiles LLVM to an assembly file instead of an object file before linking
        #[arg(short, long)]
        assembly: bool,

        /// Don't link the standard library
        #[arg(short, long)]
        no_std: bool,

        /// Strip the resulting executable
        #[arg(short, long)]
        strip: bool,

        /// Prints the LLVM IR when compiling
        #[arg(short = 'i', long)]
        print_ir: bool,

        /// Prints the AST when parsing
        #[arg(short = 't', long)]
        print_ast: bool,
    },
    /// Builds and runs program
    Serve {
        /// Enables release mode, longer build but more optimizations.
        #[arg(short, long)]
        release: bool,

        /// Compiles LLVM to an assembly file instead of an object file before linking
        #[arg(short, long)]
        assembly: bool,

        /// Don't link the standard library
        #[arg(short, long)]
        no_std: bool,

        /// Strip the resulting executable
        #[arg(short, long)]
        strip: bool,

        /// Prints the LLVM IR when compiling
        #[arg(short = 'i', long)]
        print_ir: bool,

        /// Prints the AST when parsing
        #[arg(short = 't', long)]
        print_ast: bool,

        /// Optional arguments to pass to the program.
        args: Option<Vec<String>>,
    },
    /// Removes build dir
    Clean,
    /// Creates a new walter project
    Rise {
        /// If you don't specify a name it is created in the current directory with the current directories name if it is empty.
        name: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    main_r(args).unwrap_or_else(|x| error!("{}", x));
}

fn main_r(args: Args) -> Result<()> {
    logger::init().unwrap();

    match args.command {
        Commands::Cook {
            release,
            assembly,
            no_std,
            print_ir,
            print_ast,
            strip,
        } => {
            let output_file = cook(release, assembly, no_std, print_ir, print_ast, strip)?;
            log::info!(
                "Done! Executable is avalible at {}",
                output_file.to_str().unwrap().bold()
            );
        }
        Commands::Rise { name } => {
            let cwd = env::current_dir()?;
            let path = name.map(|name| cwd.join(name)).unwrap_or(cwd);

            fs::create_dir_all(&path)?;
            let is_empty = fs::read_dir(&path)?.count() == 0;

            let pathstr = path.to_str().unwrap().bold();

            if !is_empty {
                return Err(format!("{} exists and is not empty", pathstr).into());
            }

            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            log::info!("Creating {} at {}", name.bold(), pathstr);

            const TEMPLATE_URL: &str = "https://github.com/elijah629/redditlang";
            const TEMPLATE_REFNAME: &str = "refs/remotes/origin/template";

            generate(TEMPLATE_URL, Some(TEMPLATE_REFNAME), &path)?;

            let yaml = serde_yaml::to_string(&ProjectConfiguration {
                name,
                version: Version::new(0, 0, 1),
            })
            .unwrap();

            fs::write(&path.join("walter.yml"), yaml)?;
        }
        Commands::Clean => {
            let project = Project::from_current()?;
            let build_dir = Path::new(&project.path).join("build");

            log::info!("Cleaning");
            fs::remove_dir_all(build_dir).unwrap();
        }
        Commands::Serve {
            release,
            assembly,
            no_std,
            strip,
            print_ir,
            print_ast,
            args,
        } => {
            let output_file = cook(release, assembly, no_std, print_ir, print_ast, strip)?;
            log::info!("Running {}", output_file.to_str().unwrap().bold());

            let mut command = Command::new(output_file);
            if let Some(args) = args {
                command.args(args);
            }

            command.spawn()?;
        }
    }
    Ok(())
}

fn parse_file(file: &str) -> Result<Tree> {
    match RLParser::parse(Rule::Program, file) {
        Ok(x) => parse(x),
        Err(x) => syntax_error(x),
    }
}

// should be a config struct
fn cook(
    release: bool,
    assembly: bool,
    no_std: bool,
    print_ir: bool,
    print_ast: bool,
    strip: bool,
) -> Result<PathBuf> {
    let project = Project::from_current()?;
    let std_path = build_libstd()?;

    let project_dir = Path::new(&project.path);
    let build_dir = project_dir
        .join("build")
        .join(if release { "release" } else { "debug" });
    let src_dir = project_dir.join("src");
    let main_file = src_dir.join("main.rl");
    let main_file = fs::read_to_string(&main_file)?;

    fs::create_dir_all(&build_dir)?;

    log::info!("Lexing/Parsing");

    let tree = parse_file(&main_file)?;

    if print_ast {
        println!("{:#?}", tree);
    }

    log::info!("Building module tree");

    /// Collects every created module and canocalizes the path
    /// parent_module_path is the module path of the modules parent,
    /// ex if the full module file path is a.b.c, this should be a.b
    /// base_path is the FS path where main.rl is located
    fn get_all_modules<P: AsRef<Path>>(
        tree: &Tree,
        src_dir: P,
    ) -> Result<HashMap<PathBuf, Tree>> {
        fn recursive(
            tree: &Tree,
            parent_module_path: &Path,
            base_path: &Path,
            imports: &mut HashMap<PathBuf, Tree>,
        ) -> Result<()> {
            for node in tree {
                match node {
                    parser::Node::Import(import) => {
                        // base_path + parent_module_path + import.0 + ".rl" = path to file
                        let module_path = parent_module_path.join(&import.0);
                        let file_path = base_path
                            .clone()
                            .to_path_buf()
                            .join(module_path.with_extension("rl"));

                        let file_contents = fs::read_to_string(&file_path)?;

                        if !imports.contains_key(&module_path) {
                            let tree = parse_file(&file_contents)?;

                            imports.insert(module_path, tree.clone());
                            recursive(&tree, parent_module_path, base_path, imports)?;
                        }
                    }
                    _ => (),
                }
            }

            Ok(())
        }
        let mut imports = HashMap::new();
        recursive(&tree, "".as_ref(), &src_dir.as_ref(), &mut imports)?;
        Ok(imports)
    }

    // 1. recursively navigate tree, following all imports.
    // 2. compile each tree
    // 3. link all modules together

    let mut trees = get_all_modules(&tree, &src_dir)?;
    trees.insert(PathBuf::from("main"), tree);

    log::info!(
        "Compiling {} {}",
        trees.len().to_string().bold(),
        if trees.len() == 1 { "tree" } else { "trees" }
    );

    let context = Context::create();
    let builder = context.create_builder();

    let combined_module = trees
        .into_iter()
        .map(|(name, tree)| {
            let name = name
                .components()
                .map(|x| x.as_os_str().to_str().unwrap())
                .collect::<Vec<_>>()
                .join(".");

            let module = context.create_module(&name);
            let compiler = Compiler {
                context: &context,
                module,
                builder: &builder,
            };

            define_libstd(&compiler);

            let compiler = &compiler;
            let main_type = compiler.context.i32_type().fn_type(&[], false);
            let main_fn = compiler.module.add_function(
                if name == "main" {
                    "main".to_string()
                } else {
                    format!("{}.main", &name)
                }
                .as_str(),
                main_type,
                None,
            );

            let entry_basic_block = compiler.context.append_basic_block(main_fn, "");
            compiler.builder.position_at_end(entry_basic_block);

            compile(
                &compiler,
                &tree,
                &mut CompileMetadata {
                    r#loop: None,
                    fn_value: main_fn,
                    function_scope: Scope {
                        variables: HashMap::new(),
                    },
                },
            )?;

            // Add return
            compiler
                .builder
                .build_return(Some(&compiler.context.i32_type().const_zero()));

            let module_name = &compiler.module.get_name().to_str()?;
            if print_ir {
                println!("Module: {}", module_name.bold());
                println!("{}", &compiler.module.print_to_string().to_str().unwrap());
            }

            // LLVM errors
            if let Err(x) = compiler.module.verify() {
                log::error!("│ Module verification for {} failed", module_name.bold());
                let lines: Vec<&str> = x.to_str().unwrap().lines().collect();
                for line in &lines[0..lines.len() - 1] {
                    log::error!("│  {}", line);
                }
                error!("└─ {}\n", lines.last().unwrap());
            };

            Ok(compiler.module.clone())
        })
        .reduce(|a: Result<Module<'_>>, c| {
            let a = a?;
            let c = c?;

            c.link_in_module(a)?;

            Ok(c)
        })
        .unwrap()?;

    // TODO: allow user chosen targets
    Target::initialize_x86(&InitializationConfig::default());

    let opt = if release {
        OptimizationLevel::Aggressive
    } else {
        OptimizationLevel::None
    };

    let reloc = RelocMode::PIC; // required for some bizzare reason
    let model = CodeModel::Default;

    let object_path = &build_dir.join(format!(
        "{}.reddit.{}",
        project.config.name,
        if assembly { "s" } else { "o" } // "s" being asm, could do .asm but whatever
    ));

    let target = Target::from_name("x86-64").unwrap();
    let target_triple = &TargetMachine::get_default_triple();
    let target_machine = target
        .create_target_machine(target_triple, "x86-64", "+avx2", opt, reloc, model)
        .unwrap();

    target_machine.write_to_file(
        &combined_module,
        if assembly {
            FileType::Assembly
        } else {
            FileType::Object
        },
        &object_path,
    )?;

    log::info!("Linking");

    link(
        &project,
        &target_triple,
        &build_dir,
        &object_path,
        &std_path,
        release,
        no_std,
        strip,
    )
}
