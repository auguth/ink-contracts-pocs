#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::storage::{
    traits::ManualKey,
    Mapping,
};


#[ink::contract]
mod rewardclimer {

    #[ink(storage)]
    pub struct ClaimContract {
        owner: AccountId,
        last_claim_block: ink::storage::Mapping<AccountId,u32>,
        claim_amount: Balance,
    }

    impl ClaimContract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                last_claim_block: ink::storage::Mapping::new(),
                claim_amount: 10,
            }
        }

        #[ink(message)]
        pub fn claim( &mut self,
            owner: AccountId,
            delegate_to: AccountId,
            delegate_at: BlockNumber,
            reputation: u64,
            recent_blockheight: BlockNumber,
            stake_score: u128) {

            let caller = self.env().caller();
            
            if delegate_to == self.owner{
                let current_block = self.env().block_number();
                let last_claim = self.last_claim_block.get(&caller).unwrap_or(0);
                let range = current_block.saturating_sub(last_claim);
                if last_claim == 0 || range > 10 {
                    self.last_claim_block.insert(&caller, &current_block);
                    let claim_amount_in_smallest_units = self
                        .claim_amount
                        .checked_mul(1_000_000_000_000)
                        .expect("Overflow during multiplication");
                    self.env()
                        .transfer(owner, claim_amount_in_smallest_units)
                        .expect("Transfer failed");
                }

            }
        }

        #[ink(message)]
        pub fn get_last_claim_block(&self, account: AccountId) -> u32 {
            self.last_claim_block.get(&account).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_current_block(&self) -> u32 {
            self.env().block_number()
        }

        #[ink(message)]
        pub fn get_contract_account(&self) -> AccountId {
            self.env().account_id()
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use ink::env::test;
    use ink::env::DefaultEnvironment;
    use crate::rewardclimer::ClaimContract;

    fn advance_blocks(n: u32) {
        for _ in 0..n {
            test::advance_block::<DefaultEnvironment>();
        }
    }

    #[ink::test]
    fn test_first_claim() {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        let contract_owner = accounts.alice;
        let caller = accounts.bob;
        test::set_caller::<DefaultEnvironment>(caller);
        let mut contract = ClaimContract::new(contract_owner);

        let contract_account = contract.get_contract_account();
        test::set_account_balance::<DefaultEnvironment>(
            contract_account,
            1_000_000_000_000_000,
        );

        let current_block = contract.get_current_block();
        contract.claim(contract_owner, contract_owner, 0, 0, 0, 0);
        let last_claim = contract.get_last_claim_block(caller);
        assert_eq!(last_claim, current_block);
    }

    #[ink::test]
    fn test_claim_too_soon() {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        let contract_owner = accounts.alice;
        let caller = accounts.bob;
        test::set_caller::<DefaultEnvironment>(caller);
        let mut contract = ClaimContract::new(contract_owner);
        let contract_account = contract.get_contract_account();
        test::set_account_balance::<DefaultEnvironment>(contract_account, 1_000_000_000_000_000);

        advance_blocks(1);
        contract.claim(contract_owner, contract_owner, 0, 0, 0, 0);
        let initial_claim = contract.get_last_claim_block(caller);

        advance_blocks(5);
        contract.claim(contract_owner, contract_owner, 0, 0, 0, 0);
        let last_claim = contract.get_last_claim_block(caller);
        assert_eq!(last_claim, initial_claim);
    }

    #[ink::test]
    fn test_claim_after_wait() {
        let accounts = test::default_accounts::<DefaultEnvironment>();
        let contract_owner = accounts.alice;
        let caller = accounts.bob;
        test::set_caller::<DefaultEnvironment>(caller);
        let mut contract = ClaimContract::new(contract_owner);
        let contract_account = contract.get_contract_account();
        test::set_account_balance::<DefaultEnvironment>(
            contract_account,
            1_000_000_000_000_000,
        );

        contract.claim(contract_owner, contract_owner, 0, 0, 0, 0);
        let _initial_claim = contract.get_last_claim_block(caller);

        advance_blocks(10);
        contract.claim(contract_owner, contract_owner, 0, 0, 0, 0);
        let current_block = contract.get_current_block();
        let last_claim = contract.get_last_claim_block(caller);
        assert_eq!(last_claim, current_block);
    }
}
