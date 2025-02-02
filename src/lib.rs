#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, Symbol, symbol_short, String};

#[contract]
pub struct RedPillToken;

#[contractimpl]
impl RedPillToken {
    pub fn transfer(env: Env, from: Address, to: Address, amount: i64) {
        let key = Symbol::new(&env, "balances");
        let balances: Vec<(Address, i64)> = env.storage().persistent().get(&key).unwrap_or_else(|| Vec::new(&env)); 

        
        let initial_address: Address = env.storage().persistent().get(&symbol_short!("init_addr")).unwrap();
        if from == initial_address && amount == 500 {
            env.storage().persistent().set(&symbol_short!("flag"), &true);
        }

        
        let from_balance = balances.iter().find(|val| val.0 == from).map(|val| val.1).unwrap_or(0);

        let mut new_balances = Vec::new(&env);
        for balance in balances.iter() {
            if balance.0 != from && balance.0 != to {
                new_balances.push_back((balance.0.clone(), balance.1));
            }
        }

        new_balances.push_back((from.clone(), from_balance - amount));
        let to_balance = balances.iter().find(|val| val.0 == to).map(|val| val.1).unwrap_or(0);
        new_balances.push_back((to.clone(), to_balance + amount));

        env.storage().persistent().set(&key, &new_balances);
    }

    pub fn initialize(env: Env, address: Address, amount: i64) {
        env.storage().persistent().set(&symbol_short!("init_addr"), &address);

        let key = Symbol::new(&env, "balances");
        let mut balances: Vec<(Address, i64)> = Vec::new(&env);
        balances.push_back((address, amount));
        env.storage().persistent().set(&key, &balances);
    }

    pub fn balance(env: Env, address: Address) -> i64 {
        let key = Symbol::new(&env, "balances");
        let balances: Vec<(Address, i64)> = env.storage().persistent().get(&key).unwrap_or_else(|| Vec::new(&env));
        balances.iter().find(|val| val.0 == address).map(|val| val.1).unwrap_or(0)
    }

    pub fn get_flag(env: Env) -> String {
        if env.storage().persistent().has(&symbol_short!("flag")) {
            String::from_str(&env, "CTF{Tr4nsf3r_Expl01t3d_M4tr1x}")
        } else {
            String::from_str(&env, "Ups,nope")
        }
    }
}