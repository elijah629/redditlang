use inkwell::{
    builder::Builder, context::Context, module::Module, passes::PassManager, values::FunctionValue,
};

use crate::parser::{Node, Tree};

use self::compile_node::Compile;

pub mod compile_node;

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub fpm: PassManager<FunctionValue<'ctx>>,
    pub module: Module<'ctx>,
}

pub fn llvm<'ctx>(compiler: &Compiler, tree: &Tree) {
    for node in tree {
        llvm_one(&compiler, node);
    }
}

pub fn llvm_one(compiler: &Compiler, node: &Node) {
    match node {
        Node::Loop(_) => todo!(),
        Node::Break(_) => todo!(),
        Node::Function(_) => todo!(),
        Node::Call(call) => call.compile(compiler),
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
