// The `quote!` macro requires deep recursion.
#![recursion_limit = "512"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate darling;

use proc_macro::TokenStream;

use syn::DeriveInput;

mod attr;
mod to_bytes;
mod from_bytes;

#[proc_macro_derive(ToBytes, attributes(binform))]
pub fn derive_to_bytes(input: TokenStream) -> TokenStream {
	let input: DeriveInput = parse_macro_input!(input as DeriveInput);
	to_bytes::expand_derive(&input)
		.unwrap_or_else(to_compile_errors)
		.into()
}

#[proc_macro_derive(FromBytes, attributes(binform))]
pub fn derive_from_bytes(input: TokenStream) -> TokenStream {
	let input: DeriveInput = parse_macro_input!(input as DeriveInput);
	from_bytes::expand_derive(&input)
		.unwrap_or_else(to_compile_errors)
		.into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
	let compile_errors = errors.iter().map(syn::Error::to_compile_error);
	quote!(#(#compile_errors)*)
}