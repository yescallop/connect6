#![warn(rust_2018_idioms, unreachable_pub, missing_docs)]

//! A library for hosting [Connect6] games asynchronously.
//!
//! [Connect6]: https://en.wikipedia.org/wiki/Connect6

use tokio::sync::mpsc;

/// Connect6 boards.
pub mod board;

use board::{Board, Point, Stone};

/// Message types that may be passed between tasks.
pub mod message;

use message::{CmdError::*, *};

/// Channel types for message passing between tasks.
pub mod channel;

use channel::*;

/// Module for console logging and input.
pub mod console;

/// Player trait and implementations.
pub mod player;

use player::Player;

/// Heavily optimized algorithms for game solving.
pub mod algorithm;

macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

/// Builder for a game.
#[derive(Debug, Clone)]
pub struct Builder {
    board_size: u32,
}

impl Builder {
    /// Creates a new builder with default values.
    #[inline]
    pub fn new() -> Self {
        Builder { board_size: 19 }
    }

    /// Sets the board size.
    #[inline]
    pub fn board_size(&mut self, size: u32) -> &mut Self {
        self.board_size = size;
        self
    }

    /// Builds the game handle.
    pub fn build(&self) -> Handle {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        Handle {
            event_rx,
            cmd_tx: CmdSender {
                tx: cmd_tx,
                stone: None,
            },
            ctrl: Control::new(self, event_tx, cmd_rx),
        }
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Builder::new()
    }
}

/// A game handle.
pub struct Handle {
    /// The global event receiver.
    pub event_rx: Receiver<FullEvent>,
    /// The command sender.
    pub cmd_tx: CmdSender,
    /// The game control.
    pub ctrl: Box<Control>,
}

impl Handle {
    /// Starts a game with the given players, logging the events in the console.
    pub async fn start(mut self, black: impl Player, white: impl Player) -> GameResult {
        let (black_rx, white_rx) = self.ctrl.subscribe();
        let (black_tx, white_tx) = self.cmd_tx.split();
        tokio::join!(
            self.ctrl.start(),
            console::log(self.event_rx),
            black.attach(black_rx, black_tx),
            white.attach(white_rx, white_tx),
        )
        .0
    }

    /// Starts a game with the given players silently.
    pub async fn start_silent(mut self, black: impl Player, white: impl Player) -> GameResult {
        let (black_rx, white_rx) = self.ctrl.subscribe();
        let (black_tx, white_tx) = self.cmd_tx.split();
        tokio::join!(
            self.ctrl.start(),
            black.attach(black_rx, black_tx),
            white.attach(white_rx, white_tx),
        )
        .0
    }
}

enum Action {
    Wait,
    Next,
}

/// A game control.
pub struct Control {
    /// The global event sender.
    event_tx: Sender<FullEvent>,
    /// The notification event senders.
    notification_txs: Option<[Sender<Event>; 2]>,
    /// The command receiver.
    cmd_rx: Receiver<FullCmd>,

    /// The board.
    board: Board,
    /// The current stone.
    cur_stone: Stone,
    /// Whether the last move is a pass.
    last_pass: bool,
    /// The stone that offered a draw.
    last_draw_offer: Option<Stone>,

    /// The result of the game, or `None` if the game is not ended.
    result: Option<GameResult>,
}

impl Control {
    fn new(builder: &Builder, event_tx: Sender<FullEvent>, cmd_rx: Receiver<FullCmd>) -> Box<Self> {
        Box::new(Self {
            event_tx,
            notification_txs: None,
            cmd_rx,
            board: Board::new(builder.board_size),
            cur_stone: Stone::White,
            last_pass: false,
            last_draw_offer: None,
            result: None,
        })
    }

    /// Subscribes two notification event receivers (Black, White) from the game.
    pub fn subscribe(&mut self) -> (Receiver<Event>, Receiver<Event>) {
        let first = mpsc::unbounded_channel();
        let second = mpsc::unbounded_channel();
        self.notification_txs = Some([first.0, second.0]);
        (first.1, second.1)
    }

    /// Notifies one stone of the event.
    fn notify(&self, stone: Stone, event: Event) {
        if let Some(txs) = &self.notification_txs {
            let _ = txs[stone as usize - 1].send(event);
        }
    }

    /// Notifies both stones of the event.
    fn notify_both(&self, event: Event) {
        if let Some(txs) = &self.notification_txs {
            let _ = txs[0].send(event);
            let _ = txs[1].send(event);
        }
    }

