use rate_limiter_macro::rate_limit;
use std::time::Duration;
use tokio::time::sleep;

#[rate_limit(key = "test", capacity = 2, refill_rate = 0.1)]
pub async fn my_api(i: i32) {
    println!("Call {}: Function called successfully!", i);
}

#[tokio::main]
async fn main() {
    println!("Starting per-function rate limiter test...");
    println!("Custom capacity: 2, Refill rate: 0.1 token/sec");
    
    for i in 1..=10 {
        println!("Attempting call {}...", i);
        // We expect this to panic on call 6 because the capacity is 5 and we call it rapidly
        my_api(i).await;
        // Small sleep to make output readable, but not enough to refill significantly
        sleep(Duration::from_millis(100)).await;
    }
}
