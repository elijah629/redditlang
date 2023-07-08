use inkwell::{
    basic_block::BasicBlock,
    types::IntType,
    values::{BasicMetadataValueEnum, FloatValue},
};

use crate::{
    bug, error,
    parser::{Call, Expr, IfBlock, IfNode, Loop, MathOperator, Term, Variable},
};

use super::{compile, Compiler};

pub trait Compile<'a> {
    fn compile(&self, compiler: &Compiler<'a>, basic_block: &BasicBlock<'a>);
}

pub trait Compute<'a, T> {
    fn compute(&self, compiler: &Compiler<'a>) -> Result<T, Box<dyn std::error::Error>>;
}

impl Compile<'_> for Call {
    fn compile(&self, compiler: &Compiler, _basic_block: &BasicBlock) {
        let function = match compiler.module.get_function(self.ident.0.as_str()) {
            Some(x) => x,
            None => error!("Function `{}` not defined", self.ident.0),
        };

        if function.is_null() || function.is_undef() {
            error!("Function `{}` is null or undefined", self.ident.0);
        }

        let args: Vec<BasicMetadataValueEnum> = self
            .args
            .iter()
            .map(|x| match x {
                Term::Number(x) => compiler
                    .context
                    .f64_type()
                    .const_float((*x).try_into().unwrap())
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

impl<'a> Compile<'a> for Loop {
    fn compile(&self, compiler: &Compiler<'a>, start_block: &BasicBlock<'a>) {
        let function = start_block.get_parent().unwrap();
        let loop_block = compiler.context.append_basic_block(function, "loop");
        let exit_block = compiler.context.append_basic_block(function, "exit");

        compiler.builder.build_unconditional_branch(loop_block);

        compiler.builder.position_at_end(loop_block);
        compile(&compiler, &self.body, &exit_block);

        compiler.builder.build_unconditional_branch(loop_block);
        compiler.builder.position_at_end(exit_block);

        // TODO::proposal: no break just remove loop and add goto keyword ( overyonder )
    }
}

// impl Compile for Break {
//     fn compile(&self, compiler: &Compiler, return_block: &BasicBlock) {
//         compiler.builder.build_unconditional_branch(*return_block);
//         // compiler.builder.position_at_end(*return_block);
//     }
// }

impl<'a> Compile<'a> for IfBlock {
    fn compile(&self, compiler: &Compiler<'a>, basic_block: &BasicBlock<'a>) {
        let nodes = &self.if_nodes;

        // for node in nodes {
        //     match node {
        //         IfNode::If(x) => {
        //             // x.
        //         }
        //         IfNode::ElseIf(x) => todo!(),
        //         IfNode::Else(x) => todo!(),
        //     }
        // }

        if let IfNode::If(x) = &nodes[0] {
            println!("{:?}", x.expr.compute(compiler));
        }
        // todo!()
    }
}

#[derive(Debug)]
pub enum ExprValue<'a> {
    BinaryExpr(FloatValue<'a>),
    /// Bit-width of 1 ( boolean )
    ConditionalExpr(IntType<'a>),
    IndexExpr,
    Term,
    Null,
}

impl<'a> Compute<'a, ExprValue<'a>> for Expr {
    fn compute(
        &self,
        compiler: &Compiler<'a>,
    ) -> Result<ExprValue<'a>, Box<dyn std::error::Error>> {
        match self {
            Expr::BinaryExpr(x) => {
                let mut result = compiler.context.f64_type().const_zero();
                for term in &x.terms {
                    let operator = term.operator.as_ref().unwrap_or(&MathOperator::Add);
                    let operand = compiler.context.f64_type().const_float(match term.operand {
                        Term::Number(x) => x,
                        _ => bug!("INVALID_OPERAND({:?})", term.operand),
                    });

                    result = match operator {
                        MathOperator::Add => compiler
                            .builder
                            .build_float_add(result, operand, "expr_add"),
                        MathOperator::Subtract => compiler
                            .builder
                            .build_float_sub(result, operand, "expr_sub"),
                        MathOperator::Multiply => compiler
                            .builder
                            .build_float_mul(result, operand, "expr_mul"),
                        MathOperator::Divide => compiler
                            .builder
                            .build_float_div(result, operand, "expr_div"),
                        MathOperator::XOR => {
                            let result_flt =
                                result.const_to_signed_int(compiler.context.i64_type());
                            let operand_flt =
                                operand.const_to_signed_int(compiler.context.i64_type());

                            result_flt
                                .const_xor(operand_flt)
                                .const_signed_to_float(compiler.context.f64_type())
                        }
                    };
                }

                Ok(ExprValue::BinaryExpr(result))
            }
            Expr::ConditionalExpr(_) => todo!(),
            Expr::IndexExpr(_) => todo!(),
            Expr::Term(_) => todo!(),
            Expr::Null => todo!(),
        }
    }
}

impl Compile<'_> for Variable {
    fn compile(&self, compiler: &Compiler, basic_block: &BasicBlock) {
        // let ptr = compiler
        //     .builder
        //     .build_alloca(compiler.context.f64_type(), &self.declaration.ident.0);
        // compiler.builder.build_store(ptr, self.value);
    }
}
