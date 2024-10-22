use std::{collections::HashMap, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use slot_algorithm::pool::{Pool, RATIO};
use tokio::sync::Mutex;
use validator::Validate;

use crate::{
    models::manage_model::{CreatePoolRequest, PoolResponse},
    orm::pool,
    routes::error::AppError,
};

pub fn routes_manage() -> Router {
    Router::new()
        .route("/create_pool", post(create_pool))
        .route("/get_pools", post(get_pools))
}

async fn create_pool(
    Extension(database): Extension<DatabaseConnection>,
    Extension(pools): Extension<Arc<Mutex<HashMap<u32, Pool>>>>,
    Json(payload): Json<CreatePoolRequest>,
) -> impl IntoResponse {
    match payload.validate() {
        Ok(_) => {
            let new_pool = pool::ActiveModel {
                owner_id: Set(1),
                bet_unit: Set(payload.bet_unit),
                brokerage_ratio: Set(payload.brokerage_ratio as i64),
                jackpot_ratio: Set(payload.jackpot_ratio as i64),
                pot_ratio: Set(RATIO as i64
                    - payload.jackpot_ratio as i64
                    - payload.brokerage_ratio as i64),
                advance: Set(payload.advance as i64 * RATIO as i64),
                boundary: Set(payload.boundary as i64 * RATIO as i64),
                ..Default::default()
            };
            let result = new_pool.insert(&database).await.unwrap();
            let pool = Pool::new(
                result.id,
                1,
                result.bet_unit as u64,
                result.brokerage_ratio as u64,
                result.jackpot_ratio as u64,
                result.boundary as u64,
                result.advance as u64,
            );
            pools.lock().await.insert(result.id, pool);
            let response = PoolResponse {
                id: result.id,
                boundary: payload.boundary,
                brokerage_ratio: payload.brokerage_ratio,
                jackpot_ratio: payload.jackpot_ratio,
                advance: payload.advance,
                bet_unit: payload.bet_unit,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => AppError::new(StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn get_pools(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
    let pool_models = pool::Entity::find().all(&db).await.unwrap();
    let mut pools = Vec::new();
    for model in pool_models {
        let pool = PoolResponse {
            id: model.id,
            bet_unit: model.bet_unit,
            boundary: model.boundary as u32,
            brokerage_ratio: model.brokerage_ratio as u32,
            jackpot_ratio: model.jackpot_ratio as u32,
            advance: model.advance as u32,
        };
        pools.push(pool);
    }
    (StatusCode::OK, Json(pools)).into_response()
}
