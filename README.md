## Dependencies
- rust
- [trunk](https://github.com/trunk-rs/trunk)

## Setup
Add web assembly as a compilation target:

```
rustup target add wasm32-unknown-unknown
```

Install trunk with `cargo install --locked trunk`

## Usage

Serve frontend with `trunk serve` inside the `./frontend` directory

Run backend with `cargo run --bin backend`
