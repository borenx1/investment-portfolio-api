use axum::http::{Method, StatusCode};
use investment_portfolio_api::response::{DataResponse, ErrorResponse};
use serde_json::{de, Value};

mod common;

#[tokio::test]
async fn valid_request() {
    let response = common::request_json(
        Method::POST,
        "/reports",
        "
            {
                \"period_start\": 1640995200,
                \"period_end\": 1643673600,
                \"transactions\": []
            }
        ",
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::extract_response_body(response).await;
    let body: DataResponse<Value> = de::from_str(&body).unwrap();
    assert!(body.data.is_object() || body.data.is_array());
    // TODO: validate response data
}

#[tokio::test]
async fn validate_payload_format() {
    let response = common::request_json(Method::POST, "/reports", "{}").await;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

/// Period end must be greater than period start.
#[tokio::test]
async fn validate_period() {
    let period_start = 1660000000;
    for period_end in [1659999999, 1660000000] {
        let body = format!(
            "{{
                \"period_start\": {period_start},
                \"period_end\": {period_end},
                \"transactions\": []
            }}"
        );
        let response = common::request_json(Method::POST, "/reports", &body).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = common::extract_response_body(response).await;
        let body: ErrorResponse<Value> = de::from_str(&body).unwrap();
        let message = body.messages.get(0).unwrap();

        assert_eq!(message, "Period end must be after period start");
    }
}

/// Transactions must be valid (have non-zero amounts).
#[tokio::test]
async fn validate_transactions() {
    let response = common::request_json(
        Method::POST,
        "/reports",
        "
            {
                \"period_start\": 1640995200,
                \"period_end\": 1643673600,
                \"transactions\": [
                    {
                        \"base\": \"BTC\",
                        \"quote\": \"USD\",
                        \"base_amount\": 1,
                        \"quote_amount\": 0,
                        \"timestamp\": 1640995200
                    }
                ]
            }
        ",
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = common::extract_response_body(response).await;
    let body: ErrorResponse<Value> = de::from_str(&body).unwrap();
    let message = body.messages.get(0).unwrap();

    assert_eq!(message, "Transactions must not have 0 as an amount");
    // TODO: Add more transaction validation tests
}
