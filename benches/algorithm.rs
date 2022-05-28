#![feature(test)]

extern crate test;

use connect6::{
    algorithm::BitBoard,
    board::{Board, Point, Stone},
};
use test::{black_box, Bencher};

const CENTER: Point = Point::new(9, 9);
const RUNS_PER_ITER: u32 = 100;

#[bench]
fn bench_check_win(b: &mut Bencher) {
    let board = BitBoard::new();
    b.iter(|| unsafe {
        let p = black_box(CENTER);
        for _ in 0..RUNS_PER_ITER {
            black_box(board.check_win(p, Stone::Black));
        }
    })
}

#[bench]
fn bench_check_win_potential(b: &mut Bencher) {
    let board = BitBoard::new();
    b.iter(|| unsafe {
        let p = black_box(CENTER);
        for _ in 0..RUNS_PER_ITER {
            black_box(board.check_win_potential(p, Stone::Black));
        }
    })
}

#[bench]
fn bench_set_and_check_win(b: &mut Bencher) {
    let mut board = BitBoard::new();
    b.iter(|| unsafe {
        let p = black_box(CENTER);
        for _ in 0..RUNS_PER_ITER {
            black_box(board.set_and_check_win(p, Stone::Black));
        }
    })
}

#[bench]
fn bench_check_win_naive_best(b: &mut Bencher) {
    let board = Board::new(19);
    b.iter(|| {
        for _ in 0..RUNS_PER_ITER {
            board.check_win((0, 0).into(), Stone::Black);
        }
    })
}

#[bench]
fn bench_check_win_naive_worst(b: &mut Bencher) {
    let mut board = Board::new(19);
    for i in 1..3 {
        board.make_move(((9 + i, 9).into(), (9 - i, 9).into()), Stone::Black);
        board.make_move(((9, 9 + i).into(), (9, 9 - i).into()), Stone::Black);
        board.make_move(((9 + i, 9 + i).into(), (9 - i, 9 - i).into()), Stone::Black);
        board.make_move(((9 + i, 9 - i).into(), (9 - i, 9 + i).into()), Stone::Black);
    }
    b.iter(|| {
        for _ in 0..RUNS_PER_ITER {
            board.check_win(CENTER, Stone::Black);
        }
    })
}
