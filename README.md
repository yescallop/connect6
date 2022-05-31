# connect6

A library for hosting [Connect6] games asynchronously.

[API Docs](https://yescallop.cn/connect6/connect6)

[Connect6]: https://en.wikipedia.org/wiki/Connect6

## Examples

Play a game interactively in the console ([examples/console.rs](/examples/console.rs)):

```rust
use connect6::{player::Console, Builder};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    Builder::new().build().start(Console, Console).await;
}
```

Demonstration:

![Demonstration](/assets/console.png)

For records of real life games: [records](/records).

## Roadmap

- Server & clients.
- Implement AI players with different algorithms (Alpha-beta pruning, MCTS, AlphaZero, etc.).

## License

This project is licensed under the [MIT License](/LICENSE).
