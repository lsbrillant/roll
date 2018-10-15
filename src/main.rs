use std::env::args;
extern crate roll;

use roll::ast::Eval;

fn main() -> Result<(), Box<std::error::Error>> {
    let args = args().skip(1).collect::<Vec<String>>();
    if args.iter().count() < 1 {
        println!("Usage roll <dice>");
        return Ok(())
    }
    let line: String = args.join(" ");
    let result = roll::parse::parse_expr(&mut roll::scan::Scanner::new(line.chars()))?.eval();
    println!("{}", result);
    Ok(())
}
