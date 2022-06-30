pub mod cart;
pub mod db;
pub mod product;
pub mod repository;
pub mod trader;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(diesel::Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub struct Postee {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use diesel::PgConnection;
use r2d2_diesel::ConnectionManager;

fn create_db_pool() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}
