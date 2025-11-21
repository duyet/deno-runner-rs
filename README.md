# deno_runner

[![CI](https://github.com/duyet/deno-runner-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/duyet/deno-runner-rs/actions/workflows/ci.yaml)
[![Security](https://github.com/duyet/deno-runner-rs/actions/workflows/security.yaml/badge.svg)](https://github.com/duyet/deno-runner-rs/actions/workflows/security.yaml)
[![codecov](https://codecov.io/gh/duyet/deno-runner-rs/branch/master/graph/badge.svg?token=TiJMubpWuH)](https://codecov.io/gh/duyet/deno-runner-rs)
[![crates.io](https://img.shields.io/crates/v/deno_runner.svg)](https://crates.io/crates/deno_runner)
[![docs.rs](https://docs.rs/deno_runner/badge.svg)](https://docs.rs/deno_runner)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A **secure** and **elegant** JavaScript runner for Rust, powered by [Deno Core](https://crates.io/crates/deno_core).

Execute JavaScript code from Rust with ease, bind Rust functions to JavaScript, and build powerful embedded scripting capabilities into your applications.

## ✨ Features

- 🔒 **Security First**: Variable name validation, JSON serialization, no unsafe code in public API
- 🚀 **Fast & Lightweight**: Powered by V8 via Deno Core
- 🔄 **Bidirectional Bindings**: Call Rust from JavaScript and vice versa
- 📦 **Easy to Use**: Clean builder pattern API
- 🎯 **Type Safe**: Leverage Rust's type system for JavaScript interop
- 🧪 **Well Tested**: Comprehensive test suite with security tests
- 📚 **Great Documentation**: API docs, examples, and guides

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
deno_runner = "0.2"
tokio = { version = "1", features = ["rt", "macros", "rt-multi-thread"] }
```

## 📖 Usage Examples

### Basic JavaScript Execution

```rust
use deno_runner::Builder;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = r#"
        const add = (a, b) => a + b;
        add(a, b)
    "#;

    let runner = Builder::new().build();
    let vars = HashMap::from([("a", 1), ("b", 2)]);

    let result = runner.run(code, Some(vars)).await?;
    println!("Result: {}", result); // Result: 3

    Ok(())
}
```

### Calling Rust Functions from JavaScript

```rust
use deno_runner::{op, Builder};
use std::collections::HashMap;

#[op]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[op]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = r#"
        const sum = add(10, 20);
        const greeting = greet("World");
        JSON.stringify({ sum, greeting })
    "#;

    let runner = Builder::new()
        .add_op(add::decl())
        .add_op(greet::decl())
        .build();

    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("{}", result); // {"sum":30,"greeting":"Hello, World!"}

    Ok(())
}
```

### Working with Complex Data

```rust
use deno_runner::Builder;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let code = r#"
        `User #${user.id}: ${user.name} (${user.email})`
    "#;

    let runner = Builder::new().build();
    let mut vars = HashMap::new();
    vars.insert("user", user);

    let result = runner.run(code, Some(vars)).await?;
    println!("{}", result); // User #1: Alice (alice@example.com)

    Ok(())
}
```

### Error Handling

```rust
use deno_runner::{Builder, RunnerError};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let code = "undefinedVariable + 1";
    let runner = Builder::new().build();

    match runner.run::<_, String, i32>(code, None).await {
        Ok(result) => println!("Success: {}", result),
        Err(RunnerError::ExecutionError(e)) => {
            eprintln!("JavaScript error: {}", e);
        }
        Err(RunnerError::InvalidVariableName(name)) => {
            eprintln!("Invalid variable name: {}", name);
        }
        Err(e) => eprintln!("Other error: {}", e),
    }
}
```

### Using Timeouts

```rust
use deno_runner::Builder;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = r#"
        // Some potentially long-running code
        let sum = 0;
        for (let i = 0; i < 1000000; i++) {
            sum += i;
        }
        sum
    "#;

    let runner = Builder::new().build();

    // Set a 5-second timeout
    match timeout(
        Duration::from_secs(5),
        runner.run::<_, String, i32>(code, None)
    ).await {
        Ok(Ok(result)) => println!("Result: {}", result),
        Ok(Err(e)) => eprintln!("Execution error: {}", e),
        Err(_) => eprintln!("Timeout!"),
    }

    Ok(())
}
```

## 🔒 Security

Security is a top priority for deno_runner. The library includes several security features:

- **Variable Name Validation**: All variable names must match `[a-zA-Z_][a-zA-Z0-9_]*` to prevent code injection
- **JSON Serialization**: Values are safely serialized as JSON, not string interpolated
- **No Unsafe Code**: Public API contains no unsafe code
- **Security Testing**: Comprehensive tests for injection prevention
- **Regular Audits**: Automated security scanning via cargo-audit and cargo-deny

For more details, see [SECURITY.md](SECURITY.md).

### Security Best Practices

1. **Always validate user input** before executing
2. **Implement timeouts** to prevent infinite loops
3. **Monitor resource usage** in production
4. **Run in isolated environments** for untrusted code
5. **Keep dependencies updated** regularly

## 📚 Examples

The [`examples/`](examples/) directory contains comprehensive examples:

- [`basic.rs`](examples/basic.rs) - Simple JavaScript execution
- [`rust_functions.rs`](examples/rust_functions.rs) - Rust function bindings
- [`error_handling.rs`](examples/error_handling.rs) - Error handling patterns
- [`advanced.rs`](examples/advanced.rs) - Complex use cases

Run an example:

```bash
cargo run --example basic
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific example
cargo run --example rust_functions

# Check code coverage
cargo llvm-cov --all-features
```

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/duyet/deno-runner-rs.git
cd deno-runner-rs

# Build
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy --all-targets --all-features

# Format code
cargo fmt
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Deno Core](https://github.com/denoland/deno_core) - The underlying JavaScript runtime
- [Deno](https://deno.land/) - Inspiration and runtime technology
- All our [contributors](https://github.com/duyet/deno-runner-rs/graphs/contributors)

## 📮 Links

- [Documentation](https://docs.rs/deno_runner)
- [Crates.io](https://crates.io/crates/deno_runner)
- [Repository](https://github.com/duyet/deno-runner-rs)
- [Issue Tracker](https://github.com/duyet/deno-runner-rs/issues)
- [Changelog](CHANGELOG.md)

## 🔮 Roadmap

- [ ] Top-level async/await support
- [ ] Execution timeout configuration via Builder
- [ ] Memory limit configuration
- [ ] Module system support
- [ ] TypeScript support
- [ ] WASM module integration
- [ ] Performance optimizations
- [ ] More built-in operations

---

**Made with ❤️ by [Duyet Le](https://github.com/duyet)**
