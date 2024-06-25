mod taint_s;
mod typescript_parser;
mod golang_parser;
// use typescript_parser::parse;
use golang_parser::parse_go;
use taint_s::{self as not_taint, schema_build};
use typescript_parser::{self as tsparse, parse_ts} ;

fn main() {
    println!("Hello, world!");
    parse_ts();
    schema_build();
    println!("NOT SO GOODDD");
    parse_go();
    println!("parsing go");
    // parse();
}
