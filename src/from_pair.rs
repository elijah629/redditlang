use crate::errors::error;
use crate::parser::{
    parse, parse_one, Assignment, BinaryExpr, BinaryExprTerm, Break, Call, Declaration, Expr,
    Function, FunctionMod, Ident, Import, Loop, MathOperator, Module, Node, Term, Throw, TryCatch,
    Type, Variable, VariableMod,
};
use crate::utils::is_unique;
use crate::Rule;
use pest::error::Error;
use pest::iterators::Pair;

pub trait Parse {
    fn parse_from(pair: Pair<'_, Rule>) -> Self;
}

impl Parse for Declaration {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap());
        let r#type = inner.next().map(|x| x.into_inner()).map(|mut x| Type {
            ident: Ident::parse_from(x.next().unwrap()),
            is_array: x.next().is_some(),
        });

        Self { ident, r#type }
    }
}

impl Parse for Function {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
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
                _ => error(Error::new_from_pos(
                    pest::error::ErrorVariant::CustomError {
                        message: "Invalid modifier".to_owned(),
                    },
                    start_pos,
                )),
            })
            .collect();

        let declaration = Declaration::parse_from(inner.next().unwrap());

        let raw_args = inner.next().unwrap();
        let start_pos = raw_args.as_span().start_pos();
        let args: Vec<Declaration> = raw_args
            .into_inner()
            .map(|x| Declaration::parse_from(x))
            .collect();

        // Check for duplicate argument idents
        let has_duplicates = !is_unique(args.iter().map(|x| &x.ident.0));
        if has_duplicates {
            error(Error::new_from_pos(
                pest::error::ErrorVariant::CustomError {
                    message: "Duplicate arguments".to_owned(),
                },
                start_pos,
            ))
        }
        let body = parse(inner.next().unwrap().into_inner());
        Function {
            modifiers,
            declaration,
            args,
            body,
        }
    }
}

impl Parse for Term {
    fn parse_from(pair: Pair<'_, Rule>) -> Term {
        let start_pos = pair.as_span().start_pos();
        match pair.as_rule() {
            Rule::String => Term::String(enquote::unquote(pair.as_str()).unwrap().to_string()),
            Rule::Number => Term::Number(pair.as_str().parse().unwrap()),
            Rule::Ident => Term::Ident(Ident::parse_from(pair)),
            _ => error(Error::new_from_pos(
                pest::error::ErrorVariant::CustomError {
                    message: format!("Unimplemented Term \"{:?}\"", pair.as_rule()).to_owned(),
                },
                start_pos,
            )),
        }
    }
}

impl Parse for Module {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap());
        return Module { ident };
    }
}

impl Parse for Call {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap());
        let args = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|x| Term::parse_from(x.into_inner().next().unwrap()))
            .collect();
        Call { ident, args }
    }
}

impl Parse for Break {
    fn parse_from(_pair: Pair<'_, Rule>) -> Self {
        Break
    }
}

impl Parse for Throw {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let value = Expr::parse_from(inner.next().unwrap());
        Throw { value }
    }
}
impl Parse for Import {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let ident = inner.next();
        let path = Term::parse_from(ident.unwrap());
        Import { path }
    }
}

impl Parse for Loop {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        Loop {
            body: parse(inner.next().unwrap().into_inner()),
        }
    }
}

impl Parse for TryCatch {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let mut next_tree = || {
            parse(
                inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .into_inner(),
            )
        };

        let r#try = next_tree();
        let r#catch = next_tree();

        TryCatch { r#try, r#catch }
    }
}

impl Parse for Variable {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        let start_pos = pair.as_span().start_pos();
        let mut inner = pair.into_inner();
        let modifiers: Vec<VariableMod> = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|modifier| modifier.as_str().trim_end().to_string())
            .map(|modifier| match modifier.as_str() {
                "bar" => VariableMod::Public,
                _ => error(Error::new_from_pos(
                    pest::error::ErrorVariant::CustomError {
                        message: "Invalid modifier".to_owned(),
                    },
                    start_pos,
                )),
            })
            .collect();
        let declaration = Declaration::parse_from(inner.next().unwrap());
        let value = Expr::parse_from(inner.next().unwrap());

        Variable {
            modifiers,
            declaration,
            value,
        }
    }
}

impl Parse for BinaryExpr {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        BinaryExpr {
            terms: pair
                .into_inner()
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|x| BinaryExprTerm {
                    operand: Term::parse_from((x[0]).clone()),
                    operator: x.get(1).and_then(|x| {
                        match x.clone().into_inner().next().unwrap().as_rule() {
                            Rule::Subtract => Some(MathOperator::Subtract),
                            Rule::Multiply => Some(MathOperator::Multiply),
                            Rule::Divide => Some(MathOperator::Divide),
                            Rule::XOR => Some(MathOperator::XOR),
                            _ => panic!("Unknown operator"),
                        }
                    }),
                })
                .collect::<Vec<_>>(),
        }
    }
}

impl Parse for Assignment {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap());
        let value = Expr::parse_from(inner.next().unwrap());
        Assignment { ident, value }
    }
}

impl Parse for Ident {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        Ident(pair.as_str().to_string())
    }
}

impl Parse for Expr {
    fn parse_from(pair: Pair<'_, Rule>) -> Self {
        let start_pos = pair.as_span().start_pos();
        let value = parse_one(pair).unwrap();
        match value {
            Node::Expr(x) => x,
            _ => error(Error::new_from_pos(
                pest::error::ErrorVariant::CustomError {
                    message: "Value is not an expression".to_owned(),
                },
                start_pos,
            )),
        }
    }
}

// TODO: MORE STATEMENTS + EXPRS
