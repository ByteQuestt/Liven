use std::fs;
use std::path::Path;

pub fn file_read(dir:&str) -> Option<String>{
  if let Ok(entries)=fs::read_dir(&dir){
    for entry in entries{
        let path = entry.unwrap().path();
        if path.is_file(){
            if let Ok( contents)= fs::read_to_string(&path){
                println!("{}", path.display());
                println!("{}", &contents);
                return Some(contents);
            }
        }
    }
  }
  None


}
