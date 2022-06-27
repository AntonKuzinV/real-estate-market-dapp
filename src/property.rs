#[allow(unused_imports)]
use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Property {
    pub property_name: String,
    pub property_type: String,
    pub location: String,
    pub rooms: u16,
    pub floor: u16,
    pub storeys: u8,
    pub squarespace: u16,
    pub price: U128,
    pub owner: AccountId,
    pub is_for_sale: bool,
}

impl Property {
    pub fn new(property_name: String, property_type: String, location: String, rooms: u16, floor: u16, storeys: u8, squarespace: u16, price: u128) -> Self {
        assert!(property_name.len() > 0, "Property name is required");
        assert!(property_type.len() > 0, "Property type is required");
        assert!(location.len() > 0, "Property location is required");
        assert!(rooms > 0, "Number of rooms is required");
        assert!(floor > 0, "Floor is required");
        assert!(storeys > 0, "Number of storeys is required");
        assert!(squarespace > 0, "Squarespace is required");
        assert!(price > 0, "Property price is required");
        Property {
            property_name,
            property_type,
            location,
            rooms,
            floor,
            storeys,
            squarespace,
            price: U128::from(price),
            owner: env::signer_account_id(),
            is_for_sale: true,
        }
    }

    pub fn update_owner(&mut self, owner: AccountId) {
        self.owner = owner;
    }
    pub fn set_is_not_for_sale(&mut self) {
        self.is_for_sale = false;
    }

    pub fn set_is_for_sale(&mut self) {
        self.is_for_sale = true;
    }
}

