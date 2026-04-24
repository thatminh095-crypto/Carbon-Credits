#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

#[contract]
pub struct CarbonCredits;

#[contractimpl]
impl CarbonCredits {
    /// Initialize the contract with admin address
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
        env.storage().instance().set(&"admin", &admin);
        env.storage().instance().set(&"total_supply", &0u64);
        env.storage().instance().set::<_, Map<Address, u64>>(&"balances", &Map::new(&env));
    }

    /// Admin mints carbon credits to an address
    pub fn mint_credits(env: Env, admin: Address, owner: Address, amount: u64) {
        admin.require_auth();

        let mut balances: Map<Address, u64> = env
            .storage()
            .instance()
            .get(&"balances")
            .unwrap_or(Map::new(&env));

        let mut total_supply: u64 = env
            .storage()
            .instance()
            .get(&"total_supply")
            .unwrap_or(0u64);

        let current_balance = balances.get(owner.clone()).unwrap_or(0);
        balances.set(owner, current_balance + amount);
        total_supply += amount;

        env.storage().instance().set(&"balances", &balances);
        env.storage().instance().set(&"total_supply", &total_supply);
    }

    /// Transfer credits from one address to another
    pub fn transfer_credits(env: Env, from: Address, to: Address, amount: u64) {
        from.require_auth();

        let mut balances: Map<Address, u64> = env
            .storage()
            .instance()
            .get(&"balances")
            .unwrap_or(Map::new(&env));

        let from_balance = balances.get(from.clone()).unwrap_or(0);
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        balances.set(from.clone(), from_balance - amount);
        let to_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, to_balance + amount);

        env.storage().instance().set(&"balances", &balances);
    }

    /// Retire (burn) credits - removes them from circulation
    pub fn retire_credits(env: Env, user: Address, amount: u64) {
        user.require_auth();

        let mut balances: Map<Address, u64> = env
            .storage()
            .instance()
            .get(&"balances")
            .unwrap_or(Map::new(&env));

        let mut total_supply: u64 = env
            .storage()
            .instance()
            .get(&"total_supply")
            .unwrap_or(0u64);

        let user_balance = balances.get(user.clone()).unwrap_or(0);
        if user_balance < amount {
            panic!("Insufficient balance");
        }

        balances.set(user.clone(), user_balance - amount);
        total_supply -= amount;

        env.storage().instance().set(&"balances", &balances);
        env.storage().instance().set(&"total_supply", &total_supply);
    }

    /// Get the credit balance of an address
    pub fn get_balance(env: Env, user: Address) -> u64 {
        let balances: Map<Address, u64> = env
            .storage()
            .instance()
            .get(&"balances")
            .unwrap_or(Map::new(&env));
        balances.get(user).unwrap_or(0)
    }

    /// Get the total supply of credits in circulation
    pub fn get_total_supply(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&"total_supply")
            .unwrap_or(0u64)
    }
}
