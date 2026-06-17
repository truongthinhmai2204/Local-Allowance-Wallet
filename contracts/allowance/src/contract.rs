// contract.rs
use soroban_sdk::{
    contract,
    contractimpl,
    Address,
    Env,
    Symbol,
};

use crate::{
    allowance::Allowance,

    storage::DataKey,

    events,

    errors::Error,
};

#[contract]

pub struct AllowanceContract;


#[contractimpl]

impl AllowanceContract {


    pub fn create_allowance(

        env: Env,

        sponsor: Address,

        recipient: Address,

        category: Symbol,

        amount: i128,

        expires_at: u64,

    ) {

        sponsor.require_auth();


        let allowance = Allowance {

            sponsor,

            recipient: recipient.clone(),

            category,

            amount,

            spent: 0,

            expires_at,

        };


        env.storage()

            .persistent()

            .set(

                &DataKey::Allowance(

                    recipient.clone()

                ),

                &allowance,

            );


        events::allowance_created(

            &env,

            recipient,

            amount,

        );

    }



    pub fn spend(

        env: Env,

        recipient: Address,

        amount: i128,

    ) -> Result<(), Error> {


        recipient.require_auth();


        let key =

            DataKey::Allowance(

                recipient.clone()

            );


        let mut allowance: Allowance =

            env.storage()

                .persistent()

                .get(&key)

                .ok_or(Error::NotFound)?;


        let now =

            env.ledger()

                .timestamp();


        if now > allowance.expires_at {

            return Err(

                Error::Expired

            );

        }


        if allowance.spent + amount

            > allowance.amount

        {

            return Err(

                Error::InsufficientBalance

            );

        }


        allowance.spent += amount;


        env.storage()

            .persistent()

            .set(

                &key,

                &allowance,

            );


        events::allowance_spent(

            &env,

            recipient,

            amount,

        );


        Ok(())

    }




    pub fn get_allowance(

        env: Env,

        recipient: Address,

    ) -> Result<Allowance, Error> {


        env.storage()

            .persistent()

            .get(

                &DataKey::Allowance(

                    recipient

                )

            )

            .ok_or(

                Error::NotFound

            )

    }



    pub fn get_remaining(

        env: Env,

        recipient: Address,

    ) -> Result<i128, Error> {


        let allowance: Allowance =

            env.storage()

                .persistent()

                .get(

                    &DataKey::Allowance(

                        recipient

                    )

                )

                .ok_or(

                    Error::NotFound

                )?;


        Ok(

            allowance.amount

            -

            allowance.spent

        )

    }


}