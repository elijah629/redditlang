use inkwell::values::BasicMetadataValueEnum;

use crate::parser::{Call, Term};

use super::Compiler;

pub trait Compile {
    fn compile(&self, compiler: &Compiler);
}

impl Compile for Call {
    fn compile(&self, compiler: &Compiler) {
        let function = match compiler.module.get_function(self.ident.0.as_str()) {
            Some(x) => x,
            None => panic!("Function `{}` not defined", self.ident.0),
        };

        if function.is_null() || function.is_undef() {
            panic!("Function `{}` is null or undefined", self.ident.0);
        }

        let args: Vec<BasicMetadataValueEnum> = self
            .args
            .iter()
            .map(|x| match x {
                Term::Number(x) => compiler
                    .context
                    .i128_type()
                    .const_int((*x).try_into().unwrap(), false)
                    .into(),
                Term::String(x) => compiler
                    .builder
                    .build_global_string_ptr(x, ".str")
                    .as_pointer_value()
                    .into(),
                Term::Ident(_) => {
                    // TODO: Variables, somehow
                    // compiler.builder
                    todo!()
                }
            })
            .collect();

        compiler
            .builder
            .build_call(function, args.as_slice(), "call");
    }
}
