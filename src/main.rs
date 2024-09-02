mod taint_s;
mod typescript_parser;
mod golang_parser;
mod file_reader;
mod server;
use std::env;
mod scope_graph;
mod query;


// use typescript_parser::parse;
use golang_parser::parse_go;
use taint_s::{self as not_taint, schema_build, search,index};
use tantivy::index;
use typescript_parser::parse_ts ;
use crate::file_reader::file_read as other_reader;
use crate::server as serverbuilding;
use scope_graph::scopebuild;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[1];
    println!("{}",&args[1]);
    println!("Hello, world!");
    parse_ts();
    let schema = schema_build();
    println!("NOT SO GOODDD");
    let code= file_reader::file_read(&path).unwrap();
    scopebuild();

    parse_go(&code);
    println!("parsing go");
   taint_s::index(r"".to_owned(), schema);
    
    file_reader::file_read(&path);
    search(r"", "double");
    server::build_server();

   
}
