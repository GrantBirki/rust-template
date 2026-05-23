# AGENTS.md

This repository is a Rust template for hermetic, reproducible, air-gapped-leaning projects. Treat it as public open source infrastructure: every file, comment, workflow, log line, and document change may be visible to anyone.

The template is meant to be copied into services, CLIs, and libraries that must pass the "airplane test": a normal developer or CI worker should be able to build, test, lint, and package the project without reaching the network after dependencies and toolchains have been explicitly prepared.

## North-Star Principles

- Hermetic by default: routine build, test, lint, and run workflows must not need the network.
- Explicit inputs: dependency versions, toolchain versions, release helper versions, and GitHub Actions must be pinned.
- Vendored dependencies: Cargo resolves from `vendor/cache` through `.cargo/config.toml`.
- Script-first workflow: use `script/*` entrypoints instead of ad hoc command sequences.
- Public-safe maintenance: do not add private paths, secrets, organization context, or machine-specific assumptions to tracked files.
- Repo-scoped local artifacts: keep disposable build, install, and tool extraction outputs under the working tree when practical.
- Durable documentation: if behavior changes, update `README.md`, `AGENTS.md`, and any relevant docs in the same change.

## Public Repository Hygiene

- This repository is public. Do not commit secrets, tokens, internal hostnames, private repo names, personal infrastructure details, or local absolute paths.
- Keep PR descriptions and comments generic enough for public readers.
- Prefer durable public URLs for policy references. Avoid links to private dashboards, local files, or transient logs.
- When adding examples, use placeholder names and domains instead of real private systems.
- Do not add telemetry, remote policy services, or opaque downloaded agents to the template without explicit design approval.

## Non-Negotiable Policy

- No network calls during `script/bootstrap`, `script/test`, `script/lint`, `script/build`, or `script/server`.
- `script/update` is the only normal Cargo dependency update path and is intentionally online-only.
- `script/vendor-rust` is the only normal Rust distribution lock refresh path and is intentionally online-only.
- `script/prepare-rust` is the only normal Rust installation path. It is intentionally online, checksum-gated, and must validate `rust-toolchain.lock.toml` before invoking `rustup`.
- `script/vendor-update-tools` is the only normal `cargo-audit` / `cargo-deny` lock refresh path and is intentionally online-only.
- `script/vendor-release-tools` is the only release-tool refresh path and is intentionally online-only.
- `script/install-zig` must stay offline-only. It installs Zig and `cargo-zigbuild` from committed artifacts under `vendor/release-tools`.
- Release build jobs must not run direct `curl`, `cargo install --version`, `rustup target add`, or Rust toolchain setup actions outside the repo scripts.
- Release jobs must verify exact release tool versions after installing from committed artifacts.
- Release workflows must not expose `workflow_dispatch`; releases are created only from `Cargo.toml` version bumps merged to `main`.
- All direct Cargo dependencies in `Cargo.toml` must use exact versions such as `=1.2.3`.
- `Cargo.lock` is committed and treated as source of truth.
- Vendored crates live in `vendor/cache` and are required for offline builds.
- Local non-CI scripts should prefer ignored repo-local temp roots such as `target/tmp` for Rust, Zig, Cargo, and release-tool scratch artifacts. Respect caller-provided `TMPDIR` and `RUNNER_TEMP`.
- `rust-toolchain.toml`, `.rust-version`, `Cargo.toml` `rust-version`, `rust-toolchain.lock.toml`, `.zig-version`, `.cargo-zigbuild-version`, `.cargo-audit-version`, `.cargo-deny-version`, `update-tools.lock.toml`, `release-tools.lock.toml`, and `vendor/release-tools/manifest.toml` must stay consistent with actual supported tools.
- GitHub Actions must be pinned to full commit SHAs.
- Checkout steps should use `persist-credentials: false` unless a job explicitly needs credentials persisted.
- Rust distribution artifacts are not committed. `rust-toolchain.lock.toml` pins the official Rust distribution URLs and SHA-256s that `script/prepare-rust` verifies before installing with `rustup`.

## Hermeticity Model

There are three different levels in this template. Keep them distinct when editing docs or workflows.

1. Local project workflow: checksum-gated online Rust preparation with `script/prepare-rust` when needed, then offline Cargo validation through vendored sources. `script/bootstrap`, `script/test`, `script/lint`, `script/build`, and `script/server` enforce the offline phase.
2. GitHub-hosted validation: `lint`, `test`, PR `build`, and release build jobs run `script/validate-locks --ci`, then `script/prepare-rust`, then the normal offline scripts. Hosted runners are not air-gapped infrastructure because checkout, action loading, Rust preparation, and artifact services still use network access.
3. Egress-blocked build/package jobs: after checkout, action loading, Rust preparation, and release-tool installation, build/test/package scripts can run without third-party network access because application crates and release tools are committed. Release publish/sign/verify jobs still need GitHub API access.

