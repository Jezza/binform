use darling::{
	ast,
	ast::Fields,
	FromDeriveInput,
	FromMeta,
	FromVariant,
};
use syn::{
	self,
	Ident,
	Type,
};
use syn::Generics;
use proc_macro2::Span;

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(binform))]
pub struct Container {
	pub ident: Ident,

	#[darling(default)]
	pub endian: Endian,

	pub generics: Generics,

	#[darling(default)]
	pub tag: Option<String>,

	pub data: ast::Data<EnumVariant, StructField>,
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

impl Endian {
	pub fn as_ident(&self, span: Span) -> Ident {
		match self {
			Endian::Big => Ident::new("BigEndian", span),
			Endian::Little => Ident::new("LittleEndian", span),
		}
	}
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

#[derive(Debug, FromVariant)]
#[darling(attributes(binform))]
pub struct EnumVariant {
	pub ident: Ident,

	pub tag: syn::Lit,

	pub fields: Fields<StructField>,
}
