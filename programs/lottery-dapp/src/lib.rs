mod constants;
use crate::constants::*;

use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};

declare_id!("111111111111111111111111111111111111");

#[program]
pub mod lottery_program {
    use super::*;

    pub fn init_master(ctx: Context<InitMaster>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMaster<'info> {
    #[account(
        init,
        payer  = payer,
        space = 4 + 8,
        seeds = [MASTER_SEED.as_bytes()],
        bump,
    )]
    pub master: Account<'info, Master>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Master {
    pub last_id: u32,
}
