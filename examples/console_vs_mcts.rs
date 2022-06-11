use std::time::Duration;

use connect6::{
    player::{Console, Mcts},
    Builder,
};

const ROUNDS: u64 = 256;
const TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Builder::new()
        .build()
        .start(Mcts::new(ROUNDS, TIMEOUT), Console)
        .await;
}
