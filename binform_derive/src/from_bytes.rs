use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{self, Ident};
use syn::Meta::{List, NameValue, Word};
use syn::NestedMeta::{Literal, Meta};
use syn::parse::{self, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use crate::attr::Container;
use syn::{
	Type,
	TypePath
};
use crate::attr::Endian;
use darling::ast::Style;
use darling::ast::Data;
use syn::Path;

pub fn expand_derive(input: &syn::DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
	let container = Container::from_ast(input);

	let name = &container.ident;
	let span = name.span();
	let function_body = match container.data {
		Data::Struct(fields) => {
			let (style, fields) = fields.split();
			if style == Style::Unit {
				quote!(Ok(#name))
			} else {
				let mut tokens = TokenStream::new();
				let mut names = vec![];

				for (i, f) in fields.iter().enumerate() {
					let span = f.ty.span();

					let name = format!("_{}", i);
					let ghost = Ident::new(&name, span);
					let field_name = if let Some(ident) = f.ident.as_ref() {
						ident
					} else {
						&ghost
					};
					names.push(field_name.clone());

					let extraction = {
						if let Some(name) = &f.read {
							let method = syn::parse_str::<Path>(name).unwrap();
							quote_spanned! { span =>
								#method(input)?
							}
						} else {
							let ty = &f.ty;
							let endian = match f.endian.unwrap_or(container.endian) {
								Endian::Big => Ident::new("BigEndian", span),
								Endian::Little => Ident::new("LittleEndian", span),
							};
							let len = if let Some(len) = &f.len {
								let name = Ident::new(&len, span);
								quote!(#name)
							} else {
								quote!(())
							};
							quote_spanned! { span =>
								<#ty as FromBytes<#endian, #len>>::from_bytes(input)?
							}
						}
					};

					let t = quote! {
						let #field_name = #extraction;
					};
					t.to_tokens(&mut tokens);
				}

				let t = if style == Style::Struct {
					quote! {
						Ok(#name {
							#(#names),*
						})
					}
				} else {
					quote!(Ok(#name(#(#names),*)))
				};
				t.to_tokens(&mut tokens);

				tokens
			}
		}
		_ => quote!(panic!("Unhandled"))
	};

	let endian = match container.endian {
		Endian::Big => Ident::new("BigEndian", span),
		Endian::Little => Ident::new("LittleEndian", span),
	};

	let (impl_generics, ty_generics, where_clause) = container.generics.split_for_impl();

	let t = quote! {
		impl #impl_generics FromBytes<#endian, ()> for #name #ty_generics #where_clause {
			type Output = Self;
		
			fn from_bytes<I: Read>(input: &mut I) -> ReadResult<Self::Output> {
				#function_body
			}
		}
	};

	let mut out = TokenStream::new();
	t.to_tokens(&mut out);
	Ok(out)
}