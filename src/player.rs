use std::collections::HashSet;

use crate::{
    board::Point,
    channel::{CmdSender, Receiver},
    console,
    message::Event,
};
use async_trait::async_trait;

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
        let stone = cmd_tx.stone().unwrap();
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
