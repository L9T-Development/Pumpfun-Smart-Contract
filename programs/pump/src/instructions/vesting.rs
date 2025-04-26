use crate::{errors::CustomError, state::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[derive(Accounts)]
pub struct InitializeVesting<'info> {
    #[account(
        init,
        space = VestingAccount::ACCOUNT_SIZE,
        payer = admin,
        seeds = [VestingAccount::SEED.as_bytes()],
        bump,
    )]
    pub vesting_account: Box<Account<'info, VestingAccount>>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VestingAccount {
    pub total_amount: u64,
    pub vested_amount: u64,
    pub start_time: i64,
    pub cliff_duration: u64,
    pub vesting_duration: u64,
    pub beneficiary: Pubkey,
    pub bump: u8,
}

impl VestingAccount {
    pub const ACCOUNT_SIZE: usize = 8 // discriminator
        + 8 // total_amount
        + 8 // vested_amount
        + 8 // start_time
        + 8 // cliff_duration
        + 8 // vesting_duration
        + 32 // beneficiary
        + 1; // bump

    pub const SEED: &'static str = "vesting_account";

    pub fn new(
        total_amount: u64,
        cliff_duration: u64,
        vesting_duration: u64,
        beneficiary: Pubkey,
        bump: u8,
    ) -> Self {
        Self {
            total_amount,
            vested_amount: 0,
            start_time: Clock::get().unwrap().unix_timestamp,
            cliff_duration,
            vesting_duration,
            beneficiary,
            bump,
        }
    }

    pub fn calculate_vested_amount(&self) -> u64 {
        let current_time = Clock::get().unwrap().unix_timestamp;

        if current_time < self.start_time + self.cliff_duration as i64 {
            return 0;
        }

        let elapsed_time = current_time - self.start_time;
        if elapsed_time >= self.vesting_duration as i64 {
            return self.total_amount;
        }

        let vested = (self.total_amount as u128 * elapsed_time as u128 / self.vesting_duration as u128) as u64;
        vested
    }

    pub fn claim_tokens(&mut self) -> Result<u64> {
        let vested = self.calculate_vested_amount();
        let claimable = vested.saturating_sub(self.vested_amount);

        if claimable == 0 {
            return err!(CustomError::NoTokensAvailable);
        }

        self.vested_amount += claimable;
        Ok(claimable)
    }
}

pub fn initialize_vesting(
    ctx: Context<InitializeVesting>,
    total_amount: u64,
    cliff_duration: u64,
    vesting_duration: u64,
) -> Result<()> {
    let vesting_account = &mut ctx.accounts.vesting_account;

    vesting_account.total_amount = total_amount;
    vesting_account.cliff_duration = cliff_duration;
    vesting_account.vesting_duration = vesting_duration;
    vesting_account.beneficiary = ctx.accounts.admin.key();
    vesting_account.bump = *ctx.bumps.get("vesting_account").unwrap();

    // Optional: Transfer tokens to vesting_account here if necessary.

    Ok(())
}
