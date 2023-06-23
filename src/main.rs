use crate::errors::format_error;
use parser::parse;
use pest::Parser;
use pest_derive::Parser;
use std::hash::Hash;

pub mod errors;
pub mod from_pair;
pub mod parser;
pub mod utils;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct RLParser;

fn main() {
    let pairs = RLParser::parse(Rule::Program, include_str!("test.rl"));
    if pairs.is_err() {
        let error: pest::error::Error<Rule> = pairs.unwrap_err();
        println!("{}", format_error(error));
        return;
    }
    println!("{:#?}", parse(pairs.unwrap()));
}
