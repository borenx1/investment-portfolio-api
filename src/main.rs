use investment_portfolio_api::handlers::app;

#[tokio::main]
async fn main() {
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}
