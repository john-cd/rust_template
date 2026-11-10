# {{project-name}}

Clap-based example binary backed by the `{{crate_name}}_lib` library.

```sh
cargo run -p {{project-name}} -- --help
cargo run -p {{project-name}} -- Alice
```

The positional name defaults to `world`. See [CONTRIBUTING.md](../../CONTRIBUTING.md) for development
instructions and [LICENSE](../../LICENSE) for license terms.

The CLI writes tracing events to stderr and daily rotating log files. Set `RUST_LOG` to adjust
verbosity and `APP_LOG_DIR` to choose the log directory; see the [logging guide](../../docs/guides/logging.md).
