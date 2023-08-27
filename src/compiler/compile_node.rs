use inkwell::{types::BasicTypeEnum, values::BasicValueEnum, AddressSpace};

use crate::{
    compiler::ScopeVariable,
    parser::{Assignment, Break, Expr, Loop, Term, Type, Variable},
    utils::Result as ResultE,
};

use super::{compile, CompileMetadata, Compiler, LoopMetadata};

pub trait Compile<'a> {
    fn compile(
        &self,
        compiler: &Compiler<'a>,
        compile_meta: &mut CompileMetadata<'a>,
    ) -> ResultE<()>;
}

pub trait Compute<'a, T> {
    fn compute(&self, compiler: &Compiler<'a>, compile_meta: &CompileMetadata<'a>) -> ResultE<T>;
}

impl<'a> Compile<'a> for Variable {
    fn compile(
        &self,
        compiler: &Compiler<'a>,
        compile_meta: &mut CompileMetadata<'a>,
    ) -> ResultE<()> {
        fn var<'a>(
            x: BasicValueEnum<'a>,
            ident: &str,
            compiler: &Compiler<'a>,
            compile_meta: &mut CompileMetadata<'a>,
            r#type: ValidType,
        ) {
            let alloca = compiler.builder.build_alloca(x.get_type(), &ident);
            compiler.builder.build_store(alloca, x);

            compile_meta.function_scope.variables.insert(
                ident.to_string(),
                ScopeVariable {
                    ptr: alloca,
                    r#type,
                },
            ); // allows shadowing
        }

        let r#type = ValidType::try_from(&self.declaration.r#type)?;

        match &self.value {
            Expr::BinaryExpr(_) => todo!(),
            Expr::ConditionalExpr(_) => todo!(),
            Expr::IndexExpr(_) => todo!(),
            Expr::Term(term) => {
                // All terms, besides `Ident` are available at compile time.

                if !matches!(term, Term::Null | Term::Ident(..)) {
                    if !r#type.same_as_term(term)? {
                        return Err(
                            format!("Invalid type, got {term:?}, expected {:?}", r#type).into()
                        );
                    }
                }

                let ident = self.declaration.ident.0.as_str();

                match term {
                    Term::Number(n) => {
                        var(
                            compiler.context.f64_type().const_float(*n).into(),
                            ident,
                            &compiler,
                            compile_meta,
                            r#type,
                        );
                    }
                    Term::String(s) => {
                        var(
                            compiler
                                .builder
                                .build_global_string_ptr(s, ".str")
                                .as_pointer_value()
                                .into(),
                            ident,
                            &compiler,
                            compile_meta,
                            r#type,
                        );
                        //compiler.context.i64_type().const_int(x.len() as u64, false);
                    }
                    Term::Ident(variable) => {
                        let variable = compile_meta
                            .function_scope
                            .variables
                            .get(&variable.0)
                            .ok_or(format!("Use of undefined variable {}", variable.0))?;
                        if r#type != variable.r#type {
                            return Err(format!(
                                "Attempt to assign {:?} to variable of type {:?}",
                                variable.r#type, r#type
                            )
                            .into());
                        }
                        var(variable.ptr.into(), ident, &compiler, compile_meta, r#type);
                    }
                    Term::Boolean(b) => {
                        var(
                            compiler
                                .context
                                .bool_type()
                                .const_int((*b).into(), false)
                                .into(),
                            ident,
                            &compiler,
                            compile_meta,
                            r#type,
                        );
                    }
                    Term::Array(_) => {
                        todo!(); // recursion
                    }
                    Term::Null => {
                        let t = r#type.get_llvm_type(&compiler);
                        var(
                            t.const_zero().into(),
                            ident,
                            &compiler,
                            compile_meta,
                            r#type,
                        );
                    }
                }
            }
            Expr::CallExpr(_call) => {
                // compile call, store into x address, set variable to *x
                todo!();
            }
        }
        Ok(())
    }
}

impl<'a> Compile<'a> for Assignment {
    fn compile(
        &self,
        compiler: &Compiler<'a>,
        compile_meta: &mut CompileMetadata<'a>,
    ) -> ResultE<()> {
        let var = compile_meta
            .function_scope
            .variables
            .get(&self.ident.0)
            .ok_or::<Box<dyn std::error::Error>>(
                format!("Assignment to undefined variable {}", self.ident.0).into(),
            )?;

        let value = match &self.value {
            Expr::BinaryExpr(_) => todo!(),
            Expr::ConditionalExpr(_) => todo!(),
            Expr::IndexExpr(_) => todo!(),
            Expr::Term(term) => {
                // All terms, besides `Ident` are available at compile time.

                if !matches!(term, Term::Null | Term::Ident(..)) {
                    if !var.r#type.same_as_term(&term)? {
                        return Err(format!(
                            "Invalid type, got {term:?}, expected {:?}",
                            var.r#type
                        )
                        .into());
                    }
                }

                match term {
                    Term::Number(x) => compiler.context.f64_type().const_float(*x).into(),
                    Term::String(x) => {
                        let mut chars = x
                            .chars()
                            .map(|x| compiler.context.i8_type().const_int(x.into(), false))
                            .collect::<Vec<_>>();
                        chars.push(compiler.context.i8_type().const_int(0, false)); // null terminator

                        compiler
                            .context
                            .i8_type()
                            .const_array(chars.as_slice())
                            .into()
                    }
                    Term::Boolean(x) => compiler
                        .context
                        .bool_type()
                        .const_int((*x).into(), false)
                        .into(),
                    Term::Array(_) => todo!(),
                    Term::Null => var.r#type.get_llvm_type(compiler).const_zero(),
                    Term::Ident(x) => {
                        let variable = compile_meta
                            .function_scope
                            .variables
                            .get(&x.0)
                            .ok_or(format!("Use of undefined variable {}", x.0))?;
                        if var.r#type != variable.r#type {
                            return Err(format!(
                                "Attempt to assign {:?} to variable of type {:?}",
                                variable.r#type, var.r#type
                            )
                            .into());
                        }

                        variable.ptr.into()
                    }
                }
            }
            Expr::CallExpr(_) => todo!(),
        };

        compiler.builder.build_store(var.ptr, value);
        Ok(())
    }
}

