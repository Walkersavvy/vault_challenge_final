use anchor_lang::prelude::*;

declare_id!("8mDnY22HT7JHHbUmRDGSyCX6qp3CMZbokK7Z899zvsuN");

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
