use std::collections::HashSet;

use serde::Deserialize;
use slot_algorithm::slots::fruit::FruitBet;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct FruitDrawRequest {
    pub pool_id: u32, // 每分价值
    #[validate(custom(function = "validate_unique_symbols"))]
    pub bets: Vec<FruitBet>,
}

fn validate_unique_symbols(bets: &Vec<FruitBet>) -> Result<(), ValidationError> {
    let mut symbols_set = HashSet::new();
    for bet in bets {
        if !symbols_set.insert(&bet.symbol) {
            return Err(ValidationError::new("duplicate_symbol"));
        }
    }
    Ok(())
}
