use std::time::Duration;

use connect6::{player::Mcts, Builder};

const ROUNDS_BLACK: u64 = 256;
const ROUNDS_WHITE: u64 = 256;
const TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Builder::new()
        .build()
        .start(
            Mcts::new(ROUNDS_BLACK, TIMEOUT),
            Mcts::new(ROUNDS_WHITE, TIMEOUT),
        )
        .await;
}
