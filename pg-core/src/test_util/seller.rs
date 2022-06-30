#[cfg(test)]
use crate::models::trader::seller::Casher;

#[cfg(test)]
pub fn init_seller(money: u32) -> Casher {
    let seller = Casher { wallet: money };
    seller
}
