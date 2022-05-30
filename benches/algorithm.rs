use criterion::{black_box, criterion_group, criterion_main, Criterion};

use connect6::{
    algorithm::BitBoard,
    board::{Board, Point, Stone},
};

const CENTER: Point = Point::new(9, 9);

criterion_group!(
    benches,
    bench_check_win,
    bench_check_win_potential,
    bench_set_and_check_win,
    bench_check_win_naive_best,
    bench_check_win_naive_worst,
);
criterion_main!(benches);

fn bench_check_win(c: &mut Criterion) {
    let board = BitBoard::new();
    c.bench_function("check_win", |b| {
        b.iter(|| unsafe { board.check_win(black_box(CENTER), Stone::Black) })
    });
}

fn bench_check_win_potential(c: &mut Criterion) {
    let board = BitBoard::new();
    c.bench_function("check_win_potential", |b| {
        b.iter(|| unsafe { board.check_win_potential(black_box(CENTER), Stone::Black) })
    });
}

fn bench_set_and_check_win(c: &mut Criterion) {
    let mut board = BitBoard::new();
    c.bench_function("set_and_check_win", |b| {
        b.iter(|| unsafe { board.set_and_check_win(black_box(CENTER), Stone::Black) })
    });
}

fn bench_check_win_naive_best(c: &mut Criterion) {
    let board = Board::new(19);
    c.bench_function("check_win_naive_best", |b| {
        b.iter(|| board.check_win(black_box((0, 0).into()), Stone::Black))
    });
}

fn bench_check_win_naive_worst(c: &mut Criterion) {
    let mut board = Board::new(19);
    for i in 1..3 {
        board.make_move(((9 + i, 9).into(), (9 - i, 9).into()), Stone::Black);
        board.make_move(((9, 9 + i).into(), (9, 9 - i).into()), Stone::Black);
        board.make_move(((9 + i, 9 + i).into(), (9 - i, 9 - i).into()), Stone::Black);
        board.make_move(((9 + i, 9 - i).into(), (9 - i, 9 + i).into()), Stone::Black);
    }
    c.bench_function("check_win_naive_worst", |b| {
        b.iter(|| board.check_win(black_box(CENTER), Stone::Black))
    });
}
