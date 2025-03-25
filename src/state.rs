// use borsh::{BorshDeserialize, BorshSerialize};
// use solana_program::pubkey::Pubkey;

// #[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
// pub struct Figther {
//     pub bump: u8,
//     pub owner: Pubkey,
//     pub name: String,
//     pub gender: String,
//     pub health: u32,
//     pub atack: u32,
// }

// #[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
// pub struct DeleteFighterByName {
//     pub name: String,
// }

// #[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
// pub struct ProcessBitePerson {
//     pub from_name: String,
//     pub to_name: String,
// }
pub mod fighter;
pub mod fighting;