use std::borrow::ToOwned;
use std::io::Write;

use anyhow::bail;
use anyhow::ensure;
use anyhow::Result;
use sahara::pack;
use sahara::u32s;
use sahara::un32;
use sahara::Sahara;
use sahara::U32;

/// ## Warning
/// This program is provided as an example only
/// and is only suitable for the **`MSM8953`** cpu.
const PROGRAM: &[u8] = include_bytes!("prog_emmc_firehose_8953_ddr.mbn");

/// Performs a handshake between the device to enter upload mode
fn hello(device: &mut Sahara) -> Result<(u32, usize, usize)> {
	let (kind, _) = device.read_message()?;
	ensure!(kind == 0x01, "Device in incorrect state");

	device.send(
		0x02,
		pack![u32s![2, 1, 0, /* tx mode */ 0, 1, 2, 3, 4, 5, 6]],
	)?;

	let (kind, payload) = device.read_message()?;
	ensure!(kind == 0x03 || kind == 0x12, "Operation failed");
	println!("Device in tx mode");

	let image_id = un32!(payload, 0);
	let data_offset = un32!(payload, U32);
	let data_len = un32!(payload, U32 * 2);

	println!("image_id: {image_id}");
	println!("data_offset: {data_offset}");
	println!("data_len: {data_len}");

	Ok((image_id, data_offset as usize, data_len as usize))
}

fn main() -> Result<()> {
	let mut sahara = eg::connect()?;

	// Assume that image_id is firehose
	let (_, mut data_offset, mut data_len) = hello(&mut sahara)?;
	let mut program = PROGRAM.to_owned();

	{
		let program_len = data_offset + data_len;
		if program_len > program.len() {
			program.resize(program_len, 0xFF);
		}
	}

	loop {
		let chunk = &program[data_offset..data_offset + data_len];
		let _ = sahara.write(chunk)?;
		let (kind, payload) = sahara.read_message()?;

		match kind {
			0x03 | 0x12 => {
				data_offset = un32!(payload, U32) as usize;
				data_len = un32!(payload, U32 * 2) as usize;
			}
			0x04 => {
				let status = un32!(payload, U32);

				if status == 0x00 {
					println!("Program successfully uploaded");
					break;
				}

				bail!("Failed to upload program: {status:02X}");
			}
			0x05 => {
				sahara.send(0x05, &[])?;
				let (kind, payload) = sahara.read_message()?;
				ensure!(
					kind == 0x06,
					"Failed to upload program: {}",
					un32!(payload, U32)
				);
			}
			_ => bail!("Unknown message received"),
		}
	}

	Ok(())
}