Do not describe GitHub-hosted runners as fully air-gapped. They can validate offline script behavior, but they are not an egress-blocked environment.

## Local Artifact Placement

Keep transient tool outputs close to the repo when reasonable.

- Outside CI, `script/env` defaults `RUNNER_TEMP` and `TMPDIR` to `target/tmp` when the caller has not already set them.
- Use ignored directories under `target/` for disposable Rust, Zig, Cargo, archive extraction, and release-tool build artifacts.
- Do not hard-code absolute machine-specific temp paths in scripts, docs, workflows, or examples.
- Do not commit generated temp files, extracted tool directories, build-script executables, or local cache contents.
- Preserve explicit caller or CI temp roots. GitHub Actions jobs should continue using the runner-provided environment where appropriate.
- If a future script needs a scratch directory, derive it from `TMPDIR`, `RUNNER_TEMP`, `target/`, or another ignored repo-local path instead of choosing an OS-global location by default.

## Script Contracts

All scripts live in `script/` and should use `set -euo pipefail` unless there is a documented reason not to.

- `script/env`
  - Shared environment and helper functions.
  - Exports offline Cargo defaults and disables rustup proxy auto-installation.
  - Outside CI, defaults `RUNNER_TEMP` and `TMPDIR` to `target/tmp` unless the caller already set them.
  - Defines `DIR`, `VENDOR_DIR`, Rust toolchain checks, vendor checks, and common `die`/`warn` helpers.
  - Do not add network behavior here.

- `script/prepare-rust`
  - Explicit online Rust preparation path for local developers and hosted validation.
  - Runs `script/validate-rust-toolchain --ci` before installing anything.
  - Installs the exact Rust toolchain from `rust-toolchain.toml` with the minimal profile plus `rustfmt` and `clippy`.
  - Installs extra target standard libraries only when `PREPARE_RUST_TARGETS` or `VERIFY_RUST_TARGETS` is set.
  - Does not run from offline project scripts.

- `script/bootstrap`
  - Validates Rust and Cargo availability.
  - Verifies pinned Rust toolchain files and vendor cache presence.
  - Runs `cargo check --frozen`.
  - Must stay offline.

- `script/test`
  - Validates the release-tool lockfile and manifest before running tests.
  - Runs `cargo test --frozen` by default.
  - `--coverage`, `--cov`, or `-c` requires preinstalled `cargo-llvm-cov` and `llvm-tools-preview`.
  - Coverage mode must not install tools.
  - `cargo test` itself does not emit coverage; coverage mode relies on Rust source-based coverage instrumentation and external preinstalled reporting tools.

- `script/lint`
  - Runs format check, clippy, `cargo verify-project`, and docs.
  - Uses frozen Cargo commands for lint/doc generation.
  - `--auto-fix` may run `cargo fmt`; do not use it in CI.

- `script/build`
  - Builds release binaries by default.
  - `--release` enables dist artifact packaging and supports `--targets` and `--universal-darwin`.
  - Cross builds require matching `zig` and `cargo-zigbuild`, normally installed by `script/install-zig` from committed release-tool artifacts.
  - Tool version mismatches must fail, not warn.
  - Uses `SOURCE_DATE_EPOCH` when provided for reproducible build metadata.

- `script/server`
  - Runs the CLI/app through `cargo run --frozen`.
  - Must stay offline.

- `script/update`
  - Online-only dependency refresh path.
  - Temporarily moves `.cargo/config.toml` aside to allow Cargo registry access.
  - Runs `cargo update`, re-vendors with `cargo vendor --locked --versioned-dirs`, installs checksum-locked `cargo-audit` and `cargo-deny` from `update-tools.lock.toml`, runs audit/deny checks, restores offline config, then verifies with offline scripts.
  - Dependency update PRs must include `Cargo.lock` and `vendor/cache` changes.

- `script/vendor-update-tools`
  - Online-only update-tool refresh path.
  - Fetches the locked `cargo-audit` and `cargo-deny` top-level crates from crates.io, verifies that each package includes `Cargo.lock`, and writes crate plus packaged-lockfile checksums to `update-tools.lock.toml`.
  - Must be run intentionally and reviewed like any other supply-chain update.

- `script/validate-update-tools`
  - Offline validation for `update-tools.lock.toml` schema, version-file consistency, crates.io URLs, and checksum formats.
  - With `--ci`, fetches the top-level crates and verifies crate SHA-256s plus packaged `Cargo.lock` SHA-256s before expensive CI work runs.

