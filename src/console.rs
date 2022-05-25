use crate::{
    board::{Board, Point, Stone},
    channel::Receiver,
    message::{Event, FullEvent},
};

use std::io::{self, Write};

use tokio::task;

/// Logs the game events to the console.
pub async fn log(mut event_rx: Receiver<FullEvent>) {
    let mut board = if let Some(FullEvent {
        event: Event::Settings(settings),
        ..
    }) = event_rx.recv().await
    {
        Board::new(settings.board_size)
    } else {
        unreachable!()
    };

    println!("{:-^30}", " GAME SETTINGS ");
    println!("Board size: {}", board.size());
    println!("{:-^30}", " GAME STARTED ");

    println!("{}", board);

    while let Some(FullEvent { event, stone }) = event_rx.recv().await {
        match event {
            Event::Move(mov) => {
                let stone = stone.unwrap();
                if let Some(mov) = mov {
                    board.make_move(mov, stone);
                    println!("{} moved: ({}, {})", stone, mov.0, mov.1);
                    println!("{}", board);
                } else {
                    println!("{} passed.", stone);
                }
            }
            Event::DrawOffer => {
                println!("{} offered a draw.", stone.unwrap());
            }
            Event::GameEnd(res) => {
                println!("{:-^30}", " GAME ENDED ");
                if let Some(stone) = res.winning_stone {
                    println!("The winner: {}", stone);
                } else {
                    println!("The game ended in a draw.");
                }
                println!("Reason: {}", res.kind);
            }
            Event::Error(err) => {
                println!("{} occurred an error: {}", stone.unwrap(), err);
            }
            _ => (),
        }
    }
}

/// Reads a move from the console.
pub async fn read_move(stone: Stone) -> Option<(Point, Point)> {
    task::spawn_blocking(move || {
        let mut buf = String::new();
        loop {
            print!("[{}] Please move: ", stone);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buf).unwrap();
            if buf == "!" {
                return None;
            }
            if let Some((p1, p2)) = buf.split_once(',') {
                if let (Ok(p1), Ok(p2)) = (p1.trim().parse(), p2.trim().parse()) {
                    return Some((p1, p2));
                }
            }
            eprintln!("[Error] Input mismatch");
            buf.clear();
        }
    })
    .await
    .unwrap()
}
