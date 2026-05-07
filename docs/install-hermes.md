# Install in Hermes

This repo is a skill bundle. Copy or symlink the skill into your Hermes skills directory.

## Copy install

```bash
mkdir -p ~/.hermes/skills/education
cp -R skills/bdk-dojo ~/.hermes/skills/education/bdk-dojo
```

Start a new Hermes session, then ask:

```text
Use the bdk-dojo skill and give me the first wallet-balance kata.
```

## Symlink install for contributors

From the repo root:

```bash
mkdir -p ~/.hermes/skills/education
ln -sfn "$PWD/skills/bdk-dojo" ~/.hermes/skills/education/bdk-dojo
```

Use symlink mode if you plan to edit this repo and test changes locally.

## Verify

```bash
hermes skills list | grep bdk-dojo
```

If the current chat session does not see the skill, start a new session. Skill loading is session-scoped.
