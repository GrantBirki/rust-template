# rust-template

[![lint](https://github.com/GrantBirki/rust-template/actions/workflows/lint.yml/badge.svg)](https://github.com/GrantBirki/rust-template/actions/workflows/lint.yml)
[![test](https://github.com/GrantBirki/rust-template/actions/workflows/test.yml/badge.svg)](https://github.com/GrantBirki/rust-template/actions/workflows/test.yml)
[![build](https://github.com/GrantBirki/rust-template/actions/workflows/build.yml/badge.svg)](https://github.com/GrantBirki/rust-template/actions/workflows/build.yml)
[![release](https://github.com/GrantBirki/rust-template/actions/workflows/release.yml/badge.svg)](https://github.com/GrantBirki/rust-template/actions/workflows/release.yml)

A starter template for Rust projects.

## Features

- Full dependency vendoring
- Air-gapped Cargo by default (offline + frozen)
- Pinned toolchains via `rust-toolchain.toml` + `.rust-version`
- [Scripts to rule them all](https://github.blog/engineering/scripts-to-rule-them-all/)
- Basic CI/CD setup
- Basic testing setup with code coverage
- Cross-platform build script with macOS universal binaries
- CLI example with completions + man page generation

## Getting Started

1. Clone the repository
2. Run `script/bootstrap`
3. Run `script/test` to run the tests (use `--cov` to run with coverage; requires `cargo-llvm-cov` + `llvm-tools-preview`)
4. Run `script/lint` to run the linter
5. Run `script/build` to build the project
6. Run `script/server` to run the app (or CLI)
7. Bump `version` in `Cargo.toml` and merge to `main` to trigger a release (CI creates the tag)

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

Dependency updates are explicit and must be done while online:

```console
script/update
```

This refreshes `Cargo.lock` and regenerates `vendor/cache`. All other scripts are offline-by-default.

## Release Process

Releases are triggered by version bumps in `Cargo.toml`:

1. Update `version = "X.Y.Z"` in `Cargo.toml`.
2. Commit the change, open a PR, and merge to `main`.
3. The release workflow detects the version bump, creates the `vX.Y.Z` tag, builds artifacts, and publishes a GitHub release.

Do not create or push tags manually; CI is the source of truth for tags and releases.

## Verifying Release Artifacts

Since the releases are signed using GitHub Artifact Attestations, you can verify the authenticity of the release artifacts using the GitHub CLI.

```console
$ gh attestation verify --owner grantbirki rust-template_v0.0.3_darwin-arm64
Loaded digest sha256:bd972559625347da0662076147b4353c13af8aa9ed9b2d4ce48f535c8e2c5a89 for file://rust-template_v0.0.3_darwin-arm64
Loaded 1 attestation from GitHub API

The following policy criteria will be enforced:
- Predicate type must match:................ https://slsa.dev/provenance/v1
- Source Repository Owner URI must match:... https://github.com/grantbirki
- Subject Alternative Name must match regex: (?i)^https://github.com/grantbirki/
- OIDC Issuer must match:................... https://token.actions.githubusercontent.com

✓ Verification succeeded!

The following 1 attestation matched the policy criteria

- Attestation #1
  - Build repo:..... GrantBirki/rust-template
  - Build workflow:. .github/workflows/release.yml@refs/tags/v0.0.3
  - Signer repo:.... GrantBirki/rust-template
  - Signer workflow: .github/workflows/release.yml@refs/tags/v0.0.3
```
