use crate::{
    errors::error,
    llvm::{llvm, Compiler},
};
use inkwell::{
    context::Context,
    passes::PassManager,
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine},
    AddressSpace, OptimizationLevel,
};
use parser::{parse, Tree};
use pest::Parser;
use pest_derive::Parser;
use std::{hash::Hash, path::Path};

pub mod errors;
pub mod from_pair;
pub mod llvm;
pub mod parser;
pub mod utils;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct RLParser;

fn main() {
    // joke idea: you compile this program setting the file path in this. therefore i dont have to make a compiler! just use rust
    // update: as of now, this idea has been replaced with LLVM
    let tree = parse_file(include_str!("../examples/hello_world.rl"));
    println!("Lexed\n{:?}\n", &tree);

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let compiler = &Compiler {
        context: &context,
        module,
        builder,
        fpm,
    };

    let putchar_type = compiler.context.i32_type().fn_type(
        &[compiler
            .context
            .i8_type()
            .ptr_type(AddressSpace::default())
            .into()],
        false,
    );
    compiler.module.add_function("puts", putchar_type, None);

    let main_type = compiler.context.void_type().fn_type(&[], false);
    let main_fn = compiler.module.add_function("main", main_type, None);

    let entry_basic_block = compiler.context.append_basic_block(main_fn, "entry");
    compiler.builder.position_at_end(entry_basic_block);

    llvm(&compiler, &tree);

    compiler.builder.build_return(None);

    println!(
        "LLVM RL\n{}\n",
        &compiler.module.print_to_string().to_string()
    );

    let verified = compiler.module.verify();
    if verified.is_err() {
        eprintln!(
            "Verification failed\n{}\n",
            verified.unwrap_err().to_string()
        );
        std::process::exit(1);
    }

    Target::initialize_x86(&InitializationConfig::default());

    let opt = OptimizationLevel::Aggressive;
    let reloc = RelocMode::Default;
    let model = CodeModel::Default;

    let path = Path::new("module.o");

    let target = Target::from_name("x86-64").unwrap();
    let target_machine = target
        .create_target_machine(
            &TargetMachine::get_default_triple(),
            "x86-64",
            "+avx2",
            opt,
            reloc,
            model,
        )
        .unwrap();

    target_machine
        .write_to_file(&compiler.module, inkwell::targets::FileType::Object, path)
        .unwrap();
}

fn parse_file(file: &str) -> Tree {
    let pairs = RLParser::parse(Rule::Program, file);
    if pairs.is_err() {
        error(pairs.unwrap_err());
    }
    parse(pairs.unwrap())
}
