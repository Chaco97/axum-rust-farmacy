//use axum::{Router};
use tokio::net::TcpListener;

mod routes;
mod handlers;
mod models;
mod state;
mod config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app = routes::create_router().await;

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor en http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
