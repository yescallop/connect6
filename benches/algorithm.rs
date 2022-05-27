#![feature(test)]

extern crate test;

use connect6::{
    algorithm::BitBoard,
    board::{Board, Point, Stone},
};
use test::{black_box, Bencher};

const CENTER: Point = Point::new(9, 9);

#[bench]
fn bench_detect_six(b: &mut Bencher) {
    let board = BitBoard::new();
    b.iter(|| unsafe {
        for _ in 0..100 {
            black_box(board.detect_six(black_box(CENTER), black_box(Stone::Black)));
        }
    })
}

#[bench]
fn bench_detect_six_potential(b: &mut Bencher) {
    let board = BitBoard::new();
    b.iter(|| unsafe {
        for _ in 0..100 {
            black_box(board.detect_six_potential(black_box(CENTER), black_box(Stone::Black)));
        }
    })
}

#[bench]
fn bench_set_and_detect_six(b: &mut Bencher) {
    let mut board = BitBoard::new();
    b.iter(|| unsafe {
        for _ in 0..100 {
            black_box(board.set_and_detect_six(black_box(CENTER), black_box(Stone::Black)));
        }
    })
}

#[bench]
fn bench_detect_six_naive_best(b: &mut Bencher) {
    let board = Board::new(19);
    b.iter(|| {
        for _ in 0..100 {
            board.detect_six((0, 0).into(), Stone::Black);
        }
    })
}

#[bench]
fn bench_detect_six_naive_worst(b: &mut Bencher) {
    let mut board = Board::new(19);
    for i in 1..3 {
        board.make_move(((9 + i, 9).into(), (9 - i, 9).into()), Stone::Black);
        board.make_move(((9, 9 + i).into(), (9, 9 - i).into()), Stone::Black);
        board.make_move(((9 + i, 9 + i).into(), (9 - i, 9 - i).into()), Stone::Black);
        board.make_move(((9 + i, 9 - i).into(), (9 - i, 9 + i).into()), Stone::Black);
    }
    b.iter(|| {
        for _ in 0..100 {
            board.detect_six(CENTER, Stone::Black);
        }
    })
}