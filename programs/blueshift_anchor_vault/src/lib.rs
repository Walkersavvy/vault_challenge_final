use anchor_lang::prelude::*;

declare_id!("G71c7FU5AVBkgZ9vvBpkmGG9QY3V8iczQBV8fdx1d3zG");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
