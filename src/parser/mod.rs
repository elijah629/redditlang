use crate::Rule;

use self::from_pair::Parse;

pub mod from_pair;
pub type Number = f64; // Number type

#[derive(Debug)]
pub enum Term {
    Number(Number),
    String(String),
    Ident(Ident),
}

#[derive(Debug)]
pub struct Type {
    pub ident: Ident,
    pub is_array: bool,
}

#[derive(Debug)]
pub struct Declaration {
    pub ident: Ident,
    pub r#type: Option<Type>,
}

// Statements

#[derive(Debug)]
pub struct Loop {
    pub body: Tree,
}

#[derive(Debug)]
pub struct Break;

#[derive(Debug)]
pub struct Function {
    pub modifiers: Vec<FunctionMod>,
    pub declaration: Declaration,
    pub args: Vec<Declaration>,
    pub body: Tree,
}

#[derive(Debug)]
pub enum FunctionMod {
    Debug,
    Public,
}

#[derive(Debug)]
pub struct Call {
    pub ident: Ident,
    pub args: Vec<Term>,
}

#[derive(Debug)]
pub struct Throw {
    pub value: Expr,
}

#[derive(Debug)]
pub struct Import {
    pub path: Term,
}

#[derive(Debug)]
pub struct Module {
    pub ident: Ident,
}

#[derive(Debug)]
pub struct TryCatch {
    pub r#try: Try,
    pub catch: Catch,
}

#[derive(Debug)]
pub struct Try(pub Tree);
#[derive(Debug)]
pub struct Catch(pub Option<Ident>, pub Tree);

#[derive(Debug)]
pub struct Variable {
    pub modifiers: Vec<VariableMod>,
    pub declaration: Declaration,
    pub value: Expr,
}

#[derive(Debug)]
pub enum VariableMod {
    Public,
}

#[derive(Debug)]
pub struct Assignment {
    pub ident: Ident,
    pub value: Expr,
}

#[derive(Debug)]
pub struct IfBlock {
    pub if_nodes: Vec<IfNode>,
}

#[derive(Debug)]
pub enum IfNode {
    Case(IfCase),
    Else(Else),
}

#[derive(Debug)]
pub struct IfCase {
    pub body: Tree,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Else {
    pub body: Tree,
}

#[derive(Debug)]
pub struct Class {
    pub ident: Ident,
    pub body: Tree,
}

#[derive(Debug)]
pub struct Return {
    pub value: Expr,
}

// Operators
#[derive(Debug)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    XOR,
    Modulus,
}

#[derive(Debug)]
pub enum ConditionalOperator {
    Equality,
    AntiEquality,
}

// Expressions

pub type ConditionalExpr = ChainedExpr<ConditionalOperator>;
pub type ConditionExprTerm = ChainedExprTerm<ConditionalOperator>;

pub type BinaryExpr = ChainedExpr<MathOperator>;
pub type BinaryExprTerm = ChainedExprTerm<MathOperator>;

#[derive(Debug)]
pub struct IndexExpr {
    pub term: Term,
    pub index: Index,
}

#[derive(Debug)]
pub enum Index {
    Number(Number),
    String(String),
}

#[derive(Debug)]
pub struct ChainedExpr<T> {
    pub terms: Vec<ChainedExprTerm<T>>,
}

#[derive(Debug)]
pub struct ChainedExprTerm<T> {
    pub operand: Term,

    /// None on the last term
    pub operator: Option<T>,
}

#[derive(Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    ConditionalExpr(ConditionalExpr),
    IndexExpr(IndexExpr),
    Term(Term),
    Null,
}

// AST
#[derive(Debug)]
pub enum Node {
    Loop(Loop),
    Break(Break),
    Function(Function),
    Call(Call),
    Throw(Throw),
    Import(Import),
    Module(Module),
    TryCatch(TryCatch),
    Variable(Variable),
    Assignment(Assignment),
    If(IfBlock),
    Class(Class),
    Return(Return),
    Expr(Expr),
}

pub type Tree = Vec<Node>;

pub fn parse_one(pair: pest::iterators::Pair<'_, Rule>) -> Option<Node> {
    match pair.as_rule() {
        Rule::Statement => {
            let statement = pair.into_inner().next().unwrap();
            match statement.as_rule() {
                Rule::Loop => Some(Node::Loop(Loop::parse_from(statement).unwrap())),
                Rule::Function => Some(Node::Function(Function::parse_from(statement).unwrap())),
                Rule::Call => Some(Node::Call(Call::parse_from(statement).unwrap())),
                Rule::Break => Some(Node::Break(Break::parse_from(statement).unwrap())),
                Rule::Throw => Some(Node::Throw(Throw::parse_from(statement).unwrap())),
                Rule::Import => Some(Node::Import(Import::parse_from(statement).unwrap())),
                Rule::Module => Some(Node::Module(Module::parse_from(statement).unwrap())),
                Rule::TryCatch => Some(Node::TryCatch(TryCatch::parse_from(statement).unwrap())),
                Rule::Variable => Some(Node::Variable(Variable::parse_from(statement).unwrap())),
                Rule::AssignmentStatement => {
                    Some(Node::Assignment(Assignment::parse_from(statement).unwrap()))
                }
                Rule::IfBlock => Some(Node::If(IfBlock::parse_from(statement).unwrap())),
                Rule::Class => Some(Node::Class(Class::parse_from(statement).unwrap())),
                Rule::Return => Some(Node::Return(Return::parse_from(statement).unwrap())),
                _ => None,
            }
        }
        Rule::Expr => {
            let expression = pair.into_inner().next().unwrap();
            match expression.as_rule() {
                Rule::BinaryExpr => Some(Node::Expr(Expr::BinaryExpr(
                    BinaryExpr::parse_from(expression).unwrap(),
                ))),
                Rule::ConditionalExpr => Some(Node::Expr(Expr::ConditionalExpr(
                    ConditionalExpr::parse_from(expression).unwrap(),
                ))),
                Rule::IndexExpr => Some(Node::Expr(Expr::IndexExpr(
                    IndexExpr::parse_from(expression).unwrap(),
                ))),
                Rule::Null => Some(Node::Expr(Expr::Null)),
                _ => Term::parse_from(expression).map(|x| Node::Expr(Expr::Term(x))),
            }
        }
        _ => None,
    }
}

pub fn parse(pairs: pest::iterators::Pairs<'_, Rule>) -> Tree {
    let mut tree: Tree = vec![];

    for pair in pairs {
        let node = parse_one(pair);
        if node.is_some() {
            tree.push(node.unwrap());
        }
    }
    tree
}
