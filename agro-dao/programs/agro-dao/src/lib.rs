pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HWjwngNibn1coAzqLZhg4huw5pH5gNZY8zxJaK7s3Hbj");

#[program]
pub mod agro_dao {
    use super::*;

   // In programs/agro-dao/src/lib.rs
    pub fn initialize_protocol(ctx: Context<InitializeProtocol>) -> Result<()> {
        ctx.accounts.initialize_protocol(&ctx.bumps) 
}

    pub fn update_protocol(
        ctx: Context<UpdateProtocol>, 
        params: UpdateProtocolParams
    ) -> Result<()> {
        ctx.accounts.update_protocol(params)
    }
}
