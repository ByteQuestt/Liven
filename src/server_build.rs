 use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use axum::{
    body::{Body, HttpBody},
    extract::{Multipart, Path},
    handler::Handler,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::io::{prelude::*, BufReader};

// pub(crate) async fn  build_server(){
//     // let app = Router::new().route("/upload", post(upload)).route("/", get(|| async { "Hello, World!" }));
//     // let addr = SocketAddr::from(([127,0,0,1], 8000));
//     // let tcpa = TcpListener::bind(&addr).await.unwrap();
//     // println!("server build");
    

//     // axum::serve(tcpa, app).await.unwrap();
//     // for stream in tcpa.accept().await{
//     //     let (stream,_) = stream;

//     //     handle_connection(stream);
//     // }
//     let app = Router::new().route("/", get(|| async { "Hello, world!" }));

//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("listening on {}", addr);
//     TcpListener::bind(&addr);
//     axum::serve(app.into_make_service());
      

// }
fn handle_connection(mut stream : TcpStream) {
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    println!("Request: ");
}
async fn upload(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        // checking for the file field
        if name == "file"{
            // converting file content from bytes 
            let content = String::from_utf8(data.to_vec());
        }
    };
}

#[tokio::main]
pub async fn build_server() {
    let app = axum::Router::new().route("/upload", post(upload));
    let addr = SocketAddr::from(([127,0,0,1], 8000));
    let tcpa = TcpListener::bind(&addr).await.unwrap();
    println!("server build");
    

    for stream in tcpa.accept().await{
        let (stream,_) = stream;

        handle_connection(stream);
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let tcpa= TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcpa , app.into_make_service());
      


}