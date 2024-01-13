#![feature(decl_macro)]
#![feature(concat_idents)]

use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Read;
use std::thread;
use std::time::Duration;

pub use common::Mode;
use error::BrokenError;
use error::ChangeModeError;
use error::ConnectError;
use error::EndTransferError;
use error::UnknownMessageError;
use protocol::slice_u32;
use serialport::DataBits;
use serialport::Parity;
use serialport::SerialPort;
use serialport::StopBits;

use crate::error::{DeviceResetError, ExecError};
use crate::error::InvalidPayloadLengthError;
use crate::protocol::packet;
use crate::protocol::u8_32;
use crate::protocol::RawPacket;
use crate::protocol::U32;

mod common;
pub mod error;
mod protocol;

#[derive(Debug)]
pub struct Sahara {
	port: Box<dyn SerialPort>,
	mode: Mode,
	version: u32,
	active: bool,
}

impl Display for Sahara {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "sahara: {}", self.mode)
	}
}

impl Sahara {
	pub fn wait_connect(serial_port: &str, interval: Duration) -> Result<Self, ConnectError> {
		Ok(loop {
			match Self::connect(serial_port) {
				Ok(port) => {
					break port;
				}
				Err(ConnectError::Serial(serialport::Error {
					kind: serialport::ErrorKind::NoDevice | serialport::ErrorKind::Io(_),
					..
				})) => {
					thread::sleep(interval);
				}
				Err(other) => return Err(other),
			}
		})
	}

	pub fn connect(serial_port: &str) -> Result<Self, ConnectError> {
		let mut port = serialport::new(serial_port, 115_200)
			.parity(Parity::None)
			.stop_bits(StopBits::One)
			.data_bits(DataBits::Eight)
			.timeout(Duration::from_millis(100))
			.open()?;

		let resp = RawPacket::read(&mut port, 48)?;
		Self::assert_success(&resp)?;

		if resp.kind != 1 {
			return Err(BrokenError.into());
		}

		let mode: Mode = slice_u32!(resp.payload, 12).try_into()?;
		let version = slice_u32!(resp.payload, 0);

		Ok(Self {
			port,
			mode,
			version,
			active: false,
		})
	}

	fn assert_success(resp: &RawPacket) -> Result<(), EndTransferError> {
		if resp.kind == 4 {
			return Err(slice_u32!(resp.payload, U32).into());
		}

		Ok(())
	}

	pub fn exec(&mut self, kind: u32) -> Result<Vec<u8>, ExecError> {
		self.change_mode(Mode::Command)?;
		#[rustfmt::skip]
		RawPacket::send(
			&mut self.port, 0x0D,
			packet![kind.to_le_bytes()]
		)?;

		let resp = RawPacket::read(&mut self.port, 16)?;
		Self::assert_success(&resp)?;

		if resp.kind != 0x0E {
			return Err(UnknownMessageError(resp.kind).into());
		}

		let len = slice_u32!(resp.payload, 4) as usize;

		#[rustfmt::skip]
		RawPacket::send(
			&mut self.port, 0x0F,
			packet![kind.to_le_bytes()]
		)?;

		let mut payload = vec![0; len];
		let read = self.port.read(&mut payload)?;

		if read < len {
			return Err(
				InvalidPayloadLengthError {
					expected: len,
					got: read,
				}
				.into(),
			);
		}

		Ok(payload)
	}

	pub fn change_mode(&mut self, mode: Mode) -> Result<(), ChangeModeError> {
		if self.mode == mode {
			return Ok(());
		}

		if self.active {
			#[rustfmt::skip]
			RawPacket::send(
				&mut self.port, 0x0C,
				packet![
					mode.to_le_bytes()
				]
			)?;
		} else {
			#[rustfmt::skip]
			RawPacket::send(
				&mut self.port, 0x02,
				packet![
					self.version.to_le_bytes(),
					u8_32!(1), u8_32!(0),
					mode.to_le_bytes(),
					u8_32!(1), u8_32!(2), u8_32!(3),
					u8_32!(4), u8_32!(5), u8_32!(6),
				]
			)?;

			let resp = RawPacket::read(&mut self.port, 8)?;
			Self::assert_success(&resp)?;

			if resp.kind != 0xB {
				return Err(UnknownMessageError(resp.kind).into());
			}

			self.active = true;
		}

		self.mode = mode;

		Ok(())
	}

	pub fn reset(&mut self) -> Result<(), DeviceResetError> {
		#[rustfmt::skip]
		RawPacket::send(
			&mut self.port, 0x07,
			Vec::new(),
		)?;

		let resp = RawPacket::read(&mut self.port, 8)?;
		Self::assert_success(&resp)?;

		Ok(())
	}
}
