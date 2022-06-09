use std::{
    alloc::{self, Layout},
    cmp, fmt, iter, mem,
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
///
/// The constant parameter `T` stands for the number of
/// random games generated for each feasible move.
#[derive(Debug)]
pub struct MctsState<const T: u64> {
    root: Node,
    board: Box<BitBoard>,
    sim_board: Box<BitBoard>,
    path: Vec<*mut Node>,
    index: u32,
}

struct Leaf<'a, const T: u64> {
    node: &'a mut Node,
    board: &'a BitBoard,
    sim_board: &'a mut BitBoard,
    index: u32,
    expand: bool,
}

struct Node {
    point: Point,
    wins: u64,
    sims: u64,
    terminal: bool,
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

impl<const T: u64> MctsState<T> {
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

        MctsState {
            root: Node {
                point: center,
                wins: 0,
                sims: 0,
                terminal: false,
                children: BinaryHeap::with_capacity(unvisited.len()),
                unvisited,
                visited: 0,
            },
            board,
            sim_board: BitBoard::new(),
            path: Vec::new(),
            index: 1,
        }
    }

    /// Returns `true` if the terminal is reached.
    pub fn is_terminal(&self) -> bool {
        self.root.terminal
    }

    /// Searches for the best moves within a certain amount of time.
    pub fn search<R>(&mut self, rng: &mut R, timeout: Duration)
    where
        R: Rng + ?Sized,
    {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let leaf = self.traverse();
            let expand = leaf.expand;
            let wins = leaf.simulate(rng);
            self.back_propagate(wins, expand);
        }
    }

    /// Returns the currently best pair of moves, without affecting the state.
    pub fn peek_pair(&self) -> (Point, Point) {
        let first = self.root.peek();
        let second = first.peek();
        (first.point, second.point)
    }

    /// Returns the currently best pair of moves, advancing the state by two moves.
    pub fn pop_pair(&mut self) -> (Point, Point) {
        let first = self.root.peek_mut();
        let second = first.pop();
        let pair = (first.point, second.point);

        self.root = second;
        self.index += 2;
        let stone = stone(self.index);
        unsafe {
            self.board.set(pair.0, stone);
            self.board.set(pair.1, stone);
        }

        pair
    }

    fn traverse(&mut self) -> Leaf<'_, T> {
        let mut node = &mut self.root;

        while node.unvisited.len() as u32 == node.visited {
            node = node.children.peek_mut().unwrap();
            self.path.push(node);

            self.index += 1;
            unsafe { self.board.set(node.point, stone(self.index)) }
        }

        let expand = !node.terminal;
        if expand {
            node = node.expand();
            self.path.push(node);

            self.index += 1;
            node.terminal = unsafe { self.board.set_and_check_win(node.point, stone(self.index)) };
        }

        Leaf {
            node,
            board: &self.board,
            sim_board: &mut self.sim_board,
            index: self.index,
            expand,
        }
    }

    fn back_propagate(&mut self, mut wins: u64, expand: bool) {
        let nodes = self.path.iter().rev().copied();
        let parents = nodes
            .clone()
            .skip(1)
            .chain(iter::once(&mut self.root as *mut _));
        let mut back_path = nodes.zip(parents);
        let mut index = self.index;

        let first = back_path.next().unwrap();
        unsafe {
            let node = &mut *first.0;
            node.wins += wins;
            node.sims += T;
            self.board.remove(node.point, stone(index));

            let parent = &mut *first.1;
            if expand {
                parent.children.sift_up_last();
            } else {
                parent.children.sift_down_first();
            }
        }

        for it in back_path {
            if index & 1 == 0 {
                wins = T - wins;
            }
            index -= 1;

            unsafe {
                let node = &mut *it.0;
                node.wins += wins;
                node.sims += T;
                self.board.remove(node.point, stone(index));

                let parent = &mut *it.1;
                parent.children.sift_down_first();
            }
        }

        self.path.clear();
        self.index = index - 1;
    }
}

impl<'a, const T: u64> Leaf<'a, T> {
    fn simulate<R>(self, rng: &mut R) -> u64
    where
        R: Rng + ?Sized,
    {
        if self.node.terminal {
            return T;
        }

        let uv = &mut self.node.unvisited[..];
        let len = uv.len() as u32;
        let mut wins = 0;

        for _ in 0..T {
            self.sim_board.clone_from(self.board);
            let mut index = self.index;

            for i in (1..len).rev() {
                let rand_i = rng.gen_range(0..i + 1) as usize;
                let rand = uv[rand_i];
                uv[rand_i] = mem::replace(&mut uv[i as usize], rand);

                index += 1;
                if unsafe { self.sim_board.set_and_check_win(rand, stone(index)) } {
                    break;
                }
            }

            if stone(index) == stone(self.index) {
                wins += 1;
            }
        }
        wins
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
            terminal: false,
            children: BinaryHeap::new(),
            unvisited,
            visited: 0,
        };

        self.children.push(child)
    }

    fn peek(&self) -> &Node {
        self.children
            .iter()
            .max_by(|a, b| a.sims.cmp(&b.sims))
            .unwrap()
    }

    fn peek_mut(&mut self) -> &mut Node {
        self.children
            .iter_mut()
            .max_by(|a, b| a.sims.cmp(&b.sims))
            .unwrap()
    }

    fn pop(&mut self) -> Node {
        let (i, _) = self
            .children
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.sims.cmp(&b.sims))
            .unwrap();
        self.children.swap_remove(i)
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
