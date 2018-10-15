use std::env::args;
extern crate roll;

use roll::ast::Eval;

fn main() -> Result<(), Box<std::error::Error>> {
    let line: String = args().skip(1).collect::<Vec<String>>().join(" ");
    let result = roll::parse::parse_expr(&mut roll::scan::Scanner::new(line.chars()))?.eval();
    println!("{}", result);
    Ok(())
}
