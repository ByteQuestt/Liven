use std::vec;
use std::collections;
use crate::taint_s::index;

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

fn query(s:String) {
    let q: query = query{
        query :s,
        kind : querykind::search,
        
    };
    q;
}
impl query{

fn querys(s:String) {
    let q: query = query{
        query :s,
        kind : querykind::search,
    };

// fn search(s:String){

// }

}


}