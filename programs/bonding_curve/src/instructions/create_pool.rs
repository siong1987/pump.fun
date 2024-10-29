use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, set_authority, Mint, MintTo, SetAuthority, Token, TokenAccount},
};

pub fn create_pool(ctx: Context<CreateLiquidityPool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let pool_mint_key = ctx.accounts.pool_mint.key();
    let pool_bump_seed = ctx.bumps.pool;
    let pool_signer_seeds: &[&[&[u8]]] = &[&[
        LiquidityPool::POOL_SEED_PREFIX.as_bytes(),
        pool_mint_key.as_ref(),
        &[pool_bump_seed],
    ]];

    let token_program = &ctx.accounts.token_program;

    // Mint tokens to the pool token account
    let mint_cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.pool_mint.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: pool.to_account_info(),
        },
        pool_signer_seeds,
    );
    mint_to(mint_cpi_ctx, 10000000000000)?; // TODO: check how much

    // Give up the mint authority
    let give_up_authority_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        SetAuthority {
            current_authority: pool.to_account_info(),
            account_or_mint: ctx.accounts.pool_mint.to_account_info(),
        },
        pool_signer_seeds,
    );
    set_authority(
        give_up_authority_ctx,
        spl_token::instruction::AuthorityType::MintTokens,
        None,
    )?;

    pool.set_inner(LiquidityPool::new(
        ctx.accounts.payer.key(),
        ctx.accounts.pool_mint.key(),
        ctx.bumps.pool,
    ));
    Ok(())
}

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    #[account(
        init,
        space = LiquidityPool::ACCOUNT_SIZE,
        payer = payer,
        seeds = [LiquidityPool::POOL_SEED_PREFIX.as_bytes(), pool_mint.key().as_ref()],
        bump
    )]
    pub pool: Box<Account<'info, LiquidityPool>>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 9, // TODO: check how many
        mint::authority = pool,
    )]
    pub pool_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = pool_mint,
        associated_token::authority = pool
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
