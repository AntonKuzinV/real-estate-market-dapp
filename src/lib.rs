use std::iter::once_with;
use near_sdk::{
    *,
    borsh::{self, *},
    collections::*,
    json_types::*,
    serde::{self, *},
};
#[allow(unused_imports)]
use near_sdk::{AccountId, env, near_bindgen};

pub use property::*;
use crate::serde_json::json;

mod property;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub properties: UnorderedMap<u32, Property>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            properties: UnorderedMap::new(b"a".to_vec())
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn add_property(&mut self, property_name: String, property_type: String, location: String, rooms: u16, floor: u16, storeys: u8, squarespace: u16, price: u128) {
        let id = self.properties.len() as u32;
        let property = Property::new(property_name, property_type, location, rooms, floor, storeys, squarespace, price);
        self.properties.insert(&id, &property);

        env::log_str("Added new property")
    }

    pub fn get_properties(&self) -> Vec<(u32, Property)> {
        self.properties.iter().collect()
    }

    pub fn get_property(&self, property_id: u32) -> Property {
        self.properties.get(&property_id).unwrap()
    }

    #[payable]
    pub fn buy_property(&mut self, property_id: u32) {
        let mut property = self.get_property(property_id);
        assert!(property.is_for_sale, "You cannot buy not for sale property");

        let buyer_id = env::signer_account_id().clone();
        let owner = property.owner.clone();
        assert_ne!(owner, buyer_id, "You cannot purchase your own property");

        let price = ONE_NEAR * property.price;
        let payment = env::attached_deposit();
        assert!(payment >= price, "Not enough funds {} to pay {}", payment, price);

        Promise::new(owner).transfer(price);
        property.update_owner(buyer_id);
        property.set_is_not_sale();
        self.properties.insert(&property_id, &property);

        env::log_str("You bought new property");
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::*, testing_env};

    use crate::*;

    const ONE_NEAR: u128 = u128::pow(10, 24);

    fn contract_account() -> AccountId {
        "contract".parse::<AccountId>().unwrap()
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(contract_account())
            .account_balance(15 * ONE_NEAR)
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test() {}
}
