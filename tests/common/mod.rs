use axum::{
    body::Body,
    http::{header, Method, Request},
    response::Response,
};
use investment_portfolio_api::handlers::app;
use std::str;
use tower::ServiceExt;

pub async fn request_raw(request: Request<Body>) -> Response {
    let app = app();
    app.oneshot(request).await.unwrap()
}

#[allow(dead_code)]
pub async fn request_no_body(method: Method, path: &str) -> Response {
    let request = Request::builder()
        .method(method)
        .uri(path)
        .body(Body::empty())
        .unwrap();

    request_raw(request).await
}

#[allow(dead_code)]
pub async fn request_json(method: Method, path: &str, body: &str) -> Response {
    let request = Request::builder()
        .method(method)
        .uri(path)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(String::from(body)))
        .unwrap();

    request_raw(request).await
}

/// Returns the body of a response as a string.
/// Panics if the body is not valid UTF-8.
pub async fn extract_response_body(response: Response) -> String {
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = str::from_utf8(&body).unwrap();
    String::from(body_str)
}
