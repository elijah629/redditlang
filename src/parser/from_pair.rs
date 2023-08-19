use crate::errors::syntax_error;
use crate::parser::{
    parse, parse_one, Assignment, BinaryExpr, BinaryExprTerm, Break, Call, Catch, Class,
    ConditionExprTerm, ConditionalExpr, ConditionalOperator, Declaration, Else, Expr, Function,
    FunctionMod, Ident, IfBlock, IfCase, IfNode, Import, Index, IndexExpr, Loop, MathOperator,
    Module, Node, Number, Return, Term, Throw, Tree, Try, TryCatch, Type, Variable, VariableMod,
};
use crate::utils::{is_unique, Result};
use crate::{bug, Rule};
use pest::error::Error;
use pest::iterators::Pair;

pub trait Parse {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self>
    where
        Self: Sized;
}

impl Parse for Type {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();

       // let inner_vec = inner.collect::<Vec<_>>();
        let len = inner.len() - 1;
       let generics = inner.clone().take(len).map(|x| {
            match x.as_rule() {
               Rule::Ident => {
                   Type {
                       generics: vec![],
                       root_type: Ident::parse_from(x).unwrap()
                   } 
               },
               Rule::Type => {
                  Type::parse_from(x).unwrap()
               }
               _ => bug!("UNEXPECTED_TYPE_RULE({})", x) 
            }
        }).collect::<Vec<_>>();

       let root_type = Ident::parse_from(inner.next().unwrap())?;

       Ok(Self {
           generics,
           root_type
       })
    }
}

impl Parse for Declaration {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner(); 
        let ident = Ident::parse_from(inner.next().unwrap())?; 

        let r#type = Type::parse_from(inner.next().unwrap())?;
        
        Ok(Self { ident, r#type })
    }
}

impl Parse for Function {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let start_pos = pair.as_span().start_pos();
        let mut inner = pair.into_inner();
        let modifiers: Vec<FunctionMod> = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|modifier| modifier.as_str().trim_end().to_string())
            .map(|modifier| match modifier.as_str() {
                "debug" => FunctionMod::Debug,
                "bar" => FunctionMod::Public,
                _ => syntax_error(Error::new_from_pos(
                    pest::error::ErrorVariant::CustomError {
                        message: "Invalid modifier".to_owned(),
                    },
                    start_pos,
                )),
            })
            .collect();

        let declaration = Declaration::parse_from(inner.next().unwrap()).unwrap();

        let raw_args = inner.next().unwrap();
        let start_pos = raw_args.as_span().start_pos();
        let args: Vec<Declaration> = raw_args
            .into_inner()
            .map(|x| Declaration::parse_from(x).unwrap())
            .collect();

        let has_duplicates = !is_unique(args.iter().map(|x| &x.ident.0));
        if has_duplicates {
            syntax_error(Error::new_from_pos(
                pest::error::ErrorVariant::CustomError {
                    message: "Duplicate arguments".to_owned(),
                },
                start_pos,
            ))
        }
        let body = Tree::parse_from(inner.next().unwrap()).unwrap();
        Ok(Self {
            modifiers,
            declaration,
            args,
            body,
        })
    }
}

impl Parse for Term {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        match pair.as_rule() {
            Rule::String => Ok(Self::String(
                enquote::unquote(pair.as_str()).unwrap().to_string(),
            )),
            Rule::Number => {
                let mut inner = pair.into_inner();
                let has_sign = inner.len() == 2;
                let sign = if has_sign { inner.next() } else { None };
                let is_negative = sign
                    .map(|x| match x.as_rule() {
                        Rule::Add => false,
                        Rule::Subtract => true,
                        _ => bug!("INVALID_SIGN({:?})", x.as_rule()),
                    })
                    .unwrap_or(false);

                let magnitude: Number = inner.next().unwrap().as_str().parse().unwrap();
                let value = if is_negative { -magnitude } else { magnitude };
                Ok(Self::Number(value))
            }
            Rule::Ident => Ok(Self::Ident(Ident::parse_from(pair).unwrap())),
            Rule::Expr => todo!(), // TODO: Wrapped Expr Expr(Expr)
            _ => Err("INVALID_RULE".into()),
        }
    }
}

impl Parse for Module {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap()).unwrap();
        Ok(Self { ident })
    }
}

impl Parse for Call {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap()).unwrap();
        let args = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|x| Expr::parse_from(x).unwrap())
            .collect();
        Ok(Self { ident, args })
    }
}

impl Parse for Break {
    fn parse_from(_pair: Pair<'_, Rule>) -> Result<Self> {
        Ok(Break)
    }
}

impl Parse for Throw {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let value = Expr::parse_from(inner.next().unwrap())?;
        Ok(Self { value })
    }
}

impl Parse for Import {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let path = Term::parse_from(inner.next().unwrap())?;
        Ok(Self { path })
    }
}

impl Parse for Loop {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        Ok(Self {
            body: Tree::parse_from(inner.next().unwrap())?,
        })
    }
}

impl Parse for TryCatch {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();

        let r#try = Try(Tree::parse_from(inner.next().unwrap())?);
        let mut catch = inner.next().unwrap().into_inner();

