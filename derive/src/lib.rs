extern crate proc_macro;

mod program_error;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn solana_program_error(_: TokenStream, input: TokenStream) -> TokenStream {
    program_error::program_error(input)
}
