# Contributing to deno_runner

First off, thank you for considering contributing to deno_runner! It's people like you that make this library better for everyone.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [How Can I Contribute?](#how-can-i-contribute)
- [Style Guidelines](#style-guidelines)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)
- [Documentation](#documentation)

## Code of Conduct

This project adheres to a Code of Conduct that all contributors are expected to follow. Please be respectful and constructive in all interactions.

### Our Standards

- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

## Getting Started

### Prerequisites

- Rust 1.70.0 or later (MSRV)
- Git
- Familiarity with Rust and async programming
- Basic understanding of JavaScript/V8

### Setting Up Development Environment

1. **Fork the repository**

   ```bash
   # Fork on GitHub, then clone your fork
   git clone https://github.com/YOUR_USERNAME/deno-runner-rs.git
   cd deno-runner-rs
   ```

2. **Add upstream remote**

   ```bash
   git remote add upstream https://github.com/duyet/deno-runner-rs.git
   ```

3. **Build the project**

   ```bash
   cargo build
   ```

4. **Run tests**

   ```bash
   cargo test
   ```

5. **Check code quality**

   ```bash
   cargo clippy --all-targets --all-features
   cargo fmt --all -- --check
   ```

## Development Process

1. **Create a branch**

   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

2. **Make your changes**
   - Write code
   - Add tests
   - Update documentation
   - Run tests locally

3. **Commit your changes**

   ```bash
   git add .
   git commit -m "feat: add amazing feature"
   ```

4. **Keep your fork updated**

   ```bash
   git fetch upstream
   git rebase upstream/master
   ```

5. **Push to your fork**

   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**
   - Go to GitHub and create a PR from your fork

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

When reporting bugs, include:

- **Clear title and description**
- **Steps to reproduce**
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version, etc.)
- **Code sample** (if applicable)
- **Stack trace** (if applicable)

Example:

```markdown
**Environment:**
- OS: Ubuntu 22.04
- Rust: 1.75.0
- deno_runner: 0.2.0

**Steps to Reproduce:**
1. Create a runner with...
2. Execute code...
3. Error occurs...

**Expected Behavior:**
Should return...

**Actual Behavior:**
Returns error...

**Code Sample:**
\```rust
// Minimal reproducible example
\```
```

### Suggesting Enhancements

Enhancement suggestions are welcome! Please include:

- **Clear use case**
- **Proposed API** (if applicable)
- **Examples** of how it would be used
- **Alternatives considered**
- **Impact on existing code**

### Pull Requests

Good pull requests (patches, improvements, new features) are fantastic! Please:

1. **Keep the scope focused** - one feature/fix per PR
2. **Include tests** - maintain or improve code coverage
3. **Update documentation** - keep docs in sync with code
4. **Follow style guidelines** - consistent code is happy code
5. **Add changelog entry** - help users know what changed

## Style Guidelines

### Rust Code Style

- **Follow rustfmt**: Run `cargo fmt` before committing
- **Follow clippy**: Run `cargo clippy` and fix warnings
- **Use descriptive names**: Clear is better than clever
- **Add documentation**: Public items need doc comments
- **Prefer explicit**: Avoid implicit behaviors

### Documentation Style

- **Use doc comments** (`///` for items, `//!` for modules)
- **Include examples** in doc comments
- **Document errors** in `# Errors` section
- **Document safety** in `# Safety` section (for unsafe code)
- **Keep it concise** but complete

Example:

```rust
/// Executes JavaScript code with optional variable bindings.
///
/// # Arguments
///
/// * `code` - The JavaScript code to execute
/// * `vars` - Optional variables to bind in the JS context
///
/// # Returns
///
/// The string representation of the execution result.
///
/// # Errors
///
/// Returns an error if:
/// - A variable name is invalid
/// - The JavaScript code fails to execute
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
pub async fn run<C, K, V>(...) -> Result<String> {
    // Implementation
}
```

## Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Code style changes (formatting, etc.)
- **refactor**: Code changes that neither fix bugs nor add features
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Changes to build process or auxiliary tools
- **security**: Security-related changes

### Examples

```
feat(builder): add timeout configuration option

Add ability to configure execution timeout via builder pattern.
This allows users to prevent infinite loops and long-running code.

Closes #123
```

```
fix(security): prevent code injection via variable names

Variable names are now validated against a whitelist pattern to
prevent malicious code injection.

BREAKING CHANGE: Invalid variable names now return an error instead
of panicking.
```

## Pull Request Process

1. **Update documentation** - README, API docs, examples, CHANGELOG
2. **Add tests** - maintain or improve coverage
3. **Run checks** - tests, clippy, fmt
4. **Update CHANGELOG.md** - add entry under "Unreleased"
5. **Ensure CI passes** - all checks must be green
6. **Request review** - assign or mention maintainers
7. **Address feedback** - respond to review comments
8. **Squash commits** - clean up history if needed

### PR Checklist

- [ ] Tests added/updated and passing
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Code follows style guidelines
- [ ] No clippy warnings
- [ ] Code is formatted (`cargo fmt`)
- [ ] Commits follow conventional commits format
- [ ] PR description explains changes

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run tests in a specific file
cargo test --test integration_test
```

### Writing Tests

- **Unit tests**: Place in the same file as the code
- **Integration tests**: Place in `tests/` directory
- **Doc tests**: Include in documentation comments
- **Test coverage**: Aim for high coverage, especially for critical code

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feature() {
        // Arrange
        let runner = Builder::new().build();

        // Act
        let result = runner.run("1 + 1", None::<HashMap<String, i32>>).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2");
    }
}
```

## Documentation

### Building Documentation

```bash
# Build docs
cargo doc

# Build and open docs
cargo doc --open

# Build docs with private items
cargo doc --document-private-items
```

### Documentation Requirements

- All public items must have documentation
- Include at least one example for public APIs
- Document error conditions
- Explain complex logic with comments
- Keep examples runnable and tested

## Questions?

Feel free to:

- Open an issue with the `question` label
- Start a discussion on GitHub Discussions
- Reach out to maintainers

## Recognition

Contributors will be recognized in:

- CHANGELOG.md (for significant contributions)
- GitHub contributors page
- Release notes

---

Thank you for contributing to deno_runner! 🦀✨
