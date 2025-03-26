use std::ops::RangeInclusive;
use sp_core::H160;
use super::Slot;

struct TokenRange {
	slot_rage: RangeInclusive<Slot>,
	registrant: H160,
}

impl TokenRange {
    fn new(first: Slot, last: Slot, registrant: H160) -> Self {

    TokenRange{
            slot_rage: first..=last,
            registrant: registrant,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_token_id(){
        // crate the Range
        let first_slot = Slot::try_from(1).unwrap();
        let last_slot = Slot::try_from(9).unwrap();
        let registrant = H160::default();
        let range = TokenRange::new(first_slot, last_slot, registrant);
    }
}
