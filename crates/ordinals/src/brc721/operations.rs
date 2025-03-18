/// Byte to identify which operation of brc721 is used. Just a fancy name to don't remember the
/// numerical byte of each operation. Only fieldless variants are allowed, leading to a compile
/// error otherwise.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub(crate) enum Brc721Operation {
	RegisterCollection = 0,
	// TODO: Remove this attribute as soon as register ownership flow is introduced
	#[allow(dead_code)]
	RegisterOwnership = 1,
}
