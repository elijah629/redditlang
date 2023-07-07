use inkwell::{basic_block::BasicBlock, values::BasicMetadataValueEnum};

use crate::parser::{Break, Call, Loop, Term};

use super::{compile, Compiler};

pub trait Compile {
    fn compile(&self, compiler: &Compiler, basic_block: &BasicBlock);
}

impl Compile for Call {
    fn compile(&self, compiler: &Compiler, _basic_block: &BasicBlock) {
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

impl Compile for Loop {
    fn compile(&self, compiler: &Compiler, start_block: &BasicBlock) {
        let function = start_block.get_parent().unwrap();
        let loop_block = compiler.context.append_basic_block(function, "loop");
        let exit_block = compiler.context.append_basic_block(function, "exit");

        compiler.builder.build_unconditional_branch(loop_block);

        compiler.builder.position_at_end(loop_block);
        compile(&compiler, &self.body, &exit_block);

        compiler.builder.build_unconditional_branch(loop_block);
        compiler.builder.position_at_end(exit_block);

        // TODO: no break just remove loop and add goto keyword ( overyonder )
    }
}

// impl Compile for Break {
//     fn compile(&self, compiler: &Compiler, return_block: &BasicBlock) {
//         compiler.builder.build_unconditional_branch(*return_block);
//         // compiler.builder.position_at_end(*return_block);
//     }
// }
