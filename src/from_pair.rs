use crate::errors::{error, ERR_BUG};
use crate::parser::{
    parse, parse_one, Assignment, BinaryExpr, BinaryExprTerm, Break, Call, Class,
    ConditionExprTerm, ConditionalExpr, ConditionalOperator, Declaration, Else, ElseIf, Expr,
    Function, FunctionMod, Ident, If, IfBlock, IfNode, Import, Index, IndexExpr, Loop,
    MathOperator, Module, Node, Return, Term, Throw, Tree, TryCatch, Type, Variable, VariableMod,
};
use crate::utils::is_unique;
use crate::Rule;
use pest::error::Error;
use pest::iterators::Pair;

pub trait Parse {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self>
    where
        Self: Sized;
}

impl Parse for Declaration {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap()).unwrap();
        let r#type = inner.next().map(|x| x.into_inner()).map(|mut x| Type {
            ident: Ident::parse_from(x.next().unwrap()).unwrap(),
            is_array: x.next().is_some(),
        });

        Some(Self { ident, r#type })
    }
}

impl Parse for Function {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
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

        let declaration = Declaration::parse_from(inner.next().unwrap()).unwrap();

        let raw_args = inner.next().unwrap();
        let start_pos = raw_args.as_span().start_pos();
        let args: Vec<Declaration> = raw_args
            .into_inner()
            .map(|x| Declaration::parse_from(x).unwrap())
            .collect();

        let has_duplicates = !is_unique(args.iter().map(|x| &x.ident.0));
        if has_duplicates {
            error(Error::new_from_pos(
                pest::error::ErrorVariant::CustomError {
                    message: "Duplicate arguments".to_owned(),
                },
                start_pos,
            ))
        }
        let body = Tree::parse_from(inner.next().unwrap()).unwrap();
        Some(Self {
            modifiers,
            declaration,
            args,
            body,
        })
    }
}

impl Parse for Term {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        match pair.as_rule() {
            Rule::String => Some(Self::String(
                enquote::unquote(pair.as_str()).unwrap().to_string(),
            )),
            Rule::Number => Some(Self::Number(pair.as_str().parse().unwrap())),
            Rule::Ident => Some(Self::Ident(Ident::parse_from(pair).unwrap())),
            _ => None,
        }
    }
}

impl Parse for Module {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap()).unwrap();
        Some(Self { ident })
    }
}

impl Parse for Call {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap()).unwrap();
        let args = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|x| Term::parse_from(x.into_inner().next().unwrap()).unwrap())
            .collect();
        Some(Self { ident, args })
    }
}

impl Parse for Break {
    fn parse_from(_pair: Pair<'_, Rule>) -> Option<Self> {
        Some(Break)
    }
}

impl Parse for Throw {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();
        let value = Expr::parse_from(inner.next().unwrap()).unwrap();
        Some(Self { value })
    }
}

impl Parse for Import {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();
        let path = Term::parse_from(inner.next().unwrap()).unwrap();
        Some(Self { path })
    }
}

impl Parse for Loop {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();
        Some(Self {
            body: Tree::parse_from(inner.next().unwrap()).unwrap(),
        })
    }
}

impl Parse for TryCatch {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();
        let mut next_tree =
            || Tree::parse_from(inner.next().unwrap().into_inner().next().unwrap()).unwrap();

        let r#try = next_tree();
        let r#catch = next_tree();

        Some(Self { r#try, r#catch })
    }
}

impl Parse for Variable {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
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
        let declaration = Declaration::parse_from(inner.next().unwrap()).unwrap();
        let value = Expr::parse_from(inner.next().unwrap()).unwrap();

        Some(Self {
            modifiers,
            declaration,
            value,
        })
    }
}

impl Parse for BinaryExpr {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        Some(Self {
            terms: pair
                .into_inner()
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|x| BinaryExprTerm {
                    operand: Term::parse_from((x[0]).clone()).unwrap(),
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
        })
    }
}

impl Parse for ConditionalExpr {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        Some(Self {
            terms: pair
                .into_inner()
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|x| ConditionExprTerm {
                    operand: Term::parse_from((x[0]).clone()).unwrap(),
                    operator: x.get(1).and_then(|x| {
                        match x.clone().into_inner().next().unwrap().as_rule() {
                            Rule::Equality => Some(ConditionalOperator::Equality),
                            Rule::AntiEquality => Some(ConditionalOperator::AntiEquality),
                            _ => panic!("Unknown operator"),
                        }
                    }),
                })
                .collect::<Vec<_>>(),
        })
    }
}

impl Parse for Assignment {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();
        let ident = Ident::parse_from(inner.next().unwrap()).unwrap();
        let value = Expr::parse_from(inner.next().unwrap()).unwrap();
        Some(Self { ident, value })
    }
}

impl Parse for Ident {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        Some(Self(pair.as_str().to_string()))
    }
}

impl Parse for Expr {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let start_pos = pair.as_span().start_pos();
        let value = parse_one(pair).expect("Invalid expression");
        match value {
            Node::Expr(x) => Some(x),
            _ => error(Error::new_from_pos(
                pest::error::ErrorVariant::CustomError {
                    message: "Value is not an expression".to_owned(),
                },
                start_pos,
            )),
        }
    }
}

impl Parse for Tree {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        Some(parse(pair.into_inner()))
    }
}

impl Parse for IfBlock {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        fn if_node(pair: Pair<'_, Rule>) -> IfNode {
            let rule = pair.as_rule();
            let mut inner = pair.into_inner();
            match rule {
                Rule::If => IfNode::If(If {
                    expr: Expr::parse_from(inner.next().unwrap()).unwrap(),
                    body: Tree::parse_from(inner.next().unwrap()).unwrap(),
                }),
                Rule::ElseIf => IfNode::ElseIf(ElseIf {
                    expr: Expr::parse_from(inner.next().unwrap()).unwrap(),
                    body: Tree::parse_from(inner.next().unwrap()).unwrap(),
                }),
                Rule::Else => IfNode::Else(Else {
                    body: Tree::parse_from(inner.next().unwrap()).unwrap(),
                }),
                _ => panic!("{}", ERR_BUG),
            }
        }

        let if_nodes: Vec<IfNode> = pair
            .into_inner()
            .map(|x| match x.as_rule() {
                Rule::If | Rule::ElseIf | Rule::Else => if_node(x),
                _ => panic!("{}", ERR_BUG),
            })
            .collect();

        Some(Self { if_nodes })
    }
}

impl Parse for Return {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();
        let value = Expr::parse_from(inner.next().unwrap()).unwrap();
        Some(Self { value })
    }
}

impl Parse for Class {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();

        let ident = Ident::parse_from(inner.next().unwrap()).unwrap();
        let body = Tree::parse_from(inner.next().unwrap()).unwrap();

        Some(Self { ident, body })
    }
}

impl Parse for IndexExpr {
    fn parse_from(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut inner = pair.into_inner();

        let term = Term::parse_from(inner.next().unwrap()).unwrap();
        let index = Term::parse_from(inner.next().unwrap()).unwrap();
        let index = match index {
            Term::Number(x) => Index::Number(x),
            Term::String(x) => Index::String(x),
            _ => panic!("{:?}", ERR_BUG),
        };
        Some(Self { term, index })
    }
}
