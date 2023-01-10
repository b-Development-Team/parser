pub mod parser;

use crate::parser::parse;
use crate::parser::Type::PROGRAM;

fn main() {
    let mut buffer = vec![];
    println!("== use debug in your IDE to get syntax tree ==");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let tree = parse(&mut input, PROGRAM, 0, 1, &mut buffer);
        println!("parsed!");
    }
}
