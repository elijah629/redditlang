use crate::{from_pair::Parse, Rule};

#[derive(Debug)]
pub enum Term {
    Number(i64),
    String(String),
}

#[derive(Debug)]
pub struct Type {
    pub ident: String,
    pub is_array: bool,
}

#[derive(Debug)]
pub struct Declaration {
    pub ident: String,
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
    pub ident: String,
    pub args: Vec<Term>,
}

#[derive(Debug)]
pub struct Throw {
    pub ident: String,
}

#[derive(Debug)]
pub struct Import {
    pub path: Term,
}

#[derive(Debug)]
pub struct Module {
    pub ident: String,
}

#[derive(Debug)]
pub struct TryCatch {
    pub r#try: Tree,
    pub r#catch: Tree,
}

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
pub struct TypeDef {}

// Operators
#[derive(Debug)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    XOR,
}

// Expressions

#[derive(Debug)]
pub struct BinaryExpr {
    pub terms: Vec<BinaryExprTerm>,
}

#[derive(Debug)]
pub struct BinaryExprTerm {
    pub operand: Term,
    /// None on the last term
    pub operator: Option<MathOperator>,
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
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
    TypeDef(TypeDef),
    Expr(Expr),
}

pub type Tree = Vec<Node>;

pub fn parse_one(pair: pest::iterators::Pair<'_, Rule>) -> Option<Node> {
    match pair.as_rule() {
        Rule::Statement => {
            let statement = pair.into_inner().next().unwrap();
            match statement.as_rule() {
                Rule::Loop => Some(Node::Loop(Loop::parse_from(statement))),
                Rule::Function => Some(Node::Function(Function::parse_from(statement))),
                Rule::Call => Some(Node::Call(Call::parse_from(statement))),
                Rule::Break => Some(Node::Break(Break::parse_from(statement))),
                Rule::Throw => Some(Node::Throw(Throw::parse_from(statement))),
                Rule::Import => Some(Node::Import(Import::parse_from(statement))),
                Rule::Module => Some(Node::Module(Module::parse_from(statement))),
                Rule::TryCatch => Some(Node::TryCatch(TryCatch::parse_from(statement))),
                Rule::Variable => Some(Node::Variable(Variable::parse_from(statement))),
                // Rule::TypeDef => Some(Node::TypeDef(TypeDef::parse_from(statement))), // MIGHT REMOVE FROM SPEC
                _ => None,
            }
        }
        Rule::Expr => {
            let expression = pair.into_inner().next().unwrap();
            match expression.as_rule() {
                // Mathematical expression
                Rule::BinaryExpr => Some(Node::Expr(Expr::BinaryExpr(BinaryExpr::parse_from(
                    expression,
                )))),
                Rule::Null => Some(Node::Expr(Expr::Null)),
                _ => None,
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
