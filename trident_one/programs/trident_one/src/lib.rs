use anchor_lang::prelude::*;

declare_id!("Md24hSwkYVniz5pakfn53unmQQGrygZ4uCuuiykAbSH");

#[program]
pub mod trident_one {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
