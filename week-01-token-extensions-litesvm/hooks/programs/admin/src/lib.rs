#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("8UTBWoFJczymeBeJDUPjNajMLSYNhjmf1SkCBU3cBzrh");

mod errors;
mod instructions;
mod state;
use instructions::*;

#[program]
pub mod admin {
    use super::*;

    pub fn initialize(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.init()
    }

    pub fn whitelist_add(ctx: Context<WhiteListAdd>, user: Pubkey) -> Result<()> {
        ctx.accounts.whitelist_add(user)
    }

    pub fn whitelist_remove(ctx: Context<WhiteListRemove>, user: Pubkey) -> Result<()> {
        ctx.accounts.whitelist_remove(user)
    }
}
