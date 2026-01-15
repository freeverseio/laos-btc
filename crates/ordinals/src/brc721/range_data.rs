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
		unimplemented!()
	}

	fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
	where
		Self: 'b,
	{
		unimplemented!()
	}

	fn type_name() -> redb::TypeName {
		redb::TypeName::new("brc721::range_data")
	}
}