- `script/vendor-rust`
  - Online-only Rust distribution lock refresh path.
  - Fetches the official Rust channel manifest, verifies the manifest `.sha256`, and writes `rust-toolchain.lock.toml`.
  - Locks upstream URLs and SHA-256s for `rustc`, `cargo`, `rustfmt`, `clippy`, and configured Rust target standard libraries.
  - Must be run intentionally and reviewed like any other supply-chain update.

- `script/validate-rust-toolchain`
  - Offline validation for Rust version-file consistency and lockfile coverage.
  - With `--ci`, fetches the official Rust channel metadata and fails if locked URLs or SHA-256s differ.

- `script/validate-locks`
  - Top-level fast lock gate for CI.
  - Runs Rust, update-tool, release-tool, Cargo/vendor, GitHub Actions SHA, and workflow image digest checks.
  - CI should run `script/validate-locks --ci` immediately after checkout.

- `script/install-zig`
  - Offline-only release-tool installer.
  - Selects the host Zig tarball from `vendor/release-tools/manifest.toml`.
  - Verifies SHA-256 before extracting Zig under `${RUNNER_TEMP}`.
  - Verifies and expands committed `cargo-zigbuild` source/vendor archives under `${RUNNER_TEMP}`.
  - Installs `cargo-zigbuild` from expanded source and vendored dependencies with `cargo install --path --locked --offline`.
  - Must fail if installed versions do not match pinned version files.
  - Must not call `curl`, `rustup target add`, `cargo install --version`, or unset offline environment variables.

- `script/vendor-release-tools`
  - Online-only release-tool refresh path.
  - Reads upstream release-tool URLs and checksums from `release-tools.lock.toml`.
  - Fetches locked Zig host archives and the locked `cargo-zigbuild` crate.
  - Generates/preserves the `cargo-zigbuild` lockfile, commits a standalone reviewable lockfile copy, vendors its transitive crates, writes deterministic source/vendor `.tar.gz` archives, and writes `vendor/release-tools/manifest.toml`.
  - Must be run intentionally and reviewed like any other supply-chain update.

- `script/validate-release-tools`
  - Offline validation for committed release-tool artifacts.
  - Verifies lockfile and manifest version consistency, lockfile/manifest agreement, artifact existence, SHA-256 checksums, archive path safety, standalone lockfile consistency, `cargo-zigbuild` source/lock/vendor archive state, and release workflow/install-script network guardrails.
  - Must fail if release-tool scripts contain embedded SHA-256 literals; expected upstream hashes belong in `release-tools.lock.toml`.
  - Must fail if the release workflow exposes a manual `workflow_dispatch` trigger.

- `script/verify-release-toolchain`
  - Offline verification for release builders.
  - Confirms Rust, Zig, `cargo-zigbuild`, and optional `VERIFY_RUST_TARGETS`.
  - Use this after `script/install-zig` in release build jobs.

## Dependency Policy

- Direct dependencies in `Cargo.toml` must be exact-pinned.
- Do not run `cargo add`, `cargo update`, `cargo vendor`, or `cargo install` manually as a tracked application workflow replacement. Use or update `script/update`.
- Do not edit vendored crates by hand.
- Do not add git dependencies unless the change is explicitly justified and pinned to an immutable revision.
- Do not add path dependencies to template defaults unless the repo becomes a workspace template.
- `vendor/cache` should be generated by Cargo, not manually curated.
- `rust-toolchain.lock.toml` is the human-reviewed lock for upstream Rust distribution URLs and checksums.
- `update-tools.lock.toml` is the human-reviewed lock for online update-path Cargo tool crate URLs, crate checksums, and packaged `Cargo.lock` checksums.
- `release-tools.lock.toml` is the human-reviewed lock for upstream release-tool URLs and checksums.
- `vendor/release-tools` should be generated by `script/vendor-release-tools`, not manually curated. Review release-tool updates by checking pinned versions, upstream URLs, `release-tools.lock.toml`, generated manifest checksums, archive regeneration behavior, and install/validation scripts; do not treat archived third-party tool contents as first-party template code.
- New dependency governance tools must be pinned and either preinstalled for offline paths or limited to `script/update`.
- `cargo-vet`, SBOM generation, and auditable binaries are intended staged follow-ups. Do not quietly add online release downloads for those tools.

## Version Files

Keep these aligned:

- `rust-toolchain.toml`: exact Rust toolchain and components.
- `.rust-version`: same Rust version as `rust-toolchain.toml`.
- `Cargo.toml` `rust-version`: same enforced Rust version unless the repo intentionally adopts a separate MSRV policy with CI coverage.
- `rust-toolchain.lock.toml`: upstream Rust distribution URL and checksum lock.
- `.zig-version`: Zig version required for cross release builds.
- `.cargo-zigbuild-version`: `cargo-zigbuild` version required for cross release builds.
- `.cargo-audit-version`: online update path `cargo-audit` version.
- `.cargo-deny-version`: online update path `cargo-deny` version.
- `update-tools.lock.toml`: upstream update-tool crate URL, crate checksum, and packaged `Cargo.lock` checksum lock.
- `release-tools.lock.toml`: upstream release-tool URL and checksum lock.
- `vendor/release-tools/manifest.toml`: committed release-tool artifact inventory and checksums.

