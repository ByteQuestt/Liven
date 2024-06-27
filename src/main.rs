mod taint_s;
mod typescript_parser;
mod golang_parser;
mod file_reader;
// use typescript_parser::parse;
use golang_parser::parse_go;
use taint_s::{self as not_taint, schema_build};
use typescript_parser::parse_ts ;
use crate::file_reader::file_read as other_reader;


fn main() {
    let path =r"C:\Users\pranav\Desktop\Bit torrent\bittorrent\server.go";
    // C:\Users\pranav\Desktop\Backend

    println!("Hello, world!");
    parse_ts();
    schema_build();
    println!("NOT SO GOODDD");
    parse_go();
    println!("parsing go");
    file_reader::file_read(&path);
    // parse();
}
