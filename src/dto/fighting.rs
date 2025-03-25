use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitializeFightingDto { 
    pub name: String,
    pub room_pin: String,
}

pub struct AddFigther {
    pub fighter: u8
}