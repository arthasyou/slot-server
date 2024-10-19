use std::{collections::HashMap, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use slot_algorithm::pool::Pool;
use tokio::sync::Mutex;
use validator::Validate;

use crate::models::game_model::FruitDrawRequest;

use super::error::AppError;

pub fn routes_game() -> Router {
    Router::new().route("/fruit_draw", post(fruit_draw))
}

async fn fruit_draw(
    Extension(pools): Extension<Arc<Mutex<HashMap<u32, Pool>>>>,
    Json(payload): Json<FruitDrawRequest>,
) -> impl IntoResponse {
    match payload.validate() {
        Ok(_) => {
            match pools.lock().await.get_mut(&1) {
                Some(pool) => {
                    let r = pool.draw(1, 2);
                    println!("{:?}", r);
                    println!("{:?}", pool)
                }
                None => todo!(),
            };
            // pools.lock().await.insert(1, pool);
            // "ok".to_owned();
            (StatusCode::OK, Json(payload)).into_response()
        }
        Err(e) => AppError::new(StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
