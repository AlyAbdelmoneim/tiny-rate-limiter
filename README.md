# Async Rate Limiter for Rust

A simple, thread-safe, in-memory rate limiter for Rust applications using attribute macros.

## Features

- **Attribute Macro**: Apply custom rate limits to any async function using `#[rate_limit]`.
- **Per-Function Configuration**: Each function can have its own `capacity` and `refill_rate`.
- **Token Bucket Algorithm**: Simple and effective rate limiting logic.
- **Thread-safe**: Uses a global thread-safe limiter.
- **Modern Rust**: Built with Rust 2024 edition and Tokio.

## Project Structure

- `rate-limiter-core`: Core logic and `RateLimiter` implementation.
- `rate-limiter-macro`: Attribute macro implementation.
- `rate-limiter-examples`: Example usage demonstrating the rate limit.

## Getting Started

### Installation

Add the dependencies to your `Cargo.toml`:

```toml
[dependencies]
rate_limiter_core = { path = "rate-limiter-core" }
rate_limiter_macro = { path = "rate-limiter-macro" }
```

### Usage

Annotate your async functions with the `#[rate_limit]` macro:

```rust
use rate_limiter_macro::rate_limit;

#[rate_limit(key = "my_api", capacity = 5, refill_rate = 1.0)]
pub async fn my_api_function() {
    println!("Called successfully!");
}
```

### Running the Web API Example

There is also a real-world web API example using `axum`. To run it:

```bash
cargo run --package axum-example
```

Once running, you can test the endpoints:
- `http://127.0.0.1:3000/limited` (Cap: 5, Refill: 1.0/s)
- `http://127.0.0.1:3000/burst` (Cap: 2, Refill: 0.1/s)

Example using `curl`:
```bash
# Succeeds for first 5 calls, then fails
for i in {1..6}; do curl -i http://127.0.0.1:3000/limited; done
```

## How it Works

The `#[rate_limit]` macro expands your function to check a global `RateLimiter` before execution. If the limit is exceeded, it currently triggers a panic (ideal for demonstration or fail-fast scenarios).

```rust
// Macro expansion simplified:
pub async fn my_api_function() {
    let key = "my_api";
    if let Err(_) = RATE_LIMITER.lock().unwrap().try_consume(key) {
        panic!("Rate Limit Exceeded");
    }
    // original function body
}
```

## License

MIT
