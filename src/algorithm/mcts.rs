use std::{
    fmt, mem,
    time::{Duration, Instant},
};

use super::{stone, BitBoard, SIZE};
use crate::board::{Point, Stone};

use rand::prelude::*;

mod internal {
    use super::*;

    pub struct Node {
        pub point: Point,
        pub wins: u64,
        pub sims: u64,
        pub visits: u32,
        pub sure_win: bool,
        pub unvisited: Vec<Point>,
        pub visited: u32,
        pub children: Vec<Node>,
    }
}

use internal::Node;

/// A policy for MCTS.
pub trait Policy: 'static + Send {
    /// Peeks the best child of a node.
    fn peek_best<'a>(&self, node: &'a mut Node) -> Option<&'a mut Node>;
}

/// Pure MCTS policy.
pub struct Pure;

impl Policy for Pure {
    #[inline]
    fn peek_best<'a>(&self, node: &'a mut Node) -> Option<&'a mut Node> {
        node.children
            .iter_mut()
            .max_by(|a, b| (a.wins * b.sims).cmp(&(a.sims * b.wins)))
    }
}

/// UCT-based MCTS policy.
pub struct Uct(pub f64);

impl Policy for Uct {
    #[inline]
    fn peek_best<'a>(&self, node: &'a mut Node) -> Option<&'a mut Node> {
        let k = (node.visits as f64).ln();
        node.children.iter_mut().max_by(|a, b| {
            let a = a.wins as f64 / a.sims as f64 + self.0 * (k / a.visits as f64).sqrt();
            let b = b.wins as f64 / b.sims as f64 + self.0 * (k / b.visits as f64).sqrt();
            a.partial_cmp(&b).unwrap()
        })
    }
}

/// A state for Monte-Carlo tree search (MCTS).
#[derive(Debug)]
pub struct MctsState<P: Policy> {
    root: Box<Node>,
    board: Box<BitBoard>,
    sim_board: Box<BitBoard>,
    path: Vec<*mut Node>,
    index: u32,
    policy: P,
}

struct Leaf<'a> {
    node: &'a mut Node,
    board: &'a mut BitBoard,
    sim_board: &'a mut BitBoard,
    index: u32,
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

impl<P: Policy> MctsState<P> {
    /// Creates a new `MctsState`.
    pub fn new(policy: P) -> Self {
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
            visits: 0,
            sure_win: false,
            children: Vec::with_capacity(unvisited.len()),
            unvisited,
            visited: 0,
        });

        MctsState {
            path: vec![&mut *root],
            root,
            board,
            sim_board: BitBoard::new(),
            index: 1,
            policy,
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
            let leaf = self.traverse();
            let wins = leaf.simulate(rng, rounds);
            self.back_propagate(rounds, wins);
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

        if let Some(mov) = mov {
            let size = SIZE as u32;
            assert!(
                mov.0.x < size && mov.0.y < size && mov.1.x < size && mov.1.y < size,
                "out of board"
            );

            let stone = stone(self.index);

            if self.try_reuse(mov) || self.try_reuse((mov.1, mov.0)) {
                unsafe {
                    self.board.set(mov.0, stone);
                    self.board.set(mov.1, stone);
                }
                return;
            }

            self.root.point = mov.1;
            self.root.sure_win = unsafe {
                self.board.set_and_check_win(mov.0, stone)
                    || self.board.set_and_check_win(mov.1, stone)
            };

            self.root.unvisited.retain(|&p| p != mov.0 && p != mov.1);
        }

        self.root.wins = 0;
        self.root.sims = 0;
        self.root.visits = 0;
        self.root.visited = 0;
        self.root.children.clear();
    }

    fn try_reuse(&mut self, mov: (Point, Point)) -> bool {
        if let Some(node) = self.root.children.iter_mut().find(|n| n.point == mov.0) {
            if let Some(i) = node.children.iter().position(|n| n.point == mov.1) {
                *self.root = node.children.swap_remove(i);
                return true;
            }
        }
        false
    }

    fn traverse(&mut self) -> Leaf<'_> {
        let mut node = &mut *self.root;

        while node.unvisited.len() as u32 == node.visited {
            if node.children.is_empty() {
                break;
            }

            node = self.policy.peek_best(node).unwrap();
            self.path.push(node);

            self.index += 1;
            unsafe { self.board.set(node.point, stone(self.index)) }
        }

        if !node.is_terminal() {
            node = node.expand();
            self.path.push(node);

            self.index += 1;
            node.sure_win = unsafe { self.board.set_and_check_win(node.point, stone(self.index)) };
        }

        Leaf {
            node,
            board: &mut self.board,
            sim_board: &mut self.sim_board,
            index: self.index,
        }
    }

    fn back_propagate(&mut self, rounds: u64, mut wins: u64) {
        let (&node, path) = self.path.split_last().unwrap();
        let mut node = unsafe { &mut *node };

        node.wins += wins;
        node.sims += rounds;
        node.visits += 1;
        unsafe { self.board.remove(node.point, stone(self.index)) }

        for node_i in (0..path.len()).rev() {
            if self.index & 1 == 0 {
                wins = rounds - wins;
            }
            self.index -= 1;

            unsafe { node = &mut *path[node_i] }
            node.wins += wins;
            node.sims += rounds;
            node.visits += 1;

            if node_i != 0 {
                // Don't remove the stone for the root.
                unsafe { self.board.remove(node.point, stone(self.index)) }
            }
        }

        self.path.truncate(1);
    }
}

unsafe impl<P: Policy> Send for MctsState<P> {}
unsafe impl<P: Policy> Sync for MctsState<P> {}

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
            visits: 0,
            sure_win: false,
            children: Vec::new(),
            unvisited,
            visited: 0,
        };

        self.children.push(child);
        self.children.last_mut().unwrap()
    }

    fn is_terminal(&self) -> bool {
        self.sure_win || self.unvisited.is_empty()
    }

    fn peek(&self) -> Option<&Node> {
        self.children.iter().max_by(|a, b| a.visits.cmp(&b.visits))
    }

    fn pop(&mut self) -> Option<Node> {
        let opt = self
            .children
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.visits.cmp(&b.visits));
        match opt {
            Some((i, _)) => Some(self.children.swap_remove(i)),
            None => None,
        }
    }
}
