# Release Checklist for v0.2.0

This checklist ensures a smooth and professional release process.

## Pre-Release Checks

### Code Quality
- [x] All tests passing locally
- [x] No clippy warnings
- [x] Code formatted with rustfmt
- [x] Documentation complete and accurate
- [x] Examples tested and working
- [x] Benchmarks running successfully

### Security
- [x] Security vulnerability fixed and tested
- [x] No unsafe code in public API
- [x] Input validation comprehensive
- [x] SECURITY.md reviewed and accurate
- [x] cargo-audit clean
- [x] cargo-deny passing

### Documentation
- [x] README.md complete with examples
- [x] CHANGELOG.md updated with all changes
- [x] API documentation complete
- [x] All examples documented
- [x] CONTRIBUTING.md clear and helpful
- [x] CODE_OF_CONDUCT.md in place
- [x] Migration guide for breaking changes

### Package Metadata
- [x] Cargo.toml version updated to 0.2.0
- [x] Keywords and categories appropriate
- [x] License specified correctly
- [x] Repository and homepage URLs correct
- [x] Authors list updated
- [x] MSRV (1.70.0) specified

### CI/CD
- [x] All workflows passing
- [x] Cross-platform tests (Ubuntu, macOS, Windows)
- [x] MSRV testing configured
- [x] Security scanning automated
- [x] Code coverage tracking

## Release Process

### 1. Final Testing
```bash
# Clean build
cargo clean
cargo build --release

# Run all tests
cargo test --all-features

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Format check
cargo fmt --all -- --check

# Run examples
cargo run --example basic
cargo run --example rust_functions
cargo run --example error_handling
cargo run --example advanced

# Run benchmarks
cargo bench

# Build documentation
cargo doc --no-deps --all-features

# Test documentation examples
cargo test --doc
```

### 2. Version Update
- [x] Update version in Cargo.toml
- [x] Update CHANGELOG.md
  - Move "Unreleased" to "0.2.0 - YYYY-MM-DD"
  - Add comparison links
- [ ] Create version git tag

### 3. Git Operations
```bash
# Ensure all changes committed
git status

# Create annotated tag
git tag -a v0.2.0 -m "Release v0.2.0 - Security and Quality Update"

# Push tag
git push origin v0.2.0
```

### 4. Create GitHub Release
- [ ] Go to GitHub releases page
- [ ] Draft new release
- [ ] Select tag v0.2.0
- [ ] Title: "v0.2.0 - Security and Quality Update"
- [ ] Description: Copy from CHANGELOG.md
- [ ] Attach release notes
- [ ] Mark as latest release
- [ ] Publish release

### 5. Publish to crates.io
```bash
# Dry run first
cargo publish --dry-run

# Actually publish
cargo publish
```

### 6. Post-Release
- [ ] Verify crates.io page looks correct
- [ ] Verify docs.rs builds successfully
- [ ] Update main branch if needed
- [ ] Announce on:
  - [ ] GitHub discussions
  - [ ] Project blog (if exists)
  - [ ] Social media (if applicable)
  - [ ] Rust community forums (if appropriate)

## Communication Template

### GitHub Release Notes
```markdown
# 🎉 deno_runner v0.2.0

A major security and quality update with comprehensive improvements across all aspects of the library.

## 🔒 Critical Security Fix

**IMPORTANT**: This release fixes a critical code injection vulnerability in variable binding. All users should upgrade immediately.

## ✨ Highlights

- Fixed critical security vulnerability in variable binding
- Comprehensive API documentation with examples
- 4 detailed usage examples
- Modern CI/CD with cross-platform testing
- Security scanning and automated auditing
- Professional project infrastructure

## 📦 Installation

\`\`\`toml
[dependencies]
deno_runner = "0.2"
\`\`\`

## 💥 Breaking Changes

This release includes breaking changes. See the [Migration Guide](CHANGELOG.md) for details.

## 📚 Documentation

- [Documentation](https://docs.rs/deno_runner)
- [Examples](examples/)
- [Security Policy](SECURITY.md)
- [Contributing Guide](CONTRIBUTING.md)

## 🙏 Acknowledgments

Thanks to all contributors and the Deno Core team!

---

**Full Changelog**: [v0.1.0...v0.2.0](https://github.com/duyet/deno-runner-rs/compare/v0.1.0...v0.2.0)
```

### Social Media Announcement
```
🦀 deno_runner v0.2.0 is out!

🔒 Critical security fix
📚 Comprehensive docs
✨ 4 detailed examples
🧪 Full test coverage
🚀 Cross-platform CI

Check it out: https://crates.io/crates/deno_runner

#rustlang #deno #javascript
```

## Verification

### After Publishing
- [ ] crates.io page displays correctly
- [ ] docs.rs builds successfully
- [ ] All badges in README work
- [ ] Links in documentation work
- [ ] Examples can be copied and run
- [ ] Issue templates work
- [ ] PR template displays correctly

## Rollback Plan

If issues are discovered after release:

1. **Minor Issues**: Create v0.2.1 patch release
2. **Critical Issues**:
   - Yank v0.2.0 from crates.io
   - Fix issue
   - Release v0.2.1 immediately
   - Notify users via GitHub

## Notes

- MSRV: 1.70.0
- License: MIT
- Platform support: Linux, macOS, Windows
- Stability: Production ready

---

**Prepared by**: Claude (AI Assistant)
**Date**: 2025-11-19
**Release Manager**: @duyet
