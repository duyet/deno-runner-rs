/// Example demonstrating Rust function bindings
///
/// This example shows how to:
/// - Define Rust functions that can be called from JavaScript
/// - Register functions with the runner
/// - Call Rust functions from JavaScript code
/// - Handle different data types
use deno_runner::{op, Builder};
use std::collections::HashMap;

// Simple synchronous operation
#[op]
fn add(a: i32, b: i32) -> i32 {
    println!("[Rust] add({}, {}) called", a, b);
    a + b
}

// String operation
#[op]
fn greet(name: String) -> String {
    println!("[Rust] greet('{}') called", name);
    format!("Hello, {}! Welcome to deno_runner.", name)
}

// More complex operation
#[op]
fn fibonacci(n: i32) -> i32 {
    println!("[Rust] fibonacci({}) called", n);
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

// Async operation
#[op]
async fn fetch_data(id: i32) -> String {
    println!("[Rust] fetch_data({}) called", id);
    // Simulate async work
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    format!("Data for ID {}", id)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust Function Bindings Example ===\n");

    // Example 1: Calling a simple Rust function
    println!("Example 1: Simple function call");
    let code = r#"
        const result = add(10, 32);
        `The answer is: ${result}`
    "#;
    let runner = Builder::new().add_op(add::decl()).build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("Result: {}\n", result);

    // Example 2: String function
    println!("Example 2: String function");
    let code = r#"
        greet("Developer")
    "#;
    let runner = Builder::new().add_op(greet::decl()).build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("Result: {}\n", result);

    // Example 3: Multiple operations
    println!("Example 3: Multiple operations");
    let code = r#"
        const sum = add(5, 3);
        const greeting = greet("Alice");
        JSON.stringify({ sum, greeting })
    "#;
    let runner = Builder::new()
        .add_op(add::decl())
        .add_op(greet::decl())
        .build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("Result: {}\n", result);

    // Example 4: Fibonacci calculation
    println!("Example 4: Fibonacci sequence");
    let code = r#"
        const results = [];
        for (let i = 0; i <= 10; i++) {
            results.push(fibonacci(i));
        }
        results.join(', ')
    "#;
    let runner = Builder::new().add_op(fibonacci::decl()).build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("First 11 Fibonacci numbers: {}\n", result);

    // Example 5: Combining Rust and JavaScript logic
    println!("Example 5: Mixed Rust/JavaScript logic");
    let code = r#"
        // JavaScript logic
        const numbers = [1, 2, 3, 4, 5];

        // Use Rust function for computation
        const doubled = numbers.map(n => add(n, n));
        const sum = doubled.reduce((a, b) => add(a, b), 0);

        `Doubled: [${doubled}], Sum: ${sum}`
    "#;
    let runner = Builder::new().add_op(add::decl()).build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("Result: {}\n", result);

    // Example 6: Using the rust() helper
    println!("Example 6: Using rust() helper");
    let code = r#"
        // Alternative syntax using rust() helper
        const result = rust('add', 15, 27);
        `Using rust() helper: ${result}`
    "#;
    let runner = Builder::new().add_op(add::decl()).build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("Result: {}\n", result);

    println!("=== All examples completed successfully! ===");

    Ok(())
}
