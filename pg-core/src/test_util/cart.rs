#[cfg(test)]
use crate::models::cart::Cart;
use crate::models::product::{Product, ProductCategory};

#[cfg(test)]
pub fn init_cart() -> Cart {
    let mut cart = Cart::new();

    cart.add_product(Product {
        category: ProductCategory::Doughnut,
        price: 160,
    });
    cart.add_product(Product {
        category: ProductCategory::Doughnut,
        price: 240,
    });
    cart.add_product(Product {
        category: ProductCategory::Drink,
        price: 130,
    });
    cart.add_product(Product {
        category: ProductCategory::Drink,
        price: 470,
    });
    // total_proce = 1000;
    cart
}