        let first = catch.next().unwrap();
        let catch = match first.as_rule() {
            Rule::Block => Catch(None, Tree::parse_from(first)?),
            Rule::Ident => Catch(
                Ident::parse_from(first).ok(),
                Tree::parse_from(catch.next().unwrap())?,
            ),
            _ => bug!("CATCH_NOT_BLOCK_OR_IDENT({:?})", first.as_rule()),
        };
        Ok(TryCatch { r#try, catch })
    }
}

impl Parse for Variable {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let start_pos = pair.as_span().start_pos();
        let mut inner = pair.into_inner();
        let modifiers: Vec<VariableMod> = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|modifier| modifier.as_str().trim_end().to_string())
            .map(|modifier| match modifier.as_str() {
                "bar" => VariableMod::Public,
                _ => syntax_error(Error::new_from_pos(
                    pest::error::ErrorVariant::CustomError {
                        message: "Invalid modifier".to_owned(),
                    },
                    start_pos,
                    )),
            })
            .collect();
        let declaration = Declaration::parse_from(inner.next().unwrap())?;
        let value = Expr::parse_from(inner.next().unwrap())?;

        Ok(Self {
            modifiers,
            declaration,
            value,
        })
    }
}

impl Parse for BinaryExpr {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut pairs = pair.into_inner().collect::<Vec<_>>();
        let first = &[pairs.remove(0)];

        let mut pairs = pairs.chunks(2).collect::<Vec<_>>();
        pairs.insert(0, first);

        Ok(BinaryExpr {
            terms: pairs
                .into_iter()
                .map(|x| {
                    let operator = if x.len() == 2 { x.get(0) } else { None };
                    let operator =
                        operator.map(|x| match x.clone().into_inner().next().unwrap().as_rule() {
                            Rule::Add => MathOperator::Add,
                            Rule::Subtract => MathOperator::Subtract,
                            Rule::Multiply => MathOperator::Multiply,
                            Rule::Divide => MathOperator::Divide,
                            Rule::XOR => MathOperator::XOR,
                            Rule::Modulus => MathOperator::Modulus,
                            _ => bug!("UNKNOWN_OPERATOR({:?})", x.as_rule()),
                        });
                    let operand = Term::parse_from(x.last().unwrap().clone()).unwrap();
                    BinaryExprTerm { operand, operator }
                })
                .collect::<Vec<_>>(),
        })
    }
}

impl Parse for ConditionalExpr {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        Ok(Self {
            terms: pair
                .into_inner()
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|x| ConditionExprTerm {
                    operand: Term::parse_from((x[0]).clone()).unwrap(),
                    operator: x.get(1).and_then(|x| {
                        let rule = x.clone().into_inner().next()?.as_rule();
                        match rule {
                            Rule::Equality => Some(ConditionalOperator::Equality),
                            Rule::Inequality => Some(ConditionalOperator::AntiEquality),
                            _ => bug!("UNKNOWN_COND_OPERATOR({:?})", rule),
                        }
                    }),
                })
                .collect::<Vec<_>>(),
        })
    }
}

impl Parse for Assignment {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap())?;
        let value = Expr::parse_from(inner.next().unwrap())?;
        Ok(Self { ident, value })
    }
}

impl Parse for Ident {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        Ok(Self(pair.as_str().to_string()))
    }
}

impl Parse for Expr {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let start_pos = pair.as_span().start_pos();
        let value = parse_one(pair).expect("Invalid expression");
        match value {
            Node::Expr(x) => Ok(x),
            _ => syntax_error(Error::new_from_pos(
                pest::error::ErrorVariant::CustomError {
                    message: "Value is not an expression".to_owned(),
                },
                start_pos,
            )),
        }
    }
}

impl Parse for Tree {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        parse(pair.into_inner())
    }
}

impl Parse for IfBlock {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let if_nodes: Vec<IfNode> = pair
            .into_inner()
            .map(|x| match x.as_rule() {
                Rule::If | Rule::ElseIf | Rule::Else => {
                    let rule = x.as_rule();
                    let mut inner = x.into_inner();
                    match rule {
                        Rule::If | Rule::ElseIf => IfNode::Case(IfCase {
                            expr: Expr::parse_from(inner.next().unwrap()).unwrap(),
                            body: Tree::parse_from(inner.next().unwrap()).unwrap(),
                        }),
                        Rule::Else => IfNode::Else(Else {
                            body: Tree::parse_from(inner.next().unwrap()).unwrap(),
                        }),
                        _ => unreachable!()
                    }
                }
                _ => bug!("INVALID_IFNODE({:?})", x.as_rule()),
            })
            .collect();

        Ok(Self { if_nodes })
    }
}

impl Parse for Return {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let value = Expr::parse_from(inner.next().unwrap())?;
        Ok(Self { value })
    }
}

impl Parse for Class {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();

        let ident = Ident::parse_from(inner.next().unwrap())?;
        let body = Tree::parse_from(inner.next().unwrap())?;

        Ok(Self { ident, body })
    }
}

impl Parse for IndexExpr {
    fn parse_from(pair: Pair<'_, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();

        let term = Term::parse_from(inner.next().unwrap())?;
        let index = Term::parse_from(inner.next().unwrap())?;
        let index = match index {
            Term::Number(x) => Index::Number(x),
            Term::String(x) => Index::String(x),
            _ => bug!("INVALID_INDEX_TERM({:?})", index),
        };
        Ok(Self { term, index })
    }
}
