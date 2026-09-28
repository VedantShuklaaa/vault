use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserVault {
	pub user: Pubkey,
	pub mint: Pubkey,
	pub bump: u8,
}