    /// Sends a global event.
    fn event(&self, stone: Option<Stone>, event: Event) {
        let _ = self.event_tx.send(FullEvent { event, stone });
    }

    /// Switches the turn.
    fn switch(&mut self) {
        self.cur_stone = self.cur_stone.opposite();
    }

    /// Makes a move on the board if it is not a pass, switches the turn and broadcasts it.
    fn make_move(&mut self, stone: Stone, mov: Option<(Point, Point)>) {
        if let Some(mov) = mov {
            self.board.make_move(mov, stone);
        }
        self.switch();
        self.event(Some(stone), Event::Move(mov));
        self.notify_both(Event::Move(mov));
    }

    /// Ends the game with the given result.
    fn end(&mut self, kind: GameResultKind, winning_stone: Stone) {
        self.result = Some(GameResult {
            kind,
            winning_stone: Some(winning_stone),
        });
    }

    /// Ends the game in a draw with the given result.
    fn end_draw(&mut self, kind: GameResultKind) {
        self.result = Some(GameResult {
            kind,
            winning_stone: None,
        });
    }

    /// Starts the game and returns the result when the game is ended.
    pub async fn start(mut self: Box<Self>) -> GameResult {
        // Broadcast the game settings.
        let settings = Settings {
            board_size: self.board.size(),
        };
        self.event(None, Event::Settings(settings));
        self.notify_both(Event::Settings(settings));

        // Loop until the game is ended.
        'outer: while self.result.is_none() {
            if self.board.is_full() {
                // End the game for a full board.
                self.end_draw(GameResultKind::BoardFull);
                break;
            }

            self.event(Some(self.cur_stone), Event::Turn);
            self.notify(self.cur_stone, Event::Turn);

            // TODO: Calculate timeout.
            while let Some(FullCmd { cmd, stone }) = self.cmd_rx.recv().await {
                // If sent anonymously, a command should belong to the current stone.
                let stone = stone.unwrap_or(self.cur_stone);
                match self.process_cmd(stone, cmd) {
                    Err(e) => {
                        self.event(Some(stone), Event::Error(e));
                        self.notify(stone, Event::Error(e));
                    }
                    Ok(Action::Next) => continue 'outer,
                    Ok(Action::Wait) => (),
                }
            }
            self.end_draw(GameResultKind::Disconnected);
        }

        // Broadcast the result and goodbye.
        let result = self.result.unwrap();
        self.event(None, Event::GameEnd(result));
        self.notify_both(Event::GameEnd(result));
        result
    }

    fn process_cmd(&mut self, stone: Stone, cmd: Cmd) -> Result<Action, CmdError> {
        match cmd {
            Cmd::Move(mov) => {
                ensure!(self.cur_stone == stone, IllTimed);

                if self.last_draw_offer == Some(stone) {
                    self.event(Some(stone), Event::DrawOffer);
                    self.notify(stone.opposite(), Event::DrawOffer);
                } else {
                    self.last_draw_offer = None;
                }

                if let Some(mov) = mov {
                    let slot = self.board.get(mov.0).ok_or(OutOfBoard(mov.0))?;
                    ensure!(slot.is_empty(), Occupied(mov.0));

                    let slot = self.board.get(mov.1).ok_or(OutOfBoard(mov.1))?;
                    ensure!(slot.is_empty(), Occupied(mov.1));

                    self.make_move(stone, Some(mov));

                    if self.board.detect_six(mov.0, stone) || self.board.detect_six(mov.1, stone) {
                        self.end(GameResultKind::RowCompleted, stone);
                    }
                    self.last_pass = false;
                } else {
                    self.make_move(stone, None);

                    if self.last_pass {
                        self.end_draw(GameResultKind::BothPass);
                    }
                    self.last_pass = true;
                }
                Ok(Action::Next)
            }
            Cmd::AcceptOrOfferDraw => {
                if self.last_draw_offer == Some(stone.opposite()) {
                    self.end_draw(GameResultKind::DrawOfferAccepted);
                    Ok(Action::Next)
                } else {
                    ensure!(self.cur_stone == stone, IllTimed);
                    self.last_draw_offer = Some(stone);
                    Ok(Action::Wait)
                }
            }
            Cmd::Disconnect => {
                self.end(GameResultKind::Disconnected, stone.opposite());
                Ok(Action::Next)
            }
        }
    }
}
