use deno_runner::{anyhow::Result, op, Builder};
use std::collections::HashMap;

#[op]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[op]
fn string_concat(a: String, b: String) -> Result<String> {
    Ok(format!("{}{}", a, b))
}

#[tokio::test]
async fn test_bind_fn_add() {
    let custom_code = "add(a, b)";

    let runner = Builder::new().add_op(add::decl()).build();
    let vars = HashMap::from([("a", 1), ("b", 2)]);

    let result = runner.run(custom_code, Some(vars)).await.unwrap();

    assert_eq!(result, "3");
}

#[tokio::test]
async fn test_trigger_via_rust_helper() {
    let custom_code = "rust('add', a, b)";

    let runner = Builder::new().add_op(add::decl()).build();
    let vars = HashMap::from([("a", 1), ("b", 2)]);

    let result = runner.run(custom_code, Some(vars)).await.unwrap();

    assert_eq!(result, "3");
}

#[tokio::test]
async fn test_bind_fn_string_concat() {
    let custom_code = r#"string_concat(a, b)"#;

    let runner = Builder::new().add_op(string_concat::decl()).build();
    let vars = HashMap::from([("a", "a"), ("b", "hihi")]);

    let result = runner.run(custom_code, Some(vars)).await.unwrap();

    assert_eq!(result, "ahihi");
}
