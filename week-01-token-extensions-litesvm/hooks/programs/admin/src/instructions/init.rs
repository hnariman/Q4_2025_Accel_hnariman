use anchor_lang::prelude::*;

use crate::state::Config;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer=admin,
        space= 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl<'info> Init<'info> {
    pub fn init(&mut self) -> Result<()> {
        self.config.admin = self.admin.key();
        Ok(())
    }
}
