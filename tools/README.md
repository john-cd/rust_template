# tools

Development binary for this checkout. Run from anywhere inside the workspace:

```sh
cargo tools --help
cargo tools format
cargo tools check
cargo tools website
cargo tools website --serve
```

`format` and `check` dispatch to cargo-make; website commands dispatch to Zola. Commands run at the
root of the checkout that built this binary, inherit terminal input/output, and fail if the child
command fails or cannot start. This development-only binary is not intended for global installation.

See [CONTRIBUTING.md](../CONTRIBUTING.md) for required tools and [LICENSE](../LICENSE) for license terms.
