#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod decentralized_game {
    use ink_storage::collections::HashMap as StorageHashMap;
    use ink_env::hash::{Blake2x256, CryptoHash, HashOutput};

    #[ink(storage)]
    pub struct DecentralizedGame {
        participants: StorageHashMap<AccountId, Balance>,  // To store participants and their bet amount
        total_bet: Balance,                                // Total bet amount collected
        admin: AccountId,                                  // Admin address (burn wallet)
    }

    impl DecentralizedGame {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            Self {
                participants: StorageHashMap::new(),
                total_bet: 0,
                admin,
            }
        }

        #[ink(message)]
        #[payable]
        pub fn participate(&mut self) {
            let caller = self.env().caller();
            let amount = self.env().transferred_value();
            self.participants.insert(caller, amount);
            self.total_bet += amount;
        }

        #[ink(message)]
        pub fn generate_result_and_distribute(&mut self) -> Result<(), String> {
            let participants_count = self.participants.len();
            if participants_count == 0 {
                return Err(String::from("No participants"));
            }
            let winner = self.random_participant()?;
            let total_prize = self.total_bet;
            let winner_share = total_prize * 80 / 100;
            let admin_share = total_prize * 20 / 100;
            self.env().transfer(winner, winner_share)
                .map_err(|_| String::from("Failed to transfer to winner"))?;
            self.env().transfer(self.admin, admin_share)
                .map_err(|_| String::from("Failed to transfer to admin"))?;
            self.participants.clear();
            self.total_bet = 0;
            Ok(())
        }

        fn random_participant(&self) -> Result<AccountId, String> {
            let mut hasher = Blake2x256::default();
            let seed = self.env().block_number();
            hasher.hash(&seed.to_le_bytes());
            let mut output = <Blake2x256 as HashOutput>::Type::default();
            hasher.write(&output);
            let random_index = (output[0] as usize) % self.participants.len();
            let mut iter = self.participants.keys();
            let winner = iter.nth(random_index).ok_or("No participants found")?;
            Ok(*winner)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn participate_works() {
            let mut contract = DecentralizedGame::new(AccountId::from([0x01; 32]));
            assert_eq!(contract.total_bet, 0);
            contract.participate();
            assert!(contract.participants.contains_key(&AccountId::from([0x01; 32])));
        }

        #[ink::test]
        fn distribute_works() {
            let mut contract = DecentralizedGame::new(AccountId::from([0x02; 32]));
            contract.participants.insert(AccountId::from([0x03; 32]), 1000);
            contract.total_bet = 1000;
            let result = contract.generate_result_and_distribute();
            assert!(result.is_ok());
        }
    }
}
