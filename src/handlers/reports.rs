use crate::models::{
    report::{BalanceSheet, ReportSettings},
    transaction::Transaction,
};
use crate::response::{DataResponse, ErrorResponse};
use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::HashMap;

pub async fn default_report_settings() -> DataResponse<ReportSettings> {
    DataResponse::from(ReportSettings::default())
}

#[derive(Deserialize)]
pub struct GenerateReport {
    /// Period start, inclusive.
    period_start: NaiveDateTime,
    /// Period end, exclusive.
    period_end: NaiveDateTime,
    settings: Option<ReportSettings>,
    balance: Option<HashMap<String, f64>>,
    transactions: Vec<Transaction>,
}

pub async fn generate_report(Json(payload): Json<GenerateReport>) -> impl IntoResponse {
    // Validate period.
    if payload.period_start >= payload.period_end {
        return Err((
            StatusCode::BAD_REQUEST,
            ErrorResponse::from("Period end must be after period start"),
        ));
    }

    // Validate report settings if given, else use the default settings.
    let (settings, is_default_settings) = match payload.settings {
        Some(settings) => match settings.validate() {
            Ok(settings) => (settings, false),
            Err(error) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    ErrorResponse::with_data(vec![error.0], error.1),
                ));
            }
        },
        None => (ReportSettings::default(), true),
    };

    // Only process transactions within in the period.
    let transactions: Vec<Transaction> = payload
        .transactions
        .into_iter()
        .filter(|tx| {
            tx.timestamp() >= &payload.period_start && tx.timestamp() < &payload.period_end
        })
        .collect();

    // Calculate transactions.
    for tx in &transactions {
        // Validate transactions.
        if tx.is_invalid() {
            return Err((
                StatusCode::BAD_REQUEST,
                ErrorResponse::from("Transactions must not have 0 as an amount"),
            ));
        }
        if !settings.contains_transaction_asset(tx) {
            return Err((
                StatusCode::BAD_REQUEST,
                ErrorResponse::from(
                    "Transactions base and quote assets must be in the report settings",
                ),
            ));
        }
    }

    let balance_sheet = BalanceSheet::generate(&transactions, payload.balance, &settings);

    let response = if is_default_settings {
        DataResponse::with_message(
            balance_sheet.balance().clone(),
            String::from("Default report settings used"),
        )
    } else {
        DataResponse::from(balance_sheet.balance().clone())
    };

    Ok(response)
}
