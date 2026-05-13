# Changelog

## Unreleased

- No unreleased changes yet.

## v0.0.2

- Split Teach Mode into a standalone external dependency instead of vendoring it in BDK Dojo.
- Added one-command Hermes installer release asset that installs Teach Mode plus BDK Dojo.
- Added `## Required Companion Skill` to BDK Dojo.
- Renamed BDK's local Teach Mode reference to `bdk-teach-mode-supplement.md` so it is clearly not a vendored standalone Teach Mode copy.
- Strengthened dependency validation, scaffold audit, and installer tests.
- Added CI checks for scaffold audit, dependency rules, installer syntax, ShellCheck, fake-Hermes installer tests, Rust format, and Rust tests.
- Added maintenance, contribution, security, and release checklist docs.

## v0.0.1

- Initial public BDK Dojo skill package.
- Added Bitcoin/Rust wallet-engineering curriculum, scaffold lessons, and cumulative reference crate.
