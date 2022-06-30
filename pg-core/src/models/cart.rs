use crate::models::product::Product;
#[derive(Clone, Debug)]
pub struct Cart {
    id: u32,
    customer_id: u32,
}

#[derive(Clone, Debug, Queryable)]
pub struct CartItem {
    pub id: u32,
    pub cart_id: u32,
    pub product_id: u32,
}
