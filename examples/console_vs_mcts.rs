use connect6::{
    player::{Console, Mcts},
    Builder,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Builder::new().build().start(Console, Mcts).await;
}
