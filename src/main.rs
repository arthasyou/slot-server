mod db;
mod models;
mod mw;
mod orm;
mod routes;

// use chrono::{Local, TimeZone, Utc};

use std::{collections::HashMap, sync::Arc};

use db::connect_db;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use orm::pool;
use sea_orm::{DatabaseConnection, EntityTrait};
use slot_algorithm::pool::Pool;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_uri = dotenv!("DATABASE_URL");
    let port = dotenv!("PORT");
    let addr = format!("0.0.0.0:{}", port);
    let database = connect_db(database_uri).await.unwrap();

    let mut pools: HashMap<u32, Pool> = HashMap::new();
    load_pools_from_db(&database, &mut pools).await;
    let pools = Arc::new(Mutex::new(pools));

    let routes = routes::create_routes(database, pools);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, routes).await.unwrap();
}

async fn load_pools_from_db(db: &DatabaseConnection, pools: &mut HashMap<u32, Pool>) {
    let pool_models = pool::Entity::find().all(db).await.unwrap();
    for model in pool_models {
        let pool = Pool::load_pool(
            model.id,
            model.owner_id,
            model.bet_unit as u64,
            model.base_line as u64,
            model.boundary as u64,
            model.brokerage_ratio as u64,
            model.jackpot_ratio as u64,
            model.pot as u64,
            model.jackpot as u64,
            model.suction as u64,
            model.brokerage as u64,
            model.advance as u64,
        );
        pools.insert(model.id, pool);
    }
}
