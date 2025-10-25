use anchor_lang::error_code;

#[error_code]
pub enum AdminError {
    #[msg("not authorized")]
    Unauthorized,
}
