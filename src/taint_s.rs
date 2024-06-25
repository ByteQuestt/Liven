// use tantivy::collector::TopDocs;
// use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy;

pub fn schema_build(){

let mut schemabuilder = Schema::builder();
let filename = schemabuilder.add_text_field("file_name", TEXT);
println!("schemabuild")
}