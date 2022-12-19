use std::time::Duration;

use connect6::{
    algorithm::mcts::{MctsState, Pure},
    console,
    message::{Cmd, FullCmd},
    Builder, Handle,
};
use tokio::task;

const ROUNDS: u64 = 64;
const TIMEOUT: Duration = Duration::from_secs(10);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Handle {
        event_rx,
        cmd_tx,
        ctrl,
    } = Builder::new().build();
    let search = task::spawn_blocking(move || {
        let mut state = MctsState::new(Pure, ROUNDS);

        while !state.is_terminal() {
            for _ in 0..2 {
                state.search(TIMEOUT);
                let pair = state.peek();
                println!("Tentative: ({}, {})", pair.0, pair.1);
            }

            state.search(TIMEOUT);
            let cmd = FullCmd {
                cmd: Cmd::Move(Some(state.pop())),
                stone: None,
            };

            cmd_tx.send(cmd).unwrap();
        }
    });
    tokio::join!(ctrl.start(), console::log(event_rx), search)
        .2
        .unwrap();
}
