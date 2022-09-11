use crate::models::transaction::Transaction;
use crate::response::DataResponse;
use axum::Json;

pub async fn add_transaction(Json(payload): Json<Transaction>) -> DataResponse<Transaction> {
    DataResponse::with_message(payload, String::from("TODO"))
}
