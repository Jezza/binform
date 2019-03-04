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
use crate::attr::Expect;
use crate::attr::StructField;

pub fn expand_derive(input: &syn::DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
	let container = Container::from_ast(input);

	let name = &container.ident;
	let span = name.span();
	let endian = container.endian.as_ident(span);
	let function_body = match container.data {
		Data::Struct(fields) => {
			let qualified_name = quote!(#name);
			let (mut extraction, construct) = analyse_fields(fields, qualified_name, container.endian);
			let t = quote_spanned!(span => Ok(#construct));
			t.to_tokens(&mut extraction);
			extraction
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
				let (extraction, construct) = analyse_fields(fields, qualified_name, container.endian);

				// Add each match arm as a bunch of tokens, as it's all going to be dumped into the main quote!
				let t = quote_spanned!(span => #tag => {#extraction #construct});
				t.to_tokens(&mut match_arms);
			}

			quote_spanned! { span =>
				let tag = <#tag_ty as FromBytes<#endian, ()>>::from_bytes(input)?;

				let result = match tag {
					#match_arms
					_ => panic!("Unknown tag: {}", tag)
				};

				Ok(result)
			}
		}
	};

	let (impl_generics, ty_generics, where_clause) = container.generics.split_for_impl();

	let t = quote_spanned! { span =>
		#[automatically_derived]
		impl #impl_generics FromBytes<#endian, ()> for #name #ty_generics #where_clause {
			type Output = Self;
		
			fn from_bytes<I: Read>(input: &mut I) -> ReadResult<Self::Output> {
				#function_body
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
	let mut extraction = TokenStream::new();
	let mut construct = TokenStream::new();

	for (i, f) in fields.into_iter().enumerate() {
		let span = f.ty.span();

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

		let extract = if let Some(name) = &f.read {
			let method = syn::parse_str::<Path>(name).unwrap();
			quote_spanned! { span =>
				#method::<I, #generics>(input)?
			}
		} else {
			let ty = &f.ty;
			quote_spanned! { span =>
				<#ty as FromBytes<#generics>>::from_bytes(input)?
			}
		};

		let (field_name, _drop_me) = if let Some(field_name) = f.ident {
			(field_name, String::new())
		} else {
			let name = format!("_{}", i);
			(Ident::new(&name, span), name)
		};

		let t = quote_spanned!(span => #field_name,);
		t.to_tokens(&mut construct);

		if let Some(before) = f.before {
			if let Some(Expect {
							ty,
							value,
						}) = before.expect {

				let ty = syn::parse_str::<Type>(&ty).unwrap();

				let t = quote_spanned! { span =>
					let value = <#ty as FromBytes<#generics>>::from_bytes(input)?;
					if value != #value {
						return Err(ReadError::InvalidExpectation);
					}				
				};
				t.to_tokens(&mut extraction);
			}
			for name in before.read {
				let method = syn::parse_str::<Path>(&name).unwrap();
				let t = quote_spanned!(span => #method::<I, #generics>(input)?;);
				t.to_tokens(&mut extraction);
			}
		}

		let t = quote_spanned!(span => let #field_name = #extract;);
		t.to_tokens(&mut extraction);

		if let Some(after) = f.after {
			if let Some(Expect {
							ty,
							value,
						}) = after.expect {

				let ty = syn::parse_str::<Type>(&ty).unwrap();

				let t = quote_spanned! { span =>
					let value = <#ty as FromBytes<#generics>>::from_bytes(input)?;
					if value != #value {
						return Err(ReadError::InvalidExpectation);
					}				
				};
				t.to_tokens(&mut extraction);
			}
			for name in after.read {
				let method = syn::parse_str::<Path>(&name).unwrap();
				let t = quote_spanned!(span => #method::<I, #generics>(input)?;);
				t.to_tokens(&mut extraction);
			}
		}
	}

	let construction = if style == Style::Struct {
		quote_spanned!(span => #qualified_name { #construct } )
	} else {
		quote_spanned!(span => #qualified_name ( #construct ) )
	};

	(extraction, construction)
}