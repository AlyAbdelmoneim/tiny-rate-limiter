use axum::{routing::get, Router};
use rate_limiter_macro::rate_limit;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/public", get(public_api))
        .route("/limited", get(limited_api))
        .route("/burst", get(burst_api));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Web API Example running on http://{}", addr);
    println!("Endpoints:");
    println!("  - /        : Home");
    println!("  - /public  : No rate limit");
    println!("  - /limited : 5 calls capacity, 1.0 refill/sec");
    println!("  - /burst   : 2 calls capacity, 0.1 refill/sec");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Welcome to the Rate Limiter Web Example!\nTry /public, /limited, or /burst"
}

async fn public_api() -> &'static str {
    "This is a public API. No rate limit applied.\n"
}

#[rate_limit(key = "limited_endpoint", capacity = 5, refill_rate = 1.0)]
async fn limited_api() -> &'static str {
    "Success! You are within the rate limit for /limited.\n"
}

#[rate_limit(key = "burst_endpoint", capacity = 2, refill_rate = 0.1)]
async fn burst_api() -> &'static str {
    "Success! You are within the burst limit for /burst.\n"
}
