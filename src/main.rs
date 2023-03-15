pub mod parser;

use crate::parser::parse;
use crate::parser::Type::PROGRAM;
use crate::parser::Property;
use std::io;

fn main() {
    let mut buffer = vec![];
    println!("== input your B* ==");
    println!("== use debug in your IDE to get syntax tree ==");

    loop {
        let mut input = String::new();
        let mut consecutive_newlines = 0;

        while consecutive_newlines < 2 {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();

            if line.trim().is_empty() {
                consecutive_newlines += 1;
            } else {
                consecutive_newlines = 0;
            }

            input.push_str(&line);
        }

        let tree = parse(&input, PROGRAM, 0, 1, 0, &mut buffer);
        println!("parsed!");
        println!("Tree:");
        pretty_print_tree(&tree, 0);
        println!("\n== input your B* ==");
    }
}

fn pretty_print_tree(tree: &Property, level: usize) {
    let indent = "  ".repeat(level);
    println!(
        "{}[{}-{}] {:?}: {}",
        indent,
        tree.start,
        tree.end,
        tree.val_type,
        tree.raw.trim_end_matches('\n')
    );
    for child in &tree.children {
        pretty_print_tree(child, level + 1);
    }
}