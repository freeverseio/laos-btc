use super::collection_id::Brc721CollectionId;
use crate::{Deserialize, Serialize};
use sp_core::H160;
use std::fmt;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brc721Collection {
	pub collection_id: Brc721CollectionId,
	pub address: H160,
	pub rebaseable: bool,
}

impl Brc721Collection {
	// Constructor function
	pub fn new(collection_id: Brc721CollectionId, address: H160, rebaseable: bool) -> Self {
		Brc721Collection { collection_id, address, rebaseable }
	}
}

impl fmt::Display for Brc721Collection {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} - {:?} - {}", self.collection_id, self.address, self.rebaseable)
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_brc721_collection_creation() {
        let collection_id = Brc721CollectionId::new(); // Assuming you have a method to create a new ID
        let address = H160::from(hex!("0000000000000000000000000000000000000001"));
        let rebaseable = true;

        let collection = Brc721Collection::new(collection_id.clone(), address, rebaseable);

        assert_eq!(collection.collection_id, collection_id);
        assert_eq!(collection.address, address);
        assert_eq!(collection.rebaseable, rebaseable);
    }

    #[test]
    fn test_brc721_collection_display() {
        let collection_id = Brc721CollectionId::new(); // Create a collection ID
        let address = H160::from(hex!("0000000000000000000000000000000000000001"));
        let rebaseable = false;

        let collection = Brc721Collection::new(collection_id.clone(), address, rebaseable);

        let expected_display = format!("{} - {:?} - {}", collection_id, address, rebaseable);
        assert_eq!(format!("{}", collection), expected_display);
    }

    #[test]
    fn test_brc721_collection_equality() {
        let collection_id = Brc721CollectionId::new(); // Assume this method exists
        let address = H160::from(hex!("0000000000000000000000000000000000000001"));
        
        let collection_a = Brc721Collection::new(collection_id.clone(), address, true);
        let collection_b = Brc721Collection::new(collection_id.clone(), address, true);
        let collection_c = Brc721Collection::new(collection_id.clone(), address, false);

        assert_eq!(collection_a, collection_b);
        assert_ne!(collection_a, collection_c);
    }
}