#![feature(ptr_offset_from)]
#![feature(plugin)]

#![plugin(phf_macros)]
extern crate phf;
extern crate memmap;

use std::fs;
use std::env;
use std::str;

mod lex;
mod parse;

fn main() {
    // Initialise lexer
    //
    let lexer = lex::Lexer::new();

    // Get arguments
    //
    let args: Vec<String> = env::args().collect();

    // Get filename argument
    // (args[0] is the path name)
    //
    if args.len() < 2 {
        panic!("Use: `adzec example.adze`");
    }
    let path = &args[1];

    // @TODO error handling

    let file = fs::File::open(path).unwrap();

    let map = unsafe { memmap::Mmap::map(&file).unwrap() };

    // @OPTION safer: from_utf8 on smaller slices, within Lexer?
    // Or just do it the once on the whole file, given the semantics would be
    // the same?
    //
    let text = unsafe { str::from_utf8_unchecked(&map) };

    let tokens = lexer.lex(text);

    let parser = parse::Parser::new();

    let tree = parser.parse(tokens);

    // @TODO transform AST into ASM
}
