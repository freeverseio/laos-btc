use bitcoin::opcodes;

/// The opcode used to identify register collection operations.
pub(crate) const BRC721_INIT_CODE: opcodes::Opcode = opcodes::all::OP_PUSHNUM_15;

/// Byte to identify which operation of brc721 is used. Just a fancy name to don't remember the
/// numerical byte of each operation. Only fieldless variants are allowed, leading to a compile
/// error otherwise.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub(crate) enum Brc721Operation {
	RegisterCollection = 0,
	RegisterOwnership = 1,
}

impl Brc721Operation {
	pub(crate) fn byte_slice(self) -> [u8; BRC721_OPERATION_LENGTH] {
		[self as u8]
	}
}

/// The size of a brc721 flag in bytes
pub(crate) const BRC721_OPERATION_LENGTH: usize = 1;
