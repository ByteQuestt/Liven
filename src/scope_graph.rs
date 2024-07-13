use std::collections::HashMap;
use std::vec;
use petgraph::graph::*;

enum Nodekind{
    
}

struct Node{
  symbol : symbol,
  

}
enum symbol {
 class,
 function,


}

enum Edgekind{
    

}
struct Scope_graph{
    pub graph: Graph<Nodekind, Edgekind>,
    root: petgraph::graph::NodeIndex,
    
}

impl Scope_graph{
   fn build_sg(&self){
   

   }
}
#[derive(Debug)]
struct Function{
  name: String,
  start_line : usize,
  end_line : usize,
}

#[derive(Debug)]
struct File{
  name: String,
  location: String,
  functions:Vec<Function>
}

pub fn scopebuild(){
  let mut file = File{
    name : "example.ts".to_string(),
    location : "path".to_string(),
    functions : vec![Function{
                 name : "main".to_string(),
                 start_line: 4,
                 end_line:6
    }]

  };
  println!("{:#?}",file);
}