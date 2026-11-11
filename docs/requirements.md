# Requirements

Use this document as a starting point for the requirements of your project.
Replace the example below with requirements for the real application.
A requirement should describe one behavior or quality, name its scope, and have a way to tell whether it is met.

## Example application: a local todo CLI

This fictional, single-user command-line application illustrates the level of detail to aim for.
It is an example only; the starter's greeting CLI does not implement these features.

| ID | Functional requirement | Acceptance example |
| --- | --- | --- |
| FR-01 | `todo add <title>` creates an open task and prints its stable ID. | A whitespace-only title is rejected without changing the task list. |
| FR-02 | `todo list` shows each task's ID, title, and status in creation order. | An empty list prints a clear empty-state message. |
| FR-03 | `todo complete <id>` and `todo reopen <id>` change the selected task's status; repeating the current status is idempotent. | An unknown ID returns a non-zero exit code and leaves stored tasks unchanged. |
| FR-04 | `todo delete <id>` removes the selected task. | Deleting one task leaves every other task unchanged; an unknown ID returns non-zero without changing the list. |
| FR-05 | Every successful task change persists between command invocations. | Added, completed, reopened, and deleted tasks have the expected state in the next `todo list` invocation. |

The example assumes local storage and one user. Accounts, synchronization, due dates, reminders, and collaboration are out of scope unless the adopting project adds requirements for them.

## Non-functional requirements

Quality attributes are constraints on how the application behaves.
For each applicable attribute, define the workload or environment, a target, and how it will be measured.
Remove attributes that do not apply and add domain-specific ones as needed.

| Quality attribute | Requirement to define | Possible evidence |
| --- | --- | --- |
| Availability and continuity | Uptime or offline availability, maintenance windows, and behavior when a dependency is unavailable. | Availability measurements and dependency-failure exercises. |
| Compatibility and portability | Supported operating systems, architectures, Rust versions, file systems, and interface compatibility. | Builds and runtime checks on each supported target. |
| Configurability | Supported configuration sources, precedence, defaults, validation, and behavior for invalid settings. | Precedence tests and documented examples. |
| Deployability and installability | Packaging format, installation and upgrade steps, prerequisites, rollback, and uninstall behavior. | Clean-machine install, upgrade, and rollback exercises. |
| Interoperability | Data formats, protocols, command exit codes, and compatibility with other tools. | Contract tests and compatibility fixtures. |
| Maintainability, modifiability, and extensibility | Module boundaries, public API stability, extension points, and expected change cost. | Design review and representative change exercises. |
| Observability and supportability | Actionable errors, logs, metrics, traces, and support information appropriate to the deployment model. | Failure-path checks and operational exercises; verify secrets are not exposed. |
| Performance and latency | Time budgets for representative operations, with input sizes and reference hardware stated. | Repeatable benchmarks and end-to-end timings against a baseline. |
| Reliability and correctness | Input/output contracts, failure behavior, acceptable error rates, and consistency guarantees. | Boundary, regression, and fault-injection tests where useful. |
| Operability, recoverability, and durability | Routine operation, support procedures, data retention, backup, restore, and behavior after interruption or partial failure. | Runbook and restore rehearsals; interrupted-operation tests. |
| Resource efficiency | Memory, CPU, disk, network, and build or image-size budgets where they affect users or deployment. | Measurements under representative workloads. |
| Scalability and capacity | Expected data volume, throughput, concurrency, and peak demand. | Load tests at normal and peak capacity. |
| Security | Trust boundaries, authorization, input validation, dependency policy, secret handling, and least privilege. | Threat review, dependency checks, and security-focused tests. |
| Auditability | Changes and privileged operations that must be attributable, retained, and reviewable. | Audit-log review and retention checks. |
| Privacy and data handling | Sensitive data, collection, logging, retention, deletion, and sharing rules. | Data-flow review and retention or deletion checks. |
| Testability | Behaviors that must be testable without live services, plus required unit, integration, CLI, and regression coverage. | Deterministic tests, fixtures, and CI results. |
| Usability and accessibility | Intended users, task completion, help and error quality, keyboard or screen-reader needs, and UI accessibility where relevant. | Task-based review and accessibility checks where applicable. |
| Reproducibility | How toolchains, dependencies, builds, and generated artifacts are pinned and recreated. | Locked CI builds and documented release inputs. |
| Documentation | Required user, operator, API, and contributor guidance, and how examples stay current. | Documentation review and executable examples where practical. |

### Example quality targets for the todo CLI

These sample targets make the qualities concrete. Replace the thresholds, hardware, and platform
list with values agreed for the adopting project.

- **Offline availability:** After installation, add, list, complete, reopen, and delete work without
 network access.
- **Performance:** On the project's named reference machine, the 95th-percentile duration for
 `todo list` is below 250 ms with 10,000 stored tasks.
- **Durability:** A successful mutation survives process restart. An interrupted write leaves either
 the previous complete task file or the new complete task file, never a partially written file.
- **Privacy and security:** Tasks remain in a local file; the application sends no task data over the
 network and does not include task titles in diagnostic output.
- **Interoperability:** Successful commands exit with status 0; invalid input and missing IDs exit
 non-zero, with diagnostics on stderr and task-list output on stdout.
- **Usability and accessibility:** Every operation is available from the keyboard, help shows
 examples for each command, and status is expressed in text rather than color alone.
- **Portability:** Release artifacts support the operating systems and architectures explicitly
 listed by the project; the same command and storage semantics pass on every listed target.

## Current template baseline

- Workspace crates inherit shared package metadata, dependencies, and lint policy from the root manifest.
- Library APIs and modules have Rust documentation, and documentation builds with warnings denied.
- The development toolchain is pinned. CI also checks the declared minimum supported Rust version
 separately, using the locked dependency graph.
- The sample CLI emits tracing events to stderr and daily rotating per-user log files; it keeps
 standard output for command results.
- The sample configuration reader limits TOML input to 1 MiB, and configuration saves use a staged
 atomic replacement with operation and path context on errors.
- Build artifacts and compiler cache are kept outside tracked source files.

## Constraints

- The workspace uses Rust edition 2024 and declares Rust 1.98.1 as its minimum supported version.
 The normal development toolchain is pinned separately in `rust-toolchain.toml`.
- The project is licensed under MIT OR Apache-2.0 and is not published by default.
- The full local CI task requires the external tools listed in `CONTRIBUTING.md`.
- Website tasks require Zola; dependency policy checks require `cargo-deny`.
