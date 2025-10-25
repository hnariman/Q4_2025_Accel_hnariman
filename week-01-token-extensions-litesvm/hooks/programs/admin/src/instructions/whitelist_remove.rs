use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(user:Pubkey)]
pub struct WhiteListRemove<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
}

impl<'info> WhiteListRemove<'info> {
    pub fn whitelist_remove(&self, _user: Pubkey) -> Result<()> {
        msg!("removing by : {:?}", self.admin.key());
        Ok(())
    }
}
