# Security Policy

## Supported Versions

This repository is a template. Security fixes are applied to the latest `main` branch. Downstream projects should rebase, merge, or copy relevant template fixes into their own repositories.

## Dependency Policy

- Cargo dependencies must be exact-pinned where practical.
- `Cargo.lock` must be committed.
- `vendor/cache` must be committed.
- Normal build, test, lint, run, and release-build workflows must not require third-party tool downloads.
- Cargo dependency updates must use `script/update`.
- Dependency update changes must include any required `Cargo.lock` and `vendor/cache` changes.
- Release-tool updates must use `script/vendor-release-tools` and commit `vendor/release-tools` changes.

## Offline Expectations

The daily scripts are intended to validate offline Cargo behavior:

```console
script/bootstrap
script/test
script/lint
script/build
```

GitHub-hosted runners are not fully air-gapped infrastructure. They validate that the repository scripts do not ask Cargo or rustup to download during daily jobs. Checkout, action loading, artifact transfer, release publication, and attestation verification still require GitHub platform access.

## Release Tooling

Release build tooling is vendored in `vendor/release-tools`:

- Zig host archives are committed and checksum-verified before extraction.
- `cargo-zigbuild` source, lockfile, crate archive, and vendored transitive crates are committed.
- `script/install-zig` installs release tools from committed artifacts only.
- `script/vendor-release-tools` is the only online release-tool refresh path.

This repository does not yet vendor the Rust toolchain or Rust target standard libraries. Hosted runners may still hydrate the pinned Rust toolchain if it is missing. Fully egress-blocked build jobs require Rust and any required target standard libraries to already be present or vendored in a future pass.

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
