use std::time::Duration;

use connect6::{
    algorithm::MctsState,
    console,
    message::{Cmd, FullCmd},
    Builder, Handle,
};
use rand::prelude::*;
use tokio::task;

const ROUNDS: u64 = 256;
const TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Handle {
        event_rx,
        cmd_tx,
        ctrl,
    } = Builder::new().build();
    let search = task::spawn_blocking(move || {
        let mut state = MctsState::new();
        let mut rng = SmallRng::from_entropy();

        while !state.is_terminal() {
            state.search(&mut rng, ROUNDS, TIMEOUT);
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