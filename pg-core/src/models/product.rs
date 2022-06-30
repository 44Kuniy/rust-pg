pub mod doughnut;
pub mod drink;

use diesel::Queryable;
pub use doughnut::*;
pub use drink::*;

use crate::traits::trade::Cashable;

use super::db::Database;

#[derive(Clone, Debug)]
pub enum ProductCategory {
    Doughnut = 1,
    Pie = 2,
    Muffin = 3,
    Drink = 4,
}

#[derive(Clone, Debug, Queryable)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub category: ProductCategory,
    pub price: u32,
}

impl Product {
    pub fn new(id: &str, name: &str, category: ProductCategory, price: u32) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
            category,
            price,
        }
    }
}

impl Cashable for Product {
    fn price(self) -> u32 {
        self.price
    }
}
