#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, symbol_short,
    Address, String, Vec, Map
};

// Storage Keys
const SUPPLIES: Symbol = symbol_short!("SUPPLY");
const DEMANDS: Symbol = symbol_short!("DEMAND");

#[derive(Clone)]
#[contracttype]
pub struct Supply {
    pub supplier: Address,
    pub item: String,
    pub quantity: u32,
    pub matched: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct Demand {
    pub requester: Address,
    pub item: String,
    pub quantity: u32,
    pub matched: bool,
    pub matched_supplier: Option<Address>,
}

#[contract]
pub struct SupplyMatchContract;

#[contractimpl]
impl SupplyMatchContract {

    // 📌 Add Supply
    pub fn add_supply(env: Env, supplier: Address, item: String, quantity: u32) {
        supplier.require_auth();

        let mut supplies: Vec<Supply> =
            env.storage().instance().get(&SUPPLIES).unwrap_or(Vec::new(&env));

        supplies.push_back(Supply {
            supplier,
            item,
            quantity,
            matched: false,
        });

        env.storage().instance().set(&SUPPLIES, &supplies);
    }

    // 📌 Add Demand
    pub fn add_demand(env: Env, requester: Address, item: String, quantity: u32) {
        requester.require_auth();

        let mut demands: Vec<Demand> =
            env.storage().instance().get(&DEMANDS).unwrap_or(Vec::new(&env));

        demands.push_back(Demand {
            requester,
            item,
            quantity,
            matched: false,
            matched_supplier: None,
        });

        env.storage().instance().set(&DEMANDS, &demands);
    }

    // 🔗 Match Supply with Demand
    pub fn match_supply(env: Env) {
        let mut supplies: Vec<Supply> =
            env.storage().instance().get(&SUPPLIES).unwrap_or(Vec::new(&env));

        let mut demands: Vec<Demand> =
            env.storage().instance().get(&DEMANDS).unwrap_or(Vec::new(&env));

        for i in 0..demands.len() {
            let mut demand = demands.get(i).unwrap();

            if demand.matched {
                continue;
            }

            for j in 0..supplies.len() {
                let mut supply = supplies.get(j).unwrap();

                if supply.matched {
                    continue;
                }

                // Basic Matching Logic: same item & enough quantity
                if supply.item == demand.item && supply.quantity >= demand.quantity {
                    demand.matched = true;
                    demand.matched_supplier = Some(supply.supplier.clone());

                    supply.matched = true;

                    supplies.set(j, supply);
                    demands.set(i, demand.clone());

                    break;
                }
            }
        }

        env.storage().instance().set(&SUPPLIES, &supplies);
        env.storage().instance().set(&DEMANDS, &demands);
    }

    // 📊 Get All Supplies
    pub fn get_supplies(env: Env) -> Vec<Supply> {
        env.storage().instance().get(&SUPPLIES).unwrap_or(Vec::new(&env))
    }

    // 📊 Get All Demands
    pub fn get_demands(env: Env) -> Vec<Demand> {
        env.storage().instance().get(&DEMANDS).unwrap_or(Vec::new(&env))
    }
}