# Install in OpenClaw or another AI agent

BDK Dojo depends on the standalone Teach Mode skill. OpenClaw and non-Hermes runtimes should load both skills.

Load order:

1. Install/copy/load Teach Mode first: `SaucePackets/teach-mode-skill/skills/teach-mode`
2. Install/copy/load BDK Dojo second: `SaucePackets/bdk-dojo-skill/skills/bdk-dojo`
3. Start a fresh agent session.
4. Ask the agent to use both `teach-mode` and `bdk-dojo`.

Portable copy shape:

```bash
mkdir -p /path/to/agent/skills/teach-mode /path/to/agent/skills/bdk-dojo
cp -R /path/to/teach-mode-skill/skills/teach-mode/* /path/to/agent/skills/teach-mode/
cp -R /path/to/bdk-dojo-skill/skills/bdk-dojo/* /path/to/agent/skills/bdk-dojo/
```

Manual loading shape for agents without a skill directory:

```text
Load teach-mode-skill/skills/teach-mode/SKILL.md.
Load bdk-dojo-skill/skills/bdk-dojo/SKILL.md.
For the current lesson, load scaffold/<lesson>/README.md and stubs.rs.
Use learner-first behavior: learner attempts first, agent coaches, verifies, and only rescues when appropriate.
```

Keep this repo as the canonical public source. Runtime-specific paths should stay in install docs, not in the skill body.
