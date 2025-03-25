use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    pubkey::Pubkey,
    entrypoint,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
};
use crate::{instructions::FighterInstructions, processor::{fighter_processor::process_initialize_fighter, fighting_processor::{add_fighter, process_initialize_fighting}}};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = FighterInstructions::unpack(instruction_data)?;

    match instruction  {
        FighterInstructions::InitFighting(initialFightingData) => {
            process_initialize_fighting(program_id, accounts, initialFightingData)?
        }
        FighterInstructions::InitFighter(fightingData) => {
            process_initialize_fighter(program_id, accounts, fightingData)?
        }
        FighterInstructions::AddFighter => {
            add_fighter(program_id, accounts)?
        }
        // FighterInstructions::InitFighter { name, gender,health, atack,  } => {
        //     process_initialize_fighter(program_id, accounts, Figther {
        //         name, gender,health, atack,
        //     })?
        // },
        // FighterInstructions::DeleteFighter { name } => {
        //     process_delete_fighter(program_id, accounts, DeleteFighterByName {
        //         name,
        //     })?
        // },
        // FighterInstructions::BiteFighter { from_name, to_name } => {
        //     process_bite_person(program_id, accounts, ProcessBitePerson {
        //         from_name,
        //         to_name,
        //     })?
        // },
        // FighterInstructions::InitFighting {name, creator} => {
        //     let result = InitializeFighting {
        //         name, 
        //         creator,
        //         fighters: (Pubkey::default(), Pubkey::default()),
        //         winner: Pubkey::default(),
        //         start_time: 0,
        //         end_time: 0,
        //         round: 0
        //     };

        //     process_initialize_fighting(program_id, accounts, result)?
        // }
    }
    // let instruction = FavoritesInstruction::try_from_slice(instruction_data)?;

    // create_pda(program_id, accounts, instruction_data)?;

    // match instruction {
    //     FavoritesInstruction::CreatePda(data) => create_pda(program_id, accounts, data),
    //     FavoritesInstruction::GetPda => get_pda(program_id,accounts),
    // }?;

    
//    msg!("Hello, world!");
//    msg!("My instruction data is: {:?}", instruction_data);

   Ok(())
}