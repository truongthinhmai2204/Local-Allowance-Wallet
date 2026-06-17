// events.rs
use soroban_sdk::{
    symbol_short,
    Address,
    Env,
};

pub fn allowance_created(

    env: &Env,

    recipient: Address,

    amount: i128,

) {

    env.events().publish(

        (symbol_short!("created"), recipient),

        amount,

    );

}

pub fn allowance_spent(

    env: &Env,

    recipient: Address,

    amount: i128,

) {

    env.events().publish(

        (symbol_short!("spent"), recipient),

        amount,

    );

}