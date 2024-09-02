use std::collections::HashMap;
use std::vec;
use petgraph::graph::*;



struct Node{
  symbol : symbol,
  kind :Nodekind,
  range : (i8,i8),
}

#[derive(Debug, Clone)]
enum Nodekind{
  Defnode,
  scope,

}
enum symbol {
 class,
 function,



}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Edgekind{
    nested,
    scopetoscope,
    method,



}
struct Scope_graph{
    pub graph: Graph<Nodekind, Edgekind>,
    root: petgraph::graph::NodeIndex,
    
}

// impl Scope_graph{
//    fn build_sg(&self, )
//    {
//     let mut graph =Graph::new();
//     // let node  = ;
//     // let root = graph.add_node(node);
//     // return Scope_graph{
//     //   graph,
//     //   root
//     }

//     fn insertscope(&self , node:Node ){
//     // self.graph.add_node(node);


//    }
//   }
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