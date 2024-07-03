use tokio::net::TcpListener;
use std::net::SocketAddr;
use axum::{
    body::{Body, HttpBody},
    extract::{Multipart, Path},
    handler::Handler,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
async fn  build_server(){
    let app = Router::new().route("/upload", post(upload));
    let addr = SocketAddr::from(([127,0,0,1], 8000));
    let tcpa = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcpa, app).await.unwrap();


}
async fn upload(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        if name == "file"{
            let content = String::from_utf8(data.to_vec());
        }
    };
}