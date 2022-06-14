use std::{
    io::{self, Write},
    time::Duration,
};

use connect6::{
    algorithm::mcts::Uct,
    board::Stone,
    player::{Console, Mcts},
    Builder,
};
use tokio::task;

const UCT_C: f64 = 0.056;
const ROUNDS: u64 = 32;
const TIMEOUT: Duration = Duration::from_secs(60);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let stone = task::spawn_blocking(|| {
        println!("{:-^30}", " PLAY WITH MCTS ");
        print!("Choose your stone (0 for White, Black otherwise): ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        if buf.trim() == "0" {
            Stone::White
        } else {
            Stone::Black
        }
    })
    .await
    .unwrap();

    if stone == Stone::Black {
        Builder::new()
            .build()
            .start(Console, Mcts::new(Uct(UCT_C), ROUNDS, TIMEOUT))
            .await;
    } else {
        Builder::new()
            .build()
            .start(Mcts::new(Uct(UCT_C), ROUNDS, TIMEOUT), Console)
            .await;
    }
}
