use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct RLParser;

fn main() {
    let pairs = RLParser::parse(Rule::Program, "subreddit r/ProgrammerHumor").unwrap();
    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => todo!(),
            Rule::Statements => todo!(),
            Rule::Statement => todo!(),
            Rule::AccMod => todo!(),
            Rule::Loop => todo!(),
            Rule::Break => todo!(),
            Rule::Function => todo!(),
            Rule::FunctionMod => todo!(),
            Rule::FunctionMods => todo!(),
            Rule::FunctionArg => todo!(),
            Rule::FunctionArgs => todo!(),
            Rule::Return => todo!(),
            Rule::Declaration => todo!(),
            Rule::Ident => todo!(),
            Rule::If => todo!(),
            Rule::ElseIf => todo!(),
            Rule::Else => todo!(),
            Rule::IfBlock => todo!(),
            Rule::Call => todo!(),
            Rule::CallArg => todo!(),
            Rule::CallArgs => todo!(),
            Rule::ConditionalExpr => todo!(),
            Rule::BinaryExpr => todo!(),
            Rule::IndexingExpr => todo!(),
            Rule::Expr => todo!(),
            Rule::Term => todo!(),
            Rule::TypedIdent => todo!(),
            Rule::TypeGeneric => todo!(),
            Rule::TypeDef => todo!(),
            Rule::Type => todo!(),
            Rule::Throw => todo!(),
            Rule::Catch => todo!(),
            Rule::Try => todo!(),
            Rule::TryCatch => todo!(),
            Rule::Module => todo!(),
            Rule::Import => todo!(),
            Rule::Variable => todo!(),
            Rule::AssignmentStatement => todo!(),
            Rule::Equality => todo!(),
            Rule::Add => todo!(),
            Rule::Subtract => todo!(),
            Rule::Multiply => todo!(),
            Rule::Divide => todo!(),
            Rule::XOR => todo!(),
            Rule::Assignment => todo!(),
            Rule::Amongus => todo!(),
            Rule::UnaryOperator => todo!(),
            Rule::ConditionalOperator => todo!(),
            Rule::MathOperator => todo!(),
            Rule::OtherOperator => todo!(),
            Rule::Class => todo!(),
            Rule::String => todo!(),
            Rule::StringContent => todo!(),
            Rule::Char => todo!(),
            Rule::UInt => todo!(),
            Rule::Int => todo!(),
            Rule::UDecimal => todo!(),
            Rule::Decimal => todo!(),
            Rule::Number => todo!(),
            Rule::UNumber => todo!(),
            Rule::Flag => todo!(),
            Rule::Flags => todo!(),
            Rule::Block => todo!(),
            Rule::Quote => todo!(),
            _ => {}
        }
    }
    // println!("{:#?}", pairs);
}
