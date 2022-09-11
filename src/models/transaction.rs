use super::asset::Asset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    currency: Asset,
    amount: f64, // TODO: use decimal
}
