# Required Repository Settings

Some security controls cannot be fully represented in tracked files. Configure these settings in GitHub for repositories created from this template.

## Branch Protection For `main`

- Require a pull request before merging.
- Require CODEOWNER review.
- Require status checks before merge:
  - `lint`
  - `test`
  - `build`
- Require branches to be up to date before merging if that matches the repository's merge policy.
- Block force-pushes.
- Block branch deletion.
- Restrict who can dismiss reviews.

## Actions

- Set default `GITHUB_TOKEN` permissions to read-only.
- Require approval for first-time contributor workflows.
- Keep GitHub Actions pinned to full commit SHAs.
- Do not allow untrusted pull request workflows to receive write tokens.
- Test, lint, and PR build jobs may run the explicit `script/prepare-rust` preflight, then should stay on the normal offline script surface.
- Protected release-build jobs should not run Rust preparation or download third-party tools after checkout/action loading; they rely on preprovisioned Rust toolchains and committed release-tool artifacts.
- Keep the `build` workflow as the PR-based release smoke test: install vendored release tools, verify them, then run release-mode packaging.
- If an egress-blocking action is added, apply it to build/test/package jobs after checkout and before scripts run. Do not apply it to release publishing, signing, or verification jobs unless those jobs are split into an explicitly GitHub-network-allowed phase.

## CODEOWNERS

Require CODEOWNER review for sensitive paths:

- `.github/workflows/**`
- `.github/dependabot.yml`
- `.github/CODEOWNERS`
- `script/**`
- `.cargo/config.toml`
- `Cargo.toml`
- `Cargo.lock`
- `deny.toml`
- `vendor/**`
- `vendor/release-tools/**`
- Tool version files
- Security and repository policy docs

## Releases

- Create a protected `release` environment.
- Require reviewer approval before jobs using that environment can publish release assets.
- Keep release publication permissions limited to the release job.
- Verify release assets after publication by re-downloading them, checking `checksums.txt`, and verifying artifact attestations.
- Release build jobs should install Zig and `cargo-zigbuild` from `vendor/release-tools`; they should not run `script/prepare-rust`, `curl`, `cargo install --version`, `rustup target add`, or Rust toolchain setup actions.

## Dependabot

- Keep GitHub Actions and Rust toolchain update checks enabled if they are useful.
- Do not enable Cargo version update PRs unless there is automation that also regenerates `Cargo.lock` and `vendor/cache`.
- Cargo dependency updates should normally be performed with `script/update`.
