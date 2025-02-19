#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(unexpected_cfgs)]


/// This ink! smart contract implements a simple validator reward mechanism,
/// designed to integrate with the PoCS (Proof of Contract Stake) system. In PoCS,
/// smart contracts contribute to network security by participating in staking,
/// and this contract provides a framework for claiming rewards.
///
/// The contract stores the last claim block for each account, and only allows a claim
/// if a sufficient number of blocks have passed since the previous claim (a cooldown mechanism).
/// Additional PoCS-related logic, such as incorporating stake scores, reputation,
/// and further validator interactions, can be extended into this contract.

#[ink::contract]
mod rewardclaimer {

    /// The RewardClaimContract structure holds the state of the contract.
    /// 
    /// - `owner`: The account that deployed the contract and is considered the primary administrator.
    /// - `last_claimed_block`: A mapping that records the last block number when an account made a claim.
    /// - `claim_amount`: The fixed amount of tokens (in standard units) awarded per valid claim.
    /// 
    #[ink(storage)]
    pub struct RewardClaimContract {
        owner: AccountId,
        last_claimed_block: ink::storage::Mapping<AccountId, u32>,
        claim_amount: Balance,
    }

    impl RewardClaimContract {

        /// The constructor initializes the contract.
        /// It sets the contract owner, initializes the last claim mapping, and sets a default claim amount.
        /// 
        /// # Parameters
        /// - `owner`: The account ID that will own the contract.
        /// 
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                last_claimed_block: ink::storage::Mapping::new(),
                claim_amount: 10, // This value can be modified or made dynamic for more advanced use cases.
            }
        }

        /// The `claim` function is responsible for processing reward claims.
        ///
        /// This function checks if the `delegated_validator` account matches the contract owner,
        /// verifies that enough blocks have passed since the caller's last claim,
        /// and if so, transfers the reward to the provided `owner` account.
        ///
        /// # Parameters
        /// - `owner`: The account that receives the reward.
        /// - `delegated_validator`: The account that the caller is delegating to. Must match the contract owner.
        /// - `delegation_block`: The block number when delegation occurred (currently unused).
        /// - `reputation`: The reputation value of the caller's contract (currently unused but can be integrated into reward logic).
        /// - `current_block_height`: Recent block number for additional logic (currently unused).
        /// - `staking_score`: The staking score of the caller (currently unused but can be used for weighted rewards).
        ///
        /// # Remarks
        /// This basic implementation demonstrates a reward mechanism that could be expanded to include
        /// more detailed checks using PoCS parameters (e.g., reputation, staking_score) in the future.
        /// 
        #[ink(message)]
        pub fn claim(
            &mut self,
            owner: AccountId,
            delegated_validator: AccountId,
            _delegation_block: BlockNumber,
            _reputation: u64,
            _current_block_height: BlockNumber,
            _staking_score: u128,
        ) {
            // Retrieve the account making the claim.
            let caller = self.env().caller();
            
            // Check if the delegation is correctly set to the contract owner.
            if delegated_validator == self.owner {
                let current_block = self.env().block_number();
                // Retrieve the last claim block for the caller, defaulting to 0 if none exists.
                let last_claim = self.last_claimed_block.get(caller).unwrap_or(0);
                // Calculate the number of blocks that have passed since the last claim.
                let range = current_block.saturating_sub(last_claim);
                // Allow the claim if the caller has never claimed before or if more than 10 blocks have passed.
                if last_claim == 0 || range > 10 {
                    // Update the mapping with the current block number as the last claim.
                    self.last_claimed_block.insert(caller, &current_block);
                    // Convert the claim amount into the smallest unit. This is essential for precise token transfers.
                    let claim_amount_in_smallest_units = self
                        .claim_amount
                        .checked_mul(1_000_000_000_000)
                        .expect("Overflow during multiplication");
                    // Transfer the reward to the specified owner account.
                    self.env()
                        .transfer(owner, claim_amount_in_smallest_units)
                        .expect("Transfer failed");
                }
            }
            // Future enhancements can integrate parameters such as `delegation_block`, `reputation`,
            // `current_block_height`, and `staking_score` to provide more nuanced reward calculations
            // in line with PoCS consensus mechanisms.
        }

        /// Returns the last block number when the specified account made a claim.
        ///
        /// # Parameters
        /// - `account`: The account ID for which to retrieve the last claim block.
        ///
        /// # Returns
        /// The block number of the last claim, or 0 if no claim has been made.
        /// 
        #[ink(message)]
        pub fn get_last_claimed_block(&self, account: AccountId) -> u32 {
            self.last_claimed_block.get(account).unwrap_or(0)
        }

        /// Returns the current block number from the blockchain environment.
        ///
        /// This function is useful for client-side applications to verify timing constraints.
        #[ink(message)]
        pub fn get_current_block(&self) -> u32 {
            self.env().block_number()
        }

        /// Returns the contract's owner account ID.
        ///
        /// This can be used to display contract details or for further administrative functions.
        #[ink(message)]
        pub fn get_contract_account(&self) -> AccountId {
            self.env().account_id()
        }
    }
}

