# TODO Reviewed ??-??-????

ARG RUST_VERSION
ARG DEBIAN_VERSION

# Development: editor tooling and a non-root workspace user.
FROM rust:${RUST_VERSION}-${DEBIAN_VERSION} AS development

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates git sudo pkg-config libssl-dev \
    && groupadd --gid 1000 vscode \
    && useradd --uid 1000 --gid vscode --create-home vscode \
    && echo "vscode ALL=(root) NOPASSWD:ALL" > "/etc/sudoers.d/vscode" \
    && chmod 0440 "/etc/sudoers.d/vscode" \
    && rm -rf /var/lib/apt/lists/*

ARG RUSTFMT_VERSION
ARG CARGO_MAKE_VERSION
ARG CARGO_BINSTALL_VERSION
ARG CARGO_NEXTEST_VERSION
ARG CARGO_DENY_VERSION
ARG CARGO_MACHETE_VERSION
ARG CARGO_LLVM_COV_VERSION
ARG ZIZMOR_VERSION
ARG SCCACHE_VERSION
ARG DPRINT_VERSION
ARG LYCHEE_VERSION
ARG ZOLA_VERSION

RUN rustup component add clippy rustfmt \
    && rustup component add llvm-tools-preview \
    && rustup toolchain install "${RUSTFMT_VERSION}" --profile minimal --component rustfmt \
    && cargo install --locked cargo-binstall --version "${CARGO_BINSTALL_VERSION}" \
    && cargo binstall --locked --no-confirm \
        "cargo-make@${CARGO_MAKE_VERSION}" \
        "cargo-nextest@${CARGO_NEXTEST_VERSION}" \
        "cargo-deny@${CARGO_DENY_VERSION}" \
        "cargo-machete@${CARGO_MACHETE_VERSION}" \
        "cargo-llvm-cov@${CARGO_LLVM_COV_VERSION}" \
        "zizmor@${ZIZMOR_VERSION}" \
        "dprint@${DPRINT_VERSION}" \
        "lychee@${LYCHEE_VERSION}" \
        "sccache@${SCCACHE_VERSION}" \
        "zola@${ZOLA_VERSION}"

USER vscode

# Chef: install the dependency planner for production builds.
FROM rust:${RUST_VERSION}-${DEBIAN_VERSION} AS chef
ARG CARGO_CHEF_VERSION
RUN cargo install --locked cargo-chef --version "${CARGO_CHEF_VERSION}"
WORKDIR /app

# Planner: capture the workspace dependency recipe.
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder: cache dependencies, then compile the sample CLI.
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked --package my_cli --bin my_cli

# Production: run the CLI without the Rust build toolchain.
FROM debian:${DEBIAN_VERSION}-slim AS production
RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && useradd --system --no-create-home --uid 10001 app \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/my_cli /usr/local/bin/my_cli
USER app
ENTRYPOINT ["/usr/local/bin/my_cli"]
