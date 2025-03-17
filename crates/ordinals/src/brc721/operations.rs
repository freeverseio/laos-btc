/// Byte to identify which operation of brc721 is used. Just a fancy name to don't remember the
/// numerical byte of each operation. Only fieldless variants are allowed, leading to a compile
/// error otherwise.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Brc721Operation {
	RegisterCollection = 0,
	RegisterOwnership = 1,
}

impl Brc721Operation {
	pub fn byte_slice(self) -> [u8; BRC721_OPERATION_LENGTH] {
		[self as u8]
	}
}

/// The size of a brc721 flag in bytes
pub const BRC721_OPERATION_LENGTH: usize = 1;
