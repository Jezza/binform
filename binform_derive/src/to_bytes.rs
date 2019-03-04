use darling::{
	ast::Data,
	ast::Style,
};
use darling::ast::Fields;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{self, Ident};
use syn::Path;
use syn::spanned::Spanned;
use syn::Type;

use crate::attr::Container;
use crate::attr::Endian;
use crate::attr::EnumVariant;
use crate::attr::StructField;
use crate::attr::Expect;

pub fn expand_derive(input: &syn::DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
	let container = Container::from_ast(input);

	let name = &container.ident;
	let span = name.span();
	let endian = container.endian.as_ident(span);
	let function_body = match container.data {
		Data::Struct(fields) => {
			let qualified_name = quote!(#name);
			let (insertion, deconstruction) = analyse_fields(fields, qualified_name, container.endian);
			quote_spanned! { span =>
				let #deconstruction = self;
				#insertion;
			}
		}
		Data::Enum(variants) => {
			let tag = container.tag.expect("enum requires tag");
			let tag_ty = syn::parse_str::<Type>(&tag).unwrap();
			let mut match_arms = TokenStream::new();

			for EnumVariant {
				ident: variant_name,
				tag,
				fields,
			} in variants {
				let span = variant_name.span();

				let qualified_name = quote_spanned!(span => #name::#variant_name);
				let (insertion, deconstruction) = analyse_fields(fields, qualified_name, container.endian);

				// Add each match arm as a bunch of tokens, as it's all going to be dumped into the main quote!
				let t = quote_spanned!(span =>
					#deconstruction => {
						<#tag_ty as ToBytes<#endian, ()>>::to_bytes(&#tag, output)?;
						#insertion
					}
				);
				t.to_tokens(&mut match_arms);
			}

			quote_spanned! { span =>
				match self {
					#match_arms
				}
			}
		}
	};

	let endian = container.endian.as_ident(span);
	let (impl_generics, ty_generics, where_clause) = container.generics.split_for_impl();

	let t = quote_spanned! { span =>
		#[automatically_derived]
		impl #impl_generics ToBytes<#endian, ()> for #name #ty_generics #where_clause {
		
			fn to_bytes<O: Write>(&self, output: &mut O) -> WriteResult {
				#function_body;
				Ok(())
			}
		}
	};
	Ok(t)
}

fn analyse_fields(fields: Fields<StructField>, qualified_name: TokenStream, default_endian: Endian) -> (TokenStream, TokenStream) {
	let Fields {
		style,
		fields,
	} = fields;
	if style == Style::Unit {
		return (TokenStream::new(), qualified_name);
	}
	let span = qualified_name.span();
	let mut insertion = TokenStream::new();
	let mut deconstruct = TokenStream::new();

	for (i, f) in fields.into_iter().enumerate() {
		let span = f.ty.span();

		let (field_name, _drop_me) = if let Some(field_name) = f.ident {
			(field_name, String::new())
		} else {
			let name = format!("_{}", i);
			(Ident::new(&name, span), name)
		};

		let generics = {
			let endian = f.endian
				.unwrap_or(default_endian)
				.as_ident(span);
			let len = if let Some(len) = &f.len {
				let name = Ident::new(&len, span);
				quote_spanned!(span => #name)
			} else {
				quote_spanned!(span => ())
			};
			quote_spanned!(span => #endian, #len)
		};

		let insert = if let Some(name) = &f.write {
			let method = syn::parse_str::<Path>(name).unwrap();
			quote_spanned! { span =>
				#method::<O, #generics>(#field_name, output)?
			}
		} else {
			let ty = &f.ty;
			quote_spanned! { span =>
				<#ty as ToBytes<#generics>>::to_bytes(#field_name, output)?
			}
		};

		let t = quote_spanned!(span => #field_name,);
		t.to_tokens(&mut deconstruct);

		if let Some(before) = f.before {
			if let Some(Expect {
							ty,
							value,
						}) = before.expect {

				let ty = syn::parse_str::<Type>(&ty).unwrap();

				let t = quote_spanned! { span =>
					<#ty as ToBytes<#generics>>::to_bytes(&#value, output)?;
				};
				t.to_tokens(&mut insertion);
			}
			for name in before.write {
				let method = syn::parse_str::<Path>(&name).unwrap();
				let t = quote_spanned! { span =>
					#method::<O, #generics>(output)?;
				};
				t.to_tokens(&mut insertion);
			}
		}

		let t = quote_spanned!(span => #insert;);
		t.to_tokens(&mut insertion);

		if let Some(after) = f.after {
			if let Some(Expect {
							ty,
							value,
						}) = after.expect {

				let ty = syn::parse_str::<Type>(&ty).unwrap();

				let t = quote_spanned! { span =>
					<#ty as ToBytes<#generics>>::to_bytes(&#value, output)?;
				};
				t.to_tokens(&mut insertion);
			}
			for name in after.write {
				let method = syn::parse_str::<Path>(&name).unwrap();
				let t = quote_spanned! { span =>
					#method::<O, #generics>(output)?;
				};
				t.to_tokens(&mut insertion);
			}
		}
	}

	let deconstruction = if style == Style::Struct {
		quote_spanned!(span => #qualified_name { #deconstruct } )
	} else {
		quote_spanned!(span => #qualified_name ( #deconstruct ) )
	};

	(insertion, deconstruction)
}