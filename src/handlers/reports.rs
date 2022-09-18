use crate::models::{report::ReportSettings, transaction::Transaction};
use crate::response::{DataResponse, ErrorResponse};
use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;

pub async fn default_report_settings() -> DataResponse<ReportSettings> {
    DataResponse::from(ReportSettings::default())
}

#[derive(Deserialize)]
pub struct GenerateReport {
    /// Period start, inclusive.
    #[serde(with = "ts_seconds")]
    period_start: DateTime<Utc>,
    /// Period end, exclusive.
    #[serde(with = "ts_seconds")]
    period_end: DateTime<Utc>,
    settings: Option<ReportSettings>,
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
            *tx.timestamp() >= payload.period_start && *tx.timestamp() <= payload.period_end
        })
        .collect();

    // Validate transactions.
    if transactions.iter().any(|tx| tx.is_invalid()) {
        return Err((
            StatusCode::BAD_REQUEST,
            ErrorResponse::from("Transactions must not have 0 as an amount"),
        ));
    }

    // TODO

    let response = if is_default_settings {
        DataResponse::with_message(transactions, String::from("Default report settings used"))
    } else {
        DataResponse::from(transactions)
    };

    Ok(response)
}