impl<'a> Compile<'a> for Loop {
    fn compile(
        &self,
        compiler: &Compiler<'a>,
        compile_meta: &mut CompileMetadata<'a>,
    ) -> ResultE<()> {
        let fn_value = compile_meta.fn_value;
        let loop_block = compiler.context.append_basic_block(fn_value, "loop");
        let exit_block = compiler.context.append_basic_block(fn_value, "exit");

        // RT JMP start loop_block
        compiler.builder.build_unconditional_branch(loop_block);

        // COMP JMP end loop_block
        compiler.builder.position_at_end(loop_block);
        //        let (exit_block, break_values, value) = self.gen_loop_block_expr(body_expr, exit_block);
        //
        compile_meta.r#loop = Some(LoopMetadata {
            exit_block,
            loop_block,
        });

        compile(&compiler, &self.0, compile_meta)?;

        compile_meta.r#loop = None;

        compiler
            .builder
            .position_at_end(exit_block.get_previous_basic_block().unwrap());

        // RT JMP start loop_block
        compiler.builder.build_unconditional_branch(loop_block);

        // COMP JMP end exit_block
        compiler.builder.position_at_end(exit_block);

        // RT JMP start exit_block
        // compiler.builder.build_unconditional_branch(exit_block);

        //if value.is_some() {
        //    self.builder.build_unconditional_branch(loop_block);
        //}
        Ok(())
    }
}

impl<'a> Compile<'a> for Break {
    fn compile(
        &self,
        compiler: &Compiler<'a>,
        compile_meta: &mut CompileMetadata<'a>,
    ) -> ResultE<()> {
        let r#loop = compile_meta
            .r#loop
            .as_ref()
            .ok_or("Break used outside of a loop".to_string())?;
        compiler
            .builder
            .build_unconditional_branch(r#loop.exit_block);
        compiler
            .context
            .insert_basic_block_after(r#loop.loop_block, "_break_seperator");

        Ok(())
    }
}

/*
impl<'a> Compile<'a> for Return {
    fn compile(
        &self,
        compiler: &Compiler<'a>,
        compile_meta: &mut CompileMetadata<'a>,
    ) -> ResultE<()> {
        let value = match &self.0 {
            Expr::BinaryExpr(_) => todo!(),
            Expr::ConditionalExpr(_) => todo!(),
            Expr::IndexExpr(_) => todo!(),
            Expr::Term(term) => match term {
                Term::Number(_) => todo!(),
                Term::String(_) => todo!(),
                Term::Boolean(_) => todo!(),
                Term::Array(_) => todo!(),
                Term::Null => todo!(),
                Term::Ident(_) => todo!(),
            },
            Expr::CallExpr(_) => todo!(),
        };
        compiler.builder.build_return(Some(value));
    }
}

impl<'a> Compute<'a, BasicValueEnum<'a>> for Expr {
    fn compute(
        &self,
        compiler: &Compiler<'a>,
        compile_meta: &CompileMetadata<'a>,
    ) -> ResultE<BasicValueEnum<'a>> {
        match &self {
            Expr::BinaryExpr(_) => todo!(),
            Expr::ConditionalExpr(_) => todo!(),
            Expr::IndexExpr(_) => todo!(),
            Expr::Term(term) => {

            },
            Expr::CallExpr(_) => todo!(),
        }
    }
}
*/
/*
impl<'a> Compile<'a> for Call {
    fn compile(&self, compiler: &Compiler<'a>, compile_meta: &mut CompileMetadata<'a>) {
        let function = compiler
            .module
            .get_function(self.ident.0.as_str())
            .unwrap_or_else(|| error!("Use of undefined function `{}`", self.ident.0));

        if function.is_null() || function.is_undef() {
            error!("Function `{}` is null or undefined", self.ident.0);
        }

        let args: Vec<BasicMetadataValueEnum<'_>> = self
            .args
            .iter()
            /*.map(|x| x.compute(compiler, compile_meta).unwrap())
            .map(|z| match z {
                Value::Number(x) => x.into(),
                Value::Boolean(x) => x.into(),
                Value::String(x, _) => x.into(),
                Value::Array(x) => x.into(),
                Value::Null => todo!(),
            })*/
            .collect();

        compiler
            .builder
            .build_call(function, args.as_slice(), "return");
    }
}*/

