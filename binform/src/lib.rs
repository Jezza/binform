#![feature(try_from)]

extern crate binform_derive;
extern crate byteorder;
#[macro_use]
extern crate failure;

use std::convert::TryFrom;
pub use std::io::{
	Read,
	Write,
};
use std::ops::Range;

pub use byteorder::{
	BigEndian,
	ByteOrder,
	LittleEndian,
	NativeEndian,
	ReadBytesExt,
	WriteBytesExt,
};

#[doc(hidden)]
pub use binform_derive::*;

pub use crate::error::*;
use std::marker::PhantomData;

mod error;

pub type ReadResult<T> = Result<T, ReadError>;
pub type WriteResult = Result<(), WriteError>;

pub trait FromBytes<E: ByteOrder = NativeEndian, L = ()> {
	type Output: Sized;

	fn from_bytes<I: Read>(input: &mut I) -> ReadResult<Self::Output>;
}

pub trait ToBytes<E: ByteOrder = NativeEndian, L = ()> {
	fn to_bytes<O: Write>(&self, output: &mut O) -> WriteResult;
}

//fn write<O: Write, T>(value: &T, output: &mut O) -> WriteResult {
//	unimplemented!()
//}
//
//fn read<I: Read, T: Sized>(input: &mut I) -> ReadResult<T> {
//	unimplemented!()
//}

#[doc(hidden)]
type ReadFunction<I: Read, T: Sized> = fn(&mut I) -> ReadResult<T>;
#[doc(hidden)]
type WriteFunction<O: Write, T> = fn(&T, &mut O) -> WriteResult;

macro_rules! def_read {
    ($ty:ident $(<$generic:ident>)?, $input:ident => $($method:tt)*) => {
		impl<BO: ByteOrder, L $(,$generic)?> FromBytes<BO, L> for $ty $(<$generic>)? {
			type Output = Self;
		
			fn from_bytes<I: Read>($input: &mut I) -> ReadResult<Self::Output> {
				Ok($($method)*)
			}
		}
    };
}

def_read!(u8, input => input.read_u8()?);
def_read!(u16, input => input.read_u16::<BO>()?);
def_read!(u32, input => input.read_u32::<BO>()?);
def_read!(u64, input => input.read_u64::<BO>()?);
#[cfg(has_u128)]
def_read!(u128, input => input.read_u128::<BO>()?);

def_read!(i8, input => input.read_i8()?);
def_read!(i16, input => input.read_i16::<BO>()?);
def_read!(i32, input => input.read_i32::<BO>()?);
def_read!(i64, input => input.read_i64::<BO>()?);
#[cfg(has_i128)]
def_read!(i128, input => input.read_i128::<BO>()?);

def_read!(f32, input => input.read_f32::<BO>()?);
def_read!(f64, input => input.read_f64::<BO>()?);

def_read!(bool, input => input.read_u8()? != 0);

def_read!(PhantomData<T>, _input => PhantomData);

impl<BO, T, L> FromBytes<BO, L> for Vec<T>
	where
		BO: ByteOrder,
		T: FromBytes<BO>,
		L: FromBytes<BO>,
		L::Output: Default + Into<usize> + Copy,
		Range<L::Output>: Iterator {
	type Output = Vec<T::Output>;

	fn from_bytes<I: Read>(input: &mut I) -> ReadResult<Self::Output> {
		let count = L::from_bytes(input)?;
		let mut output = Vec::with_capacity(count.into());
		for _i in Default::default()..count {
			output.push(T::from_bytes(input)?);
		}
		Ok(output)
	}
}

macro_rules! def_write {
    ($ty:ident $(< $generic:ident>)?, ($value:ident, $out:ident) => $($method:tt)*) => {
		impl<BO: ByteOrder, L $(, $generic)?> ToBytes<BO, L> for $ty $(<$generic>)? {
			fn to_bytes<O: Write>(&self, $out: &mut O) -> WriteResult {
				let $value = self;
				Ok($($method)*)
			}
		}
    };
}

def_write!(u8, (v, output) => output.write_u8(*v)?);
def_write!(u16, (v, output) => output.write_u16::<BO>(*v)?);
def_write!(u32, (v, output) => output.write_u32::<BO>(*v)?);
def_write!(u64, (v, output) => output.write_u64::<BO>(*v)?);
#[cfg(has_u128)]
def_write!(u128, (v, output) => output.write_u128::<BO>(*v)?);

def_write!(i8, (v, output) => output.write_i8(*v)?);
def_write!(i16, (v, output) => output.write_i16::<BO>(*v)?);
def_write!(i32, (v, output) => output.write_i32::<BO>(*v)?);
def_write!(i64, (v, output) => output.write_i64::<BO>(*v)?);
#[cfg(has_i128)]
def_write!(i128, (v, output) => output.write_i128::<BO>(*v)?);

def_write!(f32, (v, output) => output.write_f32::<BO>(*v)?);
def_write!(f64, (v, output) => output.write_f64::<BO>(*v)?);

def_write!(bool, (v, output) => output.write_u8(if *v { 1 } else { 0 })?);

def_write!(PhantomData<T>, (_v, _output) => ());

impl<BO, T, L> ToBytes<BO, L> for Vec<T>
	where
		BO: ByteOrder,
		T: ToBytes<BO>,
		L: ToBytes<BO> + TryFrom<usize> {
	fn to_bytes<O: Write>(&self, output: &mut O) -> WriteResult {
		let len = self.len();
		let value = match L::try_from(len) {
			Result::Err(_e) => return Err(WriteError::TooLarge(len)),
			Result::Ok(value) => value,
		};
		value.to_bytes(output)?;

		for v in self {
			v.to_bytes(output)?;
		}
		Ok(())
	}
}