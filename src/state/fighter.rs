use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::dto::fighter::FighterDto;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Fighter {
    pub is_on_fight: bool,
    pub owner: Pubkey,
    pub name: String,
    pub gender: String,
    pub health: u32,
    pub attack: u32,
}

impl Fighter {
    pub fn init_space(dto: FighterDto) -> usize {
        let is_on_fight = std::mem::size_of::<bool>();
        let owner = std::mem::size_of::<Pubkey>();
        let name = 4 + dto.name.as_bytes().len();
        let gender = 4 + dto.gender.as_bytes().len();
        let health = std::mem::size_of::<u32>();
        let attack = std::mem::size_of::<u32>();

        is_on_fight +
        owner +
        name +
        gender +
        health +
        attack
    }

    // pub fn refill_health(dto: RefillHealthDto) -> u64 {
    //     let one_health_per_lamport = 500;
    //     let RefillHealthDto { health } = dto;

    //     let after_refill_health = self.health + health;

    //     if after_refill_health > 100 {
    //         self.health = 100;
    //         let refiled_amount = 100 * one_health_per_lamport;
    //     } else {
    //         self.health = after_refill_health;
    //         let refiled_amount = health * one_health_per_lamport;
    //     }
    //     // let coast_per_one_health = 500;

    //     500
    // }
}