# Security Policy

## Our Commitment to Security

Security is a top priority for deno_runner. This library executes JavaScript code within Rust applications, which requires careful attention to security considerations. We take all security vulnerabilities seriously and appreciate your efforts to responsibly disclose your findings.

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| 0.1.x   | :x:                |

## Security Features

### Current Security Measures

1. **Variable Name Validation**
   - All variable names are validated against the pattern `[a-zA-Z_][a-zA-Z0-9_]*`
   - This prevents code injection via malicious variable names
   - Invalid names return a `RunnerError::InvalidVariableName`

2. **Safe Value Serialization**
   - All values are serialized as JSON before being passed to JavaScript
   - This prevents string-based code injection attacks
   - Uses the `serde` and `serde_json` crates for reliable serialization

3. **No Unsafe Code in Public API**
   - The library enforces `#![deny(unsafe_code)]` at the crate level
   - Internal usage of unsafe code is minimized and carefully reviewed
   - All public APIs are safe to use

4. **Comprehensive Error Handling**
   - Proper error types using `thiserror`
   - JavaScript errors are safely propagated to Rust
   - No panics in normal operation

### Known Limitations

Please be aware of these security considerations:

1. **No Execution Timeout**
   - Currently, there is no built-in timeout mechanism
   - Long-running or infinite loop JavaScript code can block indefinitely
   - Users should implement their own timeout logic using `tokio::time::timeout`

2. **No Memory Limits**
   - The JavaScript runtime does not have enforced memory limits
   - Malicious or buggy code could consume excessive memory
   - Consider implementing resource monitoring in production environments

3. **File System Access**
   - The runtime uses `FsModuleLoader`, which allows file system access
   - This is necessary for module loading but could be a security concern
   - Ensure the runtime operates in a controlled environment

4. **Sandboxing**
   - This library does not provide OS-level sandboxing
   - JavaScript code runs with the same permissions as the Rust process
   - For untrusted code execution, consider additional sandboxing (containers, VMs)

## Reporting a Vulnerability

We greatly appreciate security researchers and users who report vulnerabilities to us.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please send an email to: **[me@duyet.net](mailto:me@duyet.net)**

Include the following information:

- Type of vulnerability
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if available)
- Impact of the issue, including how an attacker might exploit it

### What to Expect

1. **Acknowledgment**: We will acknowledge your email within 48 hours
2. **Investigation**: We will investigate and validate the vulnerability
3. **Updates**: We will keep you informed about our progress
4. **Fix**: We will work on a fix and prepare a security release
5. **Disclosure**: We will coordinate public disclosure with you
6. **Credit**: We will credit you in the security advisory (if desired)

### Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Target**: Within 30 days for critical issues, 90 days for others

## Security Best Practices

When using deno_runner in your application:

### 1. Validate Input

```rust
use deno_runner::Builder;
use std::collections::HashMap;

async fn run_user_code(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Validate code length
    if code.len() > 10_000 {
        return Err("Code too long".into());
    }

    // Validate code doesn't contain suspicious patterns
    if code.contains("eval(") || code.contains("Function(") {
        return Err("Potentially unsafe code".into());
    }

    let runner = Builder::new().build();
    Ok(runner.run::<_, String, i32>(code, None).await?)
}
```

### 2. Implement Timeouts

```rust
use deno_runner::Builder;
use tokio::time::{timeout, Duration};

async fn run_with_timeout(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let runner = Builder::new().build();

    // Set a 5-second timeout
    match timeout(
        Duration::from_secs(5),
        runner.run::<_, String, i32>(code, None)
    ).await {
        Ok(result) => Ok(result?),
        Err(_) => Err("Execution timeout".into()),
    }
}
```

### 3. Limit Variable Scope

```rust
use deno_runner::Builder;
use std::collections::HashMap;

async fn run_safely() -> Result<String, Box<dyn std::error::Error>> {
    let runner = Builder::new().build();

    // Only pass necessary variables
    let vars = HashMap::from([
        ("input", "safe_value"),
    ]);

    Ok(runner.run("input.toUpperCase()", Some(vars)).await?)
}
```

### 4. Monitor Resource Usage

Consider monitoring:
- Execution time
- Memory consumption
- CPU usage
- Number of concurrent executions

### 5. Run in Isolated Environment

For untrusted code:
- Use containers (Docker, etc.)
- Consider WebAssembly sandboxing
- Implement OS-level sandboxing
- Run with minimal permissions

## Security Update Policy

- **Critical vulnerabilities**: Patch released within 48 hours
- **High severity**: Patch released within 7 days
- **Medium severity**: Patch released within 30 days
- **Low severity**: Included in next regular release

## Acknowledgments

We would like to thank the following individuals for responsibly disclosing security vulnerabilities:

- *None yet - be the first!*

## Additional Resources

- [Deno Security](https://deno.land/manual/runtime/security)
- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)

---

**Last Updated**: 2024-01-01
**Version**: 1.0
