use crate::models::cart::Cart;
use crate::traits::TradeError;
use crate::traits::{Buyer, Trader};

pub struct Customer {
    pub wallet: u32,
    cart: Cart,
}

impl Trader for Customer {
    fn wallet(&mut self) -> u32 {
        self.wallet
    }
}

impl Buyer for Customer {
    fn pay(&mut self, money: &u32) -> Result<u32, Box<TradeError>> {
        if &self.wallet < money {
            return Err(Box::new(TradeError::PaymentError(money)));
        }
        self.wallet -= money;
        Ok(self.wallet)
    }
}

#[cfg(test)]
mod test {
    use crate::models::product::{Product, ProductCategory};

    use super::*;

    #[test]
    fn test_pay() {
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
            price: 180,
        });
        cart.add_product(Product {
            category: ProductCategory::Drink,
            price: 470,
        });

        let total_cost = cart.clone().total_cost();
        assert_eq!(total_cost, 1050);

        let mut buyer1 = Customer {
            wallet: 302_050,
            cart: cart.clone(),
        };
        let mut buyer2 = Customer {
            wallet: 1000,
            cart: cart.clone(),
        };

        let result1 = buyer1.pay(&1050);
        let result2 = buyer2.pay(&1050);
        assert_eq!(result1, Ok(301000));
        assert_eq!(result2, Err(TradeError::PaymentError(&total_cost)));

        // assert_eq!(result2, Err(TradeError::PaymentError(total_cost)));
    }
}
