#![doc = include_str!("../README.md")]

use anyhow::Result;
use deno_core::{FsModuleLoader, JsRuntime, RuntimeOptions};
use std::{collections::HashMap, fmt::Display, rc::Rc, sync::Arc};

pub use deno_core::{anyhow, op};
pub use tokio::runtime::Runtime;

/// Simple permissions implementation for timers
pub struct NoTimersPermission;

impl deno_web::TimersPermission for NoTimersPermission {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

/// Deno runtime
pub struct DenoRunner {
    runtime: JsRuntime,
}

impl DenoRunner {
    pub async fn run<C, K, V>(
        mut self,
        custom_code: C,
        vars: Option<HashMap<K, V>>,
    ) -> Result<String>
    where
        C: ToString,
        K: Display,
        V: Display + std::fmt::Debug,
    {
        // Bind variable to Deno runtime
        if let Some(vars) = vars {
            for (key, value) in vars {
                self.runtime
                    .execute_script("[runner]", &format!("let {} = {:?}", key, value))?;
            }
        }

        let result = self
            .runtime
            .execute_script("code.js", &custom_code.to_string())?;

        let mut scope = self.runtime.handle_scope();

        unsafe { Ok(result.into_raw().as_ref().to_rust_string_lossy(&mut scope)) }
    }
}

pub struct Builder {
    pub ops: Vec<deno_core::OpDecl>,
}

impl Builder {
    pub fn new() -> Self {
        Self { ops: vec![] }
    }

    pub fn add_op(mut self, op: deno_core::OpDecl) -> Self {
        self.ops.push(op);
        self
    }

    pub fn build(self) -> DenoRunner {
        // Create a BlobStore for deno_web
        let blob_store = Arc::new(deno_web::BlobStore::default());

        let extensions = vec![
            // deno_webidl must be initialized before deno_web
            deno_webidl::deno_webidl::init_ops_and_esm(),
            // deno_web provides console, timers, and other Web APIs
            deno_web::deno_web::init_ops_and_esm::<NoTimersPermission>(
                blob_store,
                None, // location
            ),
            deno_core::Extension::builder("custom_ops")
                .ops(self.ops)
                .build(),
        ];

        let mut runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(FsModuleLoader)),
            extensions,
            ..Default::default()
        });

        runtime
            .execute_script("[deno:runtime.js]", include_str!("./runtime.js"))
            .unwrap();

        DenoRunner { runtime }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! gen_test {
        ($code:expr, $value:expr, $expected:expr) => {{
            let runner = Builder::default().build();
            let vars = HashMap::from([("value", $value)]);
            let actual = runner.run($code, Some(vars)).await.unwrap();

            assert_eq!($expected, actual);
        }};
    }

    #[tokio::test]
    async fn monkey_test() {
        gen_test!("value + \"hihi\"", "a", "ahihi");
        gen_test!("let a = value + \"hihi\"; a", "a", "ahihi");
        gen_test!("var out = \"hihi\"; out", "random", "hihi");
        gen_test!("var out = value; out", "hihi", "hihi");
        gen_test!("let out = parseInt(value) + 1; out", "1", "2");
        gen_test!(
            "let out = parseInt(value) + 1; out",
            "this-is-not-a-number",
            "NaN"
        );
    }

    #[tokio::test]
    async fn test_bind_string() {
        let custom_code = r#"a + b"#;

        let runner = Builder::default().build();
        let vars = HashMap::from([("a", "11"), ("b", "22")]);
        let result = runner.run(custom_code, Some(vars)).await.unwrap();

        assert_eq!(result, "1122".to_string());
    }

    #[tokio::test]
    async fn test_bind_numberic() {
        let custom_code = r#"a + b"#;

        let runner = Builder::default().build();
        let vars = HashMap::from([("a", 1), ("b", 2)]);
        let result = runner.run(custom_code, Some(vars)).await.unwrap();

        assert_eq!(result, "3");
    }

    #[tokio::test]
    async fn test_deno_console() {
        let custom_code = r#"
            console.log(value);
            console.error(value);
            value
        "#;

        let runner = Builder::default().build();
        let vars = HashMap::from([("value", "hello")]);
        let result = runner.run(custom_code, Some(vars)).await.unwrap();

        assert_eq!(result, "hello");
    }

    #[tokio::test]
    #[should_panic(expected = "ReferenceError: a is not defined")]
    async fn test_exception() {
        let custom_code = r#"
            a + 1
        "#;

        let runner = Builder::default().build();
        let vars = HashMap::from([("value", "")]);
        let _ = runner.run(custom_code, Some(vars)).await.unwrap();
    }

    #[op]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[op]
    async fn add_async(a: i32, b: i32) -> i32 {
        a + b
    }

    #[tokio::test]
    async fn test_register_rust_function_to_js_function_sync() {
        let custom_code = r#"
            const out = Deno.core.opSync('add', 1, 2);
            out
        "#;

        let runner = Builder::default().add_op(add::decl()).build();
        let vars = HashMap::from([("value", "")]);
        let result = runner.run(custom_code, Some(vars)).await.unwrap();

        assert_eq!(result, "3");
    }

    #[tokio::test]
    async fn test_register_rust_function_to_js_function_sync_global() {
        let custom_code = r#"
            const out = add(1, 2);
            out
        "#;

        let runner = Builder::default().add_op(add::decl()).build();
        let result = runner
            .run::<&str, String, String>(custom_code, None)
            .await
            .unwrap();

        assert_eq!(result, "3");
    }

    #[tokio::test]
    async fn test_register_rust_function_to_js_function_async() {
        let custom_code = r#"
            (async() => {
                return await Deno.core.opAsync('add_async', 1, 2)
            })()
        "#;

        let runner = Builder::default().add_op(add_async::decl()).build();
        let vars = HashMap::from([("value", "")]);
        let result = runner.run(custom_code, Some(vars)).await.unwrap();

        // TODO: async top-level
        assert_eq!(result, "[object Promise]");
    }

    #[tokio::test]
    async fn global_should_works() {
        let custom_code = r#"
            globalThis.a = 1;
            globalThis.a
        "#;

        let runner = Builder::default().build();
        let vars = HashMap::from([("value", "")]);
        let result = runner.run(custom_code, Some(vars)).await.unwrap();

        assert_eq!(result, "1");
    }
}
