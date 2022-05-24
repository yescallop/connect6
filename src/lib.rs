#![warn(rust_2018_idioms, unreachable_pub, missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

//! A library for hosting Connect6 games.

use board::{Board, Point, Stone};

use tokio::sync::mpsc::{self, UnboundedReceiver as Receiver, UnboundedSender as Sender};

/// Connect6 boards.
pub mod board;

/// Module for message passing between tasks.
pub mod message;

use message::{CmdError::*, *};

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
    /// The event receiver.
    pub event_rx: Receiver<Event>,
    /// The command sender.
    pub cmd_tx: CmdSender,
    /// The game control.
    pub ctrl: Box<Control>,
}

enum Action {
    Wait,
    Next,
}

/// A game control.
pub struct Control {
    /// The event sender.
    event_tx: Sender<Event>,
    /// The message senders.
    msg_txs: Option<[Sender<Msg>; 2]>,
    /// The command receiver.
    cmd_rx: Receiver<StonedCmd>,

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
    fn new(builder: &Builder, event_tx: Sender<Event>, cmd_rx: Receiver<StonedCmd>) -> Box<Self> {
        Box::new(Self {
            event_tx,
            msg_txs: None,
            cmd_rx,
            board: Board::new(builder.board_size),
            cur_stone: Stone::White,
            last_pass: false,
            last_draw_offer: None,
            result: None,
        })
    }

    /// Subscribes two split message channels (Black, White) from the game.
    pub fn subscribe_split(&mut self) -> (Receiver<Msg>, Receiver<Msg>) {
        let first = mpsc::unbounded_channel();
        let second = mpsc::unbounded_channel();
        self.msg_txs = Some([first.0, second.0]);
        (first.1, second.1)
    }

    /// Sends a message to one stone.
    fn msg(&self, stone: Stone, msg: Msg) {
        if let Some(txs) = &self.msg_txs {
            let _ = txs[stone as usize - 1].send(msg);
        }
    }

    /// Sends a message to both stones.
    fn msg_all(&self, msg: Msg) {
        if let Some(txs) = &self.msg_txs {
            let _ = txs[0].send(msg);
            let _ = txs[1].send(msg);
        }
    }

    /// Sends an event.
    fn event(&self, stone: Option<Stone>, msg: Msg) {
        let _ = self.event_tx.send(Event { msg, stone });
    }

    /// Switches the turn.
    fn switch(&mut self) {
        self.cur_stone = self.cur_stone.opposite();
    }

    /// Makes a move on the board if it is not a pass,
    /// switches the turn and broadcasts it.
    fn make_move(&mut self, stone: Stone, mov: Option<(Point, Point)>) {
        if let Some(mov) = mov {
            self.board.make_move(mov, stone);
        }
        self.switch();
        self.event(Some(stone), Msg::Move(mov));
        self.msg_all(Msg::Move(mov));
    }

    /// Ends the game with the given result.
    fn end(&mut self, kind: GameResultKind, winning_stone: Stone) {
        if self.result.is_none() {
            self.result = Some(GameResult {
                kind,
                winning_stone: Some(winning_stone),
            });
        }
    }

    /// Ends the game in a draw with the given result.
    fn end_draw(&mut self, kind: GameResultKind) {
        if self.result.is_none() {
            self.result = Some(GameResult {
                kind,
                winning_stone: None,
            });
        }
    }

    /// Starts the game.
    pub async fn start(mut self: Box<Self>) -> GameResult {
        // Broadcast the game settings.
        let settings = Settings {
            board_size: self.board.size(),
        };
        self.event(None, Msg::Settings(settings));
        self.msg_all(Msg::Settings(settings));

        self.msg(Stone::Black, Msg::GameStart(Stone::Black));
        self.msg(Stone::White, Msg::GameStart(Stone::White));

        // Loop until the game is ended.
        'outer: while self.result.is_none() {
            if self.board.is_full() {
                // End the game for a full board.
                self.end_draw(GameResultKind::BoardFull);
                break;
            }

            // TODO: Calculate timeout.
            while let Some(StonedCmd { cmd, stone }) = self.cmd_rx.recv().await {
                // If sent anonymously, a command should belong to the current stone.
                let stone = stone.unwrap_or(self.cur_stone);
                match self.process_cmd(stone, cmd) {
                    Err(e) => {
                        self.event(Some(stone), Msg::Error(e));
                        self.msg(stone, Msg::Error(e));
                    }
                    Ok(Action::Next) => continue 'outer,
                    Ok(Action::Wait) => (),
                }
            }
            self.end_draw(GameResultKind::Disconnected);
        }

        // Broadcast the result and goodbye.
        let result = self.result.unwrap();
        self.event(None, Msg::GameEnd(result));
        self.msg_all(Msg::GameEnd(result));
        result
    }

    fn process_cmd(&mut self, stone: Stone, cmd: Cmd) -> Result<Action, CmdError> {
        match cmd {
            Cmd::Move(mov) => {
                ensure!(self.cur_stone == stone, IllTimed);

                if self.last_draw_offer == Some(stone) {
                    self.event(Some(stone), Msg::DrawOffer);
                    self.msg(stone.opposite(), Msg::DrawOffer);
                } else {
                    self.last_draw_offer = None;
                }

                if let Some(mov) = mov {
                    let slot = self.board.get(mov.0).ok_or(OutOfBoard(mov.0))?;
                    ensure!(slot.is_empty(), Occupied(mov.0));

                    let slot = self.board.get(mov.1).ok_or(OutOfBoard(mov.1))?;
                    ensure!(slot.is_empty(), Occupied(mov.1));

                    self.make_move(stone, Some(mov));

                    if self.board.test_six_at(mov.0, stone) || self.board.test_six_at(mov.1, stone)
                    {
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
