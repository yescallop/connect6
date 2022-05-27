use std::alloc::{self, Layout};

use crate::board::{Point, Stone};

/// The board size used by the algorithms in this module.
pub const SIZE: usize = 19;

const DIAG_SIZE: usize = SIZE * 2 - 1;

/// A bit-packed Connect6 board with optimized *six detection*.
///
/// The *six detection* code should be branchless if instructions `LZCNT` and `TZCNT` are supported.
/// You could even see it _vectorized_ with the latest `AVX512` support.
#[derive(Clone)]
pub struct BitBoard {
    black: Store,
    white: Store,
}

#[derive(Clone)]
struct Store {
    // Horizontal rows.
    h: [u32; SIZE],
    // Vertical rows.
    v: [u32; SIZE],
    // Ascending rows.
    a: [u32; DIAG_SIZE],
    // Descending rows.
    d: [u32; DIAG_SIZE],
}

impl BitBoard {
    /// Creates an empty `BitBoard`.
    #[inline]
    pub fn new() -> Box<BitBoard> {
        let layout = Layout::new::<BitBoard>();
        // SAFETY: A zero-initialized `BitBoard` is valid since all its fields can hold 0.
        let ptr = unsafe { alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }
        // SAFETY: `ptr` is allocated with `Global` with a correct layout.
        unsafe { Box::from_raw(ptr.cast()) }
    }

    /// Returns `true` if there is a stone at the point.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the point is out of board.
    #[inline]
    pub unsafe fn contains(&self, p: Point) -> bool {
        let v = self.black.h.get_unchecked(p.y as usize) | self.white.h.get_unchecked(p.y as usize);
        v & (1 << p.x) != 0
    }

    /// Sets the stone at the point.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the point is out of board or a stone is already there.
    #[inline]
    pub unsafe fn set(&mut self, p: Point, stone: Stone) {
        let store = match stone {
            Stone::Black => &mut self.black,
            Stone::White => &mut self.white,
        };

        let v = store.h.get_unchecked_mut(p.y as usize);
        *v |= 1 << p.x;

        let v = store.v.get_unchecked_mut(p.x as usize);
        *v |= 1 << p.y;

        let i = (SIZE - 1) as u32 + p.x - p.y;
        let v = store.a.get_unchecked_mut(i as usize);
        *v |= 1 << p.y;

        let i = p.x + p.y;
        let v = store.d.get_unchecked_mut(i as usize);
        *v |= 1 << p.y;
    }

    /// Returns `true` if there is a six or overline of the given stone through the point.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the point is out of board or no any stone is there.
    #[inline]
    pub unsafe fn detect_six(&self, p: Point, stone: Stone) -> bool {
        let store = match stone {
            Stone::Black => &self.black,
            Stone::White => &self.white,
        };

        let v = store.h.get_unchecked(p.y as usize).rotate_right(p.x);
        let mut len = v.trailing_ones() + v.leading_ones();

        let v = store.v.get_unchecked(p.x as usize).rotate_right(p.y);
        len = len.max(v.trailing_ones() + v.leading_ones());

        let i = (SIZE - 1) as u32 + p.x - p.y;
        let v = store.a.get_unchecked(i as usize).rotate_right(p.y);
        len = len.max(v.trailing_ones() + v.leading_ones());

        let i = p.x + p.y;
        let v = store.d.get_unchecked(i as usize).rotate_right(p.y);
        len = len.max(v.trailing_ones() + v.leading_ones());

        len >= 6
    }

    /// Returns `true` if there is a potential six or overline of the given stone through the point.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the point is out of board.
    #[inline]
    pub unsafe fn detect_six_potential(&self, p: Point, stone: Stone) -> bool {
        let store = match stone {
            Stone::Black => &self.black,
            Stone::White => &self.white,
        };

        let v = store.h.get_unchecked(p.y as usize).rotate_right(p.x) | 1;
        let mut len = v.trailing_ones() + v.leading_ones();

        let v = store.v.get_unchecked(p.x as usize).rotate_right(p.y) | 1;
        len = len.max(v.trailing_ones() + v.leading_ones());

        let i = (SIZE - 1) as u32 + p.x - p.y;
        let v = store.a.get_unchecked(i as usize).rotate_right(p.y) | 1;
        len = len.max(v.trailing_ones() + v.leading_ones());

        let i = p.x + p.y;
        let v = store.d.get_unchecked(i as usize).rotate_right(p.y) | 1;
        len = len.max(v.trailing_ones() + v.leading_ones());

        len >= 6
    }

    /// Sets the stone at the point and does `detect_six`.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the point is out of board or a stone is already there.
    #[inline]
    pub unsafe fn set_and_detect_six(&mut self, p: Point, stone: Stone) -> bool {
        let store = match stone {
            Stone::Black => &mut self.black,
            Stone::White => &mut self.white,
        };

        let v = store.h.get_unchecked_mut(p.y as usize);
        *v |= 1 << p.x;

        let r = v.rotate_right(p.x);
        let mut len = r.trailing_ones() + r.leading_ones();

        let v = store.v.get_unchecked_mut(p.x as usize);
        *v |= 1 << p.y;

        let r = v.rotate_right(p.y);
        len = len.max(r.trailing_ones() + r.leading_ones());

        let i = (SIZE - 1) as u32 + p.x - p.y;
        let v = store.a.get_unchecked_mut(i as usize);
        *v |= 1 << p.y;

        let r = v.rotate_right(p.y);
        len = len.max(r.trailing_ones() + r.leading_ones());

        let i = p.x + p.y;
        let v = store.d.get_unchecked_mut(i as usize);
        *v |= 1 << p.y;

        let r = v.rotate_right(p.y);
        len = len.max(r.trailing_ones() + r.leading_ones());

        len >= 6
    }
}
