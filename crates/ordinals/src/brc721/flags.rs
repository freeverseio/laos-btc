use bitcoin::opcodes;

/// The opcode used to identify register collection operations.
pub(crate) const BRC721_INIT_CODE: opcodes::Opcode = opcodes::all::OP_PUSHNUM_15;

/// Byte flag to identify which mode of brc721 is used. Just a fancy name to don't remember the
/// numerical byte of each operation. Only fieldless variants are allowed, leading to a compile
/// error otherwise.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Brc721Flag {
	RegisterCollection,
	// TODO: Remove this attribute as soon as register ownership flow is introduced
	#[allow(dead_code)]
	RegisterOwnership,
}

impl Brc721Flag {
	pub(crate) fn byte_slice(self) -> [u8; 1] {
		[self as u8]
	}
}

/// The size of a brc721 flag in bytes
pub(crate) const BRC721_FLAG_LENGTH: usize = 1;
