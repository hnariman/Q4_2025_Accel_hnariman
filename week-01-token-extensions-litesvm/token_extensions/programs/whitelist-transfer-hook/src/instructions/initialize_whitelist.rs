use anchor_lang::prelude::*;

use crate::state::Whitelist;

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + 4 + 1, // 8 bytes for discriminator, 4 bytes for vector length, 1 byte for bump
        seeds = [b"whitelist"],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeWhitelist<'info> {
    pub fn initialize_whitelist(&mut self, bumps: InitializeWhitelistBumps) -> Result<()> {
        // Initialize the whitelist with an empty address vector
        self.whitelist.set_inner(Whitelist {
            address: vec![],
            bump: bumps.whitelist,
        });

        Ok(())
    }
}


// so basically we define 
// admin : pubkey
// whitelist: PDA with seed of 'whitelist' string
// and send here system program - why not hardcode it?
//
//
// then we generate those in tests or on client (web too) and pass to protocol
//
// and since in transfers etc we've had some kind of CPI and executables, this instruction seems
// like something "empty", this is why it's strange from the first sight
// but It's completely OK
//
// but possibly we don't need even to init this, since we will generate PDA based on each
// users,pubkey
