/// Error handling example
///
/// This example demonstrates:
/// - How errors are propagated from JavaScript to Rust
/// - Different types of JavaScript errors
/// - Security features (variable name validation)
/// - Best practices for error handling
use deno_runner::Builder;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("=== Error Handling Examples ===\n");

    // Example 1: JavaScript runtime error
    println!("Example 1: JavaScript runtime error");
    let code = r#"
        undefinedVariable + 1
    "#;
    let runner = Builder::new().build();
    match runner.run::<_, String, i32>(code, None).await {
        Ok(result) => println!("Unexpected success: {}", result),
        Err(e) => println!("Expected error: {}\n", e),
    }

    // Example 2: JavaScript syntax error
    println!("Example 2: JavaScript syntax error");
    let code = r#"
        const x = ;  // Invalid syntax
    "#;
    let runner = Builder::new().build();
    match runner.run::<_, String, i32>(code, None).await {
        Ok(result) => println!("Unexpected success: {}", result),
        Err(e) => println!("Expected error: {}\n", e),
    }

    // Example 3: Type error
    println!("Example 3: Type error");
    let code = r#"
        const x = null;
        x.toString().toUpperCase()
    "#;
    let runner = Builder::new().build();
    match runner.run::<_, String, i32>(code, None).await {
        Ok(result) => println!("Unexpected success: {}", result),
        Err(e) => println!("Expected error: {}\n", e),
    }

    // Example 4: Invalid variable name (security)
    println!("Example 4: Invalid variable name (security)");
    let runner = Builder::new().build();
    let vars = HashMap::from([("invalid-name", 42)]);
    match runner.run("invalid_name", Some(vars)).await {
        Ok(result) => println!("Unexpected success: {}", result),
        Err(e) => println!("Expected error: {}\n", e),
    }

    // Example 5: Attempted code injection
    println!("Example 5: Attempted code injection (security)");
    let runner = Builder::new().build();
    let vars = HashMap::from([("x; maliciousCode(); let y", 1)]);
    match runner.run("x", Some(vars)).await {
        Ok(result) => println!("Unexpected success: {}", result),
        Err(e) => println!("Expected error (injection prevented): {}\n", e),
    }

    // Example 6: Proper error handling in application
    println!("Example 6: Proper error handling pattern");
    let code = r#"
        try {
            JSON.parse(invalidJson)
        } catch (e) {
            'Error: ' + e.message
        }
    "#;
    let runner = Builder::new().build();
    let vars = HashMap::from([("invalidJson", "not valid json")]);
    match runner.run(code, Some(vars)).await {
        Ok(result) => println!("Handled gracefully: {}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 7: Successful execution after errors
    println!("Example 7: Successful execution (errors don't affect subsequent runs)");
    let code = r#"
        const x = 10;
        const y = 20;
        x + y
    "#;
    let runner = Builder::new().build();
    match runner.run::<_, String, i32>(code, None).await {
        Ok(result) => println!("Success: {}\n", result),
        Err(e) => println!("Error: {}\n", e),
    }

    println!("=== Error handling examples completed! ===");
}
