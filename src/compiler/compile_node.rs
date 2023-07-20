use inkwell::{
    values::{
        ArrayValue, BasicMetadataValueEnum, BasicValueEnum, FloatValue, IntValue, PointerValue,
    },
    FloatPredicate,
};

use crate::{
    bug, error,
    parser::{Break, Call, Expr, IfBlock, Loop, MathOperator, Term, Variable},
};

use super::{compile, CompileMetadata, Compiler};

pub trait Compile<'a> {
    fn compile(&self, compiler: &Compiler<'a>, compile_meta: &mut CompileMetadata<'a>);
}

pub trait Compute<'a, T> {
    fn compute(
        &self,
        compiler: &Compiler<'a>,
        compile_meta: &CompileMetadata<'a>,
    ) -> Result<T, Box<dyn std::error::Error>>;
}

impl<'a> Compile<'a> for Call {
    fn compile(&self, compiler: &Compiler<'a>, compile_meta: &mut CompileMetadata<'a>) {
        let function = match compiler.module.get_function(self.ident.0.as_str()) {
            Some(x) => x,
            None => error!("Function `{}` not defined", self.ident.0),
        };

        if function.is_null() || function.is_undef() {
            error!("Function `{}` is null or undefined", self.ident.0);
        }

        let args: Vec<BasicMetadataValueEnum<'_>> = self
            .args
            .iter()
            .map(|x| x.compute(compiler, compile_meta).unwrap())
            .map(|z| match z {
                Value::Number(x) => x.into(),
                Value::Boolean(x) => x.into(),
                Value::String(x, _) => x.into(),
                Value::Array(x) => x.into(),
                Value::Null => todo!(),
            })
            .collect();

        compiler
            .builder
            .build_call(function, args.as_slice(), "call");
    }
}

impl<'a> Compile<'a> for Loop {
    fn compile(&self, compiler: &Compiler<'a>, compile_meta: &mut CompileMetadata<'a>) {
        let function = compile_meta.basic_block.get_parent().unwrap();
        let loop_block = compiler.context.append_basic_block(function, "loop");
        let exit_block = compiler.context.append_basic_block(function, "exit");

        compiler.builder.build_unconditional_branch(loop_block);

        compiler.builder.position_at_end(loop_block);
        compile(
            &compiler,
            &self.body,
            &mut CompileMetadata {
                basic_block: exit_block,
                function_scope: compile_meta.function_scope.clone(),
            },
        );

        // break YES
        if loop_block.get_terminator().is_none() {
            compiler.builder.build_unconditional_branch(loop_block);
        }

        compiler.builder.position_at_end(exit_block);
    }
}

impl<'a> Compile<'a> for Break {
    fn compile(&self, compiler: &Compiler, compile_meta: &mut CompileMetadata<'_>) {
        compiler
            .builder
            .build_unconditional_branch(compile_meta.basic_block);
        // compiler.builder.position_at_end(*return_block);
    }
}

impl<'a> Compile<'a> for IfBlock {
    fn compile(&self, _compiler: &Compiler<'a>, _compile_meta: &mut CompileMetadata<'_>) {
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

fn to_boolean<'a>(compiler: &Compiler<'a>, value: Value<'a>) -> IntValue<'a> {
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

    match value {
        Value::Number(x) => float(x),
        Value::Boolean(x) => x, // Is already a 0 or a 1
        Value::Null => compiler.context.bool_type().const_zero(),
        Value::String(_ptr, len) => int(len),
        Value::Array(_) => todo!(),
    }
}

#[derive(Debug)]
pub enum Value<'a> {
    Number(FloatValue<'a>),
    /// Bit-width of 1
    Boolean(IntValue<'a>),
    String(PointerValue<'a>, IntValue<'a>), //  ptr, length
    Array(ArrayValue<'a>),
    Null,
}

impl<'a> Compute<'a, Value<'a>> for Expr {
    fn compute(
        &self,
        compiler: &Compiler<'a>,
        compile_meta: &CompileMetadata<'a>,
    ) -> Result<Value<'a>, Box<dyn std::error::Error>> {
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
                        MathOperator::Modulus => compiler
                            .builder
                            .build_float_rem(result, operand, "expr_mod"),
                    };
                }

                Ok(Value::Number(result))
            }
            Expr::ConditionalExpr(_) => todo!(),
            Expr::IndexExpr(_) => todo!(),
            Expr::Term(x) => Ok(x.compute(compiler, compile_meta)?),
            Expr::Null => todo!(),
        }
    }
}

impl<'a> Compute<'a, Value<'a>> for Term {
    fn compute(
        &self,
        compiler: &Compiler<'a>,
        compile_meta: &CompileMetadata<'a>,
    ) -> Result<Value<'a>, Box<dyn std::error::Error>> {
        Ok(match self {
            Term::Number(x) => Value::Number(compiler.context.f64_type().const_float(*x)),
            Term::String(x) => Value::String(
                compiler
                    .builder
                    .build_global_string_ptr(x, ".str")
                    .as_pointer_value(),
                compiler.context.i64_type().const_int(x.len() as u64, false),
            ),
            Term::Ident(x) => {
                let ptr = compile_meta.function_scope.variables.get(&x.0).unwrap();

                let loaded = compiler.builder.build_load(
                    ptr.get_type(),
                    *ptr,
                    format!("{}_access", x.0).as_str(),
                );

                match loaded {
                    BasicValueEnum::FloatValue(x) => Value::Number(x),
                    BasicValueEnum::IntValue(x) => Value::Boolean(x),
                    BasicValueEnum::PointerValue(x) => {
                        Value::String(x, compiler.context.i64_type().const_int(3, false))
                    }
                    BasicValueEnum::ArrayValue(x) => Value::Array(x),
                    _ => bug!("UNKNOWN_VAR_TYPE({:?})", loaded),
                }
            }
        })
    }
}

impl<'a> Compile<'a> for Variable {
    fn compile(&self, compiler: &Compiler<'a>, compile_meta: &mut CompileMetadata<'a>) {
        let value = self.value.compute(compiler, compile_meta).unwrap();
        match value {
            Value::Number(x) => {
                let alloca = compiler
                    .builder
                    .build_alloca(x.get_type(), self.declaration.ident.0.as_str());
                compiler.builder.build_store(alloca, x);
            }
            Value::Boolean(x) => {
                let alloca = compiler
                    .builder
                    .build_alloca(x.get_type(), self.declaration.ident.0.as_str());
                compiler.builder.build_store(alloca, x);
            }
            Value::String(x, _) => {
                let ident = &self.declaration.ident.0;

                let alloca = compiler.builder.build_alloca(x.get_type(), &ident);
                compiler.builder.build_store(alloca, x);

                compile_meta
                    .function_scope
                    .variables
                    .insert(ident.to_string(), alloca); // allows shadowing
            }
            Value::Array(x) => {
                let alloca = compiler
                    .builder
                    .build_alloca(x.get_type(), self.declaration.ident.0.as_str());
                compiler.builder.build_store(alloca, x);
            }
            Value::Null => todo!(), // TODO: Nullptr
        }
    }
}
