use deno_runner::Builder;
use std::collections::HashMap;

#[tokio::test]
async fn test_function() {
    let custom_code = r#"
        const add = (a, b) => a + b;
        add(a, b)
    "#;

    let runner = Builder::new().build();
    let vars = HashMap::from([("a", 1), ("b", 2)]);
    let result = runner.run(custom_code, Some(vars)).await.unwrap();

    assert_eq!(result, "3");
}

#[tokio::test]
async fn test_console_log() {
    let custom_code = r#"
        console.log(value);
        console.error(value);
    "#;

    let runner = Builder::new().build();
    let vars = HashMap::from([("value", "hello")]);
    let result = runner.run(custom_code, Some(vars)).await;

    assert!(result.is_ok());
}
