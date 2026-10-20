# Contributing and reviewing changes

Keep each commit about one observable change. For example, changing the CLI and
changing the likelihood model belong in separate commits, even when both affect
the same workflow. A PR should explain the problem, the resulting behavior, and
how it was checked. Formatting belongs in its own commit so behavior changes are
easy to see in later diffs.

## Read the code in layers

- `src/cli.rs` describes command-line arguments using clap's derive macros.
- `src/main.rs` dispatches the selected command; `src/summarize_likelihoods.rs` prints reports.
- `src/bedmethyl.rs` parses records and maintains the column/header contract.
- `src/lib.rs` contains compression, safe output handling, and the statistical
  operations. The library can be tested without launching the command-line app.
- `tests/` contains integration tests, including tests that run the installed
  executable's command-line interface.

The review sequence is: formatting, one executable with subcommands, downloadable
builds, output safety, compression, parsing, classification, schema consistency,
and numerical validation. Each correctness fix has a regression test that
demonstrates the affected behavior. The README's numerical examples are tested
as well.

## Fix a bug with a regression test

1. Create the smallest input that demonstrates the problem.
2. Add a test asserting the desired behavior and run it against the old code.
   A failing test should expose the bug, rather than an unrelated setup error.
3. Make the focused implementation change and rerun that test.
4. Run the checks below before committing. Explain any behavior or file-format
   change in the README and commit message.

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release
```

For example, `cargo test --test compression` checks actual compressed bytes and
uses an independent decoder. Simply reading output with our own reader would
miss a bug where both functions accepted plain text mislabeled as gzip.

CI runs tests on Linux, macOS, and Windows for pushes and pull requests. CI checks
software behavior; biological accuracy requires representative data with known
CpG genotypes and a documented data-generation procedure.

## Publish a release

1. Update the package version in `Cargo.toml` and run `cargo check` to update
   `Cargo.lock`.
2. Run the checks above, then commit and push the version change.
3. Run **Publish release binaries** from the GitHub Actions tab, or push a tag
   named `v` followed by the package version. A pushed tag must match the version
   in `Cargo.toml`. A manual run creates the tag at the workflow's commit if it
   does not already exist.
4. Check the workflow result and the assets on the GitHub Releases page.

The release workflow tests and builds Linux, Windows, Intel macOS, and Apple
Silicon macOS executables. After all four builds pass, it creates a draft release,
uploads the executables and one standalone `README.md`, then publishes the
release. A prerelease version is marked as a prerelease on GitHub.

Published releases are not overwritten. A tag pointing at a different commit
also prevents publication. Choose a new package version for a new release.

### cargo-binstall metadata

`[package.metadata.binstall]` in `Cargo.toml` maps the package version and Rust
target to a release asset, using `pkg-fmt = "bin"`. The asset naming pattern is
`cpg-likelihoods-v{version}-{target}` with `.exe` appended for Windows. The package
is named `cpg_likelihoods`; the executable is named `cpg-likelihoods`.

The `repository` field and README installation links must identify the repository
hosting the releases. Update them when adopting the changes in another
repository. The documented `--git` install command reads the manifest from that
repository. Installation by package name alone requires publishing the package
and its metadata on crates.io.

For a local installation test, pass the absolute path of `Cargo.toml` with
`--manifest-path`. Use `--strategies crate-meta-data` to verify the prebuilt
executable without falling back to a source build, and `--install-path` to test
in a temporary directory.
