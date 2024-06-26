use tantivy::query;
use tree_sitter::{Node, Parser, Query, QueryCursor, TreeCursor};
use tree_sitter_typescript;
use tree_sitter::Language;
use crate::taint_s ;


pub fn parse_ts() {
    let code = r#"
    function double(x) {
        return x * 2;
    }
"#;
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_typescript::language_typescript())
        .expect("Error loading TypeScript grammar");
    let parsed = parser.parse(code, None).unwrap();
    // println!("{:#?}", parsed);
    let root_node: Node = parsed.root_node();
    let ind: &str = root_node.kind();
    println!("{}",root_node);
    // println!("{}",ind);
    let query_string = "(function_declaration
  (identifier) @hoist.definition.function)

(generator_function_declaration
  (identifier) @hoist.definition.generator)

(formal_parameters
  (required_parameter
    (identifier) @local.definition.parameter))
(formal_parameters
  (optional_parameter
    (identifier) @local.definition.parameter))";
    let query = match tree_sitter::Query::new(&tree_sitter_typescript::language_typescript(),&query_string ) {
        Ok(query) => query,
        Err(error) => {
            // Handle the error here (e.g., print error message, exit gracefully)
            panic!("Failed to create query: {}", error);
        }
    };
    // query
    let dou= "double";
    for (i, name) in query.capture_names().iter().enumerate() {
        let i = i as u32;
        let parts: Vec<_> = name.split('.').collect();
        match parts.as_slice() {
            [scoping, "definition", sym] => {
                println!("gof");
                let index = i;
                println!("{}", &index);
                println!("{}", &scoping);
                
            }
            [scoping, "definition"] => {
                let index = i;
                println!("{}", &index);
                println!("{}", &scoping);
                
            }
            ["local", "reference"] => {
                let index = i;
                
            }
            ["local", "scope"] => (),
            ["local", "import"] => (),
            _ if !name.starts_with('_') => (),
            _ => (),
        }

//     match query.capture_names() {
//     Some("double") =>println!("cc");
//               _     =>print!("gh");
//    }
   
    let res =query.capture_names();
    let cursor = QueryCursor::new();
    let mut tree_cursor:TreeCursor = parsed.walk();
    let node:tree_sitter::Node = parsed.root_node();
    let mut vect: Vec<String> = vec!{};
    for i in node.children(&mut tree_cursor){
    vect.push((&i.kind()).to_string());
     i.kind();
}
    // let cursor_cap = cursor.captures(query, root_node, code)
   
    //
    // == {
    //     let name = &name_node; // Handle potential None
    //     println!("Function name: {}", name);
    //   }
    //    else {
    //     println!("Function declaration without name"); // Handle missing name
    //   }
    // }
    
}
}    

