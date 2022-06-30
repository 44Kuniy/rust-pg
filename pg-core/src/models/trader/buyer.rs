use crate::models::cart::Cart;
use crate::traits::TradeError;
use crate::traits::{Buyer, Trader};

#[derive(Clone, Debug, Queryable)]
pub struct Customer {
    pub id: String,
    pub wallet: u32,
}

impl Customer {
    pub fn new(id: &str, wallet: u32, cart: Cart) -> Self {
        Customer {
            id: id.to_owned(),
            wallet,
        }
    }
}

impl Trader for Customer {
    fn wallet(&mut self) -> u32 {
        self.wallet
    }
}

impl Buyer for Customer {
    fn pay(&mut self, money: &u32) -> Result<u32, TradeError> {
        if &self.wallet < money {
            return Err(TradeError::PaymentError(*money));
        }
        self.wallet -= money;
        Ok(money.to_owned())
    }
    fn total_price(&self) -> u32 {
        self.cart.total_price()
    }
}

#[cfg(test)]
mod test {
    use crate::models::{
        db::Database,
        product::{Product, ProductCategory},
    };

    use super::*;

    #[test]
    fn test_pay() {
        let Database = Database::new();
        let customer1 = Database.customers.get("1002").unwrap();
        let customer2 = Database.customers.get("1004").unwrap();

        let result1 = customer1.pay(&1500);
        let result2 = customer2.pay(&1050);
        assert_eq!(result1, Ok(500));
        assert_eq!(result2, Err(TradeError::PaymentError(total_cost)));

        // assert_eq!(result2, Err(TradeError::PaymentError(total_cost)));
    }
}
