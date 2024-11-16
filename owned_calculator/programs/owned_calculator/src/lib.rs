use anchor_lang::prelude::*;

declare_id!("641bKqcLjVwpPCbMRhAq2ZZJdJaTvEik28qJ8jjSDHJw");

#[program]
pub mod owned_calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