#[cfg(test)]
mod tests {
    use ink::env::test;
    use ink::env::DefaultEnvironment;
    use crate::rewardclaimer::RewardClaimContract;

    /// Helper function to simulate advancing the blockchain by `n` blocks.
    fn advance_blocks(n: u32) {
        for _ in 0..n {
            test::advance_block::<DefaultEnvironment>();
        }
    }

    /// Test case to verify that an account can successfully make a claim
    /// if it has not claimed before.
    #[ink::test]
    fn test_first_claim() {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        let contract_owner = accounts.alice;
        let caller = accounts.bob;
        test::set_caller::<DefaultEnvironment>(caller);
        let mut contract = RewardClaimContract::new(contract_owner);

        let contract_account = contract.get_contract_account();
        test::set_account_balance::<DefaultEnvironment>(
            contract_account,
            1_000_000_000_000_000,
        );

        let current_block = contract.get_current_block();
        contract.claim(contract_owner, contract_owner, 0, 0, 0, 0);
        let last_claim = contract.get_last_claimed_block(caller);
        assert_eq!(last_claim, current_block);
    }

    /// Test case to verify that an account cannot claim again too soon.
    #[ink::test]
    fn test_claim_too_soon() {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        let contract_owner = accounts.alice;
        let caller = accounts.bob;
        test::set_caller::<DefaultEnvironment>(caller);
        let mut contract = RewardClaimContract::new(contract_owner);
        let contract_account = contract.get_contract_account();
        test::set_account_balance::<DefaultEnvironment>(contract_account, 1_000_000_000_000_000);

        advance_blocks(1);
        contract.claim(contract_owner, contract_owner, 0, 0, 0, 0);
        let initial_claim = contract.get_last_claimed_block(caller);

        advance_blocks(5);
        contract.claim(contract_owner, contract_owner, 0, 0, 0, 0);
        let last_claim = contract.get_last_claimed_block(caller);
        // The claim should not be updated if not enough blocks have passed.
        assert_eq!(last_claim, initial_claim);
    }

    /// Test case to verify that an account can claim successfully after waiting the required blocks.
    #[ink::test]
    fn test_claim_after_wait() {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        let contract_owner = accounts.alice;
        let caller = accounts.bob;
        test::set_caller::<DefaultEnvironment>(caller);
        let mut contract = RewardClaimContract::new(contract_owner);
        let contract_account = contract.get_contract_account();
        test::set_account_balance::<DefaultEnvironment>(
            contract_account,
            1_000_000_000_000_000,
        );

        contract.claim(contract_owner, contract_owner, 0, 0, 0, 0);
        let _initial_claim = contract.get_last_claimed_block(caller);

        advance_blocks(10);
        contract.claim(contract_owner, contract_owner, 0, 0, 0, 0);
        let current_block = contract.get_current_block();
        let last_claim = contract.get_last_claimed_block(caller);
        // After waiting, the claim should be updated to the current block.
        assert_eq!(last_claim, current_block);
    }
}
