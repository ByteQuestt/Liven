use tokio::net::TcpListener;
use std::net::SocketAddr;
use axum::{routing::{self, get}, Router};

async fn  build_server(){
    let app = Router::new();
     
    let addr = SocketAddr::from(([127,0,0,1], 8000));
    let tcpa = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcpa, app).await.unwrap();


}