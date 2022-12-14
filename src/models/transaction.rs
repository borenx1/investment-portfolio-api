use chrono::NaiveDateTime;
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
    timestamp: NaiveDateTime,
}

impl Transaction {
    pub fn base(&self) -> &String {
        &self.base
    }

    pub fn quote(&self) -> &String {
        &self.quote
    }

    pub fn base_amount(&self) -> &f64 {
        &self.base_amount
    }

    pub fn quote_amount(&self) -> &f64 {
        &self.quote_amount
    }

    pub fn timestamp(&self) -> &NaiveDateTime {
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
