use axum::{routing::post, Router};
use tower_http::cors::CorsLayer;

mod routes;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    println!("🚀 Starting Solana HTTP server...");

    let app = Router::new()
        .route("/keypair", post(routes::keypair::handle_keypair_generation))
        .route("/token/create", post(routes::token::handle_token_creation))
        .route("/token/mint", post(routes::token::handle_token_minting))
        .route("/message/sign", post(routes::message::handle_message_signing))
        .route("/message/verify", post(routes::message::handle_message_verification))
        .route("/send/sol", post(routes::send::handle_sol_transfer))
        .route("/send/token", post(routes::send::handle_token_transfer))
        .layer(CorsLayer::permissive());

    println!("📍 Available endpoints:");
    println!("  POST /keypair");
    println!("  POST /token/create");
    println!("  POST /token/mint");
    println!("  POST /message/sign");
    println!("  POST /message/verify");
    println!("  POST /send/sol");
    println!("  POST /send/token");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("🔥 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Server crashed");
}
