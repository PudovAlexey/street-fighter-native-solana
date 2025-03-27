use {
    num_derive::FromPrimitive,
    solana_program::{decode_error::DecodeError, program_error::ProgramError},
    thiserror::Error,
};


#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum StreetFighterError {
    #[error("You cannot bite the fighter because the fight was already over")]
   TheFightIsOverError,
}

impl From<StreetFighterError> for ProgramError {
    fn from(e: StreetFighterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}