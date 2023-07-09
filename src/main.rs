use crate::{
    errors::syntax_error,
    llvm::{compile, Compiler},
    project::ProjectConfiguration,
};
use clap::{Parser, Subcommand};
use colored::Colorize;
use git::{clone_else_pull, generate};
use inkwell::{
    context::Context,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    AddressSpace, OptimizationLevel,
};
use parser::{parse, Tree};
use pest::Parser as PestParser;
use pest_derive::Parser as PestParser;
use project::Project;
use std::{
    env, fs,
    hash::Hash,
    path::{Path, PathBuf},
    process::Command,
};

pub mod errors;
pub mod from_pair;
pub mod git;
pub mod llvm;
pub mod logger;
pub mod parser;
pub mod project;
pub mod utils;

#[derive(PestParser)]
#[grammar = "../grammar.pest"]
struct RLParser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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

        /// Compiles LLVM to assembly, instead of object, before linking
        #[arg(short, long)]
        assembly: bool,
    },
    /// Removes build dir
    Clean,
    /// Creates a new walter project
    New {
        /// If you don't specify a name it is created in the current directory with the current directories name if it is empty.
        name: Option<String>,
    },
}

fn get_project() -> Project {
    match Project::from_path(env::current_dir().unwrap().as_path()) {
        Some(x) => x,
        None => {
            error!("No valid {} found.", "walter.yml".bold());
        }
    }
}

const STDLIB_URL: &str = "https://github.com/elijah629/redditlang-std";

fn build_libstd() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let walter_dir = dirs::home_dir().unwrap().join(".walter");
    let std_dir = walter_dir.join("stdlib");

    fs::create_dir_all(&walter_dir)?;

    // Make sure libstd is up to date
    clone_else_pull(STDLIB_URL, &std_dir, "main").expect("Failed to clone libstd repo");

    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(&std_dir)
        .output()?;

    fs::rename(
        &std_dir.join("target/release/libstd.a"),
        &std_dir.join("libstd.a"),
    )?;

    Command::new("cargo")
        .arg("clean")
        .current_dir(&std_dir)
        .output()?;

    Ok(std_dir.join("libstd.a"))
}

fn main() {
    let args = Args::parse();
    logger::init().unwrap();

    match args.command {
        Commands::Cook { release, assembly } => {
            let project = get_project();
            let std_path = match build_libstd() {
                Ok(x) => x,
                Err(x) => {
                    error!("Error building libstd: {:?}", x);
                }
            };

            let project_dir = Path::new(&project.path);
            let build_dir =
                project_dir
                    .join("build")
                    .join(if release { "release" } else { "debug" });
            let src_dir = project_dir.join("src");
            let main_file = src_dir.join("main.rl");
            let main_file = fs::read_to_string(&main_file).unwrap();

            fs::create_dir_all(&build_dir).unwrap();

            log::info!("Lexing/Parsing");

            let tree = parse_file(&main_file);

            let context = Context::create();
            let module = context.create_module("main");
            let builder = context.create_builder();

            let compiler = &Compiler {
                context: &context,
                module,
                builder,
            };

            // Add libstd functions

            let println_type = compiler.context.void_type().fn_type(
                &[compiler
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .into()],
                false,
            );
            compiler
                .module
                .add_function("coitusinterruptus", println_type, None);

            let main_type = compiler.context.i32_type().fn_type(&[], false);
            let main_fn = compiler.module.add_function("main", main_type, None);

            let entry_basic_block = compiler.context.append_basic_block(main_fn, "");
            compiler.builder.position_at_end(entry_basic_block);

            log::info!("Converting AST to LLVM");

            compile(&compiler, &tree, &entry_basic_block);

            compiler
                .builder
                .build_return(Some(&compiler.context.i32_type().const_zero()));

            println!("{}", &compiler.module.print_to_string().to_str().unwrap());

            if let Err(x) = compiler.module.verify() {
                log::error!("│ {}", "Module verification failed".bold());
                let lines: Vec<&str> = x.to_str().unwrap().lines().collect();
                for line in &lines[0..lines.len() - 1] {
                    log::error!("│  {}", line);
                }
                error!("└─ {}\n", lines.last().unwrap());
            };

            log::info!("Compiling");

            Target::initialize_x86(&InitializationConfig::default());

            let opt = if release {
                OptimizationLevel::Aggressive
            } else {
                OptimizationLevel::None
            };

            let reloc = RelocMode::PIC;
            let model = CodeModel::Default;

            let object_path = &build_dir.join(format!(
                "{}.reddit.{}",
                project.config.name,
                if assembly { "s" } else { "o" }
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
                )
                .unwrap();

            log::info!("Linking");

            let target_str = target_triple.as_str().to_str().unwrap();

            let compiler = cc::Build::new()
                .target(&target_str)
                .out_dir(&build_dir)
                .opt_level(if release { 3 } else { 0 })
                .host(&target_str)
                .cargo_metadata(false)
                .get_compiler();

            let output_file = build_dir.join(&project.config.name);
            let output_file = output_file.to_str().unwrap();

            compiler
                .to_command()
                .arg(&object_path)
                .arg(std_path) // Could add nostd option that removes this
                .args(["-o", output_file])
                .spawn()
                .unwrap();

            log::info!("Done! Executable is avalible at {}", output_file.bold());
        }
        Commands::New { name } => {
            let cwd = env::current_dir().unwrap();
            let path = match name {
                Some(name) => cwd.join(name),
                None => cwd,
            };

            fs::create_dir_all(&path).unwrap();
            let is_empty = fs::read_dir(&path).unwrap().count() == 0;

            let pathstr = path.to_str().unwrap().bold();

            if !is_empty {
                error!("{} exists and is not empty", pathstr);
            }

            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            log::info!("Creating {} at {}", name, pathstr);

            const TEMPLATE_URL: &str = "https://github.com/elijah629/redditlang";
            const TEMPLATE_REFNAME: &str = "refs/remotes/origin/template";

            generate(TEMPLATE_URL, Some(TEMPLATE_REFNAME), &path).unwrap();

            let yaml = serde_yaml::to_string(&ProjectConfiguration {
                name,
                version: "0.0.1".to_owned(),
            })
            .unwrap();

            fs::write(&path.join("walter.yml"), yaml).unwrap();
        }
        Commands::Clean => {
            let project = get_project();
            let build_dir = Path::new(&project.path).join("build");

            log::info!("Cleaning");
            fs::remove_dir_all(build_dir).unwrap();
        }
    }
}

fn parse_file(file: &str) -> Tree {
    match RLParser::parse(Rule::Program, file) {
        Ok(x) => parse(x),
        Err(x) => syntax_error(x),
    }
}
