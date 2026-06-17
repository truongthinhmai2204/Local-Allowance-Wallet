#![no_std]

mod allowance;

mod storage;

mod events;

mod errors;

mod contract;


pub use crate::contract::AllowanceContract;


#[cfg(test)]

mod test;