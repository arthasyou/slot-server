mod db;
mod models;
mod mw;
mod orm;
mod routes;

// use chrono::{Local, TimeZone, Utc};

use db::connect_db;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_uri = dotenv!("DATABASE_URL");
    let database = connect_db(database_uri).await.unwrap();

    let routes = routes::create_routes(database);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, routes).await.unwrap();
}
