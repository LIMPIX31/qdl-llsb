use std::io::Error as IoError;
use std::io::Read;

use sahara::error::{BrokenError, ChangeModeError};
use sahara::error::EndTransferError;
use sahara::error::UnknownMessageError;
use sahara::protocol::packet;
use sahara::protocol::slice_u32;
use sahara::protocol::RawPacket;
use sahara::protocol::U32;
use sahara::Mode;
use sahara::Sahara;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid requested memory slice length. Expected: {expected}, but got: {got}")]
pub struct InvalidLengthError {
	pub expected: u64,
	pub got: usize,
}

#[derive(Debug, Error)]
pub enum ReadmemError {
	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	InvalidLength(#[from] InvalidLengthError),

	#[error(transparent)]
	EndTransfer(#[from] EndTransferError),

	#[error(transparent)]
	UnknownMessage(#[from] UnknownMessageError),

	#[error(transparent)]
	Broken(#[from] BrokenError),
}

impl From<ChangeModeError> for ReadmemError {
	fn from(value: ChangeModeError) -> Self {
		value.into()
	}
}

pub trait SaharaMemExt {
	fn readmem(&mut self, addr: u64, length: u64) -> Result<Vec<u8>, ReadmemError>;
}

impl SaharaMemExt for Sahara {
	fn readmem(&mut self, addr: u64, length: u64) -> Result<Vec<u8>, ReadmemError> {
		self.change_mode(Mode::MemoryDebug)?;
		// RawPacket::send(
		// 	&mut self.port,
		// 	0x11,
		// 	packet![addr.to_le_bytes(), length.to_le_bytes()],
		// )?;
		//
		let mut mem = vec![0; length as usize];
		let read = self.port.read(&mut mem)?;

		if read < length as usize {
			if slice_u32!(mem, 0) == 4 {
				return Err(EndTransferError::from(slice_u32!(mem, U32 * 2)).into());
			}

			return Err(
				InvalidLengthError {
					expected: length,
					got: read,
				}
				.into(),
			);
		}

		Ok(mem)
	}
}
