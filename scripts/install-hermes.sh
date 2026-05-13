#!/usr/bin/env bash
set -euo pipefail

HERMES_BIN="${HERMES_BIN:-hermes}"
CATEGORY="${HERMES_SKILL_CATEGORY:-education}"
TEACH_MODE_REF="${TEACH_MODE_REF:-v0.0.1}"
BDK_DOJO_REF="${BDK_DOJO_REF:-v0.0.2}"

TEACH_MODE_URL="https://raw.githubusercontent.com/SaucePackets/teach-mode-skill/${TEACH_MODE_REF}/skills/teach-mode/SKILL.md"
BDK_DOJO_URL="https://raw.githubusercontent.com/SaucePackets/bdk-dojo-skill/${BDK_DOJO_REF}/skills/bdk-dojo/SKILL.md"

if ! command -v "$HERMES_BIN" >/dev/null 2>&1; then
  echo "Hermes CLI not found: $HERMES_BIN" >&2
  echo "Install Hermes first: https://hermes-agent.nousresearch.com/docs" >&2
  exit 1
fi

echo "Installing Teach Mode from ${TEACH_MODE_REF}..."
"$HERMES_BIN" skills install "$TEACH_MODE_URL" --category "$CATEGORY" --yes --force

echo "Installing BDK Dojo from ${BDK_DOJO_REF}..."
"$HERMES_BIN" skills install "$BDK_DOJO_URL" --category "$CATEGORY" --yes --force

echo
echo "Installed skills:"
SKILL_LIST=$("$HERMES_BIN" skills list)
echo "$SKILL_LIST" | grep -E 'teach-mode|bdk-dojo' || true

if ! echo "$SKILL_LIST" | grep -q 'teach-mode'; then
  echo "Teach Mode was not visible after install." >&2
  echo "Check: $HERMES_BIN skills list" >&2
  exit 1
fi

if ! echo "$SKILL_LIST" | grep -q 'bdk-dojo'; then
  echo "BDK Dojo was not visible after install." >&2
  echo "Check: $HERMES_BIN skills list" >&2
  exit 1
fi

echo
echo "Done. Start a new Hermes session, then ask:"
echo "Use teach-mode and bdk-dojo. Give me the first wallet-balance kata."
