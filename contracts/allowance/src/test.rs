extern crate std;

use soroban_sdk::{
    testutils::Address as _,

    symbol_short,

    Address,

    Env,
};

use crate::AllowanceContract;


#[test]

fn test_create_allowance() {


    let env = Env::default();


    let sponsor =

        Address::generate(

            &env

        );


    let recipient =

        Address::generate(

            &env

        );



    AllowanceContract::

        create_allowance(

            env.clone(),

            sponsor,

            recipient.clone(),

            symbol_short!("FOOD"),

            1000,

            9999999999,

        );



    let allowance =

        AllowanceContract::

            get_allowance(

                env,

                recipient,

            )

            .unwrap();



    assert_eq!(

        allowance.amount,

        1000

    );


    assert_eq!(

        allowance.spent,

        0

    );

}