use std::time::Duration;

use anyhow::Result;
use sahara::Sahara;

#[cfg(target_family = "unix")]
const SERIAL_PORT: &str = "/dev/ttyUSB0";
#[cfg(target_os = "windows")]
const SERIAL_PORT: &str = "COM1";

pub fn connect() -> Result<Sahara> {
	println!("Waiting device on {SERIAL_PORT}");
	Sahara::wait(SERIAL_PORT, Duration::from_secs(1))?;
	println!("Device found");
	Ok(Sahara::connect(SERIAL_PORT)?)
}
