//! Fragment frel-compiler proc-macro. Emits FIR blobs.

extern crate proc_macro;
use proc_macro::TokenStream;

/// Placeholder macro. Replace with real DSL frel-compiler.
#[proc_macro]
pub fn fragment(input: TokenStream) -> TokenStream {
    // For now, emit input unchanged.
    input
}

mod dsl;

