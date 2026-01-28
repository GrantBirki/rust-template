# AGENTS.md

This repository is a Rust template optimized for hermetic, reproducible, air-gapped builds. Treat it as a foundation for services, CLIs, and libraries that must pass the airplane test.

## North-Star Principles

- Hermetic by default: builds and tests must succeed with no network access.
- Dependencies are explicit, pinned, vendored, and frozen in time.
- Scripts to rule them all: standardized entrypoints for every dev/CI task.
- Write it down, work in the open, and prefer durable URLs for decisions.

## Non-Negotiables (Policy)

- No network calls during build/test/lint. If a tool needs the network, it must be moved to `script/update` (online-only) or preinstalled.
- Release CI is allowed to use `script/install-zig` to fetch pinned Zig + cargo-zigbuild (hermetic enough for releases).
- All dependencies are pinned to exact versions in `Cargo.toml` (use `=x.y.z`).
- `Cargo.lock` is committed and treated as source of truth.
- Vendored dependencies live in `vendor/cache` and are required for all builds.
- `rust-toolchain.toml`, `.rust-version`, `.zig-version`, and `.cargo-zigbuild-version` must stay in sync with actual toolchain/tool versions.
- CI should run air-gapped. Hosted runners that download toolchains are not hermetic (release jobs are the only exception via `script/install-zig`).

## Scripts to Rule Them All

All scripts are in `script/` and are offline-first.

- `script/bootstrap`: validates the toolchain and vendor cache, then performs a frozen build check.
- `script/update`: **online-only** dependency update + re-vendor. Use this to refresh `Cargo.lock` and `vendor/cache`.
- `script/install-zig`: **online-only** CI helper that installs pinned Zig + cargo-zigbuild for cross-target releases. Accepts `RUSTUP_TARGETS` to install extra targets.
- `script/test`: runs tests; `--cov` requires preinstalled `cargo-tarpaulin`, `jq`, and `bc`.
- `script/lint`: format + clippy + docs; treats warnings as errors.
- `script/build`: builds release binaries (host by default). Use `--release` with `--targets` for dist packaging; supports `--universal-darwin`.
- `script/server`: runs the app/CLI via `cargo run --frozen`.
- `script/release`: tags and pushes a new release.

## Toolchain + Version Managers

- Rust toolchain is pinned in `rust-toolchain.toml` and mirrored in `.rust-version` (they must match).
- Cross-compile tooling is pinned in `.zig-version` and `.cargo-zigbuild-version`.
- Prefer shim-based version managers (mise/asdf/rustup) but **no auto-downloads** in scripts.

## Hermetic Builds (Airplane Test)

All daily workflows must pass without the network:

1. Clone the repo
2. Run `script/bootstrap`
3. Run `script/test`
4. Run `script/build`

If any step requires network access, the build is not hermetic.

## Dependency Vendoring

- Vendored crates live in `vendor/cache`.
- Cargo is configured to use vendored sources in `.cargo/config.toml` with `net.offline = true`.
- `script/update` is the only place that may touch the network for Cargo crates.
- Every dependency update must commit:
  - `Cargo.lock`
  - `vendor/cache/` changes

## CI Expectations

CI must be air-gapped for build/test/lint:

- Prefer self-hosted runners or pre-baked images with Rust, clippy, rustfmt, zig, and cargo-zigbuild preinstalled for full hermeticity.
- Do not download toolchains or tools inside workflows (except `script/install-zig` in release jobs).
- Ensure all actions are pinned to commit SHAs.
- CI should run `script/bootstrap` before any build/test/lint.
- Offline defaults come from `script/env`, so workflows do not need explicit `CARGO_NET_OFFLINE`/`RUSTUP_OFFLINE` env blocks.

## Cross-Platform + Universal macOS Builds

`script/build` supports multi-target builds:

- Example: `script/build --release --targets "x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu"`
- macOS universal binaries: `script/build --release --targets "x86_64-apple-darwin,aarch64-apple-darwin" --universal-darwin`
- Cross builds require preinstalled `zig` and `cargo-zigbuild` that match `.zig-version` and `.cargo-zigbuild-version`.

## Rust Best Practices (Template Defaults)

- `#![forbid(unsafe_code)]` everywhere.
- Prefer small, focused modules with clear public APIs.
- Keep public surface minimal; re-export intentionally.
- Treat clippy warnings as errors.
- Use `--frozen` / `--locked` for all Cargo commands outside `script/update`.
- Avoid build scripts unless absolutely necessary.

## Updating Dependencies (Online Only)

1. Ensure network access is available.
2. Run `script/update`.
3. Inspect the diff (`Cargo.lock` + `vendor/cache`).
4. Run `script/bootstrap` and `script/test` offline.

## If You Change This Template

- Update `README.md` and this file when workflows or scripts change.
- Keep version files and toolchain pins consistent.
- Preserve hermetic guarantees: no new network calls in scripts or CI.
