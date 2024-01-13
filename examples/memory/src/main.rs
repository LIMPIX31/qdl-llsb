use std::io::Read;
use std::time::Duration;

use anyhow::Result;
use sahara::{Mode, Sahara};
use sahara_mem::SaharaMemExt;

const SERIAL_PORT: &str = "/dev/ttyUSB0";

fn main() -> Result<()> {
	println!("Waiting device on {SERIAL_PORT}");
	let mut sahara = Sahara::wait_connect(SERIAL_PORT, Duration::from_secs(1))?;
	println!("Device connected");

	println!("Reading memory..");
	let mem_slice = sahara.readmem(0x_00_00_00_00, 0x_30)?;
	println!("{mem_slice:02X?}");

	Ok(())
}
