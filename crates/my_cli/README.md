# {{project-name}}

Clap-based example binary backed by the `{{lib_package_name}}` library.

```sh
cargo run -p {{cli_package_name}} -- --help
cargo run -p {{cli_package_name}} -- Alice
```

The positional name defaults to `world`. See [CONTRIBUTING.md](../../CONTRIBUTING.md) for development
instructions and [LICENSE](../../LICENSE) for license terms.

The CLI writes tracing events to stderr and daily rotating log files. Set `RUST_LOG` to adjust
verbosity and `APP_LOG_DIR` to choose the log directory; see the [logging guide](../../docs/guides/logging.md).
