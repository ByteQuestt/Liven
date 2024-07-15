// use tantivy::collector::TopDocs;
// use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::collector::TopDocs;
use tantivy::indexer::{IndexWriter, SegmentWriter};
use tantivy::{Document, IndexReader};
use tantivy::index;

pub fn schema_build(){

let mut schemabuilder = Schema::builder();
let filename = schemabuilder.add_text_field("file_name", TEXT);
let content =schemabuilder.add_text_field("content", STRING);
let repo_name = schemabuilder.add_text_field("Repo Name", STRING);
let indexed = schemabuilder.add_bool_field("is indexed", STORED|FAST );

let schema = schemabuilder.build();
println!("schemabuild");
}

pub fn index(s:String, schema:Schema){
 let path_index = index::Index::create_in_dir(s,schema).unwrap();
 let mutindex_writer:IndexWriter<TantivyDocument> = path_index.writer(1000000).unwrap();




}