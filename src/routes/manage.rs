use std::{collections::HashMap, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use sea_orm::{sqlx::database, ActiveModelTrait, DatabaseConnection, Set};
use slot_algorithm::pool::{Pool, RATIO};
use tokio::sync::Mutex;
use validator::Validate;

use crate::{
    models::manage_model::{CreatePoolRequest, CreatePoolResponse},
    orm::pool,
    routes::error::AppError,
};

pub fn routes_manage() -> Router {
    Router::new().route("/create_pool", post(create_pool))
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
                advance: Set(payload.advance as i64 * RATIO as i64),
                boundary: Set(payload.boundary as i64 * RATIO as i64),
                ..Default::default()
            };
            let resutl = new_pool.insert(&database).await.unwrap();
            let pool = Pool::new(
                1,
                1,
                1,
                resutl.brokerage_ratio as u64,
                resutl.boundary as u64,
                resutl.advance as u64,
            );
            pools.lock().await.insert(1, pool);
            // "ok".to_owned();
            (StatusCode::OK, Json(payload)).into_response()
        }
        Err(e) => AppError::new(StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
