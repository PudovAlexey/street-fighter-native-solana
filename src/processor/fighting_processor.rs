use borsh::BorshSerialize;
use solana_program::{
    clock::Clock,
    account_info::{next_account_info, AccountInfo}, borsh1::try_from_slice_unchecked, entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar};

use crate::{dto::fighting::InitializeFightingDto, state::fighter::Fighter};
use crate::state::fighting::InitializeFighting;

pub fn process_initialize_fighting(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    account_data: InitializeFightingDto,
)  -> ProgramResult {

    let InitializeFightingDto {
        name, 
        room_pin,
        ..
    } = account_data;

    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    msg!("initialize_fighting: {}", name);

    if !pda_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    let size = InitializeFighting::init_space(name.clone(), room_pin.clone());

    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(size);

    let signers_seeds: &[&[u8]] = &[
        b"init_fighting",
        name.as_bytes(),
        &payer.key.to_bytes(),
    ];

    // let pda = Pubkey::create_program_address(signers_seeds, program_id)?;
    let (pda, bump_seed) = Pubkey::find_program_address(
        signers_seeds,
        program_id,
    );

    msg!("Generated PDA: {:?}", pda);
    msg!("Expected PDA Account Key: {:?}", *pda_account.key);

    invoke_signed(
        &system_instruction::create_account(
            payer.key,          // Account paying for the new account
            pda_account.key,              // Account to be created
            required_lamports,  // Amount of lamports to transfer to the new account
            size as u64,       // Size in bytes to allocate for the data field
            program_id,         // Set program owner to our program
        ),
        &[
            payer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
             b"init_fighting",
            name.as_bytes(),
            &payer.key.to_bytes(),
            &[bump_seed],
        ]],// signer_seeds
    )?;


    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<InitializeFighting>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");
    
    let current_time = Clock::get()?.unix_timestamp;

    account_data.name = name;
    account_data.room_pin = room_pin;
    account_data.creator = Pubkey::default();
    account_data.fighters = (Pubkey::default(), Pubkey::default());
    account_data.winner = Pubkey::default();
    account_data.turn = 0;
    account_data.start_time = current_time;
    account_data.end_time = 0;
    account_data.round = 0;

    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;


    Ok(())
}

pub fn add_fighter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let fighting_pda = next_account_info(accounts_iter)?;
    let fighter_pda = next_account_info(accounts_iter)?;

    
    msg!("trying to get fighter");

    let mut fighter =
        try_from_slice_unchecked::<Fighter>(&fighter_pda.data.borrow()).unwrap();

    let mut fighting_data = try_from_slice_unchecked::<InitializeFighting>(&fighting_pda.data.borrow())?;

    if fighting_data.fighters.0 == Pubkey::default() {
        msg!("trying to add first fighter");
        fighting_data.fighters.0 = fighter_pda.key.to_owned();
        fighter.is_on_fight = true;
    } else if fighting_data.fighters.1 == Pubkey::default() {
        msg!("trying to add second fighter");
        fighting_data.fighters.1 = fighter_pda.key.to_owned();
        fighting_data.round = 1;
        fighter.is_on_fight = true;
    } else {
        return Err(ProgramError::InvalidAccountData);
    }

    fighting_data.serialize(&mut &mut fighting_pda.data.borrow_mut()[..])?;
    fighter.serialize(&mut &mut fighter_pda.data.borrow_mut()[..])?;
    
    Ok(())
}