use criterion::{criterion_group, criterion_main, Criterion};

use connect6::algorithm::mcts::{MctsState, Uct};

criterion_group!(benches, bench_mcts);
criterion_main!(benches);

fn bench_mcts(c: &mut Criterion) {
    let mut state = MctsState::new(Uct(0.056), 32);
    c.bench_function("mcts", |b| {
        b.iter(|| {
            state.search_once();
        })
    });
}
