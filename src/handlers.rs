mod reports;
mod transactions;
use self::reports::{default_report_settings, generate_report};
use self::transactions::add_transaction;
use crate::response::MessageResponse;
use axum::{
    routing::{get, post},
    Router,
};

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/transactions", post(add_transaction))
        .route("/reports/settings", get(default_report_settings))
        .route("/reports", post(generate_report))
}

async fn index() -> MessageResponse {
    MessageResponse::from("Investment Portfolio API")
}
