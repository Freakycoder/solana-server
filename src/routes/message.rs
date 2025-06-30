use axum::{
    extract::Json,
    response::Json as ResponseJson,
    routing::post,
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use solana_sdk::{pubkey::Pubkey, signature::{Keypair, Signature, Signer}};
use std::str::FromStr;

use crate::{
    routes::token::validate_pubkey,
    types::{
        message::{
            MessageSignRequest, MessageSignResponse, MessageVerifyRequest, MessageVerifyResponse,
        },
        response::ApiResponse,
    },
};

pub fn message_router() -> Router {
    Router::new()
        .route("/sign", post(handle_message_signing))
        .route("/verify", post(handle_message_verification))
}

pub async fn handle_message_signing(
    Json(req): Json<MessageSignRequest>,
) -> ResponseJson<ApiResponse<MessageSignResponse>> {
    if req.message.is_empty() || req.secret.trim().is_empty() {
        return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
    }

    if req.message.len() > 1_000_000 {
        return ResponseJson(ApiResponse::error("Message too long".to_string()));
    }

    let keypair = match parse_secret_key(&req.secret) {
        Ok(kp) => kp,
        Err(e) => {
            return ResponseJson(ApiResponse::error(format!("Invalid private key: {}", e)));
        }
    };

    let msg_bytes = req.message.as_bytes();
    let signature = keypair.sign_message(msg_bytes);

    let response_data = MessageSignResponse {
        signature: general_purpose::STANDARD.encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: req.message,
    };

    ResponseJson(ApiResponse::success(response_data))
}

pub async fn handle_message_verification(
    Json(req): Json<MessageVerifyRequest>,
) -> ResponseJson<ApiResponse<MessageVerifyResponse>> {
    if req.message.is_empty() || req.signature.trim().is_empty() || req.pubkey.trim().is_empty() {
        return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
    }

    if req.message.len() > 1_000_000 {
        return ResponseJson(ApiResponse::error("Message too long".to_string()));
    }

    let pubkey = match validate_pubkey(&req.pubkey) {
        Ok(pk) => pk,
        Err(e) => {
            return ResponseJson(ApiResponse::error(e));
        }
    };

    let sig_bytes = match general_purpose::STANDARD.decode(req.signature.trim()) {
        Ok(bytes) => bytes,
        Err(_) => {
            return ResponseJson(ApiResponse::error(
                "Invalid signature: Invalid base64 encoding".to_string(),
            ));
        }
    };

    if sig_bytes.len() != 64 {
        return ResponseJson(ApiResponse::error(
            "Invalid signature: Invalid signature length".to_string(),
        ));
    }

    let signature = match <[u8; 64]>::try_from(sig_bytes) {
        Ok(sig_array) => Signature::from(sig_array),
        Err(_) => {
            return ResponseJson(ApiResponse::error(
                "Invalid signature: Invalid signature format".to_string(),
            ));
        }
    };

    let msg_bytes = req.message.as_bytes();

    let is_valid = signature.verify(&pubkey.to_bytes(), msg_bytes);

    let response_data = MessageVerifyResponse {
        valid: is_valid,
        message: req.message,
        pubkey: req.pubkey,
    };

    ResponseJson(ApiResponse::success(response_data))
}

fn parse_secret_key(secret_str: &str) -> Result<Keypair, String> {
    let trimmed = secret_str.trim();
    
    if trimmed.is_empty() {
        return Err("Secret key cannot be empty".to_string());
    }

    if trimmed.len() < 80 || trimmed.len() > 90 {
        return Err("Invalid secret key length".to_string());
    }

    
    let decoded = bs58::decode(trimmed)
        .into_vec()
        .map_err(|_| "Invalid base58 encoding for secret key".to_string())?;

    
    if decoded.len() != 64 {
        return Err("Secret key must be 64 bytes".to_string());
    }

    
    Keypair::from_bytes(&decoded)
        .map_err(|_| "Could not create keypair from bytes".to_string())
}