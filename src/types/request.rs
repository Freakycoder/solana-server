use axum::{
    extract::{ FromRequest, Request},
    response::Json as ResponseJson,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::types::response::ApiResponse;

pub struct SafeJson<T>(pub Option<T>);

#[axum::async_trait]
impl<T, S> FromRequest<S> for SafeJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ResponseJson<ApiResponse<()>>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = match axum::body::to_bytes(req.into_body(), usize::MAX).await {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(ResponseJson(ApiResponse::error(
                    "Missing required fields".to_string(),
                )));
            }
        };

        if bytes.is_empty() {
            return Ok(SafeJson(None));
        }

        let json_value: Value = match serde_json::from_slice(&bytes) {
            Ok(value) => value,
            Err(_) => {
                return Err(ResponseJson(ApiResponse::error(
                    "Missing required fields".to_string(),
                )));
            }
        };

        match serde_json::from_value::<T>(json_value) {
            Ok(value) => Ok(SafeJson(Some(value))),
            Err(_) => {
                Ok(SafeJson(None))
            }
        }
    }
}

pub fn get_required_string(opt: Option<String>, field_name: &str) -> Result<String, String> {
    match opt {
        Some(val) if !val.trim().is_empty() => Ok(val.trim().to_string()),
        _ => Err("Missing required fields".to_string()),
    }
}

pub fn get_required_u64(opt: Option<u64>, field_name: &str) -> Result<u64, String> {
    match opt {
        Some(val) => Ok(val),
        None => Err("Missing required fields".to_string()),
    }
}

pub fn get_required_u8(opt: Option<u8>, field_name: &str) -> Result<u8, String> {
    match opt {
        Some(val) => Ok(val),
        None => Err("Missing required fields".to_string()),
    }
}
