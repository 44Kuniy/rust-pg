extern crate rust_pg;

use rust_pg::{models, traits};
use traits::trade::Cashable;
// print string 'test'
fn main() {
    println!("test");
    let product = models::product::Product {
        category: models::product::ProductCategory::Doughnut,
        price: 62,
    };

    println!("{}", product.price);
    println!("{}", product.price());
}
