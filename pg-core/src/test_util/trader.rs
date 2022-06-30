#[cfg(test)]
use crate::models::trader::{buyer::Customer, seller::Casher};
use crate::test_util::{buyer::*, seller::*};

#[cfg(test)]
pub fn init_traders(seller_money: u32, buyer_money: u32) -> (Casher, Customer) {
    let seller = init_seller(seller_money);
    let buyer = init_buyer(buyer_money);
    (seller, buyer)
}
