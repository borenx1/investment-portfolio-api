use std::{collections::HashSet, iter::FromIterator};

use super::asset::Asset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportSettings {
    accounting_currency: Asset,
    assets: Vec<Asset>,
}

impl Default for ReportSettings {
    /// The default Report Settings to use if not specified.
    fn default() -> Self {
        return Self {
            accounting_currency: Asset::new("USD", "United States Dollar", "$", 2),
            assets: vec![Asset::new("BTC", "Bitcoin", "₿", 8)],
        };
    }
}

impl ReportSettings {
    pub fn accounting_currency(&self) -> &Asset {
        &self.accounting_currency
    }

    pub fn assets(&self) -> &[Asset] {
        &self.assets
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
        // The accounting currency cannot also be an asset.
        if self.assets.contains(&self.accounting_currency) {
            return Err((
                String::from("Accounting currency must not also be an asset"),
                self,
            ));
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_report_settings() {
        let report_settings = ReportSettings::default();
        assert_eq!(report_settings.accounting_currency.ticker(), "USD");
        assert_eq!(report_settings.assets.len(), 1);
        assert_eq!(report_settings.assets[0].ticker(), "BTC");
        assert!(report_settings.validate().is_ok());
    }

    #[test]
    fn test_validate_report_settings() {
        // TODO
    }
}
