#[doc(no_inline)]
pub use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};

use std::{mem::ManuallyDrop, ptr};

use crate::{
    board::{Point, Stone},
    message::{Cmd, FullCmd},
};

/// A command sender.
///
/// Drop the sender to disconnect from the game.
pub struct CmdSender {
    pub(super) tx: Sender<FullCmd>,
    pub(super) stone: Option<Stone>,
}

impl CmdSender {
    /// Consumes this `CmdSender` and returns the underlying full command sender.
    ///
    /// # Panics
    ///
    /// Panics if this sender is not anonymous.
    pub fn into_full(self) -> Sender<FullCmd> {
        assert!(self.stone.is_none());
        let me = ManuallyDrop::new(self);
        unsafe { ptr::read(&me.tx) }
    }

    /// Splits this anonymous sender into stone-specific (Black, White) senders.
    ///
    /// # Panics
    ///
    /// Panics if this sender is not anonymous.
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

    /// Returns the stone of this sender, or `None` if this sender is anonymous.
    #[inline]
    pub fn stone(&self) -> Option<Stone> {
        self.stone
    }

    /// Makes a move.
    pub fn make_move(&self, mov: Option<(Point, Point)>) {
        self.send(Cmd::Move(mov));
    }

    /// Accepts or offers a draw.
    ///
    /// The opponent will only be notified of a draw offer after a following move.
    pub fn accept_or_offer_draw(&self) {
        self.send(Cmd::AcceptOrOfferDraw);
    }

    fn send(&self, cmd: Cmd) {
        let _ = self.tx.send(FullCmd {
            cmd,
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
