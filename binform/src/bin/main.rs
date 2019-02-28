extern crate binform;

use std::io::{
	Read,
	Write,
};
use std::io::Cursor;

use binform::*;
use std::marker::PhantomData;

pub fn main() {
//	let buf: Vec<u8> = vec![0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x52, 0x00, 0x00, 0x01, 0x00];
//	let buf: Vec<u8> = vec![0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x01, 0x00];
	let buf: Vec<u8> = vec![
		0x00,
		0x00, 0xAF,
		0x00, 0x02,
		0x01,
		0x00, 0x05,
		0x01,
		0x00, 0x0A,
	];
	println!("{:?}", buf);
	let mut buf = Cursor::new(buf);
	let t: Value = Value::from_bytes(&mut buf).unwrap();
	println!("{:?}", t);
//	let mut buf0: [u8; 4] = [0; 4];
	
	let mut out: Vec<u8> = vec![];
	let mut buf = Cursor::new(&mut out);
	t.to_bytes(&mut buf).unwrap();
	println!("{:?}", out);

//	let mut buf = Cursor::new(&mut buf0 as &mut [u8]);
//	t.to_bytes(&mut buf).unwrap();
//	println!("{:?}", buf0);

//	binform::run();
//	let f = ClassFile {
//		..Default::default()
//	};
//	f.constant_pool
//	f.to_bytes();
}

//	_u8: u8,
//	_u16: u16,
//	_u32: u32,
//	_u64: u64,
//	_u128: u128,
//	_i8: i8,
//	_i16: i16,
//	_i32: i32,
//	_i64: i64,
//	_i128: i128,

// write<O: Write>(output: &mut O, value: &T) -> WriteResult
// read<I: Read>(input: &mut I) -> ReadResult<T>
// #[binform(before(magic = 0xCAFE_BABE))]

#[derive(Debug, FromBytes, ToBytes)]
#[binform(endian = "be")]
pub struct Test {
	value: bool,
	extra: u16,
}

#[derive(Debug, FromBytes, ToBytes)]
#[binform(endian = "be")]
struct Value {
	extra: Test,
	#[binform(len = "u16")]
	payload: Vec<Test>,
}

//pub fn run() {
//	use std::io::Cursor;
//
//	let buf: Vec<u8> = vec![0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x52, 0x00, 0x00, 0x01, 0x00];
//	println!("{:?}", buf);
//	let mut buf = Cursor::new(buf);
//	let t = Test::from_bytes(&mut buf);
//	println!("{:?}", t);
//
//	let mut buf = vec![];
//	t.to_bytes(&mut buf);
//	println!("{:?}", buf);
//}

#[derive(ToBytes, FromBytes, Default)]
#[binform(endian = "be")]
pub struct ClassFile<'a> {
////	#[binform(magic = 0xCA_FE_BA_BE)]
////	_magic: u32,
//	#[binform(before(magic = 0xCA_FE_BA_BE))]
	pub minor_version: u16,
	pub major_version: u16,
	pub constant_pool: ConstantPool<'a>,
	pub access_flags: u16,
	pub this_class: u16,
	pub super_class: u16,
	#[binform(len = "u16")]
	pub interfaces: Vec<CPIndex<'a, ClassInfo<'a>>>,
	#[binform(len = "u16")]
	pub fields: Vec<FieldInfo<'a>>,
	#[binform(len = "u16")]
	pub methods: Vec<MethodInfo<'a>>,
	#[binform(len = "u16")]
	pub attributes: Vec<AttributeInfo<'a>>,
}
//
#[derive(ToBytes, FromBytes, Default)]
#[binform(endian = "be")]
pub struct ConstantPool<'a> {
//	#[binform(len = "u16")]
//	entries: Vec<CPEntry<'a>>,
	_marker: PhantomData<&'a ()>,
}

//#[derive(ToBytes, FromBytes)]
//#[binform(endian = "be")]
pub enum CPEntry<'a> {
	Class(ClassInfo<'a>),
	UTF8(UTF8Info<'a>),
}

#[derive(ToBytes, FromBytes)]
#[binform(endian = "be")]
pub struct ClassInfo<'a> {
	name_index: CPIndex<'a, UTF8Info<'a>>
}

impl<'a> CPType<'a> for ClassInfo<'a> {
	type Output = &'a ClassInfo<'a>;

	fn fetch(entry: &'a CPEntry<'a>) -> Option<Self::Output> {
		if let CPEntry::Class(info) = entry {
			Some(info)
		} else {
			None
		}
	}
}

#[derive(ToBytes, FromBytes)]
#[binform(endian = "be")]
pub struct UTF8Info<'a> {
	#[binform(len = "u16")]
//	data: &'a [u8]
	data: Vec<u8>,
	_marker: PhantomData<&'a ()>,
}

impl<'a> CPType<'a> for UTF8Info<'a> {
	type Output = &'a UTF8Info<'a>;

	fn fetch(entry: &'a CPEntry<'a>) -> Option<Self::Output> {
		if let CPEntry::UTF8(info) = entry {
			Some(info)
		} else {
			None
		}
	}
}

pub trait CPType<'a> {
	type Output;

	fn fetch(entry: &'a CPEntry<'a>) -> Option<Self::Output>;
}

#[derive(ToBytes, FromBytes)]
#[binform(endian = "be")]
pub struct CPIndex<'a, T: 'a + CPType<'a>> {
	index: u16,
	_marker: PhantomData<&'a T>,
}

impl<'a, T: 'a + CPType<'a>> Clone for CPIndex<'a, T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<'a, T: 'a + CPType<'a>> Copy for CPIndex<'a, T> {}

#[derive(ToBytes, FromBytes)]
#[binform(endian = "be")]
pub struct FieldInfo<'a> {
	_marker: PhantomData<&'a ()>
}

#[derive(ToBytes, FromBytes)]
#[binform(endian = "be")]
pub struct MethodInfo<'a> {
	_marker: PhantomData<&'a ()>
}

#[derive(ToBytes, FromBytes)]
#[binform(endian = "be")]
pub struct AttributeInfo<'a> {
	_marker: PhantomData<&'a ()>
}