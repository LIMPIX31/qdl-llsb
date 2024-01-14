pub macro pack($($it:expr),*$(,)?) {
	&[$(&$it[..]),*].concat()
}

pub macro u32($it:expr) {
	($it as u32).to_le_bytes()
}

pub macro u32s($($it:expr),*$(,)?) {
	[$(u32!($it)),*].concat()
}

pub macro un32($it:expr, $at:expr) {
	u32::from_le_bytes([$it[$at], $it[$at + 1], $it[$at + 2], $it[$at + 3]])
}

pub const U32: usize = 4;
