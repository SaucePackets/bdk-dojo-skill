# Release Checklist

Use this before publishing a BDK Dojo release.

- [ ] Update `skills/bdk-dojo/SKILL.md` frontmatter `version`.
- [ ] Update `CHANGELOG.md`.
- [ ] Update installer default refs only when the target releases exist.
- [ ] If refreshing an existing pre-1.0 tag, document that the tag/release was refreshed and verify no stale release asset remains.
- [ ] Run `python3 scripts/audit_scaffolds.py`.
- [ ] Run `python3 scripts/check_skill_dependencies.py`.
- [ ] Run `bash -n scripts/install-hermes.sh`.
- [ ] Run `shellcheck scripts/install-hermes.sh scripts/test_install_hermes.sh`.
- [ ] Run `bash scripts/test_install_hermes.sh`.
- [ ] Run `cd examples/bdk-dojo-wallet && cargo fmt -- --check`.
- [ ] Run `cd examples/bdk-dojo-wallet && cargo test`.
- [ ] Wait for GitHub Actions CI to pass.
- [ ] Create the git tag.
- [ ] Create the GitHub release.
- [ ] Upload `scripts/install-hermes.sh` as a release asset.
- [ ] Record or publish the release asset checksum.
- [ ] Test the stable `curl -fsSL ... | bash` command in a clean environment.
- [ ] Manually verify the logged-out GitHub release asset URL works.
