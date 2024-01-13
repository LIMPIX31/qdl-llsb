use std::time::Duration;

use anyhow::Result;
use sahara::Sahara;

const SERIAL_PORT: &str = "/dev/ttyUSB0";

fn main() -> Result<()> {
	println!("Waiting device on {SERIAL_PORT}");
	let mut sahara = Sahara::wait_connect(SERIAL_PORT, Duration::from_secs(1))?;
	println!("Device connected");

	let hwid = sahara.exec(0x02)?;

	println!("{:02X?}", hwid);

	Ok(())
}
