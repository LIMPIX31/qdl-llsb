pub use serialport::Error as SerialError;
pub use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
	#[error("Nothing to read from port")]
	Empty,

	#[error("Length of read data do not match expected data length. Expected: {expected}. Got: {got}")]
	LengthMismatch {
		expected: u32,
		got: usize,
	},

	#[error(transparent)]
	Serial(#[from] SerialError),

	#[error(transparent)]
	Io(#[from] IoError),
}
