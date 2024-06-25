use std::collections::HashMap;
use std::vec::Vector;
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

