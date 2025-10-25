use crate::{errors::AdminError, state::{Config, WhiteList}};
use anchor_lang::prelude::*;

pub const TRANSACTION_LIMIT:u64=10_000_000;

#[derive(Accounts)]
#[instruction(user:Pubkey)]
pub struct WhiteListAdd<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump,
        has_one = admin @ AdminError::Unauthorized
    )]
    pub config: Account<'info,Config>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"whitelist",user.key().as_ref()],
        space = WhiteList::INIT_SPACE,
        bump 
    )]
    pub whitelist: Account<'info, WhiteList>,
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

impl<'info> WhiteListAdd<'info> {
    pub fn whitelist_add(&mut self,_user:Pubkey) -> Result<()> {
        self.whitelist.set_inner(WhiteList { 
            init_date: self.clock.unix_timestamp,
            recent_transaction_date: self.clock.unix_timestamp,
            amount_limit: TRANSACTION_LIMIT
        });

        msg!("add to whitelist: {:?}", self.admin.key());
        Ok(())
    }
}
