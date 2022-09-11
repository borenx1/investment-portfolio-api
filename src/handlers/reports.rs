use crate::models::{report::ReportSettings, transaction::Transaction};
use crate::response::{DataResponse, ErrorResponse};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

pub async fn default_report_settings() -> DataResponse<ReportSettings> {
    DataResponse::from(ReportSettings::default())
}

#[derive(Deserialize)]
pub struct GenerateReport {
    settings: Option<ReportSettings>,
    transactions: Vec<Transaction>,
}

pub async fn generate_report(Json(payload): Json<GenerateReport>) -> impl IntoResponse {
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

    // TODO

    let response = if is_default_settings {
        DataResponse::with_message(
            payload.transactions,
            String::from("Default report settings used"),
        )
    } else {
        DataResponse::from(payload.transactions)
    };

    Ok(response)
}
