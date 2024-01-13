use sahara::error::ExecError;
use sahara::protocol::slice_u32;
use sahara::protocol::U32;
use sahara::Sahara;

#[derive(Debug, Clone)]
pub struct HwId {
	pub vendor: u32,
	pub msm: u32,
}

pub trait SaharaInfoExt {
	fn serial_num(&mut self) -> Result<u32, ExecError>;
	fn hwid(&mut self) -> Result<HwId, ExecError>;
	fn pkhash(&mut self) -> Result<Vec<u8>, ExecError>;
	fn sbl_version(&mut self) -> Result<u32, ExecError>;
}

impl SaharaInfoExt for Sahara {
	fn serial_num(&mut self) -> Result<u32, ExecError> {
		let payload = self.exec(0x01)?;
		Ok(slice_u32!(payload, 0))
	}

	fn hwid(&mut self) -> Result<HwId, ExecError> {
		let payload = self.exec(0x02)?;
		Ok(HwId {
			vendor: slice_u32!(payload, 0),
			msm: slice_u32!(payload, U32),
		})
	}

	fn pkhash(&mut self) -> Result<Vec<u8>, ExecError> {
		let payload = self.exec(0x03)?;
		let prefix = &payload[..4];
		let idx = payload[4..].windows(4).position(|w| w == prefix);

		if let Some(idx) = idx {
			Ok(payload[..4 + idx].to_owned())
		} else {
			Ok(payload)
		}
	}

	fn sbl_version(&mut self) -> Result<u32, ExecError> {
		let payload = self.exec(0x07)?;
		Ok(slice_u32!(payload, 0))
	}
}
