use inkwell::{basic_block::BasicBlock, builder::Builder, context::Context, module::Module};

use crate::{
    bug,
    parser::{Node, Tree},
};

use self::compile_node::Compile;

pub mod compile_node;

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
}

pub fn compile<'a>(compiler: &Compiler<'a>, tree: &Tree, basic_block: &BasicBlock<'a>) {
    for node in tree {
        llvm_one(&compiler, &node, &basic_block);
    }
}

pub fn llvm_one<'a>(compiler: &Compiler<'a>, node: &Node, basic_block: &BasicBlock<'a>) {
    match node {
        Node::Loop(r#loop) => r#loop.compile(compiler, basic_block),
        Node::Break(_break) => todo!(), // Need to fix,                                                   but won't
        Node::Function(_) => todo!(),
        Node::Call(call) => call.compile(compiler, basic_block),
        Node::Throw(_) => todo!(),
        Node::Import(_) => todo!(),
        Node::Module(_) => todo!(),
        Node::TryCatch(_) => todo!(),
        Node::Variable(_) => todo!(),
        Node::Assignment(_) => todo!(),
        Node::If(r#if) => r#if.compile(compiler, basic_block),
        Node::Class(_) => todo!(),
        Node::Return(_) => todo!(),
        Node::Expr(_) => bug!("EXPR_IS_STATEMENT_COMPILER"),
    }
}
