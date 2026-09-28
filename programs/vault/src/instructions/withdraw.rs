use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::{UserVault, SEED, TOKEN_VAULT_SEED};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
		seeds = [SEED, user.key().as_ref(), mint.key().as_ref()],
		bump = vault_state.bump,
		has_one = user,
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

    #[account(mut, token::mint = mint, token::token_program = token_program)]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub user: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let vault = &ctx.accounts.vault_state;
    let seeds = &[
        SEED,
        vault.user.as_ref(),
        vault.mint.as_ref(),
        &[vault.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.vault_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.vault_state.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    Ok(())
}
