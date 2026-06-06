#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]

//! A secure and elegant JavaScript runner for Rust, powered by Deno Core.
//!
//! This library provides a safe and ergonomic way to execute JavaScript code from Rust,
//! with support for binding Rust functions to JavaScript and vice versa.
//!
//! # Security
//!
//! This library takes security seriously:
//! - All variable bindings are properly JSON-encoded to prevent code injection
//! - Variable names are validated to prevent injection attacks
//! - No unsafe code is used in the public API
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```rust
//! use deno_runner::Builder;
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let code = r#"
//!         const add = (a, b) => a + b;
//!         add(a, b)
//!     "#;
//!
//!     let runner = Builder::new().build();
//!     let vars = HashMap::from([("a", 1), ("b", 2)]);
//!
//!     let result = runner.run(code, Some(vars)).await?;
//!     assert_eq!(result, "3");
//!     Ok(())
//! }
//! ```

use deno_core::{FsModuleLoader, JsRuntime, RuntimeOptions};
use serde::Serialize;
use std::{collections::HashMap, fmt::Display, rc::Rc};
use thiserror::Error;

pub use deno_core::op;
pub use tokio::runtime::Runtime;

/// Errors that can occur when running JavaScript code.
#[derive(Error, Debug)]
pub enum RunnerError {
    /// Invalid variable name (must match [a-zA-Z_][a-zA-Z0-9_]*)
    #[error("Invalid variable name: {0}")]
    InvalidVariableName(String),

    /// JSON serialization error
    #[error("Failed to serialize variable: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// JavaScript execution error
    #[error("JavaScript execution failed: {0}")]
    ExecutionError(#[from] anyhow::Error),
}

/// Result type for runner operations.
pub type Result<T> = std::result::Result<T, RunnerError>;

/// Validates that a variable name is safe to use in JavaScript.
///
/// Variable names must match the pattern `[a-zA-Z_][a-zA-Z0-9_]*` to prevent
/// code injection attacks.
fn is_valid_variable_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut chars = name.chars();
    let first = chars.next().unwrap();

    // First character must be a letter or underscore
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }

    // Remaining characters must be alphanumeric or underscore
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// A JavaScript runtime powered by Deno Core.
///
/// This struct provides a secure way to execute JavaScript code with optional
/// variable bindings and Rust function integration.
///
/// # Examples
///
/// ```rust
/// use deno_runner::Builder;
/// use std::collections::HashMap;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let runner = Builder::new().build();
/// let vars = HashMap::from([("x", 42)]);
/// let result = runner.run("x * 2", Some(vars)).await?;
/// assert_eq!(result, "84");
/// # Ok(())
/// # }
/// ```
pub struct DenoRunner {
    runtime: JsRuntime,
}

impl DenoRunner {
    /// Execute JavaScript code with optional variable bindings.
    ///
    /// # Arguments
    ///
    /// * `custom_code` - The JavaScript code to execute
    /// * `vars` - Optional HashMap of variables to bind in the JavaScript context
    ///
    /// # Returns
    ///
    /// The string representation of the JavaScript execution result.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A variable name is invalid (contains unsafe characters)
    /// - A variable value cannot be serialized to JSON
    /// - The JavaScript code fails to execute
    ///
    /// # Security
    ///
    /// Variable names are validated to match `[a-zA-Z_][a-zA-Z0-9_]*` pattern.
    /// Values are JSON-encoded to prevent code injection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deno_runner::Builder;
    /// use std::collections::HashMap;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let runner = Builder::new().build();
    /// let vars = HashMap::from([("name", "World")]);
    /// let result = runner.run("'Hello, ' + name", Some(vars)).await?;
    /// assert_eq!(result, "Hello, World");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run<C, K, V>(
        mut self,
        custom_code: C,
        vars: Option<HashMap<K, V>>,
    ) -> Result<String>
    where
        C: ToString,
        K: Display,
        V: Serialize,
    {
        // Bind variables to Deno runtime with proper security checks
        if let Some(vars) = vars {
            for (key, value) in vars {
                let key_str = key.to_string();

                // Validate variable name to prevent injection
                if !is_valid_variable_name(&key_str) {
                    return Err(RunnerError::InvalidVariableName(key_str));
                }

                // Safely serialize value as JSON
                let value_json = serde_json::to_string(&value)?;

                // Create safe variable binding
                let binding = format!("const {} = {};", key_str, value_json);
                self.runtime.execute_script("[runner:bindings]", &binding)?;
            }
        }

        // Execute the user code
        let result = self
            .runtime
            .execute_script("[runner:code]", &custom_code.to_string())?;

        // Convert result to string
        let mut scope = self.runtime.handle_scope();
        let result_str = result.open(&mut scope).to_rust_string_lossy(&mut scope);

        Ok(result_str)
    }
}

