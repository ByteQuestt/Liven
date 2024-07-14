use std::vec;
use std::collections;

#[derive(Debug, Clone)]
enum querykind{
    search,
    explain,
}

#[derive(Debug, Clone)]
struct query {
    query: String,
    kind : querykind,
}

impl query{

fn query(s:String) {
    let q: query = query{
        query :s,
        kind : querykind::search,
    };
}

}