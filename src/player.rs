use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    algorithm::mcts::{MctsState, Policy},
    board::Point,
    channel::{CmdSender, Receiver},
    console,
    message::Event,
};
use async_trait::async_trait;
use tokio::task;

/// A trait for Connect6 players.
#[async_trait]
pub trait Player {
    /// Attaches the player to the game.
    async fn attach(self, event_rx: Receiver<Event>, cmd_tx: CmdSender);
}

/// A player that inputs moves from the console.
pub struct Console;

#[async_trait]
impl Player for Console {
    async fn attach(self, mut event_rx: Receiver<Event>, cmd_tx: CmdSender) {
        let stone = cmd_tx.stone();
        while let Some(event) = event_rx.recv().await {
            if let Event::Turn | Event::Error(_) = event {
                cmd_tx.make_move(console::read_move(stone).await);
            }
        }
    }
}

/// A player that passes on every move.
pub struct Void;

#[async_trait]
impl Player for Void {
    async fn attach(self, mut event_rx: Receiver<Event>, cmd_tx: CmdSender) {
        while let Some(event) = event_rx.recv().await {
            if let Event::Turn = event {
                cmd_tx.make_move(None);
            }
        }
    }
}

/// A player that makes totally randomized moves.
pub struct Chaos;

#[async_trait]
impl Player for Chaos {
    async fn attach(self, mut event_rx: Receiver<Event>, cmd_tx: CmdSender) {
        let mut queue: HashSet<Point>;
        if let Some(Event::Settings(settings)) = event_rx.recv().await {
            let size = settings.board_size;
            queue = (0..size)
                .flat_map(|x| (0..size).map(move |y| (x, y).into()))
                .collect();
            queue.remove(&(size / 2, size / 2).into());
        } else {
            unreachable!()
        };

        while let Some(event) = event_rx.recv().await {
            match event {
                Event::Turn => {
                    let mut iter = queue.iter().copied();
                    cmd_tx.make_move(Some((iter.next().unwrap(), iter.next().unwrap())));
                }
                Event::Move(Some(mov)) => {
                    assert!(queue.remove(&mov.0) && queue.remove(&mov.1));
                }
                _ => (),
            }
        }
    }
}

/// A player that uses Monte-Carlo tree search.
pub struct Mcts<P: Policy> {
    policy: P,
    rounds: u64,
    timeout: Duration,
}

impl<P: Policy> Mcts<P> {
    /// Creates a new `Mcts` with the given parameters.
    pub fn new(policy: P, rounds: u64, timeout: Duration) -> Self {
        Self {
            policy,
            rounds,
            timeout,
        }
    }
}

#[async_trait]
impl<P: Policy> Player for Mcts<P> {
    async fn attach(self, mut event_rx: Receiver<Event>, cmd_tx: CmdSender) {
        let timeout = self.timeout / 3;
        let state = Arc::new(Mutex::new(MctsState::new(self.policy, self.rounds)));

        while let Some(event) = event_rx.recv().await {
            match event {
                Event::Turn => {
                    let state = state.clone();
                    let mov = task::spawn_blocking(move || {
                        let mut state = state.lock().unwrap();

                        for _ in 0..2 {
                            let depth = state.search(timeout);
                            println!("Max depth: {depth}");
                            let pair = state.peek();
                            println!("Tentative: ({}, {})", pair.0, pair.1);
                        }

                        let depth = state.search(timeout);
                        println!("Max depth: {depth}");
                        state.peek()
                    })
                    .await
                    .unwrap();
                    cmd_tx.make_move(Some(mov));
                }
                Event::Move(mov) => {
                    let state = state.clone();
                    task::spawn_blocking(move || {
                        state.lock().unwrap().advance(mov);
                    })
                    .await
                    .unwrap();
                }
                _ => (),
            }
        }
    }
}