/*impl<'a> Compile<'a> for Loop {
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

        // break, edit: no this does not fucking work. whuyyuyuyuyyyyyyyyyyyyyyyyy
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
        //     .filter(|x| x.is_some()) // collect on results can go to Result<Vec<_>>!!!! FIX THIS
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
}*/

/*fn to_boolean<'a>(compiler: &Compiler<'a>, value: Value<'a>) -> IntValue<'a> {
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
            IntPredicate::NE,
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
}*/

#[derive(Debug, PartialEq)]
pub enum ValidType {
    Number,
    Boolean,
    String,
    Array(Box<ValidType>), // Array is generic
                           // Null, // it is not a type, but a value that is any type
}

impl TryFrom<Type> for ValidType {
    type Error = String;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl<'a> TryFrom<&'a Type> for ValidType {
    type Error = String;

    fn try_from(value: &'a Type) -> Result<Self, Self::Error> {
        match value.root_type.0.as_str() {
            "Number" => Ok(Self::Number),
            "Boolean" => Ok(Self::Boolean),
            "String" => Ok(Self::String),
            "Array" => {
                let generic1 = &value.generics[0];
                let generic1 = ValidType::try_from(generic1)?;
                Ok(Self::Array(Box::from(generic1)))
            }
            "Null" => Err("Null is not a valid type, did you mean to use `wat`?".to_string()),
            _ => Err(format!("Invalid type, got {}", value.root_type.0)),
        }
    }
}

impl ValidType {
    pub fn same_as_term(&self, term: &Term) -> Result<bool, String> {
        match term {
            Term::Number(_) => {
                Ok(matches!(self, ValidType::Number))
            },
            Term::String(_) => {
                Ok(matches!(self, ValidType::String))
            },
            Term::Boolean(_) => {
                Ok(matches!(self, ValidType::Boolean))
            },
            Term::Array(_) => {
                Ok(matches!(self, ValidType::Array(..))) // TODO: check generic types
            },
            Term::Ident(_) => {
                Err("term is an Ident. It impossible to know thetype of an arbitrary identifier at compile time.".to_string())
            },
            Term::Null => {
                Err("Null is not a valid type.".to_string())
            },
        }
    }

    pub fn get_llvm_type<'a>(&self, compiler: &Compiler<'a>) -> BasicTypeEnum<'a> {
        match self {
            ValidType::Number => compiler.context.f64_type().into(),
            ValidType::Boolean => compiler.context.bool_type().into(),
            ValidType::String => compiler
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into(),
            ValidType::Array(x) => {
                let inner_type = x.get_llvm_type(&compiler);
                inner_type.into_pointer_type().into()
            }
        }
    }
}

/*impl<'a> Compute<'a, Value<'a>> for Expr {
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
            Expr::Null => Ok(Value::Null),
            Expr::CallExpr(_x) => todo!(),
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
                let ptr = compile_meta
                    .function_scope
                    .variables
                    .get(&x.0)
                    .unwrap_or_else(|| error!("Use of undefined variable {}", &x.0.bold()));

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

        fn var<'a>(
            x: BasicValueEnum<'a>,
            ident: &String,
            compiler: &Compiler<'a>,
            compile_meta: &mut CompileMetadata<'a>,
        ) {
            let alloca = compiler.builder.build_alloca(x.get_type(), &ident);
            compiler.builder.build_store(alloca, x);

            compile_meta
                .function_scope
                .variables
                .insert(ident.to_string(), alloca); // allows shadowing
        }

        let ident = &self.declaration.ident.0;

        match value {
            Value::Number(x) => var(x.into(), ident, &compiler, compile_meta),
            Value::Boolean(x) => var(x.into(), ident, &compiler, compile_meta),
            Value::String(x, _) => var(x.into(), ident, &compiler, compile_meta),
            Value::Array(x) => var(x.into(), ident, &compiler, compile_meta),
            Value::Null => todo!(),
        }
    }
}

impl<'a> Compile<'a> for Assignment {
    fn compile(&self, compiler: &Compiler<'a>, compile_meta: &mut CompileMetadata<'a>) {
        let ptr = *compile_meta
            .function_scope
            .variables
            .get(&self.ident.0)
            .unwrap_or_else(|| error!("Assignment to undefined variable {}", &self.ident.0.bold()));

        match self.value.compute(compiler, compile_meta).unwrap() {
            Value::Number(x) => {
                compiler.builder.build_store(ptr, x);
            }
            Value::Boolean(x) => {
                compiler.builder.build_store(ptr, x);
            }
            Value::String(x, _) => {
                compiler.builder.build_store(ptr, x);
            }
            Value::Array(x) => {
                compiler.builder.build_store(ptr, x);
            }
            Value::Null => {
                compiler
                    .builder
                    .build_store(ptr, ptr.get_type().const_null());
            }
        }
    }
}*/
