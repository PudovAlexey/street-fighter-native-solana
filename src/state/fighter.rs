use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::dto::fighter::FighterDto;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Fighter {
    pub is_on_fight: bool,
    pub bump: u8,
    pub owner: Pubkey,
    pub name: String,
    pub gender: String,
    pub health: u32,
    pub attack: u32,
}

impl Fighter {
    pub fn init_space(dto: FighterDto) -> usize {
        let is_on_fight = std::mem::size_of::<bool>();
        let bump = std::mem::size_of::<u8>();
        let owner = std::mem::size_of::<Pubkey>();
        let name = 4 + dto.name.as_bytes().len();
        let gender = 4 + dto.gender.as_bytes().len();
        let health = std::mem::size_of::<u32>();
        let attack = std::mem::size_of::<u32>();

        is_on_fight +
        bump +
        owner +
        name +
        gender +
        health +
        attack
    }
}