use axum::response::Json as ResponseJson;
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use solana_sdk::{signature::{Signature, Signer}};

use crate::{
    types::{
        request::{SafeJson, get_required_string},
        response::{ApiResponse, MessageSignResponse, MessageVerifyResponse},
    },
    utils::{validate_pubkey, parse_secret_key},
};

#[derive(Deserialize, Debug)]
pub struct MessageSignRequest {
    pub message: Option<String>,
    pub secret: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MessageVerifyRequest {
    pub message: Option<String>,
    pub signature: Option<String>,
    pub pubkey: Option<String>,
}

pub async fn handle_message_signing(
    SafeJson(payload): SafeJson<MessageSignRequest>,
) -> ResponseJson<ApiResponse<MessageSignResponse>> {
    println!("🔥 MESSAGE SIGN endpoint called with: {:?}", payload);

    let req = match payload {
        Some(req) => req,
        None => {
            return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
        }
    };

    let message = match req.message {
        Some(val) if !val.is_empty() => val,
        _ => return ResponseJson(ApiResponse::error("Missing required fields".to_string())),
    };

    let secret = match get_required_string(req.secret, "secret") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    if message.len() > 1_000_000 {
        return ResponseJson(ApiResponse::error("Message too long".to_string()));
    }

    let keypair = match parse_secret_key(&secret) {
        Ok(kp) => kp,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let message_bytes = message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    let response = MessageSignResponse {
        signature: general_purpose::STANDARD.encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message,
    };

    ResponseJson(ApiResponse::success(response))
}

pub async fn handle_message_verification(
    SafeJson(payload): SafeJson<MessageVerifyRequest>,
) -> ResponseJson<ApiResponse<MessageVerifyResponse>> {
    println!("🔥 MESSAGE VERIFY endpoint called with: {:?}", payload);

    let req = match payload {
        Some(req) => req,
        None => {
            return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
        }
    };

    let message = match req.message {
        Some(val) if !val.is_empty() => val,
        _ => return ResponseJson(ApiResponse::error("Missing required fields".to_string())),
    };

    let signature_str = match get_required_string(req.signature, "signature") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let pubkey_str = match get_required_string(req.pubkey, "pubkey") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    if message.len() > 1_000_000 {
        return ResponseJson(ApiResponse::error("Message too long".to_string()));
    }

    let pubkey = match validate_pubkey(&pubkey_str) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let signature_bytes = match general_purpose::STANDARD.decode(&signature_str) {
        Ok(bytes) => bytes,
        Err(_) => {
            return ResponseJson(ApiResponse::error("Invalid signature: Invalid base64 encoding".to_string()));
        }
    };

    if signature_bytes.len() != 64 {
        return ResponseJson(ApiResponse::error("Invalid signature: Invalid signature length".to_string()));
    }

    let signature = match <[u8; 64]>::try_from(signature_bytes) {
        Ok(sig_array) => Signature::from(sig_array),
        Err(_) => {
            return ResponseJson(ApiResponse::error("Invalid signature: Invalid signature format".to_string()));
        }
    };

    let message_bytes = message.as_bytes();
    let is_valid = signature.verify(&pubkey.to_bytes(), message_bytes);

    let response = MessageVerifyResponse {
        valid: is_valid,
        message,
        pubkey: pubkey_str,
    };

    ResponseJson(ApiResponse::success(response))
}
