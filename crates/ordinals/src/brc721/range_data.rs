#[derive(Clone, Debug)]
pub struct RangeData {}

impl redb::Value for RangeData {
	type SelfType<'a> = Self;
	type AsBytes<'a> = [u8; 1];

	fn fixed_width() -> Option<usize> {
		None
	}

	fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
	where
		Self: 'a,
	{
		// Since RangeData is empty, we just return a new instance
		// regardless of the input bytes
		RangeData {}
	}

	fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
	where
		Self: 'b,
	{
		[0u8]
	}

	fn type_name() -> redb::TypeName {
		redb::TypeName::new("brc721::range_data")
	}
}

