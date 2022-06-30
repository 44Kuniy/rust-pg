#[cfg(test)]
use crate::models::trader::buyer::Customer;

#[cfg(test)]
pub fn init_buyer(money: u32) -> Customer {
    use crate::test_util::cart::*;

    let cart = init_cart();

    let custoemr = Customer {
        wallet: money,
        cart,
    };
    custoemr
}
