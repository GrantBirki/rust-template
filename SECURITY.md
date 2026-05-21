# Security Policy

## Supported Versions

This repository is a template. Security fixes are applied to the latest `main` branch. Downstream projects should rebase, merge, or copy relevant template fixes into their own repositories.

## Dependency Policy

- Cargo dependencies must be exact-pinned where practical.
- `Cargo.lock` must be committed.
- `vendor/cache` must be committed.
- Routine repo scripts must not implicitly download third-party tools; hosted validation prepares Rust explicitly with `script/prepare-rust` before entering the offline script surface.
- Cargo dependency updates must use `script/update`.
- Dependency update changes must include any required `Cargo.lock` and `vendor/cache` changes.
- Release-tool updates must use `script/vendor-release-tools` and commit `vendor/release-tools` changes.

## Offline Expectations

The normal offline project scripts are intended to validate offline Cargo behavior:

```console
script/bootstrap
script/test
script/lint
script/build
```

GitHub-hosted runners are not fully air-gapped infrastructure. Hosted lint, test, and PR build validation may run `script/prepare-rust` first, then the repository scripts stay on the offline surface. Those offline scripts do not ask Cargo or rustup to hydrate dependencies or toolchains implicitly. Checkout, action loading, artifact transfer, release publication, and attestation verification still require GitHub platform access.

## Release Tooling

Release build tooling is vendored in `vendor/release-tools`:

- Zig host archives are committed and checksum-verified before extraction.
- `cargo-zigbuild` crate, source archive, lockfile, and vendored transitive dependency archive are committed and checksum-verified before extraction.
- `script/install-zig` installs release tools from committed artifacts only.
- `script/vendor-release-tools` is the only online release-tool refresh path.

This repository does not yet vendor the Rust toolchain or Rust target standard libraries. Hosted lint, test, and PR build validation prepare the pinned Rust toolchain explicitly with `script/prepare-rust`. Protected release-build jobs remain stricter: they do not run that preparation step and require Rust plus any requested target standard libraries to already be present or vendored in a future pass.

Release publication, artifact upload/download, and attestation verification are intentionally GitHub-networked operations.

## Reporting Vulnerabilities

Report vulnerabilities through GitHub Security Advisories for this repository when available. If that is not available, contact the maintainer directly. Do not open public issues with exploit details for unresolved vulnerabilities.

## Verifying Release Artifacts

Release assets include checksums and GitHub artifact attestations. Verify downloaded assets with:

```console
shasum -a 256 -c checksums.txt
gh attestation verify <artifact> \
  --repo GrantBirki/rust-template \
  --signer-workflow GrantBirki/rust-template/.github/workflows/release.yml
```

Use `sha256sum -c checksums.txt` instead of `shasum` on systems where that is the standard checksum tool.