/// Builder for creating a `DenoRunner` instance.
///
/// The builder pattern allows for flexible configuration of the JavaScript runtime,
/// including registering custom Rust operations that can be called from JavaScript.
///
/// # Examples
///
/// ```rust
/// use deno_runner::{Builder, op};
///
/// #[op]
/// fn greet(name: String) -> String {
///     format!("Hello, {}!", name)
/// }
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let runner = Builder::new()
///     .add_op(greet::decl())
///     .build();
///
/// let result = runner.run("greet('World')", None::<HashMap<String, String>>).await?;
/// assert_eq!(result, "Hello, World!");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default)]
pub struct Builder {
    ops: Vec<deno_core::OpDecl>,
}

impl Builder {
    /// Create a new builder with default configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deno_runner::Builder;
    ///
    /// let builder = Builder::new();
    /// ```
    pub fn new() -> Self {
        Self { ops: vec![] }
    }

    /// Add a Rust operation that can be called from JavaScript.
    ///
    /// # Arguments
    ///
    /// * `op` - An operation declaration created using the `#[op]` macro
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deno_runner::{Builder, op};
    ///
    /// #[op]
    /// fn add(a: i32, b: i32) -> i32 {
    ///     a + b
    /// }
    ///
    /// let runner = Builder::new()
    ///     .add_op(add::decl())
    ///     .build();
    /// ```
    pub fn add_op(mut self, op: deno_core::OpDecl) -> Self {
        self.ops.push(op);
        self
    }

    /// Build the `DenoRunner` instance.
    ///
    /// This creates a new JavaScript runtime with all registered operations
    /// and initializes the runtime environment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deno_runner::Builder;
    ///
    /// let runner = Builder::new().build();
    /// ```
    pub fn build(self) -> DenoRunner {
        let extensions = vec![
            deno_console::init(),
            deno_core::Extension::builder("deno_runner")
                .ops(self.ops)
                .build(),
        ];

        let mut runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(FsModuleLoader)),
            extensions,
            ..Default::default()
        });

        // Initialize the runtime with helper functions
        runtime
            .execute_script("[deno:runtime.js]", include_str!("./runtime.js"))
            .expect("Failed to initialize runtime");

        DenoRunner { runtime }
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
    async fn test_bind_numeric() {
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
    async fn test_exception() {
        let custom_code = r#"
            undefinedVariable + 1
        "#;

        let runner = Builder::default().build();
        let vars = HashMap::from([("value", "")]);
        let result = runner.run(custom_code, Some(vars)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_variable_name() {
        let runner = Builder::default().build();
        let vars = HashMap::from([("invalid-name", 42)]);
        let result = runner.run("invalid_name", Some(vars)).await;

        assert!(matches!(result, Err(RunnerError::InvalidVariableName(_))));
    }

    #[tokio::test]
    async fn test_code_injection_prevention() {
        let runner = Builder::default().build();
        // Attempt to inject malicious code via variable name
        let vars = HashMap::from([("x; alert('hacked'); let y", 1)]);
        let result = runner.run("x", Some(vars)).await;

        assert!(matches!(result, Err(RunnerError::InvalidVariableName(_))));
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
            const out = Deno.core.ops.add(1, 2);
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
                return await Deno.core.ops.add_async(1, 2)
            })()
        "#;

        let runner = Builder::default().add_op(add_async::decl()).build();
        let vars = HashMap::from([("value", "")]);
        let result = runner.run(custom_code, Some(vars)).await.unwrap();

        // Note: Top-level await is not yet supported
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

    #[test]
    fn test_variable_name_validation() {
        assert!(is_valid_variable_name("validName"));
        assert!(is_valid_variable_name("_private"));
        assert!(is_valid_variable_name("camelCase123"));
        assert!(is_valid_variable_name("CONSTANT_VALUE"));

        assert!(!is_valid_variable_name(""));
        assert!(!is_valid_variable_name("123invalid"));
        assert!(!is_valid_variable_name("invalid-name"));
        assert!(!is_valid_variable_name("invalid.name"));
        assert!(!is_valid_variable_name("invalid name"));
        assert!(!is_valid_variable_name("invalid;name"));
        assert!(!is_valid_variable_name("invalid\"name"));
        assert!(!is_valid_variable_name("invalid'name"));
    }
}
