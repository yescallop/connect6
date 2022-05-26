use std::time::Instant;

use connect6::{player::Void, Builder};

const TOTAL: u32 = 100000;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    for _ in 0..TOTAL {
        Builder::new().build().start_silent(Void, Void).await;
    }
    println!("{:?}", start.elapsed() / TOTAL);
}
