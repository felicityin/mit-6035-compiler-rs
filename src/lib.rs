#[macro_use] extern crate lalrpop_util;

mod ast;
mod parser;
lalrpop_mod!(#[allow(clippy::all)] decaf);

#[cfg(test)]
mod test_util;

use parser::DecafParser;

pub fn compile(code: &str) {
    let parsed = DecafParser::new().parse(code).unwrap();
    println!("-----------------parse start--------------------");
    println!("{:?}", parsed);
    println!("-----------------parse end----------------------");
}
