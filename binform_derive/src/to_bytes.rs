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
				quote!(Ok(()))
			} else {
				let mut tokens = TokenStream::new();

				for (i, f) in fields.iter().enumerate() {
					let span = f.ty.span();

					let field_access = if let Some(ident) = f.ident.as_ref() {
						quote_spanned!(span => self.#ident)
					} else {
						quote_spanned!(span => self.#i)
					};

					let extraction = if let Some(name) = &f.write {
						let method = syn::parse_str::<Path>(name).unwrap();
						quote_spanned! { span =>
							#method(output, &#field_access)?
						}
					} else {
						let ty = &f.ty;
						let endian = match f.endian.unwrap_or(container.endian) {
							Endian::Big => Ident::new("BigEndian", span),
							Endian::Little => Ident::new("LittleEndian", span),
						};
						let len = if let Some(len) = &f.len {
							let name = Ident::new(&len, span);
							quote_spanned!(span => #name)
						} else {
							quote_spanned!(span => ())
						};
						quote_spanned! { span =>
							<#ty as ToBytes<#endian, #len>>::to_bytes(&#field_access, output)?
						}
					};

					let t = quote!(#extraction;);
					t.to_tokens(&mut tokens);
				}

				let t = quote!(Ok(()));
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
		impl #impl_generics ToBytes<#endian, ()> for #name #ty_generics #where_clause {
		
			fn to_bytes<O: Write>(&self, output: &mut O) -> WriteResult {
				#function_body
			}
		}
	};

	let mut out = TokenStream::new();
	t.to_tokens(&mut out);
	Ok(out)
}