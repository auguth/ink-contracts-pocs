#![cfg_attr(not(feature = "std"), no_std, no_main)]

// Import necessary components from the ink! library
#[ink::contract]
mod factorial {

    // Declare the ink! storage struct for the Factorial contract
    #[ink(storage)]
    pub struct Factorial {
        // State variable to store the calculated factorial value
        value: i32,
    }

    // Implement the Factorial contract
    impl Factorial {

        // Constructor function for initializing the Factorial contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: 0 }
        }

        // Message function for calculating the factorial of a given number
        #[ink(message)]
        pub fn calculate_factorial(&mut self, number: i32) -> i32 {
            // Ensure that the input is a non-negative integer
            assert!(number >= 0, "Input must be a non-negative integer");

            // Handle base cases: factorial of 0 and 1 is 1
            if number == 0 || number == 1 {
                return 1;
            } else {
                // Initialize the result to 1 and calculate factorial using a loop
                let mut result = 1;
                for i in 2..=number {
                    result *= i;
                }

                // Update the stored value in the contract state
                self.value = result;

                // Return the calculated factorial
                return result;
            }
        }

        // Message function for retrieving the last calculated factorial value
        #[ink(message)]
        pub fn get_value(&self) -> i32 {
            // Return the stored factorial value
            self.value
        }
    }

}
