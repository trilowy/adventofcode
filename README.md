# How to run

## For Rust

* Install Rust
* Go to the puzzle directory
* Run:
  ```sh
  cargo run
  ```
* Launch tests:
  ```sh
  cargo test
  ```
* See how fast the program runs:
  ```sh
  cargo build --release
  time ./target/release/adventofcode
  ```

## For Zig

* Install Zig
* Go to the puzzle directory
* Run:
  ```sh
  zig run main.zig
  ```
* Launch tests:
  ```sh
  zig test main.zig
  ```
* See how fast the program runs:
  ```sh
  zig build-exe main.zig -Doptimize=ReleaseSafe
  time ./main
  ```
