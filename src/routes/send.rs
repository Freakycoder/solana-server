use axum::response::Json as ResponseJson;
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use solana_sdk::system_instruction;
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::transfer;

use crate::{
    types::{
        request::{SafeJson, get_required_string, get_required_u64},
        response::{ApiResponse, SolTransferResponse, TokenTransferResponse, TokenTransferAccount},
    },
    utils::{validate_pubkey, validate_amount},
};

#[derive(Deserialize, Debug)]
pub struct SolSendRequest {
    pub from: Option<String>,
    pub to: Option<String>,
    pub lamports: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct TokenSendRequest {
    pub destination: Option<String>,
    pub mint: Option<String>,
    pub owner: Option<String>,
    pub amount: Option<u64>,
}

pub async fn handle_sol_transfer(
    SafeJson(payload): SafeJson<SolSendRequest>,
) -> ResponseJson<ApiResponse<SolTransferResponse>> {
    println!("🔥 SEND SOL endpoint called with: {:?}", payload);

    let req = match payload {
        Some(req) => req,
        None => {
            return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
        }
    };

    let from = match get_required_string(req.from, "from") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let to = match get_required_string(req.to, "to") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let lamports = match get_required_u64(req.lamports, "lamports") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let validated_lamports = match validate_amount(lamports, Some(1_000_000_000 * 1_000_000_000)) {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let from_pk = match validate_pubkey(&from) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let to_pk = match validate_pubkey(&to) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let instruction = system_instruction::transfer(&from_pk, &to_pk, validated_lamports);

    let response = SolTransferResponse {
        program_id: instruction.program_id.to_string(),
        accounts: instruction.accounts.iter().map(|acc| acc.pubkey.to_string()).collect(),
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    ResponseJson(ApiResponse::success(response))
}

pub async fn handle_token_transfer(
    SafeJson(payload): SafeJson<TokenSendRequest>,
) -> ResponseJson<ApiResponse<TokenTransferResponse>> {
    println!("🔥 SEND TOKEN endpoint called with: {:?}", payload);

    let req = match payload {
        Some(req) => req,
        None => {
            return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
        }
    };

    let destination = match get_required_string(req.destination, "destination") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let mint = match get_required_string(req.mint, "mint") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let owner = match get_required_string(req.owner, "owner") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let amount = match get_required_u64(req.amount, "amount") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let validated_amount = match validate_amount(amount, Some(u64::MAX / 2)) {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let destination_pk = match validate_pubkey(&destination) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let mint_pk = match validate_pubkey(&mint) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let owner_pk = match validate_pubkey(&owner) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let source_token_account = get_associated_token_address(&owner_pk, &mint_pk);
    let dest_token_account = get_associated_token_address(&destination_pk, &mint_pk);

    if source_token_account == dest_token_account {
        return ResponseJson(ApiResponse::error("Cannot transfer to the same token account".to_string()));
    }

    let instruction = match transfer(
        &spl_token::id(),
        &source_token_account,
        &dest_token_account,
        &owner_pk,
        &[],
        validated_amount,
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return ResponseJson(ApiResponse::error("Failed to create transfer instruction".to_string()));
        }
    };

    let accounts = instruction
        .accounts
        .iter()
        .map(|acc| TokenTransferAccount {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
        })
        .collect();

    let response = TokenTransferResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    ResponseJson(ApiResponse::success(response))
}
