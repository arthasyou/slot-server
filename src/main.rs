mod db;
mod models;
mod mw;
mod orm;
mod routes;

// use chrono::{Local, TimeZone, Utc};

use std::{collections::HashMap, env, sync::Arc};

use db::connect_db;
use dotenvy::dotenv;
// use dotenvy_macro::dotenv;
use orm::pool;
use sea_orm::{DatabaseConnection, EntityTrait};
use slot_algorithm::pool::Pool;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("0.0.0.0:{}", port);
    println!("addr: {:?}", addr);
    let database = connect_db(&database_url).await.unwrap();

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
