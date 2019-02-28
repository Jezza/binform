use darling::{
	ast,
	FromDeriveInput,
	FromMeta,
};
use syn::{
	self,
	Ident,
	Type,
};
use syn::Meta::{List, NameValue, Word};
use syn::NestedMeta::{Literal, Meta};
use syn::parse::{self, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Generics;

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(binform))]
pub struct Container {
	pub ident: Ident,

	#[darling(default)]
	pub endian: Endian,

//	pub generics: Generics<GenericParam>,
	pub generics: Generics,

	pub data: ast::Data<darling::util::Ignored, StructField>,
}

impl Container {
	pub fn from_ast(input: &syn::DeriveInput) -> Self {
		Container::from_derive_input(input).unwrap()
	}
}

#[derive(Debug, FromMeta, Copy, Clone)]
pub enum Endian {
	#[darling(rename = "be")]
	Big,
	#[darling(rename = "le")]
	Little,
}

#[cfg(target_endian = "little")]
pub const NATIVE_ENDIAN: Endian = Endian::Little;

#[cfg(target_endian = "big")]
pub const NATIVE_ENDIAN: Endian = Endian::Big;

impl Default for Endian {
	fn default() -> Self {
		NATIVE_ENDIAN
	}
}

#[derive(Debug, FromField)]
#[darling(attributes(binform))]
pub struct StructField {
	#[darling(default)]
	pub ident: Option<Ident>,

	pub ty: Type,

	#[darling(default)]
	pub endian: Option<Endian>,

	#[darling(default)]
	pub len: Option<String>,

	#[darling(default)]
	pub read: Option<String>,

	#[darling(default)]
	pub write: Option<String>,
}
