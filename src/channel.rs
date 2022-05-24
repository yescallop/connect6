#[doc(no_inline)]
pub use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};

use std::{mem::ManuallyDrop, ptr};

use crate::{
    board::{Point, Stone},
    message::{Command, FullCommand},
};

/// A command sender.
///
/// Drop the sender to disconnect from the game.
pub struct CommandSender {
    pub(crate) tx: Sender<FullCommand>,
    pub(crate) stone: Option<Stone>,
}

impl CommandSender {
    /// Consumes this `CmdSender` and returns the underlying full command sender.
    pub fn into_full(self) -> Sender<FullCommand> {
        assert!(self.stone.is_none());
        let me = ManuallyDrop::new(self);
        unsafe { ptr::read(&me.tx) }
    }

    /// Splits this anonymous sender into stone-specific (Black, White) senders.
    pub fn split(self) -> (Self, Self) {
        assert!(self.stone.is_none());
        (
            Self {
                tx: self.tx.clone(),
                stone: Some(Stone::Black),
            },
            Self {
                tx: self.into_full(),
                stone: Some(Stone::White),
            },
        )
    }

    /// Makes a move.
    pub fn make_move(&self, mov: Option<(Point, Point)>) {
        self.send(Command::Move(mov));
    }

    /// Accepts or offers a draw.
    ///
    /// The opponent will only be notified of a draw offer after a following move.
    pub fn accept_or_offer_draw(&self) {
        self.send(Command::AcceptOrOfferDraw);
    }

    fn send(&self, cmd: Command) {
        let _ = self.tx.send(FullCommand {
            cmd,
            stone: self.stone,
        });
    }
}

impl Drop for CommandSender {
    fn drop(&mut self) {
        if self.stone.is_some() {
            self.send(Command::Disconnect);
        }
    }
}
