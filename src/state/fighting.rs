use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct InitializeFighting {
    pub name: String,
    pub room_pin: String,
    pub creator: Pubkey,
    pub fighters: (u8, u8),
    pub winner: u8,
    pub start_time: u64,
    pub end_time: u64,
    pub round: u8,
}

impl InitializeFighting {
 pub fn init_space(name: String, room_pin: String) -> usize {
    let name_length = 4 + name.as_bytes().len(); // длина строки
    let room_pin_len = 4 + room_pin.as_bytes().len(); // длина строки
    let creator_size = std::mem::size_of::<Pubkey>(); // размер Pubkey
    let fighter_size = std::mem::size_of::<u8>(); // размер каждого fighter
    let winner_size = std::mem::size_of::<u8>(); // размер winner
    let start_time_size = std::mem::size_of::<u64>(); // размер u64
    let end_time_size = std::mem::size_of::<u64>(); // размер u64
    let round_size = std::mem::size_of::<u8>(); // размер u8

    name_length +
    room_pin_len + // 4 байта для длины строки + длина строки
    creator_size + 
    (fighter_size * 2) + // два fighters
    winner_size + 
    start_time_size + 
    end_time_size + 
    round_size
 }   
}