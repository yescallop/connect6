use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use connect6::algorithm::mcts::{MctsState, Uct};

fn main() {
    let mut c = 0.049;

    loop {
        print!("{c}: ");
        io::stdout().flush().unwrap();
        let cnt = Arc::new(AtomicU32::new(0));
        for _ in 0..4 {
            let cnt_clone = cnt.clone();
            thread::spawn(move || {
                for _ in 0..25 {
                    cnt_clone.fetch_add(test(c), Ordering::Relaxed);
                }
            });
        }

        while Arc::strong_count(&cnt) != 1 {
            thread::sleep(Duration::from_millis(100));
        }

        println!("{}", cnt.load(Ordering::Relaxed));

        c += 0.001;
    }
}

fn test(c: f64) -> u32 {
    let mut state = MctsState::new(Uct(c), 64);
    state.advance(Some(((9, 10).into(), (9, 11).into())));
    state.advance(Some(((8, 8).into(), (10, 10).into())));
    state.advance(Some(((9, 12).into(), (9, 13).into())));
    state.search(Duration::from_secs(20));
    let mov = state.peek();
    if mov.0 == (9, 14) || mov.0 == (9, 15) || mov.1 == (9, 14) || mov.1 == (9, 15) {
        1
    } else {
        0
    }
}
