# Install in Hermes

This repo is a skill bundle. Install both `bdk-dojo` and the vendored `teach-mode` skill so the learner-first contract is available.

## Copy install

```bash
mkdir -p ~/.hermes/skills/education
cp -R skills/teach-mode ~/.hermes/skills/education/teach-mode
cp -R skills/bdk-dojo ~/.hermes/skills/education/bdk-dojo
```

## Hermes hub install

```bash
hermes skills install SaucePackets/bdk-dojo-skill/skills/teach-mode --yes
hermes skills install SaucePackets/bdk-dojo-skill/skills/bdk-dojo --yes
```

Start a new Hermes session, then ask:

```text
Use the bdk-dojo skill and give me the first wallet-balance kata.
```

## Symlink install for contributors

From the repo root:

```bash
mkdir -p ~/.hermes/skills/education
ln -sfn "$PWD/skills/teach-mode" ~/.hermes/skills/education/teach-mode
ln -sfn "$PWD/skills/bdk-dojo" ~/.hermes/skills/education/bdk-dojo
```

Use symlink mode if you plan to edit this repo and test changes locally.

## Verify

```bash
hermes skills list | grep -E 'teach-mode|bdk-dojo'
```

If the current chat session does not see the skill, start a new session. Skill loading is session-scoped.
