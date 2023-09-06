# QA and Raffle

## Let's Select a Winner

The sign-in sheet is numbered. Let's be Rustaceans, and use `rand` to select a winner.

```rust,editable
use rand::Rng;

fn main() {
    const MIN: usize = 1;
    const MAX: usize = 20;
    
    let mut rng = rand::thread_rng();
    let winner = rng.gen_range(MIN .. MAX);
    println!("The lucky winner is #{winner}");
}
```

## Questions?

> Any Questions?

> You can get 30% off of any ebook from [pragprog.com](https://pragprog.com/) with the code `RUSTCONF23`. Expires October 15th.

![](./Hands-on%20Rust.png) | ![](./Rust-Brain-Teasers.png)
:-------------------------:|:-------------------------: