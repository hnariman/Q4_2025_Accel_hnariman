use anchor_lang::prelude::*;

#[account]
pub struct Whitelist {
    pub amount: u64,
    pub bump: u8,
}

