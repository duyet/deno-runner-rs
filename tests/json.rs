use deno_runner::Builder;
use std::collections::HashMap;

#[tokio::test]
async fn test_parse() {
    let custom_code = r#"
        let json = JSON.parse(value)
        json['a'] + json['b']['c']
    "#;

    let runner = Builder::new().build();

    let vars = HashMap::from([("value", r#"{"a": 1, "b": {"c": 2}}"#)]);
    let expected = "3".to_string();

    let result = runner
        .run(custom_code.to_string(), Some(vars))
        .await
        .unwrap();

    assert_eq!(result, expected);
}
