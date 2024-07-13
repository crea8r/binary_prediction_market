use anchor_lang::prelude::*;

declare_id!("BApvHuZfD66YRnyXEeeorTQpAKBkTcsxTi4gavEvJkAu");

#[program]
pub mod binary_predictionm_market {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
