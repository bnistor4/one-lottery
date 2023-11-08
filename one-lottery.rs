// Import the required dependencies
use std::collections::BTreeSet;
use multiversx_blockchain::prelude::*;

// Define the lottery smart contract structure
pub struct LotterySmartContract {
    token: String,
}

impl Default for LotterySmartContract {
    fn default() -> Self {
        Self {
            token: "MOOVE-875539".to_string(),
        }
    }
}

// Implement the smart contract methods
impl SmartContract for LotterySmartContract {
    fn init(&self, _params: Params) -> ContractResult {
        ContractResult::Ok(None)
    }

    fn on_token_received(&self, params: Params) -> ContractResult {
        let token_address = params.get_address("token_address")?;
        let token_id = params.get_string("token_id")?;

        // Check if the received token matches the configured token
        if token_address == Address::this() && token_id == self.token {
            let bet_amount = Balance::from(1);  // Bet amount is 1 token

            // Transfer 10% of the bet amount as fee to the contract address
            let fee_amount = bet_amount * 10 / 100;
            let _ = Address::this().transfer(fee_amount)?;

            let winner_amount = bet_amount - fee_amount;

            // Generate a random number between 0 to 1
            let random_number = Blockchain::random(0, 1)?;

            // Check if the participant won the lottery
            if random_number == 1 {
                // Transfer the winner amount to the participant
                let _ = Address::this().transfer(winner_amount)?;

                // Return the result as a response
                ContractResult::Ok(Some(Response::new()
                    .add_value("result", "win")
                    .add_value("amount", winner_amount.to_string())))
            } else {
                ContractResult::Ok(Some(Response::new().add_value("result", "lose")))
            }
        } else {
            ContractResult::Err("Invalid token received.")
        }
    }
}