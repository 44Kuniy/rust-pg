use pg_core;

use pg_core::{
    models::{self, db::Database},
    traits,
};
use traits::trade::Cashable;

use models::product::{Product, ProductCategory};

fn main() {
    let db = Database::new();

    println!("db: {}", db);
}
