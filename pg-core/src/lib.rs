use std::env;
pub mod models;
pub mod schema;
pub mod traits;

#[macro_use]
extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
