use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SolSendRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Deserialize, Serialize)]
pub struct TokenSendRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Deserialize, Serialize)]
pub struct InstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountData>,
    pub instruction_data: String,
}

#[derive(Deserialize, Serialize)]
pub struct AccountData {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct SolTransferResponse {
    pub program_id: String,
    pub accounts: Vec<String>, 
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct TokenTransferAccount {
    pub pubkey: String,
    #[serde(rename = "isSigner")] 
    pub is_signer: bool,
}

#[derive(Serialize)]
pub struct TokenTransferResponse {
    pub program_id: String,
    pub accounts: Vec<TokenTransferAccount>,
    pub instruction_data: String,
}