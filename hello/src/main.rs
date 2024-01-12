#![feature(decl_macro)]

use std::io::Read;
use std::io::Write;
use std::thread;
use std::time::Duration;

use anyhow::bail;
use anyhow::Result;
use serialport::DataBits;
use serialport::Parity;
use serialport::StopBits;

const SERIAL_PORT: &str = "/dev/ttyUSB0";

macro pkt($($byte:literal,)*) {
	&[$($byte, 0, 0, 0),*]
}

fn main() -> Result<()> {
	println!("Waiting for a port on {SERIAL_PORT}");

	let mut port = loop {
		let result = serialport::new(SERIAL_PORT, 115_200)
			.parity(Parity::None)
			.stop_bits(StopBits::One)
			.data_bits(DataBits::Eight)
			.timeout(Duration::from_secs(1))
			.open();

		match result {
			Ok(port) => {
				break port;
			}
			Err(_) => {
				print!(".");
				std::io::stdout().flush()?;
			}
		}

		thread::sleep(Duration::from_secs(1))
	};

	println!("Device found");

	// Ожидание hello сообщения
	{
		let mut pkt = [0; 12 * 4];
		port.read_exact(&mut pkt)?;
		if pkt[0] != 0x01 {
			bail!("Device in an incorrect state");
		}
		println!("Hello packet received");
	}

	// Отправка ответа (переход в режим команд)
	{
		port.write_all(pkt![
			2,  // Id команды
			48, // Длина этого сообщения (4 * 12)
			2,  // Версия Sahara
			1,  // <неизвестно>
			0,  // <неизвестно>
			3,  // Переход в режим команд
			1, 2, 3, 4, 5, 6, // Неизвестно
		])?;
		println!("Hello response send");
	}

	{
		let mut pkt = [0; 4 * 2];
		port.read_exact(&mut pkt)?;
		if pkt[0] != 0x0B {
			bail!("Failed to enter command mode");
		}
		println!("Sahara switched to command mode");
	}

	// Открытие потока команды (get_msm_hwid)
	{
		port.write_all(pkt![
			13, // Id команды на открытие,
			12, // Длина сообщения (4 * 3),
			2, // Команда которую нужно выполнить (get_msm_hwid)
		])?;
	}

	// Получение результата выполнения команды
	let payload_len = {
		let mut pkt = [0; 4 * 4];
		port.read_exact(&mut pkt)?;
		if pkt[0] != 0x0E {
			bail!("Command failed");
		}
		println!("Command executed successfully");
		// Длина ответных данных
		u32::from_ne_bytes(pkt[12..12 + 4].try_into()?) as usize
	};

	{
		port.write_all(pkt![
			15, // Команда для запроса данных
			12, // Длина сообщения (3 * 4)
			2, // Результат какой команды запросить (get_msm_hwid)
		])?;
	}

	// Чтение ответа команды
	{
		let mut pkt = vec![0; payload_len];
		port.read_exact(&mut pkt)?;
		println!("msm_hwid: {:02X?}", pkt);
	}

	Ok(())
}
