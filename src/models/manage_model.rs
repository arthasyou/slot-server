use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePoolRequest {
    #[validate(range(
        min = 10000,
        max = 1000000,
        message = "Amount must be between 10000 and 1000000"
    ))]
    pub boundary: u32, // 最大波动
    #[validate(range(min = 1, max = 1000, message = "Amount must be between 1 and 1000"))]
    pub bet_unit: u32, // 每分价值
    #[validate(range(min = 10, max = 1000, message = "Amount must be between 10 and 1000"))]
    pub brokerage_ratio: u32, // 佣金比率(万分比)
    #[validate(range(min = 0, max = 100, message = "Amount must be between 0 and 100"))]
    pub jackpot_ratio: u32, // 彩金比率(万分比)
    #[validate(range(min = 0, max = 500000, message = "Amount must be between 0 and 500000"))]
    pub advance: u32, // 垫分
}

#[derive(Debug, Serialize)]
pub struct PoolResponse {
    pub id: u32,
    pub boundary: u32,
    pub bet_unit: u32,
    pub brokerage_ratio: u32,
    pub jackpot_ratio: u32,
    pub advance: u32,
}
