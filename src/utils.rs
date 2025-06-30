use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair};
use std::str::FromStr;
use base64::{engine::general_purpose, Engine as _};

use crate::types::response::{InstructionResponse, AccountInfo};

pub fn validate_pubkey(pubkey_str: &str) -> Result<Pubkey, String> {
    let trimmed = pubkey_str.trim();
    if trimmed.is_empty() {
        return Err("Invalid public key".to_string());
    }

    if trimmed.len() < 32 || trimmed.len() > 50 {
        return Err("Invalid public key".to_string());
    }

    if trimmed.contains(&['0', 'O', 'I', 'l'][..]) {
        return Err("Invalid public key".to_string());
    }

    match Pubkey::from_str(trimmed) {
        Ok(pk) => {
            if pk == Pubkey::default() {
                return Err("Invalid public key".to_string());
            }
            Ok(pk)
        }
        Err(_) => Err("Invalid public key".to_string()),
    }
}

pub fn convert_instruction_to_response(instruction: Instruction) -> InstructionResponse {
    let accounts = instruction
        .accounts
        .iter()
        .map(|acc| AccountInfo {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    }
}

pub fn parse_secret_key(secret_str: &str) -> Result<Keypair, String> {
    let trimmed = secret_str.trim();
    
    if trimmed.is_empty() {
        return Err("Invalid private key".to_string());
    }

    if trimmed.len() < 80 || trimmed.len() > 90 {
        return Err("Invalid private key".to_string());
    }

    let decoded = bs58::decode(trimmed)
        .into_vec()
        .map_err(|_| "Invalid private key".to_string())?;

    if decoded.len() != 64 {
        return Err("Invalid private key".to_string());
    }

    Keypair::from_bytes(&decoded).map_err(|_| "Invalid private key".to_string())
}

pub fn validate_amount(amount: u64, max_allowed: Option<u64>) -> Result<u64, String> {
    if amount == 0 {
        return Err("Invalid amount: must be greater than 0".to_string());
    }

    if let Some(max) = max_allowed {
        if amount > max {
            return Err("Invalid amount: amount too large".to_string());
        }
    }

    Ok(amount)
}

pub fn validate_decimals(decimals: u8) -> Result<u8, String> {
    if decimals > 9 {
        return Err("Invalid decimals: must be between 0 and 9".to_string());
    }
    Ok(decimals)
}
