use std::path::PathBuf;

use crate::{utils::Result, Rule};

use self::from_pair::Parse;

pub mod from_pair;
pub type Number = f64; // Number type

#[derive(Debug, Clone)]
pub enum Term {
    Number(Number),
    String(String),
    Boolean(bool),
    // Foolean(Foolean),
    Array(Vec<Expr>),
    Null,

    Ident(Ident),
}

#[derive(Debug, Clone)]
pub struct Type {
    pub generics: Vec<Type>,
    pub root_type: Ident,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub ident: Ident,
    pub r#type: Type,
}

// Statements

#[derive(Debug, Clone)]
pub struct Loop(pub Tree);

#[derive(Debug, Clone)]
pub struct Break;

#[derive(Debug, Clone)]
pub struct Function {
    pub modifiers: Vec<FunctionMod>,
    pub declaration: Declaration,
    pub args: Vec<Declaration>,
    pub body: Tree,
}

#[derive(Debug, Clone)]
pub enum FunctionMod {
    Debug,
    Public,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub ident: Ident,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Throw(pub Expr);

#[derive(Debug, Clone)]
pub struct Import(pub PathBuf); // using pathbuf for joining and canocalizations

#[derive(Debug, Clone)]
pub struct TryCatch {
    pub r#try: Try,
    pub catch: Catch,
}

#[derive(Debug, Clone)]
pub struct Try(pub Tree);
#[derive(Debug, Clone)]
pub struct Catch(pub Option<Ident>, pub Tree);

#[derive(Debug, Clone)]
pub struct Variable {
    pub modifiers: Vec<VariableMod>,
    pub declaration: Declaration,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub enum VariableMod {
    Public,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub ident: Ident,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct IfBlock {
    pub if_nodes: Vec<IfNode>,
}

#[derive(Debug, Clone)]
pub enum IfNode {
    Case(IfCase),
    Else(Else),
}

#[derive(Debug, Clone)]
pub struct IfCase {
    pub body: Tree,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct Else {
    pub body: Tree,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub ident: Ident,
    pub body: Tree,
}

#[derive(Debug, Clone)]
pub struct Return(pub Expr);

// Operators
#[derive(Debug, Clone)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    XOR,
    Modulus,
}

#[derive(Debug, Clone)]
pub enum ConditionalOperator {
    Equality,
    AntiEquality,
}

// Expressions

pub type ConditionalExpr = ChainedExpr<ConditionalOperator>;
pub type ConditionExprTerm = ChainedExprTerm<ConditionalOperator>;

pub type BinaryExpr = ChainedExpr<MathOperator>;
pub type BinaryExprTerm = ChainedExprTerm<MathOperator>;

#[derive(Debug, Clone)]
pub struct IndexExpr {
    pub term: Term,
    pub index: Index,
}

#[derive(Debug, Clone)]
pub enum Index {
    Number(Number),
    String(String),
}

#[derive(Debug, Clone)]
pub struct ChainedExpr<T> {
    pub terms: Vec<ChainedExprTerm<T>>,
}

#[derive(Debug, Clone)]
pub struct ChainedExprTerm<T> {
    pub operand: Term,

    /// None on the last term
    pub operator: Option<T>,
}

#[derive(Debug, Clone)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    ConditionalExpr(ConditionalExpr),
    IndexExpr(IndexExpr),
    Term(Term),
    CallExpr(Call),
}

// AST
#[derive(Debug, Clone)]
pub enum Node {
    Loop(Loop),
    Break(Break),
    Function(Function),
    Call(Call),
    Throw(Throw),
    Import(Import),
    TryCatch(TryCatch),
    Variable(Variable),
    Assignment(Assignment),
    If(IfBlock),
    Class(Class),
    Return(Return),
    Expr(Expr),
    EOI,
}

pub type Tree = Vec<Node>;

pub fn parse_one(pair: pest::iterators::Pair<'_, Rule>) -> Result<Node> {
    match pair.as_rule() {
        Rule::Statement => {
            let statement = pair.into_inner().next().unwrap();
            match statement.as_rule() {
                Rule::Loop => Ok(Node::Loop(Loop::parse_from(statement).unwrap())),
                Rule::Function => Ok(Node::Function(Function::parse_from(statement).unwrap())),
                Rule::Call => Ok(Node::Call(Call::parse_from(statement).unwrap())),
                Rule::Break => Ok(Node::Break(Break::parse_from(statement).unwrap())),
                Rule::Throw => Ok(Node::Throw(Throw::parse_from(statement).unwrap())),
                Rule::Import => Ok(Node::Import(Import::parse_from(statement).unwrap())),
                Rule::TryCatch => Ok(Node::TryCatch(TryCatch::parse_from(statement).unwrap())),
                Rule::Variable => Ok(Node::Variable(Variable::parse_from(statement).unwrap())),
                Rule::AssignmentStatement => {
                    Ok(Node::Assignment(Assignment::parse_from(statement)?))
                }
                Rule::IfBlock => Ok(Node::If(IfBlock::parse_from(statement)?)),
                Rule::Class => Ok(Node::Class(Class::parse_from(statement)?)),
                Rule::Return => Ok(Node::Return(Return::parse_from(statement)?)),
                _ => Err("UNEXPECTED_STATEMENT".into()),
            }
        }
        Rule::Expr => {
            let expression = pair.into_inner().next().unwrap();
            match expression.as_rule() {
                Rule::BinaryExpr => Ok(Node::Expr(Expr::BinaryExpr(BinaryExpr::parse_from(
                    expression,
                )?))),
                Rule::ConditionalExpr => Ok(Node::Expr(Expr::ConditionalExpr(
                    ConditionalExpr::parse_from(expression)?,
                ))),
                Rule::IndexExpr => Ok(Node::Expr(Expr::IndexExpr(IndexExpr::parse_from(
                    expression,
                )?))),
                Rule::Call => Ok(Node::Expr(Expr::CallExpr(Call::parse_from(expression)?))),
                _ => Term::parse_from(expression).map(|x| Node::Expr(Expr::Term(x))),
            }
        }
        Rule::EOI => Ok(Node::EOI),
        _ => Err(format!(
            "Expected either Statement or Expr, but got {:?}",
            pair.as_rule()
        )
        .into()),
    }
}

pub fn parse(pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Tree> {
    let mut tree: Tree = vec![];

    for pair in pairs {
        let node = parse_one(pair)?;
        if !matches!(node, Node::EOI) {
            tree.push(node);
        }
    }
    Ok(tree)
}
