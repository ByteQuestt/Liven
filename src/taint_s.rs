// use tantivy::collector::TopDocs;
// use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy;

pub fn schema_build(){

let mut schemabuilder = Schema::builder();
let filename = schemabuilder.add_text_field("file_name", TEXT);
let content =schemabuilder.add_text_field("content", STRING);
let repo_name = schemabuilder.add_text_field("Repo Name", STRING);
let indexed = schemabuilder.add_bool_field("is indexed", STORED|FAST );

schemabuilder.build();
println!("schemabuild");
}