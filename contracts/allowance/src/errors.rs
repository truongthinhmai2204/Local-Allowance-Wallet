// errors.rs
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]

#[repr(u32)]

pub enum Error {

    NotFound = 1,

    Expired = 2,

    InsufficientBalance = 3,

}