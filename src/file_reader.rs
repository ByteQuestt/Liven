use std::fmt::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;


pub fn read_line(dir:&str ,st_ind:usize, end_ind: usize, col_st:usize, col_end:usize){
    let _ = st_ind;
  let path = Path::new(dir);
  let file = File::open(&path).unwrap();
  let mut  buf_read = BufReader::new(file);
  //   Some(buf_read) => {buf_read;}
  //   Error(err) => (),
  // }
  let mut code_snip = String::new();
  for (lineno,line) in buf_read.lines().enumerate(){
    let line = line.unwrap();
    if lineno +1 > end_ind{
      break
    }
    if lineno + 1> st_ind{
      let start_index:usize= if lineno+1 == st_ind{
        col_st-1
      }
      else{0};
      let end_index:usize = if lineno +1 == end_ind{
        col_end-1
      }
      else{0};
      code_snip.push_str(&line[start_index..end_index]);

    }
    println!("{}", &code_snip);
  }


}

pub fn file_read(dir:&str) -> Option<String>{
  if let Ok(entries)=fs::read_dir(&dir){
    for entry in entries{
        let path = entry.unwrap().path();
        if path.is_file(){
            if let contents= fs::read_to_string(&path).unwrap(){
                println!("{}", path.display());
                println!("{}", &contents);
                return Some(contents);
            }
        }
    }}
  
  None
  
  


}