pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("14nzAcFk8UseFJqZ9iYBLRqWMUM67jAEmBTnsG3hc6ik");

#[program]
pub mod vault {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
        initialize::init_vault(ctx)
    }
}
