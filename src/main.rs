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
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};
use parser::{parse, Tree};
use pest::Parser as PestParser;
use pest_derive::Parser as PestParser;
use project::Project;
use semver::Version;
use utils::Result;
use std::{
    collections::HashMap,
    env, fs,
    hash::Hash,
    path::{Path, PathBuf},
    process::Command,
};

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

        /// Does not link the standard library
        #[arg(short, long)]
        no_std: bool,

        /// Shows the LLVM IR when compiling
        #[arg(short, long)]
        show_ir: bool,

        /// Shows the AST when parsing
        #[arg(short = 't', long)]
        show_ast: bool,
    },
    /// Builds and runs program
    Serve {
        /// Enables release mode, longer build but more optimizations.
        #[arg(short, long)]
        release: bool,

        /// Compiles LLVM to an assembly file instead of an object file before linking
        #[arg(short, long)]
        assembly: bool,

        /// Does not link the standard library
        #[arg(short, long)]
        no_std: bool,

        /// Shows the LLVM IR when compiling
        #[arg(short, long)]
        show_ir: bool,

        /// Shows the AST when parsing
        #[arg(short = 't', long)]
        show_ast: bool,

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
            show_ir,
            show_ast,
        } => {
            let output_file = cook(release, assembly, no_std, show_ir, show_ast)?;
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
            show_ir,
            args,
            show_ast,
        } => {
            let output_file = cook(release, assembly, no_std, show_ir, show_ast)?;
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
fn cook(release: bool, assembly: bool, no_std: bool, show_ir: bool, show_ast: bool) -> Result<PathBuf> {
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

    if show_ast {
        println!("{:#?}", tree);
    }

    log::info!("Compiling");

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let compiler = Compiler {
        context: &context,
        module,
        builder,
    };

    define_libstd(&compiler);

    let entry_basic_block = {
        let compiler = &compiler;
        let main_type = compiler.context.i32_type().fn_type(&[], false);
        let main_fn = compiler.module.add_function("main", main_type, None);

        let entry_basic_block = compiler.context.append_basic_block(main_fn, "");
        compiler.builder.position_at_end(entry_basic_block);
        entry_basic_block
    };

    compile(
        &compiler,
        &tree,
        &mut CompileMetadata {
            basic_block: entry_basic_block,
            function_scope: Scope {
                variables: HashMap::new(),
            },
        },
    )?;

    // Add return
    compiler
        .builder
        .build_return(Some(&compiler.context.i32_type().const_zero()));

    if show_ir {
        println!("{}", &compiler.module.print_to_string().to_str().unwrap());
    }

    // LLVM errors
    if let Err(x) = compiler.module.verify() {
        log::error!("│ {}", "Module verification failed".bold());
        let lines: Vec<&str> = x.to_str().unwrap().lines().collect();
        for line in &lines[0..lines.len() - 1] {
            log::error!("│  {}", line);
        }
        error!("└─ {}\n", lines.last().unwrap());
    };

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

    target_machine
        .write_to_file(
            &compiler.module,
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
    )
}
