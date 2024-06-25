use std::{error, fmt::{Debug, Error}, vec};




use tree_sitter::{Node, Parser, Query, QueryCursor, QueryError, Tree, TreeCursor};
use tree_sitter_go;

pub fn parse_go(){
let code = r#"
 import "fmt"

 func add(a, b int) int {
    return a + b
}
 func sub(a,b int) int {
 return a-b
}
"#;
let mut parser = Parser::new();
parser.set_language(&tree_sitter_go::language()).expect("Error loading Go grammar");
let tree: tree_sitter::Tree = parser.parse(code, None).unwrap();
let src = "(function_declaration) @local.scope
(method_declaration) @local.scope
(func_literal) @local.scope
(field_declaration_list) @local.scope
(type_switch_statement) @local.scope
(type_declaration) @local.scope
(import_spec 
  (package_identifier) @local.import)
(block) @local.scope
";
let query = match tree_sitter::Query::new(&tree_sitter_go::language(),&src ) {
  Ok(query) => query,
  Err(error) => {
      // Handle the error here (e.g., print error message, exit gracefully)
      panic!("Failed to create query: {}", error);
  }
};// let str = query.capture_names();

let mut vec2: Vec<String> = vec!{};
for (i, name) in query.capture_names().iter().enumerate() {
 vec2.push((&name).to_string());
 let i = i as u32;
 let parts: Vec<_> = name.split('.').collect();
 match parts.as_slice() {
     [scoping, "definition", sym] => {
         println!("fog");
         let index = i;
        
     }
     [scoping, "definition"] => {
         let index = i;
         println!();
       
     }
     ["local", "reference"] => {
         let index = i;
         println!("local")
       
     }
     ["local", "scope"] => {println!("scope")},
     ["local", "import"] => {println!("import")},
     _ if !name.starts_with('_') => (),
     _ => (),
 }
}
struct quey(Query);
impl Debug for quey{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Query").field("capture_names", &self.0.capture_names()).finish()
    }
}


traverse(&tree);
assert!(!tree.root_node().has_error());
print!("{}", tree.root_node());
print!("parser build")
}

fn traverse(tree : &Tree ){
  let node:Node = tree.root_node();
  let mut cursor = tree.walk();
  for i in node.children(&mut cursor){
  if node.kind() =="function_declaration"{
    let name = node.child_by_field_name("function_name").unwrap();
    println!("{}",name);
  }
}
;
let mut tree_cursor:TreeCursor = tree.walk();
let node:tree_sitter::Node = tree.root_node();
let mut vec: Vec<String> = vec!{};
for i in node.children(&mut tree_cursor){
  vec.push((&i.kind()).to_string());
  i.kind();
}
    }

  
