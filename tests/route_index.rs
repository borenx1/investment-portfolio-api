use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use investment_portfolio_api::{handlers::app, response::MessageResponse};
use serde_json::de;
use tower::{Service, ServiceExt};

mod common;

#[tokio::test]
async fn index_route() {
    let response = common::request_no_body(Method::GET, "/").await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = common::extract_response_body(response).await;
    let body: MessageResponse = de::from_str(&body).unwrap();
    let message = body.messages.get(0).unwrap();

    assert!(
        message.contains("Investment Portfolio"),
        "Response body is wrong"
    );
}

/// Response has 405 status code on invalid method.
#[tokio::test]
async fn index_route_method() {
    let mut app = app();

    let request = Request::builder()
        .method(Method::GET)
        .uri("/")
        .body(Body::empty())
        .unwrap();
    let response = app.ready().await.unwrap().call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let invalid_methods = vec![Method::POST, Method::PATCH, Method::PUT, Method::DELETE];
    for method in invalid_methods {
        let request = Request::builder()
            .method(&method)
            .uri("/")
            .body(Body::empty())
            .unwrap();
        let response = app.ready().await.unwrap().call(request).await.unwrap();
        assert_eq!(
            response.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "Unexpected status code: {}",
            &method
        );
    }
}
