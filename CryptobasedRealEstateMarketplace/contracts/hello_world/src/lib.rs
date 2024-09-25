#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, symbol_short, String};

// Define the structure for property ownership and rental details
#[contracttype]
#[derive(Clone)]
pub struct Property {
    pub owner: String,
    pub price: u64,
    pub is_rented: bool,
    pub rent_amount: u64,
}

// Create a key for tracking properties by their unique ID
const PROPERTY_KEY: Symbol = symbol_short!("PROP_KEY");

#[contract]
pub struct RealEstateMarketplaceContract;

#[contractimpl]
impl RealEstateMarketplaceContract {

    // Function to list a property for sale
    pub fn list_property(env: Env, owner: String, price: u64) -> u64 {
        let mut property_id: u64 = env.storage().instance().get(&PROPERTY_KEY).unwrap_or(0);
        property_id += 1;
        
        let new_property = Property {
            owner: owner.clone(),
            price,
            is_rented: false,
            rent_amount: 0,
        };

        // Store property data with its unique ID
        env.storage().instance().set(&PROPERTY_KEY, &new_property);
        env.storage().instance().extend_ttl(5000, 5000);

        property_id
    }

    // Function to transfer ownership of a property
    pub fn transfer_ownership(env: Env, property_id: u64, new_owner: String) {
        let mut property: Property = env.storage().instance().get(&PROPERTY_KEY).unwrap();

        // Transfer ownership
        property.owner = new_owner.clone();

        // Update property data with the new owner
        env.storage().instance().set(&PROPERTY_KEY, &property);
        env.storage().instance().extend_ttl(5000, 5000);
    }
}
