use anyhow::bail;
use anyhow::Result;

use sahara::pack;
use sahara::u32s;

fn main() -> Result<()> {
	let mut sahara = eg::connect()?;

	let (kind, payload) = sahara.read_message()?;
	println!("Kind: {kind:02X?}\nPayload: {payload:02X?}");

	if kind != 0x01 {
		bail!("Invalid device state");
	}

	sahara.send(0x02, pack![u32s![2, 1, 0, 3, 1, 2, 3, 4, 5, 6]])?;

	let (kind, payload) = sahara.read_message()?;
	println!("Kind: {kind:02X?}\nPayload: {payload:02X?}");

	if kind != 0x0B {
		bail!("Failed to change state")
	} else {
		println!("Device in command mode");
	}

	Ok(())
}
