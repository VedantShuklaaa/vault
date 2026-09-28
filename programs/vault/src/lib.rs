pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FGLrp9iRiRMVgWF3zEiTjQ6H7dCXyjbDmtwxiAbC6qaP");

#[program]
pub mod vault {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
        initialize::init_vault(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount_in: u64) -> Result<()> {
        deposit::deposit(ctx, amount_in)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw::withdraw(ctx, amount)
    }
}
