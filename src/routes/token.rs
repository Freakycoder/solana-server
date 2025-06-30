use axum::response::Json as ResponseJson;
use serde::Deserialize;
use spl_token::instruction::{initialize_mint, mint_to};

use crate::{
    types::{
        request::{SafeJson, get_required_string, get_required_u64, get_required_u8},
        response::{ApiResponse, InstructionResponse},
    },
    utils::{validate_pubkey, convert_instruction_to_response, validate_amount, validate_decimals},
};

#[derive(Deserialize, Debug)]
pub struct TokenCreateRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: Option<String>,
    pub mint: Option<String>,
    pub decimals: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct TokenMintRequest {
    pub mint: Option<String>,
    pub destination: Option<String>,
    pub authority: Option<String>,
    pub amount: Option<u64>,
}

pub async fn handle_token_creation(
    SafeJson(payload): SafeJson<TokenCreateRequest>,
) -> ResponseJson<ApiResponse<InstructionResponse>> {
    println!("🔥 TOKEN CREATE endpoint called with: {:?}", payload);

    let req = match payload {
        Some(req) => req,
        None => {
            return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
        }
    };

    let mint_authority = match get_required_string(req.mint_authority, "mintAuthority") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let mint = match get_required_string(req.mint, "mint") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let decimals = match get_required_u8(req.decimals, "decimals") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let validated_decimals = match validate_decimals(decimals) {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let mint_authority_pk = match validate_pubkey(&mint_authority) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let mint_pk = match validate_pubkey(&mint) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let instruction = match initialize_mint(
        &spl_token::id(),
        &mint_pk,
        &mint_authority_pk,
        Some(&mint_authority_pk),
        validated_decimals,
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return ResponseJson(ApiResponse::error("Failed to create mint instruction".to_string()));
        }
    };

    let response = convert_instruction_to_response(instruction);
    ResponseJson(ApiResponse::success(response))
}

pub async fn handle_token_minting(
    SafeJson(payload): SafeJson<TokenMintRequest>,
) -> ResponseJson<ApiResponse<InstructionResponse>> {
    println!("🔥 TOKEN MINT endpoint called with: {:?}", payload);

    let req = match payload {
        Some(req) => req,
        None => {
            return ResponseJson(ApiResponse::error("Missing required fields".to_string()));
        }
    };

    let mint = match get_required_string(req.mint, "mint") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let destination = match get_required_string(req.destination, "destination") {
        Ok(val) => val,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let authority = match get_required_string(req.authority, "authority") {
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

    let mint_pk = match validate_pubkey(&mint) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let destination_pk = match validate_pubkey(&destination) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let authority_pk = match validate_pubkey(&authority) {
        Ok(pk) => pk,
        Err(e) => return ResponseJson(ApiResponse::error(e)),
    };

    let instruction = match mint_to(
        &spl_token::id(),
        &mint_pk,
        &destination_pk,
        &authority_pk,
        &[],
        validated_amount,
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return ResponseJson(ApiResponse::error("Failed to create mint_to instruction".to_string()));
        }
    };

    let response = convert_instruction_to_response(instruction);
    ResponseJson(ApiResponse::success(response))
}
