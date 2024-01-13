use std::io;
use std::io::Read;
use std::io::Write;

use crate::error::BrokenError;
use crate::error::RawPacketReadError;

pub const U32: usize = 4;

pub macro slice_u32($from:expr, $at:expr) {
	u32::from_le_bytes([$from[$at], $from[$at + 1], $from[$at + 2], $from[$at + 3]])
}

pub macro u8_32($byte:expr) {
	[$byte, 0, 0, 0]
}

pub macro packet($($seq:expr),*$(,)?) {
	[$(&$seq[..]),*].concat()
}

#[derive(Debug, Clone)]
pub struct RawPacket {
	pub kind: u32,
	pub payload: Vec<u8>,
}

impl RawPacket {
	pub fn new(kind: u32, payload: Vec<u8>) -> Self {
		Self { kind, payload }
	}

	fn len(&self) -> usize {
		self.payload.len() + U32 /* len */ + U32 /* kind */
	}

	pub fn read(port: &mut impl Read, known_size: usize) -> Result<Self, RawPacketReadError> {
		let mut packet = vec![0; known_size];
		let read = port.read(&mut packet)?;

		let kind = slice_u32!(packet, 0);
		let len = slice_u32!(packet, U32) as usize;

		if read < len {
			return Err(BrokenError.into());
		}

		let payload = packet[U32 * 2..].to_owned();

		Ok(Self { kind, payload })
	}

	pub fn write(&self, port: &mut impl Write) -> io::Result<()> {
		port.write_all(&packet![
			self.kind.to_le_bytes(),
			(self.len() as u32).to_le_bytes(),
			self.payload
		])
	}

	pub fn send(port: &mut impl Write, kind: u32, payload: Vec<u8>) -> io::Result<()> {
		RawPacket::new(kind, payload).write(port)
	}
}
