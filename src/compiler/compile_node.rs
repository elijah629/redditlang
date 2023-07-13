use inkwell::{
    basic_block::BasicBlock,
    values::{BasicMetadataValueEnum, FloatValue, IntValue},
    FloatPredicate,
};

use crate::{
    bug, error,
    parser::{Break, Call, Expr, IfBlock, Loop, MathOperator, Term},
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
            .map(|x| x.compute(compiler).unwrap())
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

        // break YES
        if loop_block.get_terminator().is_none() {
            compiler.builder.build_unconditional_branch(loop_block);
        }

        compiler.builder.position_at_end(exit_block);
    }
}

impl<'a> Compile<'a> for Break {
    fn compile(&self, compiler: &Compiler, return_block: &BasicBlock) {
        compiler.builder.build_unconditional_branch(*return_block);
        // compiler.builder.position_at_end(*return_block);
    }
}

impl<'a> Compile<'a> for IfBlock {
    fn compile(&self, _compiler: &Compiler<'a>, _basic_block: &BasicBlock<'a>) {
        todo!();
        // let function = basic_block.get_parent().unwrap();
        // let after_block = compiler.context.append_basic_block(function, "if_after");
        // let mut else_block = after_block;

        // let cases = self
        //     .if_nodes
        //     .iter()
        //     .map(|x| match x {
        //         IfNode::Case(x) => {
        //             let condition = to_boolean
        // (&compiler, x.expr.compute(&compiler).unwrap());

        //             compiler.builder.position_at_end(*basic_block);
        //             let alloca = compiler
        //                 .builder
        //                 .build_alloca(condition.get_type(), "if_cond");
        //             compiler.builder.build_store(alloca, condition);
        //             let value =
        //                 compiler
        //                     .builder
        //                     .build_ptr_to_int(alloca, condition.get_type(), "read");

        //             let block = compiler.context.append_basic_block(function, "if");
        //             compiler.builder.position_at_end(block);

        //             compile(&compiler, &x.body, &block);

        //             compiler.builder.position_at_end(block);
        //             compiler.builder.build_unconditional_branch(after_block);

        //             Some((value, block))
        //         }
        //         IfNode::Else(x) => {
        //             let block = compiler.context.append_basic_block(function, "else");
        //             else_block = block;
        //             compiler.builder.position_at_end(block);

        //             compile(&compiler, &x.body, &basic_block);

        //             compiler.builder.position_at_end(block);
        //             compiler.builder.build_unconditional_branch(after_block);

        //             None
        //         }
        //     })
        //     .filter(|x| x.is_some())
        //     .map(|x| x.unwrap())
        //     .collect::<Vec<_>>();

        // // Will run the first block with a true value
        // compiler.builder.position_at_end(*basic_block);
        // compiler.builder.build_switch(
        //     compiler.context.bool_type().const_all_ones(),
        //     else_block,
        //     &cases,
        // );
        // compiler.builder.position_at_end(after_block);
    }
}

fn to_boolean<'a>(compiler: &Compiler<'a>, expr_value: ExprValue<'a>) -> IntValue<'a> {
    let float = |x: FloatValue<'a>| {
        compiler.builder.build_float_compare(
            FloatPredicate::ONE,
            x,
            x.get_type().const_zero(),
            "expr_truthy",
        )
    };

    let int = |x: IntValue<'a>| {
        compiler.builder.build_int_compare(
            inkwell::IntPredicate::NE,
            x,
            x.get_type().const_zero(),
            "expr_truthy",
        )
    };

    match expr_value {
        ExprValue::BinaryExpr(x) => float(x),
        ExprValue::ConditionalExpr(x) => x, // Should already be a 0 or a 1
        ExprValue::IndexExpr => todo!(), // Get value and call truthy again ( should be able to do some pointer stuff )
        ExprValue::Term(x) => match x {
            BasicMetadataValueEnum::ArrayValue(_) => todo!(),
            BasicMetadataValueEnum::IntValue(x) => int(x),
            BasicMetadataValueEnum::FloatValue(x) => float(x),
            BasicMetadataValueEnum::PointerValue(_) => todo!(),
            BasicMetadataValueEnum::StructValue(_) => todo!(),
            BasicMetadataValueEnum::VectorValue(_) => todo!(),
            BasicMetadataValueEnum::MetadataValue(_) => todo!(),
        },
        ExprValue::Null => compiler.context.bool_type().const_zero(),
    }
}

#[derive(Debug)]
pub enum ExprValue<'a> {
    BinaryExpr(FloatValue<'a>),
    /// Bit-width of 1 ( boolean )
    ConditionalExpr(IntValue<'a>),
    Term(BasicMetadataValueEnum<'a>),
    IndexExpr, // TODO
    Null,      // TODO
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
            Expr::Term(x) => Ok(ExprValue::Term(x.compute(compiler)?)),
            Expr::Null => todo!(),
        }
    }
}

impl<'a> Compute<'a, BasicMetadataValueEnum<'a>> for Term {
    fn compute(
        &self,
        compiler: &Compiler<'a>,
    ) -> Result<BasicMetadataValueEnum<'a>, Box<dyn std::error::Error>> {
        Ok(match self {
            Term::Number(x) => compiler.context.f64_type().const_float(*x).into(),
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
    }
}
