#[cfg(test)]
mod integration_tests {
    use super::*;
    use ink_lang as ink;

    #[ink::test]
    fn test_participation() {
        let mut contract = DecentralizedGame::new(AccountId::from([0x01; 32]));

        // Before participation, the total bet is 0
        assert_eq!(contract.total_bet, 0);

        // Participating in the game
        contract.participate();
        assert_eq!(contract.total_bet, self.env().transferred_value());
    }

    #[ink::test]
    fn test_result_distribution() {
        let mut contract = DecentralizedGame::new(AccountId::from([0x02; 32]));

        // Add two participants
        contract.participants.insert(AccountId::from([0x03; 32]), 1000);
        contract.participants.insert(AccountId::from([0x04; 32]), 1500);
        contract.total_bet = 2500;

        // Simulate result distribution
        assert!(contract.generate_result_and_distribute().is_ok());

        // Check if the game state resets after result distribution
        assert_eq!(contract.total_bet, 0);
        assert_eq!(contract.participants.len(), 0);
    }
}
