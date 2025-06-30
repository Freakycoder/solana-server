use crate::types::{
    instruction::{AccountData, InstructionResponse},
    response::ApiResponse,
    token::{TokenCreateRequest, TokenMintRequest},
};
use axum::{
    extract::Json,
    response::Json as ResponseJson,
    routing::post,
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use spl_token::instruction::{initialize_mint, mint_to};
use std::str::FromStr;

pub fn token_router() -> Router {
    Router::new()
        .route("/create", post(handle_token_creation))
        .route("/mint", post(handle_token_minting))
}

pub async fn handle_token_creation(
    Json(req): Json<TokenCreateRequest>,
) -> ResponseJson<ApiResponse<InstructionResponse>> {
    if req.mint_authority.trim().is_empty() || req.mint.trim().is_empty() {
        return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
    }

    if req.decimals > 9 {
        return ResponseJson(ApiResponse::error("Invalid decimals: must be between 0 and 9".to_string()));
    }

    let mint_auth = match validate_pubkey(&req.mint_authority) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let mint_pubkey = match validate_pubkey(&req.mint) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    if mint_pubkey == mint_auth {
    }

    let instruction = match initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &mint_auth,
        Some(&mint_auth), 
        req.decimals,
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return ResponseJson(ApiResponse::error("Failed to create mint instruction".to_string()));
        }
    };

    let resp_data = convert_instruction(instruction);
    ResponseJson(ApiResponse::success(resp_data))
}

pub async fn handle_token_minting(
    Json(req): Json<TokenMintRequest>,
) -> ResponseJson<ApiResponse<InstructionResponse>> {
    if req.mint.trim().is_empty() || req.destination.trim().is_empty() || req.authority.trim().is_empty() {
        return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
    }

    if req.amount == 0 {
        return ResponseJson(ApiResponse::error("Invalid amount: must be greater than 0".to_string()));
    }

    if req.amount > u64::MAX / 2 {
        return ResponseJson(ApiResponse::error("Invalid amount: amount too large".to_string()));
    }

    let mint = match validate_pubkey(&req.mint) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let dest = match validate_pubkey(&req.destination) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let authority = match validate_pubkey(&req.authority) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let instruction = match mint_to(
        &spl_token::id(),
        &mint,
        &dest,
        &authority,
        &[], 
        req.amount,
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return ResponseJson(ApiResponse::error("Failed to create mint_to instruction".to_string()));
        }
    };

    let resp_data = convert_instruction(instruction);
    ResponseJson(ApiResponse::success(resp_data))
}

pub fn validate_pubkey(pubkey_str: &str) -> Result<Pubkey, String> {
    let trimmed = pubkey_str.trim();
    if trimmed.is_empty() {
        return Err("Invalid public key: empty or whitespace".to_string());
    }

    if trimmed.len() < 32 || trimmed.len() > 50 {
        return Err(format!("Invalid public key: {}", pubkey_str));
    }

    match Pubkey::from_str(trimmed) {
        Ok(pk) => {
            if pk == Pubkey::default() {
                return Err("Invalid public key: cannot use default pubkey".to_string());
            }
            Ok(pk)
        }
        Err(_) => Err(format!("Invalid public key: {}", pubkey_str)),
    }
}

pub fn convert_instruction(inst: Instruction) -> InstructionResponse {
    let account_list: Vec<AccountData> = inst
        .accounts
        .iter()
        .map(|acc| AccountData {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    InstructionResponse {
        program_id: inst.program_id.to_string(),
        accounts: account_list,
        instruction_data: general_purpose::STANDARD.encode(&inst.data),
    }
}