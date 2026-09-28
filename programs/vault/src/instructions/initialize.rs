use crate::{UserVault, SEED, TOKEN_VAULT_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + UserVault::INIT_SPACE,
        seeds = [SEED, owner.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, UserVault>,

    #[account(
        init,
        payer = owner,
        token::mint = mint,
        token::authority = vault_state,
        token::token_program = token_program,
        seeds = [TOKEN_VAULT_SEED, vault_state.key().as_ref()],
        bump
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault_state;
    vault.user = ctx.accounts.owner.key();
    vault.mint = ctx.accounts.mint.key();
    vault.bump = ctx.bumps.vault_state;

    Ok(())
}