If any version file changes, update docs and verify the corresponding script behavior.

## CI Expectations

- The `build` workflow is the PR-based release smoke test. It should validate locks, prepare Rust, install release tools, verify them, run `script/bootstrap`, and run `script/build --release`.
- Hosted lint/test/build workflows should run `script/validate-locks --ci`, then `script/prepare-rust`, then `script/bootstrap`, then their offline validation command or release-smoke path.
- Hosted validation should rely on offline defaults from `script/env` after explicit preparation completes.
- Do not add Rust toolchain setup actions to hosted lint/test/build workflows; use `script/prepare-rust` so the preparation path stays explicit, checksum-gated, and repo-owned.
- Release build jobs should run `script/validate-locks --ci`, then `script/prepare-rust` with the required `PREPARE_RUST_TARGETS`, then `script/install-zig`, then `script/verify-release-toolchain`, then `script/bootstrap`, then `script/build --release ...`.
- Release build jobs must install release tools from committed artifacts. Cross-target release jobs must fail if required Rust targets are missing.
- Release publication should use a protected `release` environment.
- Final release assets should be re-downloaded from GitHub Releases, checksum-verified, and attestation-verified.
- Job permissions should be least-privilege. Keep top-level workflow permissions empty where practical.
- It is acceptable for publish/sign/verify jobs to use GitHub API access. Do not claim those jobs are zero-network.
- Be explicit about CI time tradeoffs. Building `cargo-zigbuild` from committed source on PR runners is slower than downloading a binary or using a runner image, but it removes release-time crates.io/tool availability from the build path.

## Release Expectations

- `Cargo.toml` `version` is the release trigger.
- Merging a version bump to `main` creates the `vX.Y.Z` release through CI.
- The release workflow is intentionally not manually dispatchable.
- Do not create or push release tags manually unless the workflow is intentionally being recovered.
- Release artifacts should include binaries, completions, man pages, checksums, and attestations.
- Release timestamps should come from `SOURCE_DATE_EPOCH`, normally the commit timestamp.

## Rust Code Standards

- Keep `#![forbid(unsafe_code)]` in first-party crates.
- Prefer small modules and minimal public API.
- Avoid build scripts unless they are essential and reviewed as part of the supply-chain surface.
- Treat clippy warnings as errors.
- Keep example code simple, but avoid teaching unsafe or surprising production patterns.
- Preserve public API stability unless the task explicitly calls for a breaking template change.
- If changing CLI output, completions, man-page behavior, or release archive layout, update README examples.

## Documentation Requirements

Update docs in the same PR when changing:

- Script behavior or script arguments.
- Hermetic/offline guarantees.
- Dependency update process.
- Toolchain/version files.
- Release artifact layout or release verification.
- GitHub Actions permissions or release environment assumptions.
- Public template expectations for downstream consumers.

`README.md` is for users of the template. `AGENTS.md` is for maintainers and coding agents. `SECURITY.md` is for security policy and vulnerability reporting. `docs/repository-settings.md` is for settings that cannot be fully represented in tracked files.

## Validation Checklist

Use the smallest validation set that proves the change:

- Script/workflow/doc changes: `git diff --check`.
- Rust behavior changes: `script/bootstrap`, `script/test`, `script/lint`, and `script/build`.
- Coverage changes: `script/test --cov` only when coverage tools are already installed. Do not add a static coverage badge unless CI enforces and publishes the measured result.
- Dependency updates: `script/update`, then inspect `Cargo.lock` and `vendor/cache`, then rerun offline validation.
- Lock surface changes: run `script/validate-locks --ci`.
- Rust toolchain updates: run `script/vendor-rust`, inspect `rust-toolchain.lock.toml`, then run `script/validate-rust-toolchain --ci` and `script/prepare-rust` on a supported host.
- Update-tool changes: run `script/vendor-update-tools`, inspect `update-tools.lock.toml`, then run `script/validate-update-tools --ci`.
- Release-tool updates: inspect `release-tools.lock.toml`, run `script/vendor-release-tools`, then inspect `vendor/release-tools`, run `script/validate-release-tools`, and run `script/install-zig` on a supported host.
- Release workflow changes: inspect YAML carefully and ensure release jobs still verify published release assets.

If local validation is blocked by missing tools or a local toolchain issue, report the exact blocker instead of implying the repo passed.
