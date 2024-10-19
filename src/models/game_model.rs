use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FruitDrawRequest {
    #[validate(range(min = 1, max = 1000, message = "Amount must be between 1 and 1000"))]
    pub bet_unit: u32, // 每分价值
    #[validate(length(max = 8, message = "Bets cannot exceed 8 items"))]
    #[validate(nested)]
    pub bets: Vec<FruitBet>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FruitBet {
    #[validate(range(min = 1, max = 8, message = "Amount must be between 1 and 8"))]
    pub symbol: u32,
    #[validate(range(min = 1, max = 1000, message = "Amount must be between 1 and 1000"))]
    pub bet: u32,
}

#[derive(Debug, Serialize)]
pub struct FruitDrawResponse {
    pub id: u32,
    pub boundary: u32,        // 最大波动
    pub brokerage_ratio: u32, // 佣金比率(万分比)
    pub advance: u32,         // 垫分
    pub bet_unit: u32,
}
