use pest::{error::Error, Parser};
use pest_derive::Parser;
use std::{collections::HashSet, hash::Hash};

use crate::error_formatting::format_error;

pub mod error_formatting;
#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct RLParser;

enum Term {
    Number(i64),
    String(String),
}
#[derive(Debug)]
struct Type {
    pub ident: String,
    pub is_array: bool,
}

#[derive(Debug)]
struct Declaration {
    pub ident: String,
    pub r#type: Option<Type>,
}

#[derive(Debug)]
struct Function<'a> {
    modifiers: Vec<String>,
    declaration: Declaration,
    args: Vec<Declaration>,
    body: pest::iterators::Pair<'a, Rule>,
}

fn main() {
    let pairs = RLParser::parse(
        Rule::Program,
        r#"
debug bar callmeonmycellphone split damn String[](x damn String,) {}
"#,
    );
    if pairs.is_err() {
        let error: pest::error::Error<Rule> = pairs.unwrap_err();
        println!("{}", format_error(error));
        return;
    }
    let pairs = pairs.unwrap();
    for pair in pairs {
        match pair.as_rule() {
            Rule::Statement => {
                for statement in pair.into_inner() {
                    match statement.as_rule() {
                        Rule::Module => {
                            let mut inner = statement.into_inner();
                            let module_name = inner.nth(0).expect("No module name").as_str();
                            println!("{}", module_name);
                        }
                        Rule::Call => {
                            let mut inner = statement.into_inner();
                            let fn_name = inner.nth(0).expect("No function name").as_str();
                            let fn_args = inner.nth(0).unwrap().into_inner();
                            let fn_args: Vec<Term> =
                                fn_args.map(expr_value).map(|r| r.unwrap()).collect();
                            match fn_name {
                                "coitusinterruptus" => {
                                    let message = &fn_args[0];
                                    match message {
                                        Term::String(s) => println!("{}", s),
                                        _ => panic!("Invalid argument type, expected String"),
                                    }
                                }
                                _ => {
                                    // Look for defined function in some buffer, if it exists call with fn_args
                                }
                            }
                        }
                        Rule::Function => {
                            let mut inner = statement.into_inner();
                            let modifiers: Vec<String> = inner
                                .nth(0)
                                .unwrap()
                                .into_inner()
                                .map(|modifier| modifier.as_str().trim_end().to_string())
                                .collect();

                            let declaration = declaration_parse(inner.nth(0).unwrap());

                            let raw_args = inner.nth(0).unwrap();
                            let args: Vec<Declaration> = raw_args
                                .clone()
                                .into_inner()
                                .map(|x| declaration_parse(x))
                                .collect();

                            // Check for duplicate argument idents
                            let has_duplicates = !is_unique(args.iter().map(|x| x.ident.clone()));
                            if has_duplicates {
                                error(Error::new_from_pos(
                                    pest::error::ErrorVariant::CustomError {
                                        message: "Duplicate arguments".to_owned(),
                                    },
                                    raw_args.as_span().start_pos(),
                                ))
                            }
                            let body = inner.nth(0).unwrap();
                            println!(
                                "{:#?}",
                                Function {
                                    modifiers,
                                    declaration,
                                    args,
                                    body
                                }
                            );
                        }
                        _ => {}
                    }
                }
            }
            Rule::EOI => {
                // Should execute the code here, all other steps convert to JIT or bytecode
            }
            _ => {}
        }
        // match pair.as_rule() {
        //     Rule::EOI => todo!(),
        //     Rule::AccMod => todo!(),
        //     Rule::Loop => todo!(),
        //     Rule::Break => todo!(),
        //     Rule::Function => todo!(),
        //     Rule::Return => todo!(),
        //     Rule::Ident => todo!(),
        //     Rule::IfBlock => todo!(),
        //     Rule::Call => todo!(),
        //     Rule::ConditionalExpr => todo!(),
        //     Rule::BinaryExpr => todo!(),
        //     Rule::IndexingExpr => todo!(),
        //     Rule::Expr => todo!(),
        //     Rule::Term => todo!(),
        //     Rule::TypeDef => todo!(),
        //     Rule::Type => todo!(),
        //     Rule::Throw => todo!(),
        //     Rule::TryCatch => todo!(),
        //     Rule::Module => todo!(),
        //     Rule::Import => todo!(),
        //     Rule::Variable => todo!(),
        //     Rule::AssignmentStatement => todo!(),
        //     Rule::Equality => todo!(),
        //     Rule::Add => todo!(),
        //     Rule::Subtract => todo!(),
        //     Rule::Multiply => todo!(),
        //     Rule::Divide => todo!(),
        //     Rule::XOR => todo!(),
        //     Rule::Assignment => todo!(),
        //     Rule::UnaryOperator => todo!(),
        //     Rule::ConditionalOperator => todo!(),
        //     Rule::MathOperator => todo!(),
        //     Rule::Class => todo!(),
        //     Rule::String => todo!(),
        //     Rule::UInt => todo!(),
        //     Rule::Int => todo!(),
        //     Rule::UDecimal => todo!(),
        //     Rule::Decimal => todo!(),
        //     Rule::Number => todo!(),
        //     Rule::UNumber => todo!(),
        //     Rule::Block => todo!(),
        //     _ => {}
        // }
    }
    // println!("{:#?}", pairs);
}

fn expr_value(arg: pest::iterators::Pair<'_, Rule>) -> Result<Term, &str> {
    let value = arg.into_inner().nth(0).unwrap();
    match value.as_rule() {
        Rule::String => Ok(Term::String(
            enquote::unquote(value.as_str()).unwrap().to_string(),
        )),
        Rule::Number => Ok(Term::Number(value.as_str().parse().unwrap())),
        _ => Err("Invalid expression content"),
    }
}

fn declaration_parse(declaration: pest::iterators::Pair<'_, Rule>) -> Declaration {
    let mut declaration = declaration.into_inner();
    let ident = declaration.nth(0).unwrap().as_str().to_string();
    let r#type = declaration
        .nth(0)
        .map(|x| x.into_inner())
        .map(|mut x| Type {
            ident: x.nth(0).unwrap().as_str().to_string(),
            is_array: x.nth(0).is_some(),
        });

    Declaration { ident, r#type }
}

fn is_unique<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn error(error: pest::error::Error<Rule>) -> ! {
    println!("{}", format_error(error));
    panic!();
}
