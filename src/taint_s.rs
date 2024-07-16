// use tantivy::collector::TopDocs;
// use tantivy::query::QueryParser;
use tantivy::{doc, schema::*};
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
let file_name = schema.get_field("file_name").unwrap();
let content = schema.get_field("content").unwrap();
let repo_name= schema.get_field("repo_name").unwrap();
let indexed = schema.get_field("indexed").unwrap();

}

pub fn index(s:String, schema:Schema){
 let path_index = index::Index::create_in_dir(s,schema.clone()).unwrap();
 let mut index_writer:IndexWriter<TantivyDocument> = path_index.writer(1000000).unwrap();
 let file_name = schema.clone().get_field("file_name").unwrap();
 let content = schema.get_field("content").unwrap();
 let repo_name= schema.get_field("repo_name").unwrap();
 let indexed = schema.get_field("indexed").unwrap();
 index_writer.add_document( doc!(file_name => "example.ts",
    content => " function double(x) {
return x * 2;
}",
   repo_name=> "goodname",
   indexed => true
));
 index_writer.commit();
 


}