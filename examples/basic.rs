/// Basic example demonstrating simple JavaScript execution
///
/// This example shows how to:
/// - Create a runner
/// - Execute JavaScript code
/// - Pass variables to JavaScript
/// - Get results back to Rust
use deno_runner::Builder;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic JavaScript Execution ===\n");

    // Example 1: Simple arithmetic
    println!("Example 1: Simple arithmetic");
    let code = "2 + 2";
    let runner = Builder::new().build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("Result: {}\n", result);

    // Example 2: String manipulation
    println!("Example 2: String manipulation");
    let code = r#"
        const greeting = "Hello, " + name + "!";
        greeting.toUpperCase()
    "#;
    let runner = Builder::new().build();
    let vars = HashMap::from([("name", "World")]);
    let result = runner.run(code, Some(vars)).await?;
    println!("Result: {}\n", result);

    // Example 3: Working with arrays
    println!("Example 3: Working with arrays");
    let code = r#"
        const numbers = [1, 2, 3, 4, 5];
        numbers.reduce((sum, n) => sum + n, 0)
    "#;
    let runner = Builder::new().build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("Sum of array: {}\n", result);

    // Example 4: JSON processing
    println!("Example 4: JSON processing");
    let code = r#"
        const data = JSON.parse(jsonString);
        data.users.map(u => u.name).join(', ')
    "#;
    let runner = Builder::new().build();
    let json_data = r#"{"users": [{"name": "Alice"}, {"name": "Bob"}]}"#;
    let vars = HashMap::from([("jsonString", json_data)]);
    let result = runner.run(code, Some(vars)).await?;
    println!("User names: {}\n", result);

    // Example 5: Using variables
    println!("Example 5: Multiple variables");
    let code = r#"
        const multiply = (a, b) => a * b;
        const result = multiply(x, y);
        `${x} × ${y} = ${result}`
    "#;
    let runner = Builder::new().build();
    let vars = HashMap::from([("x", 7), ("y", 6)]);
    let result = runner.run(code, Some(vars)).await?;
    println!("Result: {}\n", result);

    println!("=== All examples completed successfully! ===");

    Ok(())
}
