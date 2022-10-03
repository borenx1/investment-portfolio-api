use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use super::asset::Asset;
use super::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportSettings {
    accounting_currency: String,
    assets: Vec<Asset>,
}

impl Default for ReportSettings {
    /// The default Report Settings to use if not specified.
    fn default() -> Self {
        return Self {
            accounting_currency: String::from("USD"),
            assets: vec![
                Asset::new("USD", "United States Dollar", "$", 2),
                Asset::new("BTC", "Bitcoin", "₿", 8),
            ],
        };
    }
}

impl ReportSettings {
    pub fn accounting_currency(&self) -> &String {
        &self.accounting_currency
    }

    pub fn assets(&self) -> &[Asset] {
        &self.assets
    }

    /// Check if a Transaction's assets are listed in the asset settings, i.e.
    /// the transaction's base and quote asset are listed in `self.assets`.
    pub fn contains_transaction_asset(&self, tx: &Transaction) -> bool {
        return self.assets.iter().any(|a| a.ticker() == tx.base())
            && self.assets.iter().any(|a| a.ticker() == tx.quote());
    }

    pub fn validate(self) -> Result<Self, (String, Self)> {
        // TODO: Return all errors?
        // Assets cannot have duplicate tickers.
        let unique_assets: HashSet<&Asset> = HashSet::from_iter(self.assets.iter());
        if self.assets.len() != unique_assets.len() {
            return Err((
                String::from("Assets must not contain duplicate tickers"),
                self,
            ));
        }
        // The accounting currency must be an asset.
        if !self
            .assets
            .iter()
            .any(|a| a.ticker() == &self.accounting_currency)
        {
            return Err((
                String::from("The accounting currency must be an asset"),
                self,
            ));
        }

        Ok(self)
    }
}

#[derive(Debug)]
pub struct BalanceSheet<'a> {
    settings: &'a ReportSettings,
    balance: HashMap<String, f64>,
}

impl<'a> BalanceSheet<'a> {
    pub fn balance(&self) -> &HashMap<String, f64> {
        &self.balance
    }

    pub fn generate(
        transactions: &[Transaction],
        initial_balance: Option<HashMap<String, f64>>,
        settings: &'a ReportSettings,
    ) -> Self {
        // Initialize balance.
        let mut balance: HashMap<String, f64> = HashMap::new();
        match initial_balance {
            Some(init) => {
                for asset in settings.assets() {
                    balance.insert(
                        asset.ticker().clone(),
                        *init.get(asset.ticker()).unwrap_or(&0.0),
                    );
                }
            }
            None => {
                for asset in settings.assets() {
                    balance.insert(asset.ticker().clone(), 0.0);
                }
            }
        }
        // Add transactions (the balance must contain the base and quote assets).
        for tx in transactions {
            let base_balance = balance.entry(tx.base().clone()).or_insert(0.0);
            *base_balance += tx.base_amount();
            let quote_balance = balance.entry(tx.quote().clone()).or_insert(0.0);
            *quote_balance += tx.quote_amount();
        }

        return Self { settings, balance };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_report_settings() {
        let report_settings = ReportSettings::default();
        assert_eq!(report_settings.accounting_currency, "USD");
        assert_eq!(report_settings.assets.len(), 2);
        assert_eq!(report_settings.assets[0].ticker(), "USD");
        assert_eq!(report_settings.assets[1].ticker(), "BTC");
        assert!(report_settings.validate().is_ok());
    }

    #[test]
    fn test_validate_report_settings() {
        // TODO
    }
}
