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

//#[doc(hidden)]
//type CallReadFunction<I: Read, BO: ByteOrder, L> = fn(&mut I) -> ReadResult<()>;
//#[doc(hidden)]
//type CallWriteFunction<O: Write, BO: ByteOrder, L> = fn(&mut O) -> WriteResult;
//#[doc(hidden)]
//type ReadFunction<I: Read, BO: ByteOrder, L, T: Sized> = fn(&mut I) -> ReadResult<T>;
//#[doc(hidden)]
//type WriteFunction<O: Write, BO: ByteOrder, L, T> = fn(&T, &mut O) -> WriteResult;

#[macro_export]
macro_rules! def_read {
    ($ty:ident $( ( $($generics:tt)* ) )?, $input:ident => $body:expr) => {
		impl<BO: ByteOrder, L $( , $($generics)* )?> FromBytes<BO, L> for $ty $( < $($generics)* > )? {
			type Output = Self;
		
			#[inline]
			fn from_bytes<I: Read>($input: &mut I) -> ReadResult<Self::Output> {
				Ok($body)
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

def_read!(PhantomData(T), _input => PhantomData);

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

//impl<BO, L> FromBytes<BO, L> for Vec<u8>
//	where
//		BO: ByteOrder,
//		L: FromBytes<BO>,
//		L::Output: Into<usize> {
//	type Output = Vec<u8>;
//
//	fn from_bytes<I: Read>(input: &mut I) -> ReadResult<Self::Output> {
//		let count: usize = L::from_bytes(input)?.into();
//		let mut output = Vec::with_capacity(count);
//		input.take(count as u64)
//			.read_to_end(&mut output)?;
//		Ok(output)
//	}
//}

#[macro_export]
macro_rules! def_write {
    ($ty:ident $( ( $($generics:tt)* ) )?, ($out:ident, $value:ident) => $body:expr) => {
		impl<BO: ByteOrder, L $( , $($generics)* )?> ToBytes<BO, L> for $ty $( < $($generics)* > )? {
	    	#[inline]
			fn to_bytes<O: Write>(&self, $out: &mut O) -> WriteResult {
				let $value = self;
				Ok($body)
			}
		}
    };
}

def_write!(u8, (output, v) => output.write_u8(*v)?);
def_write!(u16, (output, v) => output.write_u16::<BO>(*v)?);
def_write!(u32, (output, v) => output.write_u32::<BO>(*v)?);
def_write!(u64, (output, v) => output.write_u64::<BO>(*v)?);
#[cfg(has_u128)]
def_write!(u128, (output, v) => output.write_u128::<BO>(*v)?);

def_write!(i8, (output, v) => output.write_i8(*v)?);
def_write!(i16, (output, v) => output.write_i16::<BO>(*v)?);
def_write!(i32, (output, v) => output.write_i32::<BO>(*v)?);
def_write!(i64, (output, v) => output.write_i64::<BO>(*v)?);
#[cfg(has_i128)]
def_write!(i128, (output, v) => output.write_i128::<BO>(*v)?);

def_write!(f32, (output, v) => output.write_f32::<BO>(*v)?);
def_write!(f64, (output, v) => output.write_f64::<BO>(*v)?);

def_write!(bool, (output, v) => output.write_u8(if *v { 1 } else { 0 })?);

def_write!(PhantomData(T), (_output, _v) => ());

impl<BO, T, L> ToBytes<BO, L> for Vec<T>
	where
		BO: ByteOrder,
		T: ToBytes<BO>,
		L: ToBytes<BO> + TryFrom<usize> {

	#[inline]
	fn to_bytes<O: Write>(&self, output: &mut O) -> WriteResult {
		let len = self.len();
		L::try_from(len)
			.map_err(|_| WriteError::TooLarge(len))?
			.to_bytes(output)?;

		for v in self {
			v.to_bytes(output)?;
		}
		Ok(())
	}
}