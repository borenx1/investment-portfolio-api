use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

pub enum TransactionType {
    Buy,
    Sell,
    Income,
    Expense,
    Invalid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    base: String,
    quote: String,
    base_amount: f64, // TODO: use decimal
    quote_amount: f64,
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>,
}

impl Transaction {
    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn get_transaction_type(&self) -> TransactionType {
        if self.base_amount == 0. || self.quote_amount == 0. {
            return TransactionType::Invalid;
        }
        if self.base_amount > 0. {
            return if self.quote_amount > 0. {
                TransactionType::Income
            } else {
                TransactionType::Buy
            };
        }
        if self.quote_amount > 0. {
            TransactionType::Sell
        } else {
            TransactionType::Income
        }
    }

    pub fn is_buy(&self) -> bool {
        match self.get_transaction_type() {
            TransactionType::Buy => true,
            _ => false,
        }
    }

    pub fn is_sell(&self) -> bool {
        match self.get_transaction_type() {
            TransactionType::Sell => true,
            _ => false,
        }
    }

    pub fn is_income(&self) -> bool {
        match self.get_transaction_type() {
            TransactionType::Income => true,
            _ => false,
        }
    }

    pub fn is_expense(&self) -> bool {
        match self.get_transaction_type() {
            TransactionType::Expense => true,
            _ => false,
        }
    }

    pub fn is_invalid(&self) -> bool {
        match self.get_transaction_type() {
            TransactionType::Invalid => true,
            _ => false,
        }
    }
}
