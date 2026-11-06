# Requirements

This document records the requirements for adopting and extending the Rust workspace template.
Replace these starter requirements with application-specific requirements when creating a project.

## Functional requirements

- The workspace can contain reusable library crates under `crates/`.
- A command-line application can parse arguments with Clap and call the reusable library.
- The `tools` package provides repository-local formatting, validation, and website commands.
- Contributors can run formatting, checks, Clippy, tests, documentation, and dependency policy checks
  through the documented Cargo Make tasks.

## Non-functional requirements

The qualities below are a checklist for an adopting project, not claims that the starter already
meets production targets. Choose the applicable qualities, define a measurable target and workload,
and record how each target will be checked. Mark an item not applicable when its behavior does not
exist (for example, request latency for a CLI with no server).

| Quality                           | Requirement to define for the project                                                                                                             | Possible evidence                                                           |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| Readability and documentation     | Keep modules cohesive and public behavior understandable; document CLI usage, configuration, errors, and public APIs.                             | Review of examples, help output, API docs, and contributor setup.           |
| Testability                       | Make core behavior testable without a live service; specify unit, integration, CLI, and regression coverage for critical paths.                   | Deterministic tests, fixtures, and CI results.                              |
| Correctness and reliability       | Define valid inputs, output contracts, failure behavior, and acceptable failure rates.                                                            | Boundary tests, fault injection where useful, and incident or error trends. |
| Performance and latency           | Set a time budget for representative operations, including startup and response latency where relevant. State the input size and target hardware. | Repeatable benchmarks and end-to-end timings with a baseline.               |
| Throughput and scalability        | State expected work per second and concurrency limits when the application processes sustained or parallel work.                                  | Load tests at normal and peak demand.                                       |
| Resource efficiency               | Set limits for memory, CPU, disk, network use, and build or image size where they affect users or deployment.                                     | Measurements under representative workloads.                                |
| Security                          | Define trust boundaries, input validation, dependency policy, secret handling, and least-privilege execution.                                     | Threat review, dependency checks, and security-focused tests.               |
| Privacy and data handling         | Identify sensitive data, retention, deletion, and logging rules when the project handles user or operational data.                                | Data-flow review and retention checks.                                      |
| Observability and diagnosability  | Specify useful error messages, exit codes, logs, metrics, and traces appropriate to the deployment model. Avoid exposing secrets.                 | Failure-path tests and operational exercises.                               |
| Portability and compatibility     | List supported operating systems, architectures, Rust versions, and external tools or services.                                                   | Builds and relevant runtime tests on each supported target.                 |
| Reproducibility                   | Pin toolchains and dependencies; define how builds and generated artifacts are recreated.                                                         | Locked CI builds and documented release inputs.                             |
| Usability and accessibility       | Make help, defaults, errors, and documentation usable by the intended audience; define accessibility needs for any UI.                            | Task-based review and accessibility checks where applicable.                |
| Maintainability and extensibility | Keep package responsibilities clear and document supported extension points and upgrade expectations.                                             | Code review, dependency updates, and change exercises.                      |
| Operability and recoverability    | Define deployment, rollback, backup, and recovery procedures if the project stores state or runs as a service.                                    | Deployment rehearsal and recovery tests.                                    |

### Current template baseline

- Workspace crates inherit shared package metadata, dependencies, and lint policy from the root manifest.
- Library APIs and modules have Rust documentation, and documentation builds with warnings denied.
- Local development and CI use the same pinned Rust toolchain and locked dependency graph.
- Build artifacts and compiler cache are kept outside tracked source files.

## Constraints

- The workspace uses Rust edition 2024 and declares Rust 1.98.1 as its minimum supported version.
- The project is licensed under MIT OR Apache-2.0 and is not published by default.
- The full local CI task requires the external tools listed in `CONTRIBUTING.md`.

## Assumptions

- Application-specific behavior, package names, and deployment requirements are defined by the project adopting this template.
- Website tasks require Zola; dependency policy checks require `cargo-deny`.
