use std::{error, fmt};

pub trait Cashable {
    fn price(self) -> u32;
}

pub trait Trader {
    fn wallet(&mut self) -> u32;
}

pub trait Buyer {
    fn pay(&mut self, money: &u32) -> Result<u32, TradeError>;
    fn total_price(&self) -> u32;
}

pub trait Seller {
    fn receive(&mut self, money: &u32) -> u32;
}
#[derive(Debug, PartialEq)]
pub enum TradeError {
    PaymentError(u32),
    ReceiveError,
}

impl fmt::Display for TradeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            TradeError::PaymentError(_) => "buyer couldn't pay moeny for some reason.",
            TradeError::ReceiveError => "seller couldn't receive moeny for some reason.",
        };
        f.write_str(description)
    }
}

impl error::Error for TradeError {}
