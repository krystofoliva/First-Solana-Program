use anchor_lang::prelude::*;

declare_id!("koP6s2ftoKx9G1yqH5xSgAtq7xWvmbbzcFoVXe3zivT");

#[program]
pub mod first_anchor_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
