# Contributing and reviewing changes

Keep each commit about one observable change. For example, changing the CLI and
changing the likelihood model belong in separate commits, even when both affect
the same workflow. A PR should explain the problem, the resulting behavior, and
how it was checked. Formatting belongs in its own commit so behavior changes are
easy to see in later diffs.

## Read the code in layers

- `src/cli.rs` describes command-line arguments using clap's derive macros.
- `src/main.rs` dispatches the selected command; `src/summary.rs` prints reports.
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

CI runs tests on Linux, macOS, and Windows for pushes and pull requests. The
downloadable-build workflow runs separately on manual dispatch and version tags.
When changing the package version, update both `Cargo.toml` and `Cargo.lock`
before pushing the matching tag. CI checks software behavior; biological
accuracy requires representative data with known CpG genotypes and a documented
data-generation procedure.
