pub mod error;

pub mod game;
pub mod manage;

use std::{collections::HashMap, sync::Arc};

use crate::mw::cors::create_cors;

use axum::{Extension, Router};
use game::routes_game;
use manage::routes_manage;
use sea_orm::DatabaseConnection;
use slot_algorithm::pool::Pool;
use tokio::sync::Mutex;

pub fn create_routes(
    database: DatabaseConnection,
    pools: Arc<Mutex<HashMap<u32, Pool>>>,
) -> Router {
    let cors = create_cors();

    Router::new()
        // .merge(routes_manage())
        .nest("/manage", routes_manage())
        .nest("/game", routes_game())
        .layer(Extension(database))
        .layer(Extension(pools))
        .layer(cors)
}
