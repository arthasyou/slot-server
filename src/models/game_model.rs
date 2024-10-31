use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use slot_algorithm::slots::fruit::FruitBet;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct FruitDrawRequest {
    pub pool_id: u32, // 每分价值
    #[validate(custom(function = "validate_unique_symbols"))]
    #[validate(nested)]
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

#[derive(Debug, Deserialize, Validate)]
pub struct SimpleDrawRequest {
    pub pool_id: u32,
    #[validate(range(min = 1, max = 10000, message = "Amount must be between 1 and 10000"))]
    pub bets: u64,
    #[validate(range(min = 1, max = 1000, message = "Amount must be between 1 and 1000"))]
    pub odds: u64,
}

#[derive(Debug, Serialize)]
pub struct SimpleDrawRespones {
    pub flag: bool,
    pub reward: u64,
}
