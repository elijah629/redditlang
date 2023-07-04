use crate::parser::{Node, Term, Tree};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    values::{BasicMetadataValueEnum, FunctionValue},
};

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
        Node::Call(call) => {
            let function = compiler.module.get_function(call.ident.0.as_str());
            if function.is_none() {
                panic!("Function `{}` not defined", call.ident.0);
            }
            let function = function.unwrap();
            if function.is_null() || function.is_undef() {
                panic!("Function `{}` is null or undefined", call.ident.0);
            }
            let args: Vec<BasicMetadataValueEnum> = call
                .args
                .iter()
                .map(|x| match x {
                    Term::Number(_) => todo!(),
                    Term::String(x) => compiler
                        .builder
                        .build_global_string_ptr(x, ".str")
                        .as_pointer_value()
                        .into(),
                    Term::Ident(_) => todo!(),
                })
                .collect();

            compiler
                .builder
                .build_call(function, args.as_slice(), "call");
        }
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
