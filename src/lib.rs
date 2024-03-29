use near_sdk::{
    borsh::{self, *},
    collections::*,
    *,
};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen, AccountId};

pub use property::*;

mod property;

const LIMIT: u64 = 50;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    properties: UnorderedMap<u32, Property>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            properties: UnorderedMap::<u32, Property>::new(b"p"),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn add_property(
        &mut self, property_name: String, property_type: String, location: String, rooms: u16,
        floor: u16, storeys: u8, squarespace: u16, price: u128,
    ) {
        let id = self.properties.len() as u32;
        let property = Property::new(
            property_name,
            property_type,
            location,
            rooms,
            floor,
            storeys,
            squarespace,
            price,
        );
        self.properties.insert(&id, &property);

        env::log_str("You added new property")
    }

    pub fn get_properties(&self, from_index: u64) -> Vec<(u32, Property)> {
        let keys = self.properties.keys_as_vector();
        let values = self.properties.values_as_vector();
        (from_index..std::cmp::min(from_index + LIMIT, self.properties.len()))
            .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap()))
            .collect()
    }

    pub fn get_property(&self, property_id: u32) -> Property {
        self.properties
            .get(&property_id)
            .expect("failed to get property with provided id")
    }

    pub fn get_own_property(&self) -> Vec<(u32, Property)> {
        let caller = env::signer_account_id();

        self.properties
            .iter()
            .filter(|property| property.1.owner == caller)
            .collect()
    }

    #[payable]
    pub fn buy_property(&mut self, property_id: u32) {
        let mut property = self.get_property(property_id);
        assert!(property.is_for_sale, "You cannot buy not for sale property");

        let buyer_id = env::signer_account_id().clone();
        let owner = property.owner.clone();
        assert_ne!(owner, buyer_id, "You cannot purchase your own property");

        let price = ONE_NEAR * u128::from(property.price);
        let payment = env::attached_deposit();
        assert!(
            payment >= price,
            "Not enough funds {} to pay {}",
            payment / ONE_NEAR,
            price / ONE_NEAR
        );

        Promise::new(owner).transfer(price);
        property.update_owner(buyer_id);
        property.set_is_not_for_sale();
        self.properties.insert(&property_id, &property);

        env::log_str("You bought new property");
    }

    pub fn put_property_on_sale(&mut self, property_id: u32) {
        let mut property = self.get_property(property_id);

        let caller = env::signer_account_id().clone();
        let owner = property.owner.clone();
        assert_eq!(owner, caller, "You are not an owner of this property");

        assert!(!property.is_for_sale, "Property is already on sale");

        property.set_is_for_sale();
        self.properties.insert(&property_id, &property);

        env::log_str("You placed your property on sale");
    }

    pub fn put_property_off_sale(&mut self, property_id: u32) {
        let mut property = self.get_property(property_id);

        let caller = env::signer_account_id().clone();
        let owner = property.owner.clone();
        assert_eq!(owner, caller, "You are not an owner of this property");

        assert!(property.is_for_sale, "Property is already not on sale");

        property.set_is_not_for_sale();
        self.properties.insert(&property_id, &property);

        env::log_str("You placed your property on sale");
    }

    pub fn delete_property(&mut self, property_id: u32) {
        let property = self.get_property(property_id);

        let caller = env::signer_account_id().clone();
        let owner = property.owner.clone();
        assert_eq!(owner, caller, "You are not an owner of this property");

        self.properties.remove(&property_id);
        env::log_str("Successfully deleted property from a market");
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::*, testing_env};

    use crate::*;

    use super::*;

    fn contract_account() -> AccountId {
        "contract.testnet".parse::<AccountId>().unwrap()
    }

    fn owner_account_id() -> AccountId {
        "owner.testnet".parse::<AccountId>().unwrap()
    }

    fn buyer_account_id() -> AccountId {
        "buyer.testnet".parse::<AccountId>().unwrap()
    }

    fn get_context(signer: &AccountId, deposit: Option<u128>) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(contract_account())
            .account_balance(1000 * ONE_NEAR)
            .signer_account_id(signer.clone())
            .attached_deposit(deposit.unwrap_or(0))
            .prepaid_gas(Gas(30_000_000_000_000));
        builder
    }

    #[test]
    fn add_property() {
        let context = get_context(&owner_account_id(), None);
        testing_env!(context.build());

        let mut contract = Contract::default();
        contract.add_property(
            String::from("Testname"),
            String::from("Apartment"),
            String::from("Ukraine"),
            2,
            4,
            1,
            45,
            100,
        );

        assert_eq!(contract.properties.len(), 1, "Property wasn't created");
    }

    #[test]
    fn get_own_property() {
        let context = get_context(&owner_account_id(), None);
        testing_env!(context.build());
        let mut contract = Contract::default();

        let num_of_properties = 5;
        for _ in 0..num_of_properties {
            contract.add_property(
                String::from("Testname"),
                String::from("Apartment"),
                String::from("Ukraine"),
                2,
                4,
                1,
                45,
                100,
            );
        }

        let properties = contract.get_own_property();
        assert_eq!(
            properties.len(),
            num_of_properties,
            "Not all properties has been added"
        )
    }

    #[test]
    fn buy_property() {
        let context = get_context(&owner_account_id(), None);
        testing_env!(context.build());

        let mut contract = Contract::default();
        contract.add_property(
            String::from("Testname"),
            String::from("Apartment"),
            String::from("Ukraine"),
            2,
            4,
            1,
            45,
            100,
        );

        let property_to_buy = contract.get_property(0);

        let context = get_context(
            &buyer_account_id(),
            Some(u128::from(property_to_buy.price) * ONE_NEAR),
        );
        testing_env!(context.build());

        contract.buy_property(0);

        let own_properties = contract.get_own_property();
        assert_eq!(own_properties.len(), 1, "Property wasn't bought");
    }

    #[test]
    #[should_panic(expected = "You cannot purchase your own property")]
    fn buy_own_property() {
        let context = get_context(&owner_account_id(), None);
        testing_env!(context.build());

        let mut contract = Contract::default();
        contract.add_property(
            String::from("Testname"),
            String::from("Apartment"),
            String::from("Ukraine"),
            2,
            4,
            1,
            45,
            100,
        );

        contract.buy_property(0);
    }

    #[test]
    #[should_panic(expected = "Not enough funds 1 to pay 100")]
    fn buy_property_with_insufficient_deposit() {
        let context = get_context(&owner_account_id(), None);
        testing_env!(context.build());

        let mut contract = Contract::default();
        contract.add_property(
            String::from("Testname"),
            String::from("Apartment"),
            String::from("Ukraine"),
            2,
            4,
            1,
            45,
            100,
        );

        let context = get_context(&buyer_account_id(), Some(ONE_NEAR));
        testing_env!(context.build());

        contract.buy_property(0);
    }

    #[test]
    fn put_property_off_sale() {
        let context = get_context(&owner_account_id(), None);
        testing_env!(context.build());

        let mut contract = Contract::default();
        contract.add_property(
            String::from("Testname"),
            String::from("Apartment"),
            String::from("Ukraine"),
            2,
            4,
            1,
            45,
            100,
        );

        contract.put_property_off_sale(0);

        let property = contract.get_property(0);
        assert_eq!(
            property.is_for_sale, false,
            "Failed to put property off sale"
        );
    }

    #[test]
    fn put_property_on_sale() {
        let context = get_context(&owner_account_id(), None);
        testing_env!(context.build());

        let mut contract = Contract::default();
        contract.add_property(
            String::from("Testname"),
            String::from("Apartment"),
            String::from("Ukraine"),
            2,
            4,
            1,
            45,
            100,
        );

        contract.put_property_off_sale(0);

        contract.put_property_on_sale(0);
        let property = contract.get_property(0);
        assert_eq!(property.is_for_sale, true, "Failed to put property on sale");
    }

    #[test]
    fn delete_property() {
        let context = get_context(&owner_account_id(), None);
        testing_env!(context.build());

        let mut contract = Contract::default();
        contract.add_property(
            String::from("Testname"),
            String::from("Apartment"),
            String::from("Ukraine"),
            2,
            4,
            1,
            45,
            100,
        );

        contract.delete_property(0);
        assert_eq!(contract.properties.len(), 0, "Failed to delete property");
    }

    #[test]
    #[should_panic(expected = "You are not an owner of this property")]
    fn delete_not_owned_property() {
        let context = get_context(&owner_account_id(), None);
        testing_env!(context.build());

        let mut contract = Contract::default();
        contract.add_property(
            String::from("Testname"),
            String::from("Apartment"),
            String::from("Ukraine"),
            2,
            4,
            1,
            45,
            100,
        );

        let context = get_context(&buyer_account_id(), None);
        testing_env!(context.build());

        contract.delete_property(0);
    }
}
