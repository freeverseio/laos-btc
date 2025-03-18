/// Byte to identify which operation of brc721 is used. Just a fancy name to don't remember the
/// numerical byte of each operation. Only fieldless variants are allowed, leading to a compile
/// error otherwise.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Brc721Operation {
	RegisterCollection = 0x00,
}

impl TryFrom<u8> for Brc721Operation {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x00 => Ok(Brc721Operation::RegisterCollection),
			_ => Err(()),
		}
	}
}


