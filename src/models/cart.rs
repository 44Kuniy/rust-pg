use crate::models::product::Product;
#[derive(Clone)]
pub struct Cart {
    products: Vec<Product>,
}

impl Cart {
    pub fn new() -> Self {
        let products: Vec<Product> = Vec::new();
        Cart { products }
    }
    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }
    pub fn total_cost(self) -> u32 {
        self.products
            .iter()
            .fold(0, |accum, product| accum + product.price)
    }
    pub fn count(self) -> u32 {
        self.products.len() as u32
    }
}
