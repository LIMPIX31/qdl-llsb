#![feature(decl_macro)]

use std::io::Read;
use std::io::Write;
use std::ops::Deref;
use std::ops::DerefMut;
use std::thread;
use std::time::Duration;

use error::IoError;
use error::ReadError;
use error::SerialError;
pub use pkt::pack;
pub use pkt::u32;
pub use pkt::un32;
pub use pkt::U32;
pub use pkt::u32s;
use serialport::DataBits;
use serialport::Parity;
use serialport::SerialPortBuilder;
use serialport::StopBits;

pub mod error;
mod pkt;

const BAUD_RATE: u32 = 115_200;

pub type SerialPort = Box<dyn serialport::SerialPort>;

#[derive(Debug)]
pub struct Sahara {
	dev: String,
	port: SerialPort,
}

impl Deref for Sahara {
	type Target = SerialPort;

	fn deref(&self) -> &Self::Target {
		&self.port
	}
}

impl DerefMut for Sahara {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.port
	}
}

impl From<SerialPort> for Sahara {
	fn from(port: SerialPort) -> Self {
		let dev = port.name().expect("Virtual devices not supported");
		Self { port, dev }
	}
}

impl From<Sahara> for SerialPort {
	fn from(value: Sahara) -> Self {
		value.port
	}
}

impl Sahara {
	fn create(dev: &str) -> SerialPortBuilder {
		serialport::new(dev, BAUD_RATE)
			.parity(Parity::None)
			.stop_bits(StopBits::One)
			.data_bits(DataBits::Eight)
			.timeout(Duration::from_millis(10000))
	}

	pub fn connect(dev: &str) -> Result<Self, SerialError> {
		Ok(Self {
			port: Self::create(dev).open()?,
			dev: dev.to_owned(),
		})
	}

	pub fn reconnect(&mut self) -> Result<(), SerialError> {
		self.port = Self::create(&self.dev).open()?;
		Ok(())
	}

	pub fn wait(dev: &str, interval: Duration) -> Result<(), SerialError> {
		loop {
			let avail = serialport::available_ports()?;

			if avail.iter().any(|it| it.port_name == dev) {
				break Ok(());
			}

			thread::sleep(interval);
		}
	}

	pub fn send(&mut self, kind: u32, payload: &[u8]) -> Result<(), IoError> {
		let len = /* kind */ U32 + /* len */ U32 + payload.len();

		self.port.write_all(pack![u32!(kind), u32!(len), payload])
	}

	pub fn read_message(&mut self) -> Result<(u32, Vec<u8>), ReadError> {
		// Allocating minimal buffer for packet header
		let mut header = [0; U32 + U32];
		// Reading bytes from port
		self.port.read_exact(&mut header)?;
		// Extracting packet kind and length
		let kind = un32!(header, 0);
		let len = un32!(header, U32) as usize;
		// Allocating buffer for packet payload
		let mut payload = vec![0; len - U32 - U32];
		// Reading rest bytes
		self.port.read_exact(&mut payload)?;
		// Wrapping into tuple
		Ok((kind, payload))
	}
}
