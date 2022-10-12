use deno_runner::{Builder, Runtime};
use std::collections::HashMap;

#[test]
fn test_tokio_runtime() {
    // Using re-exported tokio runtime
    let rt = Runtime::new().unwrap();
    let out = rt.block_on(async {
        let custom_code = r#"
            const add = (a, b) => a + b;
            add(a, b)
        "#;

        let runner = Builder::new().build();
        let vars = HashMap::from([("a", 1), ("b", 2)]);
        let result = runner.run(custom_code, Some(vars)).await.unwrap();

        result
    });

    assert_eq!(out, "3");
}
