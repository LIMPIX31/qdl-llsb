use std::time::Duration;

use anyhow::Result;
use sahara::Sahara;
use sahara_info::SaharaInfoExt;

const SERIAL_PORT: &str = "/dev/ttyUSB0";

fn main() -> Result<()> {
	println!("Waiting device on {SERIAL_PORT}");
	let mut sahara = Sahara::wait_connect(SERIAL_PORT, Duration::from_secs(1))?;
	println!("Device connected");

	let serial_num = sahara.serial_num();
	println!("serial_num: {serial_num:08X?}");

	let hwid = sahara.hwid();
	println!("hwid: {hwid:?}");

	let pkhash = sahara.pkhash();
	println!("pkhash: {pkhash:02X?}");

	let sbl = sahara.sbl_version();
	println!("sbl: {sbl:08X?}");

	Ok(())
}
