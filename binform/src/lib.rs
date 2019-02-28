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

macro_rules! def_read {
    ($ty:ty => $($method:tt)*) => {
		impl<BO: ByteOrder, L> FromBytes<BO, L> for $ty {
			type Output = Self;
		
			fn from_bytes<I: Read>(input: &mut I) -> ReadResult<Self::Output> {
				Ok(input.$($method)*)
			}
		}
    };
}

def_read!(u8 => read_u8()?);
def_read!(u16 => read_u16::<BO>()?);
def_read!(u32 => read_u32::<BO>()?);
def_read!(u64 => read_u64::<BO>()?);
#[cfg(has_u128)]
def_read!(u128 => read_u128::<BO>()?);

def_read!(i8 => read_i8()?);
def_read!(i16 => read_i16::<BO>()?);
def_read!(i32 => read_i32::<BO>()?);
def_read!(i64 => read_i64::<BO>()?);
#[cfg(has_i128)]
def_read!(i128 => read_i128::<BO>()?);

def_read!(f32 => read_f32::<BO>()?);
def_read!(f64 => read_f64::<BO>()?);

def_read!(bool => read_u8()? != 0);

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

impl<BO: ByteOrder, T> FromBytes<BO, ()> for PhantomData<T> {
	type Output = Self;

	fn from_bytes<I: Read>(input: &mut I) -> Result<Self::Output, ReadError> {
		Ok(PhantomData)
	}
}

impl<BO: ByteOrder, T> ToBytes<BO, ()> for PhantomData<T> {
	fn to_bytes<O: Write>(&self, output: &mut O) -> Result<(), WriteError> {
		Ok(())
	}
}

macro_rules! def_write {
    ($ty:ty, $ident:ident => $($method:tt)*) => {
		impl<BO: ByteOrder, L> ToBytes<BO, L> for $ty {
			fn to_bytes<O: Write>(&self, output: &mut O) -> WriteResult {
				let $ident = self;
				Ok(output.$($method)*)
			}
		}
    };
}

def_write!(u8, v => write_u8(*v)?);
def_write!(u16, v => write_u16::<BO>(*v)?);
def_write!(u32, v => write_u32::<BO>(*v)?);
def_write!(u64, v => write_u64::<BO>(*v)?);
#[cfg(has_u128)]
def_write!(u128, v => write_u128::<BO>(*v)?);

def_write!(i8, v => write_i8(*v)?);
def_write!(i16, v => write_i16::<BO>(*v)?);
def_write!(i32, v => write_i32::<BO>(*v)?);
def_write!(i64, v => write_i64::<BO>(*v)?);
#[cfg(has_i128)]
def_write!(i128, v => write_i128::<BO>(*v)?);

def_write!(f32, v => write_f32::<BO>(*v)?);
def_write!(f64, v => write_f64::<BO>(*v)?);

def_write!(bool, v => write_u8(if *v { 1 } else { 0 })?);


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