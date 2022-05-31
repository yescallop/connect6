#[doc(no_inline)]
pub use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};

use crate::{
    board::{Point, Stone},
    message::{Cmd, FullCmd},
};

/// A stone-specific command sender.
///
/// Drop the sender to disconnect from the game.
pub struct CmdSender {
    pub(super) tx: Sender<FullCmd>,
    pub(super) stone: Stone,
}

impl CmdSender {
    /// Splits a full command sender into stone-specific (Black, White) senders.
    ///
    /// # Panics
    ///
    /// Panics if this sender is not anonymous.
    pub fn split(tx: Sender<FullCmd>) -> (Self, Self) {
        (
            Self {
                tx: tx.clone(),
                stone: Stone::Black,
            },
            Self {
                tx,
                stone: Stone::White,
            },
        )
    }

    /// Returns the stone of this sender.
    #[inline]
    pub fn stone(&self) -> Stone {
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
            stone: Some(self.stone),
        });
    }
}

impl Drop for CmdSender {
    fn drop(&mut self) {
        self.send(Cmd::Disconnect);
    }
}
