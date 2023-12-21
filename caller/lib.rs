#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[no_main]

// Import necessary components from the ink! library
#[ink::contract]
mod call_solidity {
    use ink::env::{
        call::{build_call, Call, ExecutionInput, Selector},
        debug_println, DefaultEnvironment,
    };

    // Declare the ink! storage struct for the Caller contract
    #[ink(storage)]
    pub struct Caller {
        // State variable indicating the current state of the Caller contract
        state: bool,
    }

    // Implement the ink! contract for calling other contracts at runtime
    impl Caller {
        // Constructor function for initializing the Caller contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                state: true,
            }
        }

        // Message function for delegate call to the flip contract
        #[ink(message)]
        pub fn delegate_call(
            &mut self,
            callee: AccountId,
        ) {
            // Build a delegate call to the specified contract using ink! call APIs
            let my_return_value = build_call::<DefaultEnvironment>()
                .call_type(Call::new(callee))
                .exec_input(
                    // Specify the function selector for the "flip" function in the flip contract
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("flip")))
                )
                .returns::<bool>()
                .invoke();
        }

        // Message function for static call to the flip contract and debug printing the result
        #[ink(message)]
        pub fn static_call(
            &mut self,
            callee: AccountId,
        ) -> bool {
            // Build a static call to the specified contract using ink! call APIs
            let my_return_value = build_call::<DefaultEnvironment>()
                .call_type(Call::new(callee))
                .exec_input(
                    // Specify the function selector for the "get" function in the flip contract
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("get")))
                )
                .returns::<bool>()
                .invoke();

            // Debug print the returned value from the static call
            debug_println!("{:?}", my_return_value);

            // Return the result of the static call
            my_return_value
        }
    }
}
