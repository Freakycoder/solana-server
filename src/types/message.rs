use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MessageSignRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Deserialize, Serialize)]
pub struct MessageSignResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct MessageVerifyRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Deserialize, Serialize)]
pub struct MessageVerifyResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}