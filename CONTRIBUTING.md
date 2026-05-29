# Contributing

Thanks for improving BDK Dojo.

## Do not vendor Teach Mode

BDK Dojo depends on the standalone `teach-mode` skill. Do not add `skills/teach-mode/` to this repo.

## Adding a scaffold lesson

Each authored lesson must have:

- `scaffold/<lesson-id>-<slug>/README.md`
- `scaffold/<lesson-id>-<slug>/stubs.rs`
- expected behavior
- required proof/tests/artifact
- reference implementation path
- BDK bridge note
- done criteria

Use lesson IDs like `5.7` or future IDs like `5.10`; avoid assuming single-digit decimals in tooling.

## Required tests vs Required artifact

Use `## Required tests` for code katas that should be proven with Rust tests.

Use `## Required artifact` or `## Required proof` for Markdown/reflection lessons where the learner writes an explanation, contribution note, skeleton plan, or other review artifact.

## BDK source stamps

When a lesson mentions real BDK APIs, crate names, example files, or contribution workflow details, include a `## BDK source stamp` section. State that exact API paths are version-sensitive and must be re-checked upstream before teaching exact details.

## Local checks

Run:

```bash
python3 scripts/audit_scaffolds.py
python3 scripts/check_skill_dependencies.py
bash -n scripts/install-hermes.sh
shellcheck scripts/install-hermes.sh scripts/test_install_hermes.sh
bash scripts/test_install_hermes.sh
cd examples/bdk-dojo-wallet
cargo fmt -- --check
cargo test
```

## Public-clean rule

Do not commit private learner history, personal critique logs, secrets, private repo paths, or production wallet material.
