# rust-template

[![lint](https://github.com/GrantBirki/rust-template/actions/workflows/lint.yml/badge.svg)](https://github.com/GrantBirki/rust-template/actions/workflows/lint.yml)
[![test](https://github.com/GrantBirki/rust-template/actions/workflows/test.yml/badge.svg)](https://github.com/GrantBirki/rust-template/actions/workflows/test.yml)
[![build](https://github.com/GrantBirki/rust-template/actions/workflows/build.yml/badge.svg)](https://github.com/GrantBirki/rust-template/actions/workflows/build.yml)
[![release](https://github.com/GrantBirki/rust-template/actions/workflows/release.yml/badge.svg)](https://github.com/GrantBirki/rust-template/actions/workflows/release.yml)

A starter template for Rust projects.

## Features

- Full dependency vendoring
- Air-gapped Cargo by default (offline + frozen)
- Checksum-locked Rust distribution metadata for `rustc`, `cargo`, `rustfmt`, `clippy`, and target standard libraries
- [Scripts to rule them all](https://github.blog/engineering/scripts-to-rule-them-all/)
- Basic CI/CD setup
- Basic testing setup with optional coverage support
- Cross-platform build script with macOS universal binaries
- CLI example with completions + man page generation
- Pinned online dependency audit tooling (`cargo-audit` + `cargo-deny`) for `script/update`
- Vendored release tooling for Zig + `cargo-zigbuild`
- Offline release-tool installation and verification for release builds

## Getting Started

1. Clone the repository
2. Run `script/prepare-rust` to install the checksum-locked Rust toolchain
3. Run `script/bootstrap`
4. Run `script/test` to run the tests
5. Run `script/lint` to run the linter
6. Run `script/build` to build the project
7. Run `script/server` to run the app (or CLI)
8. Bump `version` in `Cargo.toml` and merge to `main` to trigger a release (CI creates the tag)

## Hermeticity Model

The normal project workflow uses checksum-gated online Rust preparation, then runs offline project scripts. Run `script/prepare-rust` when the pinned Rust toolchain needs to be installed, then use `script/bootstrap`, `script/test`, `script/lint`, `script/build`, and `script/server` with vendored Cargo sources and frozen/offline Cargo behavior.

Outside CI, shared script setup defaults `RUNNER_TEMP` and `TMPDIR` to the ignored repo-local `target/tmp` directory when the caller has not already set them, so disposable Rust, Zig, Cargo, and release-tool scratch artifacts stay near the working tree.

GitHub-hosted `lint`, `test`, PR `build`, and release build jobs run `script/validate-locks --ci`, then `script/prepare-rust`, then enter the normal offline script surface. Hosted runners are not fully air-gapped infrastructure. Checkout, action loading, Rust preparation, artifact upload, release publication, and attestation verification still use networked platform services.

Release build jobs install Zig and `cargo-zigbuild` from committed artifacts under `vendor/release-tools`. Zig is kept as upstream `.tar.xz` archives. `cargo-zigbuild` source and vendored dependencies are kept as deterministic `.tar.gz` archives that CI verifies and expands under `${RUNNER_TEMP}`. Those artifacts are refreshed only by `script/vendor-release-tools`, which is intentionally online-only. Upstream release-tool URLs and checksums are locked in `.cargo/tooling/release-tools.lock.toml`; the generated committed-artifact inventory lives in `vendor/release-tools/manifest.toml`.

Rust toolchain metadata is refreshed only by `script/vendor-rust`, which is intentionally online-only. Upstream Rust distribution inputs and checksums are locked in `.cargo/tooling/rust-toolchain.lock.toml`; Rust distribution tarballs are not committed.

The `build` workflow is the PR-based release smoke test. It validates locks, prepares checksum-locked Rust, installs the vendored release tools, verifies them, then runs `script/build --release` so PRs exercise most of the release build path before a merge to `main` can publish a release.

## CLI Usage (Example)

```console
# default greeting
./target/release/rust-template

# greet someone by name
./target/release/rust-template --name grantbirki

# repeat + shout
./target/release/rust-template --name monalisa --times 3 --shout

# arithmetic helpers
./target/release/rust-template add 2 3
./target/release/rust-template sub 10 4

# extended version metadata
./target/release/rust-template version
```

## Completions + Man Pages

Release artifacts include shell completions and a man page:

```console
./target/release/rust-template completions bash > rust-template.bash
./target/release/rust-template completions zsh > _rust-template
./target/release/rust-template completions fish > rust-template.fish
./target/release/rust-template man > rust-template.1
```

## Homebrew (Custom Tap)

This template is designed to ship prebuilt release tarballs that Homebrew can install.
In your tap, point the formula at the release artifacts (including the macOS universal binary).

Example paths (release tag `vX.Y.Z`):

- macOS universal: `rust-template_vX.Y.Z_darwin-universal.tar.gz`
- Linux amd64: `rust-template_vX.Y.Z_linux-amd64.tar.gz`
- Linux arm64: `rust-template_vX.Y.Z_linux-arm64.tar.gz`

The tarballs include:

- the binary at the archive root
- `completions/` for bash/zsh/fish/powershell
- `man/` for the man page

## Dependency Updates (Online Only)

Application dependency updates are explicit and must be done while online:

```console
script/update
```

This refreshes `Cargo.lock` and regenerates `vendor/cache`. All other scripts are offline-by-default.
It also runs pinned `cargo-audit` and `cargo-deny` checks. Tool versions are pinned in `.cargo/tooling/cargo-audit-version` and `.cargo/tooling/cargo-deny-version`, and their top-level crate hashes plus packaged `Cargo.lock` hashes are locked in `.cargo/tooling/update-tools.lock.toml`.

Cargo Dependabot updates are intentionally disabled because dependency changes must include the lockfile and vendored crates. Use `script/update` for Cargo dependency refreshes.

Rust toolchain updates are separate from application dependency updates:

```console
script/vendor-rust
```

This refreshes `.cargo/tooling/rust-toolchain.lock.toml` from the official Rust channel manifest after verifying the manifest checksum. Review Rust toolchain updates by checking the Rust version files, upstream distribution URLs, checksums, and validation scripts.

Update-tool lock refreshes are separate from application dependency updates:

```console
script/vendor-update-tools
```

This refreshes `.cargo/tooling/update-tools.lock.toml` for `cargo-audit` and `cargo-deny`. Review update-tool changes by checking version pins, crates.io URLs, crate SHA-256s, and the packaged `Cargo.lock` SHA-256s extracted from each crates.io package.

Release-tool updates are separate from application dependency updates:

```console
script/vendor-release-tools
```

This refreshes committed Zig tarballs, the `cargo-zigbuild` crate, deterministic `cargo-zigbuild` source/vendor archives, the standalone reviewable `cargo-zigbuild` lockfile, and `vendor/release-tools/manifest.toml`. Review release-tool updates by checking version pins, upstream URLs, `.cargo/tooling/release-tools.lock.toml`, generated manifest changes, lockfile changes, and the vendoring scripts rather than treating GitHub's expanded archive diff as first-party code. Do not mix release-tool vendoring with normal application dependency updates.

## Coverage

`cargo test` runs the Rust test suite, but it does not report line, branch, region, or function coverage. Rust coverage uses compiler instrumentation through [`rustc -C instrument-coverage`](https://doc.rust-lang.org/rustc/instrument-coverage.html) plus LLVM reporting tools.

This template keeps coverage optional to avoid adding another required binary or CI dependency. `cargo-llvm-cov` and `llvm-tools-preview` are not part of the committed baseline toolchain. If they are already provisioned, run:

```console
script/test --cov
```

That command writes LCOV and HTML output under `coverage/` and enforces 100% line coverage for the current example code. Do not add a README coverage badge unless coverage is also enforced in CI and the badge is generated from that CI result; a static badge can drift from reality.

## Release Process

Releases are triggered by version bumps in `Cargo.toml`:

1. Update `version = "X.Y.Z"` in `Cargo.toml`.
2. Refresh `Cargo.lock` so the root `rust-template` package entry has the same version.
3. Commit only those version changes, open a PR, and merge to `main`.
4. The release workflow detects the version bump, builds artifacts, then creates the `vX.Y.Z` tag and publishes a GitHub release.

Do not create or push tags manually; CI is the source of truth for tags and releases. The release workflow intentionally has no manual dispatch path.

Release publication should use the protected `release` environment described in `docs/repository-settings.md`.

Release build jobs use checksum-gated Rust preparation and committed release-tool artifacts. They must not run direct `curl`, `cargo install --version`, `rustup target add`, or Rust toolchain setup actions outside the repo scripts. Release publishing, signing, and verification remain GitHub-networked operations by design.

## Verifying Release Artifacts

Since the releases are signed using GitHub Artifact Attestations, you can verify the authenticity of the release artifacts using the GitHub CLI.

```console
gh release download vX.Y.Z --dir release-download
cd release-download
shasum -a 256 -c checksums.txt
gh attestation verify rust-template_vX.Y.Z_darwin-universal.tar.gz \
  --repo GrantBirki/rust-template \
  --signer-workflow GrantBirki/rust-template/.github/workflows/release.yml
```

Release assets include `checksums.txt`. Verify checksums before verifying attestations:

```console
shasum -a 256 -c checksums.txt
```

Use `sha256sum -c checksums.txt` on systems where `sha256sum` is the standard checksum tool.

## Security + Repository Settings

- See `SECURITY.md` for the vulnerability reporting, dependency, offline, and release verification policy.
- See `docs/repository-settings.md` for branch protection, Actions, CODEOWNERS, and protected release environment settings that must be configured in GitHub.
