use anchor_lang::prelude::*;

declare_id!("B6vGDhganemog1R3f98vCj3ZYzzg1gMHeafgsCsB2mPY");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
