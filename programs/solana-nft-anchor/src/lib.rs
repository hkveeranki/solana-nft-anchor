use anchor_lang::prelude::*;

declare_id!("GgetUDPNoFhHFt85BHmHJtWZ7iBBdgWiXHxeaN9ikSfL");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
