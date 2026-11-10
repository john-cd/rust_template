# Logging

The CLI uses `tracing` events. By default, it writes `info` and higher-level events to stderr and
to a daily rotating log file. Standard output stays reserved for command results.

## Configure verbosity

Set `RUST_LOG` to change the event filter. For example, from the repository root:

```sh
RUST_LOG=my_cli=debug,my_lib=debug cargo run -p my_cli -- Alice
```

Replace `my_cli` and `my_lib` with the generated crate names after adopting the template. Use
`RUST_LOG=warn` to show warnings and errors only. Invalid filter values produce a stderr diagnostic
and fall back to `info`.

## Find the log files

Unless `APP_LOG_DIR` is set, files are written under the operating system's per-user state or log
directory, in a subdirectory named after the CLI package:

- Windows: `%LOCALAPPDATA%\<package>\logs`
- macOS: `~/Library/Logs/<package>`
- Linux and other Unix targets: `$XDG_STATE_HOME/<package>/logs`, or
  `~/.local/state/<package>/logs` when `XDG_STATE_HOME` is not set to an absolute path.

Set `APP_LOG_DIR` to choose another directory. For example, in PowerShell:

```powershell
$env:APP_LOG_DIR = 'D:\temp\my_cli-logs'
cargo run -p my_cli -- Alice
```

The appender rotates files daily and retains up to seven matching log files. The file writer buffers
up to 1,024 events; when that queue is full, the producing thread waits for the writer instead of
silently dropping events. This bounds queued log data, while daily files can still grow with event
volume during a day. Use a size-rotating backend or deployment-level disk quota when the application
needs a strict byte limit. ANSI color is disabled in files. If the log directory or file cannot be
initialized, the CLI reports the issue and continues with stderr-only tracing. Output errors remain
visible on stderr when `RUST_LOG` filters their tracing event out. Keep secrets and private user data
out of tracing fields; the sample logs command names and lifecycle events, not the supplied greeting
name.
