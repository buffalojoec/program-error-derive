use solana_program_error_derive::solana_program_error;

/// Errors that may be returned by the mint-to-hook interface.
#[solana_program_error]
pub enum MintToHookError {
    /// Mint has no mint authority
    #[error("Mint has no mint authority")]
    MintHasNoMintAuthority,
    /// Incorrect mint authority has signed the instruction
    #[error("Incorrect mint authority has signed the instruction")]
    IncorrectMintAuthority,
}

#[test]
fn test() {
    let _ = MintToHookError::MintHasNoMintAuthority;
}
