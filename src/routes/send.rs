use axum::{
    extract::Json,
    response::Json as ResponseJson,
    routing::post,
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use solana_sdk::{pubkey::Pubkey, system_instruction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::transfer;

use crate::{
    routes::token::validate_pubkey,
    types::{
        instruction::{SolSendRequest, TokenSendRequest, SolTransferResponse, TokenTransferResponse, TokenTransferAccount},
        response::ApiResponse,
    },
};

pub fn send_router() -> Router {
    Router::new()
        .route("/sol", post(handle_sol_transfer))
        .route("/token", post(handle_token_transfer))
}

pub async fn handle_sol_transfer(
    Json(req): Json<SolSendRequest>,
) -> ResponseJson<ApiResponse<SolTransferResponse>> {
    if req.from.trim().is_empty() || req.to.trim().is_empty() {
        return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
    }

    if req.lamports == 0 {
        return ResponseJson(ApiResponse::error("Invalid amount: must be greater than 0".to_string()));
    }

    if req.lamports > 1_000_000_000 * 1_000_000_000 {
        return ResponseJson(ApiResponse::error("Invalid amount: amount too large".to_string()));
    }

    let from_pubkey = match validate_pubkey(&req.from) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let to_pubkey = match validate_pubkey(&req.to) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    if from_pubkey == to_pubkey {
    }

    let transfer_ix = system_instruction::transfer(&from_pubkey, &to_pubkey, req.lamports);

    let resp_data = SolTransferResponse {
        program_id: transfer_ix.program_id.to_string(),
        accounts: transfer_ix.accounts.iter().map(|acc| acc.pubkey.to_string()).collect(),
        instruction_data: general_purpose::STANDARD.encode(&transfer_ix.data),
    };

    ResponseJson(ApiResponse::success(resp_data))
}

pub async fn handle_token_transfer(
    Json(req): Json<TokenSendRequest>,
) -> ResponseJson<ApiResponse<TokenTransferResponse>> {
    if req.destination.trim().is_empty() || req.mint.trim().is_empty() || req.owner.trim().is_empty() {
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

    let owner = match validate_pubkey(&req.owner) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let destination = match validate_pubkey(&req.destination) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    if owner == destination {
    }

    let source_token_account = get_associated_token_address(&owner, &mint);
    let dest_token_account = get_associated_token_address(&destination, &mint);

    if source_token_account == dest_token_account {
        return ResponseJson(ApiResponse::error("Cannot transfer to the same token account".to_string()));
    }

    let transfer_ix = match transfer(
        &spl_token::id(),
        &source_token_account,
        &dest_token_account,
        &owner,
        &[], 
        req.amount,
    ) {
        Ok(instruction) => instruction,
        Err(_) => {
            return ResponseJson(ApiResponse::error("Failed to create transfer instruction".to_string()));
        }
    };

    let accounts = transfer_ix
        .accounts
        .iter()
        .map(|acc| TokenTransferAccount {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
        })
        .collect();

    let resp_data = TokenTransferResponse {
        program_id: transfer_ix.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&transfer_ix.data),
    };

    ResponseJson(ApiResponse::success(resp_data))
}