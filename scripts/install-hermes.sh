#!/usr/bin/env bash
set -euo pipefail

HERMES_BIN="${HERMES_BIN:-hermes}"
CATEGORY="${HERMES_SKILL_CATEGORY:-education}"
TEACH_MODE_ID="${TEACH_MODE_ID:-SaucePackets/teach-mode-skill/skills/teach-mode}"
BDK_DOJO_ID="${BDK_DOJO_ID:-SaucePackets/bdk-dojo-skill/skills/bdk-dojo}"

if ! command -v "$HERMES_BIN" >/dev/null 2>&1; then
  echo "Hermes CLI not found: $HERMES_BIN" >&2
  echo "Install Hermes first: https://hermes-agent.nousresearch.com/docs" >&2
  exit 1
fi

install_skill() {
  local label="$1"
  local identifier="$2"

  printf 'Installing %s...\n' "$label"
  "$HERMES_BIN" skills install "$identifier" --category "$CATEGORY" --yes --force
}

install_skill "Teach Mode" "$TEACH_MODE_ID"
install_skill "BDK Dojo" "$BDK_DOJO_ID"

echo
echo "Done. Start a new Hermes session, then ask:"
echo "Use bdk-dojo. Give me the first wallet-balance kata."
