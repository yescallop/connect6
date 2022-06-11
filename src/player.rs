use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    algorithm::MctsState,
    board::Point,
    channel::{CmdSender, Receiver},
    console,
    message::Event,
};
use async_trait::async_trait;
use rand::prelude::*;
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
pub struct Mcts {
    rounds: u64,
    timeout: Duration,
}

impl Mcts {
    /// Creates a new `Mcts` with the given parameters.
    pub fn new(rounds: u64, timeout: Duration) -> Mcts {
        Mcts { rounds, timeout }
    }
}

#[async_trait]
impl Player for Mcts {
    async fn attach(self, mut event_rx: Receiver<Event>, cmd_tx: CmdSender) {
        let Mcts { rounds, timeout } = self;
        let state = Arc::new(Mutex::new(MctsState::new()));

        while let Some(event) = event_rx.recv().await {
            match event {
                Event::Turn => {
                    let state = state.clone();
                    let mov = task::spawn_blocking(move || {
                        let mut state = state.lock().unwrap();
                        let mut rng = SmallRng::from_entropy();

                        let mut last = (Point::new(0, 0), Point::new(0, 0));
                        loop {
                            state.search(&mut rng, rounds, timeout);
                            let pair = state.peek();
                            if pair == last {
                                break;
                            }
                            println!("Tentative: ({}, {})", pair.0, pair.1);
                            last = pair;
                        }
                        last
                    })
                    .await
                    .unwrap();
                    cmd_tx.make_move(Some(mov));
                }
                Event::Move(mov) => {
                    state.lock().unwrap().advance(mov);
                }
                _ => (),
            }
        }
    }
}
