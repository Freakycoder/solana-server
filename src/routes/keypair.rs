use axum::response::Json as ResponseJson;
use solana_sdk::signature::{Keypair, Signer};

use crate::types::response::{ApiResponse, KeypairResponse};

pub async fn handle_keypair_generation() -> ResponseJson<ApiResponse<KeypairResponse>> {
    println!("🔥 KEYPAIR endpoint called");
    
    let keypair = Keypair::new();
    let response = KeypairResponse {
        pubkey: keypair.pubkey().to_string(),
        secret: bs58::encode(keypair.to_bytes()).into_string(),
    };

    ResponseJson(ApiResponse::success(response))
}
