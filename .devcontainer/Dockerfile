# TODO Reviewed ??-??-????

ARG RUST_VERSION
FROM rust:${RUST_VERSION}-trixie

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
ARG SCCACHE_VERSION
ARG DPRINT_VERSION
ARG LYCHEE_VERSION
ARG ZOLA_VERSION

RUN rustup component add clippy rustfmt \
    && rustup toolchain install "${RUSTFMT_VERSION}" --profile minimal --component rustfmt \
    && cargo install --locked cargo-binstall --version "${CARGO_BINSTALL_VERSION}" \
    && cargo binstall --locked --no-confirm \
        "cargo-make@${CARGO_MAKE_VERSION}" \
        "cargo-nextest@${CARGO_NEXTEST_VERSION}" \
        "cargo-deny@${CARGO_DENY_VERSION}" \
        "dprint@${DPRINT_VERSION}" \
        "lychee@${LYCHEE_VERSION}" \
        "sccache@${SCCACHE_VERSION}" \
        "zola@${ZOLA_VERSION}"

USER vscode
