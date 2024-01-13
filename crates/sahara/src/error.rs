use std::io::Error as IoError;

use serialport::Error as SerialError;
use thiserror::Error;

macro upcast($from:tt for $to:tt) {
	impl From<$from> for $to {
		fn from(value: $from) -> Self {
			value.into()
		}
	}
}

#[derive(Debug, Error)]
#[error("Sahara in unknown mode {0:02X?}")]
pub struct UnknownModeError(pub u32);

#[derive(Debug, Error)]
#[error("Unknown sahara incoming message {0:08X?}")]
pub struct UnknownMessageError(pub u32);

#[derive(Debug, Error)]
#[error("Sahara does not want to deal with you anymore")]
pub enum EndTransferError {
	#[error("0x00: Invalid command received in current state")]
	InvalidCommand,
	#[error("0x01: Protocol mismatch between host and target")]
	ProtocolMismatch,
	#[error("0x02: Invalid target protocol version")]
	InvalidTargetProtocolVersion,
	#[error("0x03: Invalid host protocol version")]
	InvalidHostProtocolVersion,
	#[error("0x04: Invalid packet size received")]
	InvalidPacketSize,
	#[error("0x05: Unexpected image ID received")]
	UnexpectedImageId,
	#[error("0x06: Invalid image header size received")]
	InvalidImageHeaderSize,
	#[error("0x07: Invalid image data size received")]
	InvalidImageDataSize,
	#[error("0x08: Invalid image type received")]
	InvalidImageType,
	#[error("0x09: Invalid tranmission length")]
	InvalidTransmissionLength,
	#[error("0x0A: Invalid reception length")]
	InvalidReceptionLength,
	#[error("0x0B: General transmission or reception error")]
	GenerealError,
	#[error("0x0C: Error while transmitting READ_DATA packet")]
	ErrorTransmittingReadData,
	#[error("0x0D: Cannot receive specified number of program headers")]
	CannotReceiveProgramHeaders,
	#[error("0x0E: Invalid data length received for program headers")]
	InvalidProgramDataLengthReceived,
	#[error("0x0F: Multiple shared segments found in ELF image")]
	MultipleSharedSegmentsFound,
	#[error("0x10: Uninitialized program header location")]
	UninitializedProgramHeaderLoaction,
	#[error("0x11: Invalid destination address")]
	InvalidDestinationAddress,
	#[error("0x12: Invalid data size received in image header")]
	InvalidImageHeaderDataSize,
	#[error("0x13: Invalid ELF header received")]
	InvalidElfHEader,
	#[error("0x14: Unknown host error received in HELLO_RESP")]
	UnknownHostHelloError,
	#[error("0x15: Timeout while receiving data")]
	TimeoutWhileReceiving,
	#[error("0x16: Timeout while transmitting data")]
	TimeoutWhileTransmitting,
	#[error("0x17: Invalid mode received from host")]
	InvalidModeReceivedFromHost,
	#[error("0x18: Invalid memory read access")]
	InvalidMemoryReadAccess,
	#[error("0x19: Host cannot handle read data size requested")]
	HostCannotHandleReadDataSize,
	#[error("0x1A: Memory debug not supported")]
	MemoryDebugNotSupported,
	#[error("0x1B: Invalid mode switch")]
	InvalidModeSwitch,
	#[error("0x1C: Failed to execute command")]
	FailedToExecuteCommand,
	#[error("0x1D: Invalid parameter passed to command execution")]
	InvalidParameterPassedToCommand,
	#[error("0x1E: Unsupported client command received")]
	UnsupportedClientCommand,
	#[error("0x1F: Invalid client command received for data response")]
	InvalidClientCommand,
	#[error("0x20: Failed to authenticate hash table")]
	FailedToAuthenticateHashTable,
	#[error("0x21: Failed to verify hash for a given segment of ELF image")]
	FailedToVerifyElfImageSegment,
	#[error("0x22: Failed to find hash table in ELF image")]
	FailedToFindHashTableInElfImage,
	#[error("0x23: Target failed to initialize")]
	TargetFailedToInitialize,
	#[error("0x24: Failed to authenticate generic image")]
	FailedToAuthenticateGenericImage,
	#[error("0x25: Invalid ELF hash table size.  Too bit or small.")]
	InvalidElfHashTableSize,
	#[error("0x26: Invalid IMG Hash Table Size")]
	InvalidHashTableSize,
	#[error("0x27: Enumeration failed")]
	EnumerationFailed,
	#[error("0x28: Hardware Bulk transfer error")]
	HardwareBulkTransferError,
	#[error("Unknown sahara error")]
	Unknown,
}

