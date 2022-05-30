use std::time::Instant;

use connect6::{player::Void, Builder};

const RUNS: u32 = 100000;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    for _ in 0..RUNS {
        Builder::new().build().start_silent(Void, Void).await;
    }
    println!("{:?}", start.elapsed() / RUNS);
}
