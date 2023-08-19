use std::{
    fs,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use inkwell::{targets::TargetTriple, AddressSpace};
use log::info;

use crate::{compiler::Compiler, git::update, project::Project};

const STDLIB_URL: &str = "https://github.com/elijah629/redditlang-std";

/// Builds libstd and returns a path to it
pub fn build_libstd() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let walter_dir = dirs::home_dir().unwrap().join(".walter");
    let std_dir = walter_dir.join("stdlib");

    fs::create_dir_all(&walter_dir)?;
 
    // Ensure libstd is up to date, should just not do this. ie check for new commits
    let up_to_date = update(STDLIB_URL, &std_dir, "main").expect("Failed to update libstd repo");

    if up_to_date {
        return Ok(std_dir.join("libstd.a"));
    }
    
    info!("Updating libstd");

    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(&std_dir)
        .output()?;

    // TODO: Windows
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

pub fn link(
    project: &Project,
    target_triple: &TargetTriple,
    build_dir: &PathBuf,
    object_path: &PathBuf,
    std_path: &PathBuf,
    release: bool,
    no_std: bool,
) -> PathBuf {
    let target_str = target_triple.as_str().to_str().unwrap();

    let compiler = cc::Build::new()
        .target(&target_str)
        .out_dir(&build_dir)
        .opt_level(if release { 3 } else { 0 })
        .host(&target_str)
        .cargo_metadata(false)
        .get_compiler();

    let output_file = build_dir.join(&project.config.name);

    let mut command = compiler.to_command();
    command.arg(&object_path);

    if !no_std {
        command.arg(&std_path);
    }

    command.arg("-o");
    command.arg(&output_file);

    let output = command.output().unwrap();

    if !ExitStatus::success(&output.status) {
        for line in String::from_utf8(output.stderr).unwrap().lines() {
            log::error!("{}", line);
        }
        std::process::exit(1);
    }

    output_file
}

pub fn define_libstd(compiler: &Compiler) {
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

    let println_type = compiler
        .context
        .i8_type()
        .ptr_type(AddressSpace::default())
        .fn_type(&[compiler.context.f64_type().into()], false);
    compiler.module.add_function("nums", println_type, None);
}
