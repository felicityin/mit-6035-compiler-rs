#[macro_use] extern crate lalrpop_util;

mod ast;
mod parser;
mod semantic_analyzer;
lalrpop_mod!(#[allow(clippy::all)] decaf);

#[cfg(test)]
mod test_util;

use parser::DecafParser;
use semantic_analyzer::SemanticAnalyzer;

pub fn compile(code: &str) {
    let parsed = DecafParser::new().parse(code).unwrap();
    println!("-----------------parse start--------------------");
    println!("{:?}", parsed);
    println!("-----------------parse end----------------------");
    println!("------------semantic analyze start--------------");
    let ir = SemanticAnalyzer::new().create_ir(parsed);
    println!("{:?}", ir);
    println!("------------semantic analyze end----------------");
}
