use std::{
    alloc::{self, Layout},
    cmp, fmt, mem,
    time::{Duration, Instant},
};

mod binary_heap;
use binary_heap::BinaryHeap;

use rand::prelude::*;

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

/// A state for Monte-Carlo tree search (MCTS).
#[derive(Debug)]
pub struct MctsState {
    root: Box<Node>,
    board: Box<BitBoard>,
    sim_board: Box<BitBoard>,
    path: Vec<*mut Node>,
    index: u32,
}

struct Leaf<'a> {
    node: &'a mut Node,
    board: &'a mut BitBoard,
    sim_board: &'a mut BitBoard,
    index: u32,
}

struct Node {
    point: Point,
    wins: u64,
    sims: u64,
    sure_win: bool,
    unvisited: Vec<Point>,
    visited: u32,
    children: BinaryHeap<Node>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("point", &self.point)
            .field("wins", &self.wins)
            .field("sims", &self.sims)
            .field("visited", &self.visited)
            .field("children", &self.children)
            .finish()
    }
}

fn stone(index: u32) -> Stone {
    if index & 2 == 0 {
        Stone::Black
    } else {
        Stone::White
    }
}

impl MctsState {
    /// Creates a new `MctsState`.
    pub fn new() -> Self {
        let size = SIZE as u32;
        let center = (size / 2, size / 2).into();

        let mut board = BitBoard::new();
        unsafe { board.set(center, Stone::Black) }

        let mut unvisited: Vec<_> = (0..size)
            .flat_map(|x| (0..size).map(move |y| (x, y).into()))
            .collect();
        unvisited.swap_remove(unvisited.len() / 2);

        let mut root = Box::new(Node {
            point: center,
            wins: 0,
            sims: 0,
            sure_win: false,
            children: BinaryHeap::with_capacity(unvisited.len()),
            unvisited,
            visited: 0,
        });

        MctsState {
            path: vec![&mut *root],
            root,
            board,
            sim_board: BitBoard::new(),
            index: 1,
        }
    }

    /// Returns `true` if the terminal is reached.
    pub fn is_terminal(&self) -> bool {
        self.root.is_terminal()
    }

    /// Searches for the best moves within a certain amount of time.
    pub fn search<R>(&mut self, rng: &mut R, rounds: u64, timeout: Duration)
    where
        R: Rng + ?Sized,
    {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let (leaf, expand) = self.traverse();
            let wins = leaf.simulate(rng, rounds);
            self.back_propagate(expand, rounds, wins);
        }
    }

    /// Returns the currently best pair of moves, without affecting the state.
    pub fn peek(&self) -> (Point, Point) {
        let first = self.root.peek().expect("no children for terminal");
        let second = match first.peek() {
            Some(node) => node.point,
            None => first.unvisited[0],
        };
        (first.point, second)
    }

    /// Returns the currently best pair of moves, advancing the state by two moves.
    pub fn pop(&mut self) -> (Point, Point) {
        *self.root = self.root.pop().expect("no children for terminal");
        self.index += 1;
        let first = self.root.point;

        let second = match self.root.pop() {
            Some(node) => {
                *self.root = node;
                self.index += 1;
                self.root.point
            }
            None => self.root.unvisited[0],
        };

        let stone = stone(self.index);
        unsafe {
            self.board.set(first, stone);
            self.board.set(second, stone);
        }
        (first, second)
    }

    /// Advances through the given pair of moves, if any.
    pub fn advance(&mut self, mov: Option<(Point, Point)>) {
        self.index += 2;
        let mov = match mov {
            Some(mov) => mov,
            None => {
                self.root.wins = 0;
                self.root.sims = 0;
                self.root.visited = 0;
                self.root.children.clear();
                return;
            }
        };

        let stone = stone(self.index);
        let sure_win = unsafe {
            self.board.set_and_check_win(mov.0, stone) | self.board.set_and_check_win(mov.1, stone)
        };

        let unvisited: Vec<_> = self
            .root
            .unvisited
            .iter()
            .copied()
            .filter(|&p| p != mov.0 && p != mov.1)
            .collect();

        *self.root = Node {
            point: mov.1,
            wins: 0,
            sims: 0,
            sure_win,
            children: BinaryHeap::with_capacity(unvisited.len()),
            unvisited,
            visited: 0,
        };
    }

    fn traverse(&mut self) -> (Leaf<'_>, bool) {
        let mut node = &mut *self.root;

        while node.unvisited.len() as u32 == node.visited {
            if node.children.is_empty() {
                break;
            }

            node = node.children.peek_mut().unwrap();
            self.path.push(node);

            self.index += 1;
            unsafe { self.board.set(node.point, stone(self.index)) }
        }

        let expand = !node.is_terminal();
        if expand {
            node = node.expand();
            self.path.push(node);

            self.index += 1;
            node.sure_win = unsafe { self.board.set_and_check_win(node.point, stone(self.index)) };
        }

        let leaf = Leaf {
            node,
            board: &mut self.board,
            sim_board: &mut self.sim_board,
            index: self.index,
        };
        (leaf, expand)
    }

    fn back_propagate(&mut self, mut expand: bool, rounds: u64, mut wins: u64) {
        let (&node, path) = self.path.split_last().unwrap();
        let mut node = unsafe { &mut *node };

        node.wins += wins;
        node.sims += rounds;
        unsafe { self.board.remove(node.point, stone(self.index)) }

        for node_i in (0..path.len()).rev() {
            if self.index & 1 == 0 {
                wins = rounds - wins;
            }
            self.index -= 1;

            unsafe {
                node = &mut *path[node_i];
                node.wins += wins;
                node.sims += rounds;

                if expand {
                    node.children.sift_up_last();
                    expand = false;
                } else {
                    node.children.sift_down_first();
                }
            }

            if node_i != 0 {
                // Don't remove the stone for the root.
                unsafe { self.board.remove(node.point, stone(self.index)) }
            }
        }

        self.path.truncate(1);
    }
}

