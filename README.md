# Javascript runner in Rust

[![cargo-test](https://github.com/fossil-engineering/deno-runner-rs/actions/workflows/cargo-test.yaml/badge.svg)](https://github.com/fossil-engineering/deno-runner-rs/actions/workflows/cargo-test.yaml)
[![cargo-doc](https://github.com/fossil-engineering/deno-runner-rs/actions/workflows/cargo-doc.yaml/badge.svg)](https://github.com/fossil-engineering/deno-runner-rs/actions/workflows/cargo-doc.yaml)
[![cargo-clippy](https://github.com/fossil-engineering/deno-runner-rs/actions/workflows/cargo-clippy.yml/badge.svg)](https://github.com/fossil-engineering/deno-runner-rs/actions/workflows/cargo-clippy.yml)
[![codecov](https://codecov.io/gh/fossil-engineering/deno-runner-rs/branch/master/graph/badge.svg?token=TiJMubpWuH)](https://codecov.io/gh/fossil-engineering/deno-runner-rs)

This runner using [deno_core](https://crates.io/crates/deno_core) with additional helper function to run Javascript inside Rust.
You can also binding the Rust functions into Javascript.

# Usage

Add this crate to your Cargo.toml 

```toml
[dependencies]
deno_runner = { git = "https://github.com/fossil-engineering/deno-runner-rs" }
```

## Example

Basic usage

```rust
use deno_runner::Builder;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let code = r#"
        const add = (a, b) => a + b;
        add(a, b)
    "#;

    let runner = Builder::new().build();
    let vars = HashMap::from([("a", 1), ("b", 2)]);

    let result = runner.run(code, Some(vars)).await.unwrap();

    assert_eq!(result, "3");
}
```

Calling Rust functions from Javascript

```rust
use deno_runner::{anyhow::Result, op, Builder};
use std::collections::HashMap;

#[op]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[tokio::main]
async fn main() {
    let code = "add(a, b)";

    let runner = Builder::new().add_op(add::decl()).build();
    let vars = HashMap::from([("a", 1), ("b", 2)]);

    let result = runner.run(code, Some(vars)).await.unwrap();

    assert_eq!(result, "3");
}
```

# License

MIT
