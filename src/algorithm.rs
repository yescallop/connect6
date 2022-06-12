use std::alloc::{self, Layout};

use crate::board::{Point, Stone};

/// The board size used by the algorithms in this module.
pub const SIZE: usize = 19;

const DIAG_SIZE: usize = SIZE * 2 - 1;

/// A bit-packed Connect6 board with optimized win checking algorithm.
///
/// Use `RUSTFLAGS='-C target-cpu=native'` for maximum performance on your machine.
///
/// The win check should be branchless if target features `bmi1` and `lzcnt` are enabled.
/// You could even see some decent [auto vectorization] with adequate `AVX-512` support.
///
/// [auto vectorization]: https://github.com/yescallop/connect6/blob/main/assets/check_win_avx512.asm
#[derive(Clone, Debug)]
pub struct BitBoard {
    black: Store,
    white: Store,
}

#[derive(Clone, Debug)]
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
        // SAFETY: `ptr` is allocated with `Global` and a correct layout.
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
    /// It is incorrect behavior to call this method if a stone is already at the point.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the point is out of board.
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

    /// Removes the stone at the point.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the point is out of board.
    #[inline]
    pub unsafe fn remove(&mut self, p: Point, stone: Stone) {
        let store = match stone {
            Stone::Black => &mut self.black,
            Stone::White => &mut self.white,
        };

        let v = store.h.get_unchecked_mut(p.y as usize);
        *v &= !(1 << p.x);

        let v = store.v.get_unchecked_mut(p.x as usize);
        *v &= !(1 << p.y);

        let i = (SIZE - 1) as u32 + p.x - p.y;
        let v = store.a.get_unchecked_mut(i as usize);
        *v &= !(1 << p.y);

        let i = p.x + p.y;
        let v = store.d.get_unchecked_mut(i as usize);
        *v &= !(1 << p.y);
    }

    /// Returns `true` if there is a six or overline of the given stone through the point.
    ///
    /// It is incorrect behavior to call this method if no any stone is at the point.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the point is out of board.
    #[inline]
    pub unsafe fn check_win(&self, p: Point, stone: Stone) -> bool {
        let store = match stone {
            Stone::Black => &self.black,
            Stone::White => &self.white,
        };
        let mut res = false;

        let v = store.h.get_unchecked(p.y as usize).rotate_right(p.x);
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        let v = store.v.get_unchecked(p.x as usize).rotate_right(p.y);
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        let i = (SIZE - 1) as u32 + p.x - p.y;
        let v = store.a.get_unchecked(i as usize).rotate_right(p.y);
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        let i = p.x + p.y;
        let v = store.d.get_unchecked(i as usize).rotate_right(p.y);
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        res
    }

    /// Returns `true` if there is a potential six or overline of the given stone through the point.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the point is out of board.
    #[inline]
    pub unsafe fn check_win_potential(&self, p: Point, stone: Stone) -> bool {
        let store = match stone {
            Stone::Black => &self.black,
            Stone::White => &self.white,
        };
        let mut res = false;

        let v = store.h.get_unchecked(p.y as usize).rotate_right(p.x) | 1;
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        let v = store.v.get_unchecked(p.x as usize).rotate_right(p.y) | 1;
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        let i = (SIZE - 1) as u32 + p.x - p.y;
        let v = store.a.get_unchecked(i as usize).rotate_right(p.y) | 1;
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        let i = p.x + p.y;
        let v = store.d.get_unchecked(i as usize).rotate_right(p.y) | 1;
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        res
    }

    /// Sets the stone at the point and returns the result of [`check_win`].
    ///
    /// It is incorrect behavior to call this method if a stone is already at the point.
    ///
    /// [`check_win`]: Self::check_win
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the point is out of board.
    #[inline]
    pub unsafe fn set_and_check_win(&mut self, p: Point, stone: Stone) -> bool {
        #[inline]
        unsafe fn set(rows: &mut [u32], row_i: u32, i: u32) -> u32 {
            let slot = rows.get_unchecked_mut(row_i as usize);
            let v = *slot | (1 << i);
            *slot = v;
            v
        }

        let store = match stone {
            Stone::Black => &mut self.black,
            Stone::White => &mut self.white,
        };
        let mut res = false;

        let v = set(&mut store.h, p.y, p.x).rotate_right(p.x);
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        let v = set(&mut store.v, p.x, p.y).rotate_right(p.y);
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        let i = (SIZE - 1) as u32 + p.x - p.y;
        let v = set(&mut store.a, i, p.y).rotate_right(p.y);
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        let i = p.x + p.y;
        let v = set(&mut store.d, i, p.y).rotate_right(p.y);
        res |= v.trailing_ones() + v.leading_ones() >= 6;

        res
    }
}

fn stone(index: u32) -> Stone {
    if index & 2 == 0 {
        Stone::Black
    } else {
        Stone::White
    }
}

mod mcts;
pub use mcts::MctsState;
