use crate::traits::{Seller, Trader};

#[derive(Debug, Clone)]
pub struct Casher {
    pub id: String,
    pub wallet: u32,
}
impl Casher {
    pub fn new(id: &str, wallet: u32) -> Self {
        Casher {
            id: id.to_owned(),
            wallet,
        }
    }
}

impl Trader for Casher {
    fn wallet(&mut self) -> u32 {
        self.wallet
    }
}

impl Seller for Casher {
    fn receive(&mut self, money: &u32) -> u32 {
        self.wallet += money;
        self.wallet
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wallet() {
        let seller = Casher { wallet: 300_000 };

        assert_eq!(seller.wallet, 300_000);
    }

    #[test]
    fn test_receive() {
        let mut seller = Casher { wallet: 300_000 };
        let total = seller.receive(&20_000);

        assert_eq!(total, 320_000);
        assert_eq!(seller.wallet, 320_000);
    }
}
