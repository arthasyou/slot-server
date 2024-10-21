use std::{collections::HashMap, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use slot_algorithm::{pool::Pool, slots::fruit::draw};
use tokio::sync::Mutex;
use validator::Validate;

use crate::{models::game_model::FruitDrawRequest, orm::pool::ActiveModel};

use super::error::AppError;

pub fn routes_game() -> Router {
    Router::new().route("/fruit_draw", post(fruit_draw))
}

async fn fruit_draw(
    Extension(database): Extension<DatabaseConnection>,
    Extension(pools): Extension<Arc<Mutex<HashMap<u32, Pool>>>>,
    Json(payload): Json<FruitDrawRequest>,
) -> impl IntoResponse {
    match payload.validate() {
        Ok(_) => match pools.lock().await.get_mut(&payload.pool_id) {
            Some(pool) => {
                let result = draw(payload.bets, pool);

                // 异步后台任务进行数据库同步，不阻塞当前请求
                let pool_clone = pool.clone();
                let db_clone = database.clone();
                tokio::spawn(async move {
                    let pool_model = pool_to_active_model(&pool_clone);
                    if let Err(e) = pool_model.update(&db_clone).await {
                        eprintln!("Failed to update pool: {}", e); // 错误处理
                    }
                });
                println!(
                    "pot: {}, segment: {:?}  waves: {}",
                    pool.pot,
                    pool.get_segment(),
                    pool.get_waves_len()
                );

                // 返回绘制结果
                (StatusCode::OK, Json(result)).into_response()
            }
            None => AppError::new(StatusCode::BAD_REQUEST, "pool id not existed".to_string())
                .into_response(),
        },
        Err(e) => AppError::new(StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

// 将 Pool 结构体转换为 ActiveModel
fn pool_to_active_model(pool: &Pool) -> ActiveModel {
    ActiveModel {
        id: Set(pool.id),
        owner_id: Set(pool.owner_id),
        bet_unit: Set(pool.bet_unit as u32),
        base_line: Set(pool.base_line as i64),
        boundary: Set(pool.boundary as i64),
        brokerage_ratio: Set(pool.brokerage_ratio as i64),
        jackpot_ratio: Set(pool.jackpot_ratio as i64),
        pot_ratio: Set(pool.pot_ratio as i64),
        pot: Set(pool.pot as i64),
        suction: Set(pool.suction as i64),
        brokerage: Set(pool.brokerage as i64),
        jackpot: Set(pool.jackpot as i64),
        bonus: Set(pool.bonus as i64),
        advance: Set(pool.advance as i64),
        ..Default::default() // 其他字段保持默认
    }
}