impl From<u32> for EndTransferError {
	fn from(value: u32) -> Self {
		match value {
			0x00 => Self::InvalidCommand,
			0x01 => Self::ProtocolMismatch,
			0x02 => Self::InvalidTargetProtocolVersion,
			0x03 => Self::InvalidHostProtocolVersion,
			0x04 => Self::InvalidPacketSize,
			0x05 => Self::UnexpectedImageId,
			0x06 => Self::InvalidImageHeaderSize,
			0x07 => Self::InvalidImageDataSize,
			0x08 => Self::InvalidImageType,
			0x09 => Self::InvalidTransmissionLength,
			0x0A => Self::InvalidReceptionLength,
			0x0B => Self::GenerealError,
			0x0C => Self::ErrorTransmittingReadData,
			0x0D => Self::CannotReceiveProgramHeaders,
			0x0E => Self::InvalidProgramDataLengthReceived,
			0x0F => Self::MultipleSharedSegmentsFound,
			0x10 => Self::UninitializedProgramHeaderLoaction,
			0x11 => Self::InvalidDestinationAddress,
			0x12 => Self::InvalidImageHeaderDataSize,
			0x13 => Self::InvalidElfHEader,
			0x14 => Self::UnknownHostHelloError,
			0x15 => Self::TimeoutWhileReceiving,
			0x16 => Self::TimeoutWhileTransmitting,
			0x17 => Self::InvalidModeReceivedFromHost,
			0x18 => Self::InvalidMemoryReadAccess,
			0x19 => Self::HostCannotHandleReadDataSize,
			0x1A => Self::MemoryDebugNotSupported,
			0x1B => Self::InvalidModeSwitch,
			0x1C => Self::FailedToExecuteCommand,
			0x1D => Self::InvalidParameterPassedToCommand,
			0x1E => Self::UnsupportedClientCommand,
			0x1F => Self::InvalidClientCommand,
			0x20 => Self::FailedToAuthenticateHashTable,
			0x21 => Self::FailedToVerifyElfImageSegment,
			0x22 => Self::FailedToFindHashTableInElfImage,
			0x23 => Self::TargetFailedToInitialize,
			0x24 => Self::FailedToAuthenticateGenericImage,
			0x25 => Self::InvalidElfHashTableSize,
			0x26 => Self::InvalidHashTableSize,
			0x27 => Self::EnumerationFailed,
			0x28 => Self::HardwareBulkTransferError,
			_ => Self::Unknown,
		}
	}
}

#[derive(Debug, Error)]
#[error("Connection is broken or unsable")]
pub struct BrokenError;

#[derive(Debug, Error)]
#[error("Payload length expected: {expected}, but got: {got}")]
pub struct InvalidPayloadLengthError {
	pub expected: usize,
	pub got: usize,
}

#[derive(Debug, Error)]
pub enum ConnectError {
	#[error(transparent)]
	Serial(#[from] SerialError),

	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	EndTransfer(#[from] EndTransferError),

	#[error(transparent)]
	UnknownMode(#[from] UnknownModeError),

	#[error(transparent)]
	Broken(#[from] BrokenError),
}

upcast!(RawPacketReadError for ConnectError);

#[derive(Debug, Error)]
pub enum ChangeModeError {
	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	UnknownMessage(#[from] UnknownMessageError),

	#[error(transparent)]
	EndTransfer(#[from] EndTransferError),

	#[error(transparent)]
	Broken(#[from] BrokenError),
}

upcast!(RawPacketReadError for ChangeModeError);

#[derive(Debug, Error)]
pub enum RawPacketReadError {
	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	Broken(#[from] BrokenError),
}

#[derive(Debug, Error)]
pub enum ExactMessageReadError {
	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	Broken(#[from] BrokenError),
}

upcast!(RawPacketReadError for ExactMessageReadError);

#[derive(Debug, Error)]
pub enum ExecError {
	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	UnknownMessage(#[from] UnknownMessageError),

	#[error(transparent)]
	EndTransfer(#[from] EndTransferError),

	#[error(transparent)]
	Broken(#[from] BrokenError),

	#[error(transparent)]
	InvalidPayloadLength(#[from] InvalidPayloadLengthError),
}

upcast!(ChangeModeError for ExecError);
upcast!(RawPacketReadError for ExecError);

#[derive(Debug, Error)]
pub enum DeviceResetError {
	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	EndTransfer(#[from] EndTransferError),
}

upcast!(RawPacketReadError for DeviceResetError);
