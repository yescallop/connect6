use crate::board::{Point, Stone};

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

/// An event sent from the game task.
#[derive(Debug, Clone, Copy)]
pub enum Event {
    /// Game settings.
    Settings(Settings),
    /// Your turn.
    Turn,
    /// Move made.
    Move(Option<(Point, Point)>),
    /// A draw is offered.
    DrawOffer,
    /// Error occurred by the last command.
    Error(CmdError),
    /// Game ended.
    GameEnd(GameResult),
}

/// A full event sent from the game task.
#[derive(Debug, Clone)]
pub struct FullEvent {
    /// The event.
    pub event: Event,
    /// The stone the event is associated with.
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

/// A full command sent from the player task.
#[derive(Debug, Clone, Copy)]
pub struct FullCmd {
    /// The command.
    pub cmd: Cmd,
    /// The stone that sent the command, or `None` if sent anonymously.
    pub stone: Option<Stone>,
}
