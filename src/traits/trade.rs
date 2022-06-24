use std::{error, fmt};

pub trait Cashable {
    fn price(self) -> u32;
}

pub trait Trader {
    fn wallet(&mut self) -> u32;
}

pub trait Buyer {
    fn pay(&mut self, money: &u32) -> Result<u32, Box<TradeError>>;
}

pub trait Seller {
    fn receive(&mut self, money: &u32) -> u32;
}
#[derive(Debug, PartialEq)]
pub enum TradeError<'a> {
    PaymentError(&'a u32),
    ReceiveError,
}

impl fmt::Display for TradeError<'static> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description: &str = match *self {
            TradeError::PaymentError(&money) => {
                format!("buyer runs short of moeney: {}", money).as_str()
            }
            TradeError::ReceiveError => "seller couldn't receive moeny for some reason.",
        };
        f.write_str(description)
    }
}

impl error::Error for TradeError<'static> {}
