# Install in Hermes

BDK Dojo depends on the standalone `teach-mode` skill. Install both skills separately so future dojo skills can share one canonical teaching contract instead of vendoring copied snapshots.

## One-command install

Stable `v0.0.2` install:

```bash
curl -fsSL https://github.com/SaucePackets/bdk-dojo-skill/releases/download/v0.0.2/install-hermes.sh | bash
```

The installer uses `--force` for these two SaucePackets skill URLs because Hermes' community-skill scanner currently flags a false-positive phrase in Teach Mode. It still installs only these explicit pinned skill files.

To install from current `main` instead:

```bash
BDK_DOJO_REF=main TEACH_MODE_REF=main \
  bash <(curl -fsSL https://raw.githubusercontent.com/SaucePackets/bdk-dojo-skill/main/scripts/install-hermes.sh)
```

## Manual Hermes install

```bash
hermes skills install SaucePackets/teach-mode-skill/skills/teach-mode --yes
hermes skills install SaucePackets/bdk-dojo-skill/skills/bdk-dojo --yes
```

Start a new Hermes session, then ask:

```text
Use teach-mode and bdk-dojo. Give me the first wallet-balance kata.
```

## Copy install

Clone both repos, then copy each skill into Hermes:

```bash
mkdir -p ~/.hermes/skills/education
cp -R /path/to/teach-mode-skill/skills/teach-mode ~/.hermes/skills/education/teach-mode
cp -R /path/to/bdk-dojo-skill/skills/bdk-dojo ~/.hermes/skills/education/bdk-dojo
```

## Symlink install for contributors

From each repo root:

```bash
mkdir -p ~/.hermes/skills/education
ln -sfn "$PWD/skills/teach-mode" ~/.hermes/skills/education/teach-mode  # run inside teach-mode-skill
ln -sfn "$PWD/skills/bdk-dojo" ~/.hermes/skills/education/bdk-dojo      # run inside bdk-dojo-skill
```

Use symlink mode if you plan to edit either repo and test changes locally.

## Verify

```bash
hermes skills list | grep -E 'teach-mode|bdk-dojo'
```

If the current chat session does not see the skills, start a new session. Skill loading is session-scoped.
