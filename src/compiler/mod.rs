use std::collections::HashMap;

use self::compile_node::{Compile, ValidType};
use crate::{
    bug,
    parser::{Node, Tree},
    utils::Result,
};
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    values::{FunctionValue, PointerValue},
};

pub mod compile_node;
pub mod linking;

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub builder: &'ctx Builder<'ctx>,
    pub module: Module<'ctx>,
}

//#[derive(Clone)]
pub struct ScopeVariable<'a> {
    pub ptr: PointerValue<'a>,
    pub r#type: ValidType,
}

//#[derive(Clone)]
pub struct Scope<'a> {
    pub variables: HashMap<String, ScopeVariable<'a>>,
}

pub struct LoopMetadata<'a> {
    exit_block: BasicBlock<'a>,
    loop_block: BasicBlock<'a>,
}

pub struct CompileMetadata<'a> {
    pub r#loop: Option<LoopMetadata<'a>>,
    pub function_scope: Scope<'a>,
    pub fn_value: FunctionValue<'a>,
}

pub fn compile<'a>(
    compiler: &Compiler<'a>,
    tree: &Tree,
    compile_meta: &mut CompileMetadata<'a>,
) -> Result<()> {
    for node in tree {
        // these cannot be compiled
        if !matches!(node, Node::EOI | Node::Import(..)) {
            compile_one(&compiler, &node, compile_meta)?;
        }
    }
    Ok(())
}

pub fn compile_one<'a>(
    compiler: &Compiler<'a>,
    node: &Node,
    compile_meta: &mut CompileMetadata<'a>,
) -> Result<()> {
    let impl_compile: &dyn Compile<'a> = match node {
        Node::EOI => unreachable!(), // EOI is skipped above
        Node::Expr(_) => bug!("Expected statement, got an expression, COMPILE_EXPRESSION"),

        Node::Import(_) => unreachable!(), // import is a compiler directive

        Node::Variable(x) => x,
        Node::Assignment(x) => x,

        Node::Loop(x) => x,
        Node::Break(x) => x,

        Node::Function(_) => todo!(),
        Node::Call(call) => todo!(),
        Node::Throw(_) => todo!(),
        Node::TryCatch(_) => todo!(),
        Node::If(r#if) => todo!(),
        Node::Class(_) => todo!(),
        Node::Return(_) => todo!(),
    };

    impl_compile.compile(compiler, compile_meta)
}
