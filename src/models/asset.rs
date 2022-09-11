use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct Asset {
    name: String,
    ticker: String,
    symbol: String,
    precision: u8,
}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.ticker == other.ticker
    }
}

impl Hash for Asset {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ticker.hash(state)
    }
}

impl Asset {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn ticker(&self) -> &String {
        &self.ticker
    }

    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn precision(&self) -> &u8 {
        &self.precision
    }

    pub fn new(name: &str, ticker: &str, symbol: &str, precision: u8) -> Self {
        Self {
            name: String::from(name),
            ticker: String::from(ticker),
            symbol: String::from(symbol),
            precision,
        }
    }
}
