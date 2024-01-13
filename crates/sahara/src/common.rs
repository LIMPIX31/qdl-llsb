use std::fmt::Display;
use std::fmt::Formatter;

use crate::error::UnknownModeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
	TxPending,
	TxComplete,
	MemoryDebug,
	Command,
}

impl Display for Mode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Mode::TxPending => write!(f, "tx_pending"),
			Mode::TxComplete => write!(f, "tx_complete"),
			Mode::MemoryDebug => write!(f, "memory_debug"),
			Mode::Command => write!(f, "command"),
		}
	}
}

impl TryFrom<u32> for Mode {
	type Error = UnknownModeError;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::TxPending),
			1 => Ok(Self::TxComplete),
			2 => Ok(Self::MemoryDebug),
			3 => Ok(Self::Command),
			unknown => Err(UnknownModeError(unknown)),
		}
	}
}

impl Mode {
	pub fn to_le_bytes(&self) -> [u8; 4] {
		match self {
			Mode::TxPending => 0u32.to_le_bytes(),
			Mode::TxComplete => 1u32.to_le_bytes(),
			Mode::MemoryDebug => 2u32.to_le_bytes(),
			Mode::Command => 3u32.to_le_bytes(),
		}
	}
}
