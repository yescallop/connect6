use connect6::{player::Chaos, Builder};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Builder::new().build().start(Chaos, Chaos).await;
}
