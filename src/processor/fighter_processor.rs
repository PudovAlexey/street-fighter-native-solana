use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo}, borsh1::try_from_slice_unchecked, entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar
};

use crate::{dto::fighter::FighterDto, state::fighter::Fighter};


pub fn process_initialize_fighter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_value: FighterDto,
) -> ProgramResult {
    let FighterDto { name, gender, health, attack, bump, .. } = initial_value.clone();
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    msg!("Initializing fighter with name: {}, gender: {}, health: {}, attack: {}",
         name, gender, health, attack);

    // Проверка, что аккаунт PDA записываемый
    if !pda_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
 
    let size = Fighter::init_space(initial_value);

    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(size);

    // Генерация bump
    let signers_seeds: &[&[u8]] = &[
        b"fighter",
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


    if pda != *pda_account.key {
        msg!("invalid seeds for PDA");
        return Ok(())
    }

    // Создание аккаунта
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
            b"fighter",
            name.as_bytes(),
            &payer.key.to_bytes(),
            &[bump_seed],
        ]],// signer_seeds
    )?;

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<Fighter>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    account_data.is_on_fight = false;
    account_data.bump = bump;
    account_data.owner = payer.key.to_owned();
    account_data.name = name;
    account_data.gender = gender;
    account_data.health = health;
    account_data.attack = attack;

    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}

// pub fn process_delete_fighter(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     account_data: DeleteFighterByName,
// ) -> ProgramResult {
//     let DeleteFighterByName { name } = account_data;
//     let accounts_iter = &mut accounts.iter();
//     let payer = next_account_info(accounts_iter)?;
//     let pda_account = next_account_info(accounts_iter)?;

//     // Проверка, подписал ли транзакцию инициатор
//     if !payer.is_signer {
//         msg!("initializer didn't sign the tx");
//         return Err(ProgramError::MissingRequiredSignature);
//     }

//     // Генерация PDA
//     let signers_seeds: &[&[u8]] = &[
//         b"name",
//         name.as_bytes(),
//         &payer.key.to_bytes(),
//     ];

//     let (pda, _bump) = Pubkey::find_program_address(
//         signers_seeds,
//         program_id,
//     );

//     // Проверка, что переданный PDA соответствует ожидаемому
//     if pda != *pda_account.key {
//         msg!("invalid PDA account passed");
//         return Err(ProgramError::InvalidArgument);
//     }

//     // Проверка, что PDA аккаунт не пустой
//     if **pda_account.lamports.borrow() == 0 {
//         msg!("PDA account is already empty");
//         return Err(ProgramError::InvalidAccountData);
//     }

//     // Получение количества лемпортов в PDA
//     let pda_balance = **pda_account.lamports.borrow();

//     // Перемещение лемпортов из PDA обратно в инициатора
//     **payer.lamports.borrow_mut() += pda_balance;
//     **pda_account.lamports.borrow_mut() = 0; // Обнуляем баланс PDA

//     // При очистке данных в аккаунте можно использовать:
//     // *pda_account.try_borrow_mut_data()? = &mut []; // Это обнулит данные
    
//     // Однако, скорее всего, лучше просто оставить его, если вы собираетесь повторно использовать его.

//     Ok(())
// }

// pub fn process_bite_person(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     program_data: ProcessBitePerson
// ) -> ProgramResult {
//     let ProcessBitePerson {from_name, to_name} = program_data;
//     let accounts_iter = &mut accounts.iter();

//     let payer = next_account_info(accounts_iter);

//     let from_pda_account = next_account_info(accounts_iter)?;
//     let to_pda_account = next_account_info(accounts_iter)?;

//     // Загрузка данных о бойцах
//     let mut from_fighter =
//         try_from_slice_unchecked::<Figther>(&from_pda_account.data.borrow()).unwrap();
//         let mut to_fighter =
//         try_from_slice_unchecked::<Figther>(&to_pda_account.data.borrow()).unwrap();
        
//         if from_fighter.atack > to_fighter.health {
//             to_fighter.health = 0; // Убиваем бойца
//         } else {
//             to_fighter.health -= from_fighter.atack; // Уменьшаем здоровье
//         }

//         to_fighter.serialize(&mut &mut to_pda_account.data.borrow_mut()[..])?;

//         msg!("{} attacked {}. Remaining health: {}", from_fighter.name, to_fighter.name, to_fighter.health);


//     Ok(())
// }