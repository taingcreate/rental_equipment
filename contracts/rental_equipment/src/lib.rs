#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map};

#[contract]
pub struct RentalEquipment;

#[contractimpl]
impl RentalEquipment {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// Register equipment with an owner and daily rate.
    pub fn register_equipment(env: Env, equipment_id: u64, owner: Address, daily_rate: u64) {
        owner.require_auth();

        let mut equipment: Map<u64, EquipmentData> = env
            .storage()
            .instance()
            .get(&"equipment")
            .unwrap_or(Map::new(&env));

        if equipment.contains_key(equipment_id) {
            panic!("Equipment already registered");
        }

        let data = EquipmentData {
            owner,
            daily_rate,
            rented: false,
            renter: None,
            return_block: 0,
        };

        equipment.set(equipment_id, data);
        env.storage().instance().set(&"equipment", &equipment);
    }

    /// Rent equipment for a specified number of days.
    /// Calculates total cost as daily_rate * days.
    pub fn rent_equipment(env: Env, equipment_id: u64, renter: Address, days: u64) -> u64 {
        renter.require_auth();

        let mut equipment: Map<u64, EquipmentData> = env
            .storage()
            .instance()
            .get(&"equipment")
            .unwrap_or(Map::new(&env));

        let data = equipment
            .get(equipment_id)
            .ok_or("Equipment not found")
            .unwrap();

        if data.rented {
            panic!("Equipment already rented");
        }

        let total_cost = data.daily_rate * days;
        let return_block = env.ledger().sequence() as u64 + days;

        let new_data = EquipmentData {
            owner: data.owner,
            daily_rate: data.daily_rate,
            rented: true,
            renter: Some(renter),
            return_block,
        };

        equipment.set(equipment_id, new_data);
        env.storage().instance().set(&"equipment", &equipment);

        total_cost
    }

    /// Owner claims payment for rented equipment.
    pub fn claim_payment(env: Env, equipment_id: u64) {
        let mut equipment: Map<u64, EquipmentData> = env
            .storage()
            .instance()
            .get(&"equipment")
            .unwrap_or(Map::new(&env));

        let data = equipment
            .get(equipment_id)
            .ok_or("Equipment not found")
            .unwrap();

        if !data.rented {
            panic!("Equipment not rented");
        }

        if (env.ledger().sequence() as u64) < data.return_block {
            panic!("Rental period not yet ended");
        }

        let mut new_data = EquipmentData {
            owner: data.owner,
            daily_rate: data.daily_rate,
            rented: false,
            renter: None,
            return_block: 0,
        };

        equipment.set(equipment_id, new_data);
        env.storage().instance().set(&"equipment", &equipment);
    }

    /// Get equipment details.
    pub fn get_equipment(env: Env, equipment_id: u64) -> EquipmentData {
        let equipment: Map<u64, EquipmentData> = env
            .storage()
            .instance()
            .get(&"equipment")
            .unwrap_or(Map::new(&env));

        equipment
            .get(equipment_id)
            .ok_or("Equipment not found")
            .unwrap()
    }
}

#[derive(Clone)]
#[contracttype]
pub struct EquipmentData {
    pub owner: Address,
    pub daily_rate: u64,
    pub rented: bool,
    pub renter: Option<Address>,
    pub return_block: u64,
}
