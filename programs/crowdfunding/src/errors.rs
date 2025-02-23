use anchor_lang::prelude::*;


#[error_code]
pub enum IDOProgramErrors {
    #[msg("PDA account not matched")]
    PdaNotMatched,
    #[msg("Only authority is allowed to call this function")]
    NotAuthorized,
    #[msg("Invalid round index")]
    InvalidInDex,
    #[msg("Invalid rounds specified")]
    InvalidRounds,
    #[msg("Insufficient amount to withdraw.")]
    InsufficientAmount,
    #[msg("Invalid tiers specified")]
    InValidTier,
    #[msg("Invalid release index")]
    InvalidReleaseIndex,
    #[msg("Release token not yet defined")]
    InvalidReleaseToken,
    #[msg("No tokens left in the pool")]
    NoTokensLeft,
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
    #[msg("Participation not valid/open")]
    ParticipationNotValid,
    #[msg("Amount exceeds remaining allocation")]
    AmountExceedsRemainingAllocation,
    #[msg("IDO token account not match")]
    IDoTokenAccountNotMatch,
    #[msg("User token account not match")]
    UserTokenAccountNotMatch,
    #[msg("Admin token account not match")]
    WithdrawTokenAccountNotMatch,
    #[msg("Release token account of user not match")]
    ReleaseTokenAccountNotMatch,
    #[msg("Cannot parse data to account")]
    CannotParseData,
}

impl From<IDOProgramErrors> for ProgramError {
    fn from(e: IDOProgramErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}