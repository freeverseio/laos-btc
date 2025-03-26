use std::ops::RangeInclusive;
use sp_core::H160;
use super::Slot;

struct TokenRange {
	slot_rage: RangeInclusive<Slot>,
	registrant: H160,
}
