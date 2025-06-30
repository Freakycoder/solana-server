use axum::{
    response::Json as ResponseJson,
    routing::post,
    Router,
};
use solana_sdk::signature::{Keypair, Signer};
use tower_http::cors::CorsLayer;

pub mod routes;
pub mod types;

use crate::types::response::{ApiResponse, KeypairResponse};

async fn handle_keypair_generation() -> ResponseJson<ApiResponse<KeypairResponse>> {
    let new_keypair = Keypair::new();
    let response_data = KeypairResponse {
        pubkey: new_keypair.pubkey().to_string(),
        secret: bs58::encode(new_keypair.to_bytes()).into_string(),
    };

    ResponseJson(ApiResponse::success(response_data))
}

#[tokio::main]
async fn main() {
    println!("Starting Solana HTTP server...");

    let app = Router::new()
        .route("/keypair", post(handle_keypair_generation))
        .nest("/token", routes::token::token_router())
        .nest("/message", routes::message::message_router())
        .nest("/send", routes::send::send_router())
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("🚀 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Server crashed");
}