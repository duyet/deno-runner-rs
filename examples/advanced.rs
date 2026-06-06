/// Advanced usage examples
///
/// This example demonstrates:
/// - Complex data structures
/// - Advanced JavaScript features
/// - Performance considerations
/// - Real-world use cases
use deno_runner::{op, Builder};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Serialize)]
struct Config {
    debug: bool,
    timeout: i32,
    features: Vec<String>,
}

// Custom operation for processing data
#[op]
fn process_data(data: String) -> String {
    println!("[Rust] Processing: {}", data);
    data.to_uppercase()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Advanced Examples ===\n");

    // Example 1: Working with complex objects
    println!("Example 1: Complex object manipulation");
    let user = User {
        id: 1,
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
    };

    let code = r#"
        const fullInfo = `User #${user.id}: ${user.name} (${user.email})`;
        const domain = user.email.split('@')[1];
        JSON.stringify({ fullInfo, domain })
    "#;

    let runner = Builder::new().build();
    let mut vars = HashMap::new();
    vars.insert("user", user);
    let result = runner.run(code, Some(vars)).await?;
    println!("Result: {}\n", result);

    // Example 2: Configuration processing
    println!("Example 2: Configuration processing");
    let config = Config {
        debug: true,
        timeout: 5000,
        features: vec!["auth".to_string(), "api".to_string(), "cache".to_string()],
    };

    let code = r#"
        const summary = {
            mode: config.debug ? 'development' : 'production',
            timeoutSec: config.timeout / 1000,
            featureCount: config.features.length,
            enabledFeatures: config.features.join(', ')
        };
        JSON.stringify(summary, null, 2)
    "#;

    let runner = Builder::new().build();
    let mut vars = HashMap::new();
    vars.insert("config", config);
    let result = runner.run(code, Some(vars)).await?;
    println!("Configuration summary:\n{}\n", result);

    // Example 3: Data transformation pipeline
    println!("Example 3: Data transformation pipeline");
    let code = r#"
        const data = [
            { name: 'Alice', score: 85 },
            { name: 'Bob', score: 92 },
            { name: 'Charlie', score: 78 },
            { name: 'Diana', score: 95 }
        ];

        // Transform data
        const results = data
            .filter(item => item.score >= 80)
            .map(item => ({
                name: item.name,
                grade: item.score >= 90 ? 'A' : 'B',
                passed: true
            }))
            .sort((a, b) => b.grade.localeCompare(a.grade));

        JSON.stringify(results)
    "#;

    let runner = Builder::new().build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("Filtered results: {}\n", result);

    // Example 4: Template engine
    println!("Example 4: Simple template engine");
    let code = r#"
        const template = `
            <div class="card">
                <h2>${title}</h2>
                <p>${description}</p>
                <span class="badge ${priority}">${priority}</span>
            </div>
        `;

        // Evaluate template
        const render = (tpl, data) => {
            return tpl.replace(/\$\{(\w+)\}/g, (_, key) => data[key] || '');
        };

        render(template, data)
    "#;

    let runner = Builder::new().build();
    let vars = HashMap::from([
        ("title", "Important Task"),
        ("description", "Complete the documentation"),
        ("priority", "high"),
    ]);
    let mut all_vars = HashMap::new();
    all_vars.insert("data", vars);
    let result = runner.run(code, Some(all_vars)).await?;
    println!("Rendered template:\n{}\n", result);

    // Example 5: Using Rust functions for validation
    println!("Example 5: Data validation with Rust");
    let code = r#"
        const inputs = ['hello', 'HELLO', 'HeLLo', 'world'];

        // Process all inputs with Rust function
        const processed = inputs.map(input => process_data(input));

        // Find unique values
        const unique = [...new Set(processed)];

        JSON.stringify({
            original: inputs.length,
            unique: unique.length,
            values: unique
        })
    "#;

    let runner = Builder::new().add_op(process_data::decl()).build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("Validation result: {}\n", result);

    // Example 6: Mathematical computations
    println!("Example 6: Mathematical computations");
    let code = r#"
        const stats = {
            mean: arr => arr.reduce((a, b) => a + b, 0) / arr.length,
            median: arr => {
                const sorted = [...arr].sort((a, b) => a - b);
                const mid = Math.floor(sorted.length / 2);
                return sorted.length % 2 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
            },
            stdDev: arr => {
                const mean = arr.reduce((a, b) => a + b, 0) / arr.length;
                const variance = arr.reduce((sum, x) => sum + Math.pow(x - mean, 2), 0) / arr.length;
                return Math.sqrt(variance);
            }
        };

        const data = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        JSON.stringify({
            count: data.length,
            mean: stats.mean(data).toFixed(2),
            median: stats.median(data).toFixed(2),
            stdDev: stats.stdDev(data).toFixed(2)
        })
    "#;

    let runner = Builder::new().build();
    let result = runner.run::<_, String, i32>(code, None).await?;
    println!("Statistics: {}\n", result);

    println!("=== All advanced examples completed successfully! ===");

    Ok(())
}
