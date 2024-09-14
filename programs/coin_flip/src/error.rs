use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Invalid Player Pool Owner")]
    InvalidPlayerPool,
    #[msg("Invalid Admin to Withdraw")]
    InvalidAdmin,
    #[msg("Insufficient Reward SOL Balance")]
    InsufficientRewardVault,
    #[msg("Insufficient User SOL Balance")]
    InsufficientUserBalance,
}