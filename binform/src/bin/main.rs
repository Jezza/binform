extern crate binform;

use std::io::{
	Read,
	Write,
};
use std::io::Cursor;
use std::marker::PhantomData;

use binform::*;

//pub fn main0() {
////	let buf: Vec<u8> = vec![0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x52, 0x00, 0x00, 0x01, 0x00];
////	let buf: Vec<u8> = vec![0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x01, 0x00];
//	let buf: Vec<u8> = vec![
//		0x00,
//		0x00, 0xAF,
//		0x00, 0x02,
//		0x01,
//		0x00, 0x05,
//		0x01,
//		0x00, 0x0A,
//	];
//	println!("{:?}", buf);
//	let mut buf = Cursor::new(buf);
//	let t: Value = Value::from_bytes(&mut buf).unwrap();
//	println!("{:?}", t);
////	let mut buf0: [u8; 4] = [0; 4];
//
//	let mut out: Vec<u8> = vec![];
//	let mut buf = Cursor::new(&mut out);
//	t.to_bytes(&mut buf).unwrap();
//	println!("{:?}", out);
//
////	let mut buf = Cursor::new(&mut buf0 as &mut [u8]);
////	t.to_bytes(&mut buf).unwrap();
////	println!("{:?}", buf0);
//
////	binform::run();
////	let f = ClassFile {
////		..Default::default()
////	};
////	f.constant_pool
////	f.to_bytes();
//}

//pub fn main() {
////	let buf: Vec<u8> = vec![0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x52, 0x00, 0x00, 0x01, 0x00];
////	let buf: Vec<u8> = vec![0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x01, 0x00];
//	let buf: Vec<u8> = vec![
//		0x02,
//		0x00, 0x01,
//		0x00, 0x02,
//	];
//	println!("{:?}", buf);
//
//	let mut input = Cursor::new(buf);
//	let container = Container::from_bytes(&mut input).unwrap();
//	println!("{:?}", container);
//
//	let mut out: Vec<u8> = vec![];
//	let mut buf = Cursor::new(&mut out);
//	container.to_bytes(&mut buf).unwrap();
//	println!("{:?}", out);
//}
//
//#[derive(Debug, FromBytes, ToBytes)]
//#[binform(endian = "be")]
//struct Container<'a>(#[binform(len = "u8")] Vec<CPIndex<'a, ClassInfo<'a>>>);


pub fn main0() {
	let buf: Vec<u8> = vec![
		0x01,
		0x05,
	];

	let mut input = Cursor::new(buf);
	let value = Container::from_bytes(&mut input).unwrap();
	println!("{:?}", value);

}

#[derive(Debug, FromBytes)]
struct Container {
	#[binform(len = "u8")]
	value: Vec<u8>
}

pub fn main() {
//	let buf: Vec<u8> = vec![0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x52, 0x00, 0x00, 0x01, 0x00];
//	let buf: Vec<u8> = vec![0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x01, 0x00];
	let buf: Vec<u8> = vec![
		0xCA, 0xFE, 0xBA, 0xBE,
		0x00, 0x03, // Count
		0x01, // UTF8
			0x00, 0x02, // len = 2
				0x01, 0x01, // 01 01
		0x07, // ClassInfo
			0x00, 0x01,
		0x07, // ClassInfo
			0x00, 0x02,
	];
	println!("{:?}", buf);

	let mut input = Cursor::new(buf);
	let value = ConstantPool::from_bytes(&mut input).unwrap();
	println!("{:?}", value);

	let mut out: Vec<u8> = vec![];
	let mut buf = Cursor::new(&mut out);
	value.to_bytes(&mut buf).unwrap();
	println!("{:?}", out);
}

#[derive(Debug, FromBytes)]
#[binform(endian = "be")]
struct Info {
//	#[binform(len = "u8", read = "read")]
//	data: &'a [u8]
	#[binform(len = "u8")]
	data: Vec<u8>
}

//fn read<'a, 'b, I: Read, BO: ByteOrder, L>(input: &'a mut I) -> ReadResult<&'b [u8]> {
//	input.read()
//	unimplemented!()
//}

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

//fn write<O: Write, T>(value: &T, output: &mut O) -> WriteResult {
//	unimplemented!()
//}
//
//fn read<I: Read, T: Sized>(input: &mut I) -> ReadResult<T> {
//	unimplemented!()
//}

// #[binform(before(magic = 0xCAFE_BABE))]

//#[derive(Debug, FromBytes, ToBytes)]
//#[binform(endian = "be")]
//pub struct Test {
//	value: bool,
//	extra: u16,
//}
//
//#[derive(Debug, FromBytes, ToBytes)]
//#[binform(endian = "be")]
//struct Value {
//	extra: Test,
//	#[binform(len = "u16")]
//	payload: Vec<Test>,
//}

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

#[derive(Debug, ToBytes, FromBytes, Default)]
#[binform(endian = "be")]
pub struct ClassFile<'a> {
	#[binform(before(expect(type = "u32", value = 0xCA_FE_BA_BE)))]
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
#[derive(Debug, ToBytes, FromBytes, Default)]
#[binform(endian = "be")]
pub struct ConstantPool<'a> {
	#[binform(len = "u16", before(expect(type = "u32", value = 0xCA_FE_BA_BE)))]
	entries: Vec<CPEntry<'a>>,
//	_marker: PhantomData<&'a ()>,
}

pub const CONSTANT_CLASS_TAG: u8 = 7;
pub const CONSTANT_UTF8_TAG: u8 = 1;

#[derive(Debug, FromBytes, ToBytes)]
#[binform(endian = "be", tag = "u8")]
pub enum CPEntry<'a> {
	#[binform(tag = 2)]
	Value,
	#[binform(tag = 1)]
	UTF8(UTF8Info<'a>),
	#[binform(tag = 7)]
	Class(ClassInfo<'a>),
	#[binform(tag = 3)]
	Other {
		value: u16,
		extra: u16,
	}
}

#[derive(Debug, ToBytes, FromBytes)]
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

#[derive(Debug, ToBytes, FromBytes)]
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

#[derive(Debug, ToBytes, FromBytes)]
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

#[derive(Debug, ToBytes, FromBytes)]
#[binform(endian = "be")]
pub struct FieldInfo<'a> {
	_marker: PhantomData<&'a ()>
}

#[derive(Debug, ToBytes, FromBytes)]
#[binform(endian = "be")]
pub struct MethodInfo<'a> {
	_marker: PhantomData<&'a ()>
}

#[derive(Debug, ToBytes, FromBytes)]
#[binform(endian = "be")]
pub struct AttributeInfo<'a> {
	_marker: PhantomData<&'a ()>
}