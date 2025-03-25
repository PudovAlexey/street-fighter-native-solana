use borsh::BorshDeserialize;
use crate::dto::{fighter::FighterDto, fighting::InitializeFightingDto};
use solana_program::program_error::ProgramError;

pub enum FighterInstructions {
    InitFighting(InitializeFightingDto),
    InitFighter(FighterDto),
    AddFighter
    // InitFighter {name: String, gender: String, health: u32, atack: u32},
    // DeleteFighter {name: String},
    // BiteFighter {from_name: String, to_name: String},
}

impl FighterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Получаем тип инструкции из первого байта
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match variant {
            0 => {
                let fighting = InitializeFightingDto::try_from_slice(rest)?;

                Ok(Self::InitFighting(InitializeFightingDto {
                    name: fighting.name,
                    room_pin: fighting.room_pin,
                }))
            },
            1 => {
                let fighter = FighterDto::try_from_slice(rest)?;

                Ok(Self::InitFighter(FighterDto {
                    name: fighter.name,
                    bump: fighter.bump,
                    gender: fighter.gender,
                    health: fighter.health,
                    attack: fighter.attack,
                }))
            },
            2 => {
                Ok(Self::AddFighter)
            }
            _ => Err(ProgramError::InvalidInstructionData),
            // 0 => {
            //     // Десериализуем Fighter из оставшихся байтов
            //     let fighter = Figther::try_from_slice(rest)?;

            //     Ok(Self::InitFighter {
            //         name: fighter.name,
            //         gender: fighter.gender,
            //         health: fighter.health,
            //         atack: fighter.atack,
            //     })
            // },
            // 1 => {
            //     let fighter = DeleteFighterByName::try_from_slice(rest)?;

            //     Ok(Self::DeleteFighter {
            //      name: fighter.name   
            //     }
            //     )
            // },
            // 2 => {
            //     let fighter = ProcessBitePerson::try_from_slice(rest)?;

            //     Ok(Self::BiteFighter {
            //         from_name: fighter.from_name,
            //         to_name: fighter.to_name,
            //     })
            // }

            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}