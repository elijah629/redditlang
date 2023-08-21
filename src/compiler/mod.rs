use std::collections::HashMap;

use self::compile_node::{Compile, ValidType};
use crate::{
    bug,
    parser::{Node, Tree, Type}, utils::Result,
};
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, module::Module,
    values::PointerValue,
};

pub mod compile_node;
pub mod linking;

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
}

//#[derive(Clone)]
pub struct ScopeVariable<'a> {
    pub ptr: PointerValue<'a>,
    pub r#type: ValidType
}

//#[derive(Clone)]
pub struct Scope<'a> {
    pub variables: HashMap<String, ScopeVariable<'a>>,
}

pub struct CompileMetadata<'a> {
    pub basic_block: BasicBlock<'a>,
    pub function_scope: Scope<'a>,
}

pub fn compile<'a>(compiler: &Compiler<'a>, tree: &Tree, compile_meta: &mut CompileMetadata<'a>) -> Result<()> {
    for node in tree {
        if !matches!(node, Node::EOI) {
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
    match node {
        Node::Loop(r#loop) => todo!(),
        Node::Break(r#break) => todo!(),
        Node::Function(_) => todo!(),
        Node::Call(call) => todo!(),
        Node::Throw(_) => todo!(),
        Node::Import(_) => todo!(),
        Node::Module(_) => todo!(),
        Node::TryCatch(_) => todo!(),
        Node::Variable(variable) => variable.compile(compiler, compile_meta),
        Node::Assignment(assignment) => todo!(),
        Node::If(r#if) => todo!(),
        Node::Class(_) => todo!(),
        Node::Return(_) => todo!(),
        Node::Expr(_) => bug!("Expected statement, got an expression, COMPILE_EXPRESSION"),
        Node::EOI => unreachable!() // EOI will never get here
    }
}
