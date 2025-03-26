use solana_program::{program_error::ProgramError, pubkey::Pubkey};

pub fn validate_participants(
    form_fighter_pda: &Pubkey,
    to_fighter_pda: &Pubkey,
    fighters_in_fight: (&Pubkey, &Pubkey),
) -> bool {
    fighters_in_fight.0.eq(form_fighter_pda) || fighters_in_fight.1.eq(form_fighter_pda) && 
    fighters_in_fight.0.eq(to_fighter_pda) || fighters_in_fight.1.eq(to_fighter_pda)
}

pub struct RefillBalanceData {
    pub new_health: u32,
    pub transaction_fee: u64,
}

pub fn refill_health(current_health: u32, new_health: u32) -> Result<RefillBalanceData, ProgramError> {
    let payment_per_one = 500;
    let after_refill_health = current_health + new_health;

    if after_refill_health <= 100 {
        Ok(RefillBalanceData {
            new_health: after_refill_health,
            transaction_fee: (new_health * payment_per_one) as u64,
        })
    } else {
        Err(ProgramError::AccountAlreadyInitialized)
    }
}