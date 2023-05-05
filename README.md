# Solana Program Error Derive

Proc macro for creating Solana Program error enums (WIP).

---

Annotate your enum:
```rust
#[solana_program_error]
pub enum MintToHookError {
    /// Mint has no mint authority
    #[error("Mint has no mint authority")]
    MintHasNoMintAuthority,
    /// Incorrect mint authority has signed the instruction
    #[error("Incorrect mint authority has signed the instruction")]
    IncorrectMintAuthority,
}
```

Get:
```rust
pub enum MintToHookError {
    /// Mint has no mint authority
    #[error("Mint has no mint authority")]
    MintHasNoMintAuthority,
    /// Incorrect mint authority has signed the instruction
    #[error("Incorrect mint authority has signed the instruction")]
    IncorrectMintAuthority,
}
#[automatically_derived]
impl ::core::clone::Clone for MintToHookError {
    #[inline]
    fn clone(&self) -> MintToHookError {
        match self {
            MintToHookError::MintHasNoMintAuthority => {
                MintToHookError::MintHasNoMintAuthority
            }
            MintToHookError::IncorrectMintAuthority => {
                MintToHookError::IncorrectMintAuthority
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MintToHookError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                MintToHookError::MintHasNoMintAuthority => "MintHasNoMintAuthority",
                MintToHookError::IncorrectMintAuthority => "IncorrectMintAuthority",
            },
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for MintToHookError {}
#[automatically_derived]
impl ::core::cmp::Eq for MintToHookError {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[allow(unused_qualifications)]
impl std::error::Error for MintToHookError {}
#[allow(unused_qualifications)]
impl std::fmt::Display for MintToHookError {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            MintToHookError::MintHasNoMintAuthority {} => {
                __formatter.write_fmt(format_args!("Mint has no mint authority"))
            }
            MintToHookError::IncorrectMintAuthority {} => {
                __formatter
                    .write_fmt(
                        format_args!(
                            "Incorrect mint authority has signed the instruction"
                        ),
                    )
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_qualifications)]
const _IMPL_NUM_FromPrimitive_FOR_MintToHookError: () = {
    #[allow(clippy::useless_attribute)]
    #[allow(rust_2018_idioms)]
    extern crate num_traits as _num_traits;
    impl _num_traits::FromPrimitive for MintToHookError {
        #[allow(trivial_numeric_casts)]
        #[inline]
        fn from_i64(n: i64) -> Option<Self> {
            if n == MintToHookError::MintHasNoMintAuthority as i64 {
                Some(MintToHookError::MintHasNoMintAuthority)
            } else if n == MintToHookError::IncorrectMintAuthority as i64 {
                Some(MintToHookError::IncorrectMintAuthority)
            } else {
                None
            }
        }
        #[inline]
        fn from_u64(n: u64) -> Option<Self> {
            Self::from_i64(n as i64)
        }
    }
};
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MintToHookError {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MintToHookError {
    #[inline]
    fn eq(&self, other: &MintToHookError) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
impl From<MintToHookError> for solana_program::program_error::ProgramError {
    fn from(e: MintToHookError) -> Self {
        solana_program::program_error::ProgramError::Custom(e as u32)
    }
}
impl<T> solana_program::decode_error::DecodeError<T> for MintToHookError {
    fn type_of() -> &'static str {
        "MintToHookError"
    }
}
impl solana_program::program_error::PrintProgramError for MintToHookError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + solana_program::decode_error::DecodeError<E>
            + solana_program::program_error::PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            MintToHookError::MintHasNoMintAuthority => {
                ::solana_program::log::sol_log("Mint has no mint authority");
            }
            MintToHookError::IncorrectMintAuthority => {
                ::solana_program::log::sol_log(
                    "Incorrect mint authority has signed the instruction",
                );
            }
        }
    }
}
```