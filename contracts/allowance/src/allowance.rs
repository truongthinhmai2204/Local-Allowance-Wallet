// allowance.rs
use soroban_sdk::{
    contracttype,
    Address,
    Symbol,
};

#[derive(Clone)]
#[contracttype]
pub struct Allowance {

    pub sponsor: Address,

    pub recipient: Address,

    pub category: Symbol,

    pub amount: i128,

    pub spent: i128,

    pub expires_at: u64,

}