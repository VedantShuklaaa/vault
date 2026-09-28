use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::{SEED, TOKEN_VAULT_SEED, UserVault};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
		seeds = [SEED, owner.key().as_ref(), mint.key().as_ref()],
		bump = vault_state.bump
	)]
    pub vault_state: Account<'info, UserVault>,

    #[account(
		mut,
		seeds = [TOKEN_VAULT_SEED, vault_state.key().as_ref()],
		bump,
        token::mint = mint,
        token::authority = vault_state,
        token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
		mut,
		token::mint = mint,
		token::authority = owner,
		token::token_program = token_program,
	)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub owner: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn deposit(ctx: Context<Deposit>, amount_in: u64) -> Result<()> {
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.user_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount_in,
        ctx.accounts.mint.decimals,
    )?;

    Ok(())
}
