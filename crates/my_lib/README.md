# {{lib_package_name}}

Starter library with configuration, errors, domain models, and service modules.

```rust
assert_eq!({{lib_package_name | replace: "-", "_"}}::greeting("Alice"), format!("Hello, Alice from {}!", env!("CARGO_PKG_NAME")));
```

Run `cargo test -p {{lib_package_name}}`, `cargo run -p {{lib_package_name}} --example greeting`, and `cargo bench -p {{lib_package_name}}` from the workspace root.

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for the contributor workflow and [LICENSE](../../LICENSE) for terms.
