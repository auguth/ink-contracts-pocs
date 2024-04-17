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
        pub fn claim(    &mut self,
            owner: AccountId,
            delegate_to: AccountId,
            delegate_at: BlockNumber,
            reputation: u64,
            recent_blockheight: BlockNumber,
            stake_score: u128) {

            let caller = self.env().caller();
            
            if delegate_to == self.owner{
                let current_block = self.env().block_number();
                let range = current_block - self.last_claim_block.get(&caller).unwrap_or(0);
                if range < 10 || self.last_claim_block.get(&caller).unwrap_or(0) == 0{
                    self.last_claim_block.insert(caller, &current_block);
                    Self::env().transfer(owner, self.claim_amount);
                }

            }
        }
    }
}