unsafe impl Send for MctsState {}
unsafe impl Sync for MctsState {}

impl<'a> Leaf<'a> {
    fn simulate<R>(self, rng: &mut R, rounds: u64) -> u64
    where
        R: Rng + ?Sized,
    {
        if self.node.sure_win {
            return rounds;
        }

        let uv = &mut self.node.unvisited[..];
        let len = uv.len() as u32;
        let mut wins = 0;
        let mut draws = 0;

        'outer: for _ in 0..rounds {
            self.sim_board.clone_from(self.board);
            let mut index = self.index;

            for i in (1..len).rev() {
                let rand_i = rng.gen_range(0..i + 1) as usize;
                let rand = uv[rand_i];
                uv[rand_i] = mem::replace(&mut uv[i as usize], rand);

                index += 1;
                if unsafe { self.sim_board.set_and_check_win(rand, stone(index)) } {
                    if stone(index) == stone(self.index) {
                        wins += 1;
                    }
                    continue 'outer;
                }
            }
            draws += 1;
        }
        wins + draws / 2
    }
}

impl Node {
    fn expand(&mut self) -> &mut Node {
        let i = self.visited as usize;
        self.visited += 1;

        let point = self.unvisited[i];
        let unvisited: Vec<_> = self.unvisited[..i]
            .iter()
            .chain(&self.unvisited[i + 1..])
            .copied()
            .collect();

        let child = Node {
            point,
            wins: 0,
            sims: 0,
            sure_win: false,
            children: BinaryHeap::new(),
            unvisited,
            visited: 0,
        };

        self.children.push(child)
    }

    fn is_terminal(&self) -> bool {
        self.sure_win || self.unvisited.is_empty()
    }

    fn peek(&self) -> Option<&Node> {
        self.children.iter().max_by(|a, b| a.sims.cmp(&b.sims))
    }

    fn pop(&mut self) -> Option<Node> {
        let opt = self
            .children
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.sims.cmp(&b.sims));
        match opt {
            Some((i, _)) => Some(self.children.swap_remove(i)),
            None => None,
        }
    }
}

impl PartialEq for Node {
    #[inline]
    fn eq(&self, other: &Node) -> bool {
        self.wins * other.sims == self.sims * other.wins
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.wins * other.sims).cmp(&(self.sims * other.wins))
    }
}
