use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct WhiteList {
    pub init_date: i64,
    pub recent_transaction_date: i64,
    pub amount_limit: u64,
}
