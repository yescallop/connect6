use std::thread;

use connect6::{
    board::Board,
    message::{Event, Msg},
    Builder, Handle,
};

use tokio::runtime::Builder as RtBuilder;

fn main() {
    let Handle {
        mut event_rx,
        cmd_tx,
        ctrl,
    } = Builder::new().build();

    thread::spawn(|| {
        RtBuilder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(ctrl.start());
    });

    for i in 0..6 {
        cmd_tx.make_move(((i * 2, 0).into(), (i * 2 + 1, 0).into()));
        cmd_tx.make_move(((i * 2, 1).into(), (i * 2 + 1, 1).into()));
    }

    let mut board = if let Some(Event {
        msg: Msg::Settings(s),
        ..
    }) = event_rx.blocking_recv()
    {
        Board::new(s.board_size)
    } else {
        unreachable!();
    };

    println!("{:-^30}", " GAME SETTINGS ");
    println!("Board size: {}", board.size());
    println!("{:-^30}", " GAME STARTED ");
    println!("{}", board);

    while let Some(event) = event_rx.blocking_recv() {
        match event.msg {
            Msg::Move(mov) => {
                let stone = event.stone.unwrap();
                if let Some(mov) = mov {
                    board.make_move(mov, stone);
                    println!("{} moved: ({}, {})", stone, mov.0, mov.1);
                    println!("{}", board);
                } else {
                    println!("{} passed.", stone);
                }
            }
            Msg::DrawOffer => {
                println!("{} offered a draw.", event.stone.unwrap());
            }
            Msg::GameEnd(res) => {
                println!("{:-^30}", " GAME ENDED ");
                if let Some(stone) = res.winning_stone {
                    println!("The winner: {}", stone);
                } else {
                    println!("The game ended in a draw.");
                }
                println!("Reason: {}", res.kind);
            }
            Msg::Error(err) => {
                println!("{} occurred an error: {}", event.stone.unwrap(), err);
            }
            _ => (),
        }
    }
}
