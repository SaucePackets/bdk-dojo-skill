# Install in Hermes

BDK Dojo depends on `teach-mode`. Use the installer to install both skills.

```bash
curl -fsSL https://raw.githubusercontent.com/SaucePackets/bdk-dojo-skill/main/scripts/install-hermes.sh | bash
```

Then start a new Hermes session and ask:

```text
Use bdk-dojo. Give me the first wallet-balance kata.
```

The installer installs:

- `SaucePackets/teach-mode-skill/skills/teach-mode`
- `SaucePackets/bdk-dojo-skill/skills/bdk-dojo`

Both are installed under `$HERMES_HOME/skills/education/`.

## Contributor symlink install

Use symlinks only if you are editing the repos locally and want Hermes to load your working copy.

From each repo root:

```bash
mkdir -p ~/.hermes/skills/education
ln -sfn "$PWD/skills/teach-mode" ~/.hermes/skills/education/teach-mode  # teach-mode-skill
ln -sfn "$PWD/skills/bdk-dojo" ~/.hermes/skills/education/bdk-dojo      # bdk-dojo-skill
```
