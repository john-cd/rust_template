# {{project-name}}_lib

Starter library with configuration, errors, domain models, and service modules.

```rust
assert_eq!({{crate_name}}_lib::greeting("Alice"), format!("Hello, Alice from {}!", env!("CARGO_PKG_NAME")));
```

Run `cargo test -p {{project-name}}_lib`, `cargo run -p {{project-name}}_lib --example greeting`, and `cargo bench -p {{project-name}}_lib` from the workspace root.

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for the contributor workflow and [LICENSE](../../LICENSE) for terms.
