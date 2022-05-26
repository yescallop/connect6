use connect6::{player::Console, Builder};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Builder::new().build().start(Console, Console).await;
}
