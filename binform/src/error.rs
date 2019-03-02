use ::std::io::Error as IOError;
//use ::std::num::TryFromIntError;

#[derive(Debug, Fail)]
pub enum ReadError {
	#[fail(display = "IO error")]
	IO(#[fail(cause)] IOError),

	#[fail(display = "Expectation not met")]
	InvalidExpectation,
}

#[derive(Debug, Fail)]
pub enum WriteError {
	#[fail(display = "IO error")]
	IO(#[fail(cause)] IOError),

	#[fail(display = "Shit's too big, mate...")]
	TooLarge(usize),
}

impl From<IOError> for ReadError {
	fn from(cause: IOError) -> Self {
		ReadError::IO(cause)
	}
}

impl From<IOError> for WriteError {
	fn from(cause: IOError) -> Self {
		WriteError::IO(cause)
	}
}

//impl From<TryFromIntError> for WriteError {
//	fn from(cause: TryFromIntError) -> Self {
//		WriteError::TooLarge(cause)
//	}
//}