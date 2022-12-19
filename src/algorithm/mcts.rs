use std::{
    fmt, mem,
    time::{Duration, Instant},
};

use super::{stone, BitBoard, SIZE};
use crate::board::{Point, Stone};

use fastrand::Rng;

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
            a.total_cmp(&b)
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
    rounds: u64,
    rng: Rng,
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
    pub fn new(policy: P, rounds: u64) -> Self {
        let size = SIZE as u32;
        let center = (size / 2, size / 2).into();

        let mut board = BitBoard::new();
        unsafe { board.set(center, Stone::Black) }

        let mut unvisited: Vec<_> = (0..size)
            .flat_map(|x| (0..size).map(move |y| (x, y).into()))
            .collect();
        unvisited.swap_remove(unvisited.len() / 2);

        let root = Box::new(Node {
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
            path: vec![],
            root,
            board,
            sim_board: BitBoard::new(),
            index: 1,
            rounds,
            policy,
            rng: Rng::new(),
        }
    }

    /// Returns `true` if the terminal is reached.
    pub fn is_terminal(&self) -> bool {
        self.root.is_terminal()
    }

    /// Searches for the best moves within a certain amount of time.
    pub fn search(&mut self, timeout: Duration) {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            self.search_once();
        }
    }

    /// Searches once for the best moves.
    pub fn search_once(&mut self) {
        let leaf = traverse(
            &mut self.root,
            &mut self.board,
            &mut self.path,
            &mut self.index,
            &self.policy,
        );
        let wins = leaf.simulate(
            &self.board,
            &mut self.sim_board,
            self.index,
            &mut self.rng,
            self.rounds,
        );
        self.back_propagate(wins);
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
            assert!(mov.0 != mov.1, "duplicate point");

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
                    | self.board.set_and_check_win(mov.1, stone)
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

    fn back_propagate(&mut self, mut wins: u64) {
        for ptr in self.path.drain(..).rev() {
            let node = unsafe { &mut *ptr };

            node.wins += wins;
            node.sims += self.rounds;
            node.visits += 1;

            if self.index & 1 == 0 {
                wins = self.rounds - wins;
            }

            unsafe { self.board.remove(node.point, stone(self.index)) }
            self.index -= 1;
        }

        self.root.wins += wins;
        self.root.sims += self.rounds;
        self.root.visits += 1;
    }
}

unsafe impl<P: Policy> Send for MctsState<P> {}
unsafe impl<P: Policy> Sync for MctsState<P> {}

fn traverse<'a>(
    root: &'a mut Node,
    board: &mut BitBoard,
    path: &mut Vec<*mut Node>,
    index: &mut u32,
    policy: &impl Policy,
) -> &'a mut Node {
    let mut node = &mut *root;

    while node.unvisited.len() as u32 == node.visited {
        if node.children.is_empty() {
            break;
        }

        node = policy.peek_best(node).unwrap();
        path.push(node);

        *index += 1;
        unsafe { board.set(node.point, stone(*index)) }
    }

    if !node.is_terminal() {
        node = node.expand();
        path.push(node);

        *index += 1;
        node.sure_win = unsafe { board.set_and_check_win(node.point, stone(*index)) };
    }
    node
}

impl Node {
    fn expand(&mut self) -> &mut Node {
        let i = self.visited as usize;
        self.visited += 1;

        let point = self.unvisited[i];
        let mut unvisited = self.unvisited.clone();
        unvisited.swap_remove(i);

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

    fn simulate(
        &mut self,
        board: &BitBoard,
        sim_board: &mut BitBoard,
        index: u32,
        rng: &mut Rng,
        rounds: u64,
    ) -> u64 {
        if self.sure_win {
            return rounds;
        }

        let uv = &mut self.unvisited[..];
        let mut wins = 0;
        let mut draws = 0;

        'outer: for _ in 0..rounds {
            sim_board.clone_from(board);
            let mut sim_index = index;

            for i in (0..uv.len()).rev() {
                let rand_i = rng.usize(0..i + 1);
                let rand = uv[rand_i];
                uv[rand_i] = mem::replace(&mut uv[i], rand);

                sim_index += 1;
                if unsafe { sim_board.set_and_check_win(rand, stone(sim_index)) } {
                    // stone(sim_index) == stone(index)
                    if (sim_index ^ index) & 2 == 0 {
                        wins += 1;
                    }
                    continue 'outer;
                }
            }
            draws += 1;
        }
        wins + draws / 2
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
