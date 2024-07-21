use tantivy::schema::document::DocumentDeserialize;
use tantivy::query::QueryParser;
// use tantivy::collector::TopDocs;
// use tantivy::query::QueryParser;
use tantivy::{doc, schema::*, ReloadPolicy};
use tantivy::collector::TopDocs;
use tantivy::indexer::{IndexWriter, SegmentWriter};
use tantivy::{Document, IndexReader};
use tantivy::index;
use std::path::{Path,PathBuf};

pub fn schema_build()-> tantivy::schema::Schema{

let mut schemabuilder = Schema::builder();
let filename = schemabuilder.add_text_field("file_name", TEXT);
let content =schemabuilder.add_text_field("content", STRING);
let repo_name = schemabuilder.add_text_field("repo_name", TEXT);
let indexed = schemabuilder.add_bool_field("indexed", STORED|FAST );

let schema = schemabuilder.build();
println!("schemabuild");
return schema;


}

pub fn index(s:String, schema:Schema){
 let path_index = index::Index::create_in_dir(s,schema.clone()).unwrap();
 let mut index_writer:IndexWriter<TantivyDocument> = path_index.writer(15000000).unwrap();
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
index_writer.add_document( doc!(file_name => "example.ts",
content => " function double(x) {
return x * 2;
}",
repo_name=> "goodname",
indexed => true
));
 let _ = index_writer.commit();
 
}

pub fn search(s : &str, name :&str){
    let index = index::Index::open_in_dir(Path::new(s)).unwrap();
    let schema = index.schema();
    let content = schema.get_field("content").unwrap();

    let query_parser = QueryParser::for_index(&index, vec![schema.get_field("content").unwrap()]);
    let query = query_parser.parse_query(name).unwrap();
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into().unwrap();
    let searcher = reader.searcher(); 
    let query_parser = QueryParser::for_index(&index, vec![ content]);
    let query = query_parser.parse_query(name).unwrap();
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();
    let num = searcher.num_docs();
    println!("{}",num);
    for (_, doc_address) in top_docs {
        let retrieved_doc:TantivyDocument = searcher.doc(doc_address).unwrap();
        println!("happy");

        println!("{}", &retrieved_doc.to_json(&schema));
       
        let path = retrieved_doc.get_first(schema.get_field("path").unwrap()).unwrap();
    }
    println!("happy");



    
}