pub mod doughnut;
pub mod drink;

pub use doughnut::*;
pub use drink::*;

use crate::traits::trade::Cashable;

#[derive(Clone)]
pub enum ProductCategory {
    Drink,
    Doughnut,
}
#[derive(Clone)]
pub struct Product {
    pub category: ProductCategory,
    pub price: u32,
}

impl Cashable for Product {
    fn price(self) -> u32 {
        self.price
    }
}
