use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct Asset {
    ticker: String,
    name: String,
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
    pub fn ticker(&self) -> &String {
        &self.ticker
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn precision(&self) -> &u8 {
        &self.precision
    }

    pub fn new(ticker: &str, name: &str, symbol: &str, precision: u8) -> Self {
        Self {
            ticker: String::from(ticker),
            name: String::from(name),
            symbol: String::from(symbol),
            precision,
        }
    }
}

// TODO: Default currencies: ISO currency codes
