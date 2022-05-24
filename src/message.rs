use std::{mem::ManuallyDrop, ptr};

use super::*;

/// The settings of a game.
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    /// The board size.
    pub board_size: u32,
}

/// The result of a game.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GameResult {
    /// The kind of the result.
    pub kind: GameResultKind,
    /// The winning stone, or `None` for a draw.
    pub winning_stone: Option<Stone>,
}

/// The reason for the end of a game.
#[derive(displaydoc::Display, Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameResultKind {
    /// A row has been completed.
    RowCompleted,
    /// Timeout.
    Timeout,
    /// A draw offer has been accepted.
    DrawOfferAccepted,
    /// Both players passed.
    BothPass,
    /// The board is full.
    BoardFull,
    /// Player or server disconnected.
    Disconnected,
}

/// A message sent from the game task.
#[derive(Debug, Clone, Copy)]
pub enum Msg {
    /// Game settings.
    Settings(Settings),
    /// Game started.
    GameStart(Stone),
    /// Move made.
    Move(Option<(Point, Point)>),
    /// A draw is offered.
    DrawOffer,
    /// Game ended.
    GameEnd(GameResult),
    /// Error occurred by the last command.
    Error(CmdError),
}

/// A game event.
#[derive(Debug, Clone)]
pub struct Event {
    /// The message sent.
    pub msg: Msg,
    /// The stone the message is associated with, or `None` if broadcast.
    pub stone: Option<Stone>,
}

/// Errors occurred by an invalid command.
#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum CmdError {
    /// The slot at the point is occupied.
    #[error("occupied: {0}")]
    Occupied(Point),
    /// The point is out of board.
    #[error("out of board: {0}")]
    OutOfBoard(Point),
    /// Ill-timed command.
    #[error("ill-timed command")]
    IllTimed,
}

/// A command sent from the player task.
#[derive(Debug, Clone, Copy)]
pub enum Cmd {
    /// A move.
    Move(Option<(Point, Point)>),
    /// Accepts or offers a draw.
    ///
    /// The opponent will only be notified of a draw offer after a following move.
    AcceptOrOfferDraw,
    /// Disconnects when the sender is dropped.
    Disconnect,
}

/// A stoned command sent from the player task.
#[derive(Debug, Clone, Copy)]
pub struct StonedCmd {
    /// The command.
    pub cmd: Cmd,
    /// The stone that sent the command, or `None` if sent anonymously.
    pub stone: Option<Stone>,
}

/// A command sender.
///
/// Drop the sender to disconnect from the game.
pub struct CmdSender {
    pub(crate) tx: Sender<StonedCmd>,
    pub(crate) stone: Option<Stone>,
}

impl CmdSender {
    /// Consumes this `CmdSender` and returns the underlying raw sender.
    pub fn into_raw(self) -> Sender<StonedCmd> {
        assert!(self.stone.is_none());
        let me = ManuallyDrop::new(self);
        unsafe { ptr::read(&me.tx) }
    }

    /// Splits this anonymous sender into stone-specific (Black, White) senders.
    pub fn split(self) -> (CmdSender, CmdSender) {
        assert!(self.stone.is_none());
        (
            CmdSender {
                tx: self.tx.clone(),
                stone: Some(Stone::Black),
            },
            CmdSender {
                tx: self.into_raw(),
                stone: Some(Stone::White),
            },
        )
    }

    /// Makes a move.
    pub fn make_move(&self, mov: (Point, Point)) {
        self.send(Cmd::Move(Some(mov)));
    }

    /// Passes.
    pub fn pass(&self) {
        self.send(Cmd::Move(None));
    }

    /// Accepts or offers a draw.
    ///
    /// The opponent will only be notified of a draw offer after a following move.
    pub fn accept_or_offer_draw(&self) {
        self.send(Cmd::AcceptOrOfferDraw);
    }

    fn send(&self, raw: Cmd) {
        let _ = self.tx.send(StonedCmd {
            cmd: raw,
            stone: self.stone,
        });
    }
}

impl Drop for CmdSender {
    fn drop(&mut self) {
        if self.stone.is_some() {
            self.send(Cmd::Disconnect);
        }
    }
}
