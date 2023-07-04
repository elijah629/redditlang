use crate::{
    errors::error,
    llvm::{llvm, Compiler},
};
use inkwell::{context::Context, passes::PassManager, AddressSpace};
use parser::{parse, Tree};
use pest::Parser;
use pest_derive::Parser;
use std::{hash::Hash, path::Path, process::Command};

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

    // let execution_engine = module
    //     .create_jit_execution_engine(OptimizationLevel::None)
    //     .expect("failed to create ExecutionEngine");

    let compiler = &Compiler {
        context: &context,
        module,
        // execution_engine,
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
    let main_fn = compiler.module.add_function("_start", main_type, None);

    // let test_fn_name = CString::new("test_fn").unwrap();
    // unsafe {
    //     LLVMAddSymbol(test_fn_name.as_ptr(), test_fn as *mut c_void);
    // }

    // let fn_type = compiler.context.void_type().fn_type(&[], false);
    // let fn_val = compiler.module.add_function("test_fn", fn_type, None);

    let entry_basic_block = compiler.context.append_basic_block(main_fn, "entry");
    compiler.builder.position_at_end(entry_basic_block);

    llvm(&compiler, &tree);

    compiler.builder.build_return(None);

    // let void_type = context.void_type();
    // let fn_type = void_type.fn_type(&[], false);

    // compiler.module.add_function("my_fn", fn_type, None);

    println!(
        "LLVM RL\n{}\n",
        &compiler.module.print_to_string().to_string()
    );
    // unsafe {
    //     compiler
    //         .execution_engine
    //         .get_function::<unsafe extern "C" fn()>("test_fn")
    //         .unwrap()
    //         .call();
    // }
    let verified = compiler.module.verify();
    if verified.is_err() {
        eprintln!(
            "Verification failed\n{}\n",
            verified.unwrap_err().to_string()
        );
        std::process::exit(1);
    }

    let path = Path::new("module.bc");
    compiler.module.write_bitcode_to_path(&path);

    Command::new("llc")
        .arg(path)
        .spawn()
        .expect("Failed to compile to ASM");
}

fn parse_file(file: &str) -> Tree {
    let pairs = RLParser::parse(Rule::Program, file);
    if pairs.is_err() {
        error(pairs.unwrap_err());
    }
    parse(pairs.unwrap())
}
