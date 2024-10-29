use crate::{errors::CustomError, sell::MAX_FEE_BPS, state::*};
use anchor_lang::prelude::*;

pub fn initialize(ctx: Context<InitializeCurveConfiguration>, fee_bps: u64) -> Result<()> {
    let dex_config = &mut ctx.accounts.dex_configuration_account;

    if fee_bps > MAX_FEE_BPS {
        return err!(CustomError::InvalidFee);
    }

    dex_config.set_inner(CurveConfiguration::new(fee_bps));

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCurveConfiguration<'info> {
    #[account(
        init,
        space = CurveConfiguration::ACCOUNT_SIZE,
        payer = admin,
        seeds = [CurveConfiguration::SEED.as_bytes()],
        bump,
    )]
    pub dex_configuration_account: Box<Account<'info, CurveConfiguration>>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
