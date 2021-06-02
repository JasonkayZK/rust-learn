## Multiple Main

Sometimes we need to run multiple demos in different main functions;

Here is how it works in Rust;

### Create `examples` folder

First, create `examples` folder in the same directory as `src`;

```shell
mkdir "examples"
```

### Create demo in `examples` folder

```shell
vi examples/main1.rs
```

write rust code:

```rust
fn main() {
    println!("Hello World in main1!");
}
```

### Run the examples

```shell
cargo run --color=always --package multiple_main --example main1
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\examples\main1.exe`
Hello World in main1!
```
