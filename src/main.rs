use std::io::{self, Write};

use connect6::{
    board::{Board, Point, Stone},
    channel::{CommandSender, Receiver},
    message::{Event, FullEvent},
    Builder, Handle,
};

use tokio::task;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Handle {
        event_rx,
        cmd_tx,
        ctrl,
    } = Builder::new().build();

    tokio::join!(ctrl.start(), attach(event_rx, Some(cmd_tx)));
}

async fn attach(mut event_rx: Receiver<FullEvent>, cmd_tx: Option<CommandSender>) {
    let mut board = if let Some(FullEvent {
        event: Event::Settings(settings),
        ..
    }) = event_rx.recv().await
    {
        Board::new(settings.board_size)
    } else {
        unreachable!();
    };

    println!("{:-^30}", " GAME SETTINGS ");
    println!("Board size: {}", board.size());
    println!("{:-^30}", " GAME STARTED ");

    while let Some(FullEvent { event, stone }) = event_rx.recv().await {
        match event {
            Event::MoveRequest => {
                let stone = stone.unwrap();
                if let Some(cmd_tx) = &cmd_tx {
                    println!("{}", board);
                    cmd_tx.make_move(
                        task::spawn_blocking(move || read_input(stone))
                            .await
                            .unwrap(),
                    );
                }
            }
            Event::Move(mov) => {
                let stone = stone.unwrap();
                if let Some(mov) = mov {
                    board.make_move(mov, stone);
                    println!("{} moved: ({}, {})", stone, mov.0, mov.1);
                } else {
                    println!("{} passed.", stone);
                }
            }
            Event::DrawOffer => {
                println!("{} offered a draw.", stone.unwrap());
            }
            Event::GameEnd(res) => {
                println!("{}", board);
                println!("{:-^30}", " GAME ENDED ");
                if let Some(stone) = res.winning_stone {
                    println!("The winner: {}", stone);
                } else {
                    println!("The game ended in a draw.");
                }
                println!("Reason: {}", res.kind);
            }
            Event::Error(err) => {
                let stone = stone.unwrap();
                println!("{} occurred an error: {}", stone, err);
                if let Some(cmd_tx) = &cmd_tx {
                    cmd_tx.make_move(
                        task::spawn_blocking(move || read_input(stone))
                            .await
                            .unwrap(),
                    );
                }
            }
            _ => (),
        }
    }
}

pub fn read_input(stone: Stone) -> Option<(Point, Point)> {
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
}
