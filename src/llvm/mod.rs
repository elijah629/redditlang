use inkwell::{basic_block::BasicBlock, builder::Builder, context::Context, module::Module};

use crate::parser::{Node, Tree};

use self::compile_node::Compile;

pub mod compile_node;

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
}

pub fn compile<'ctx>(compiler: &Compiler, tree: &Tree, basic_block: &BasicBlock) {
    for node in tree {
        llvm_one(&compiler, &node, &basic_block);
    }
}

pub fn llvm_one(compiler: &Compiler, node: &Node, basic_block: &BasicBlock) {
    match node {
        Node::Loop(r#loop) => r#loop.compile(compiler, basic_block),
        Node::Break(r#break) => todo!(), // Need to fix
        Node::Function(_) => todo!(),
        Node::Call(call) => call.compile(compiler, basic_block),
        Node::Throw(_) => todo!(),
        Node::Import(_) => todo!(),
        Node::Module(_) => todo!(),
        Node::TryCatch(_) => todo!(),
        Node::Variable(_) => todo!(),
        Node::Assignment(_) => todo!(),
        Node::If(_) => todo!(),
        Node::Class(_) => todo!(),
        Node::Return(_) => todo!(),
        Node::Expr(_) => todo!(),
    }
}
