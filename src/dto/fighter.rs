use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct FighterDto {
   pub name: String,
   pub gender: String,
   pub health: u32,
   pub attack: u32,
   pub bump: u8
}