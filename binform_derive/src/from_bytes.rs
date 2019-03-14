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
use crate::attr::Behaviour;
use syn::Expr;

pub fn expand_derive(input: &syn::DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
	let container = Container::from_ast(input);

	let name = &container.ident;
	let span = name.span();
	let generics = {
		let endian = container.endian.as_ident(span);
		quote_spanned!(span => #endian, ())
	};
	let function_body = match container.data {
		Data::Struct(fields) => {
			let qualified_name = quote!(#name);
			let (mut extraction, construct) = analyse_fields(fields, qualified_name, container.endian);
			let t = quote!(Ok(#construct));
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
				write: _,
				fields,
				before,
				after,
			} in variants {
				let span = variant_name.span();
				let tag = syn::parse_str::<Expr>(&tag).unwrap();

				let qualified_name = quote_spanned!(span => #name::#variant_name);

				let debug = if container.debug {
					let mut tokens = TokenStream::new();
					let t = quote_spanned!(span => println!("{:?}", stringify!(#qualified_name)););
					t.to_tokens(&mut tokens);
					tokens
				} else {
					TokenStream::new()
				};

				let (extraction, construct) = analyse_fields(fields, qualified_name, container.endian);

				let transform_behaviour = |behaviour: Behaviour| {
					let mut tokens = TokenStream::new();
					if let Some(Expect {
									ty,
									value,
								}) = behaviour.expect {

						let ty = syn::parse_str::<Type>(&ty).unwrap();
						let value = syn::parse_str::<Expr>(&value).unwrap();

						let t = quote_spanned! { span =>
							let value = <#ty as FromBytes<#generics>>::from_bytes(input)?;
							if value != #value {
								return Err(ReadError::InvalidExpectation);
							}				
						};
						t.to_tokens(&mut tokens);
					}
					for name in behaviour.read {
						let method = syn::parse_str::<Path>(&name).unwrap();
						let t = quote_spanned!(span => #method::<I, #generics>(input)?;);
						t.to_tokens(&mut tokens);
					}
					tokens
				};

				let before = before.map(transform_behaviour)
					.unwrap_or_else(|| TokenStream::new());
				let after = after.map(transform_behaviour)
					.unwrap_or_else(|| TokenStream::new());

				// Add each match arm as a bunch of tokens, as it's all going to be dumped into the main quote!
				let t = quote! {
					#tag => {
						#debug
						#before
						#extraction
						#after
						#construct
					}
				};
				t.to_tokens(&mut match_arms);
			}

			let t = quote! {
				_ => {
					return Err(ReadError::UnknownTag);
				}
			};
			t.to_tokens(&mut match_arms);

			quote! {
				let tag = <#tag_ty as FromBytes<#generics>>::from_bytes(input)?;

				let result = match tag {
					#match_arms
					_ => panic!("Unknown tag: {}", tag)
				};

				Ok(result)
			}
		}
	};

	let (impl_generics, ty_generics, where_clause) = container.generics.split_for_impl();

	let t = quote! {
		#[automatically_derived]
		impl #impl_generics FromBytes<#generics> for #name #ty_generics #where_clause {
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

		let transform_behaviour = |behaviour: Behaviour| {
			let mut tokens = TokenStream::new();
			if let Some(Expect {
							ty,
							value,
						}) = behaviour.expect {

				let ty = syn::parse_str::<Type>(&ty).unwrap();
				let value = syn::parse_str::<Expr>(&value).unwrap();

				let t = quote_spanned! { span =>
							let value = <#ty as FromBytes<#generics>>::from_bytes(input)?;
							if value != #value {
								return Err(ReadError::InvalidExpectation);
							}				
						};
				t.to_tokens(&mut tokens);
			}
			for name in behaviour.read {
				let method = syn::parse_str::<Path>(&name).unwrap();
				let t = quote_spanned!(span => #method::<I, #generics>(input)?;);
				t.to_tokens(&mut tokens);
			}
			tokens
		};

		f.before.map(transform_behaviour)
			.to_tokens(&mut extraction);

		let t = quote_spanned!(span => let #field_name = #extract;);
		t.to_tokens(&mut extraction);

		f.after.map(transform_behaviour)
			.to_tokens(&mut extraction);
	}

	let construction = if style == Style::Struct {
		quote_spanned!(span => #qualified_name { #construct } )
	} else {
		quote_spanned!(span => #qualified_name ( #construct ) )
	};

	(extraction, construction)
}