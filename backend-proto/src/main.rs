use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use hyper::Server; // ini penting di Axum 0.7

#[tokio::main]
async fn main() {
    // router = definisi endpoint
    let app = Router::new()
        .route("/", get(|| async {"Halo dari Rust backend 🚀"}))
        .route("/hello", get(|| async {"Hello World dari endpoint /hello"}));
    
    // alamat server 
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server berjalan di http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.info_make_service())
        .await
        .unwrap();
}