# Solutions for the 2024 Advent of Code in Rust by matei-oltean

See https://adventofcode.com/2024 to access the contest

## Workflow

All these commands should be run from the root folder and should assume the Rust toolchain presence on the computer.

### Creating a new package

To create a new package:

```rust
cargo new day_x
```

`day_x` should be added to the `members` list of the root `Cargo.toml` file.

### Running the solutions

```bash
cargo run -p day_x
```

### Running clippy on the code

Run `cargo clippy --fix --allow-dirty` to fix clippy issues in the code.
