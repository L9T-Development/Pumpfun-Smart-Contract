use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};
use solana_program::{
    sysvar::SysvarId,
    {program::invoke, system_program},
};
use spl_token::instruction::sync_native;

use crate::{
    amm_instruction, constants::{BONDING_CURVE, CONFIG}, errors::PumpfunError, events::MigrateCurveEvent, state::{BondingCurve, Config}, utils::token_burn_user
};

#[derive(Accounts)]
pub struct MigrateCurve<'info> {
  
  /**main part is private*/

}

impl<'info> MigrateCurve<'info> {
    pub fn migrate_curve(&mut self, nonce: u8) -> Result<()> {
        

        //  emit an event
        emit!(MigrateCurveEvent {
            // contact me in Telegram(@shiny0103)
        });

        Ok(())
    }
}
