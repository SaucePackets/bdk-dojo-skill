#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
INSTALLER="$ROOT/scripts/install-hermes.sh"

make_fake_hermes() {
  local dir="$1"
  cat > "$dir/hermes" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "${FAKE_HERMES_LOG:?}"

if [ "$1" = "skills" ] && [ "$2" = "install" ]; then
  case "$3" in
    SaucePackets/teach-mode-skill/skills/teach-mode|SaucePackets/bdk-dojo-skill/skills/bdk-dojo)
      exit 0
      ;;
  esac
fi

printf 'unexpected fake hermes args: %s\n' "$*" >&2
exit 1
FAKE
  chmod +x "$dir/hermes"
}

run_success_case() {
  local tmp
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' RETURN
  make_fake_hermes "$tmp"
  export FAKE_HERMES_LOG="$tmp/hermes.log"
  HERMES_BIN="$tmp/hermes" bash "$INSTALLER" > "$tmp/out"

  grep -q 'Installing Teach Mode' "$tmp/out"
  grep -q 'Installing BDK Dojo' "$tmp/out"
  grep -q 'Use bdk-dojo. Give me the first wallet-balance kata.' "$tmp/out"
  grep -q 'skills install SaucePackets/teach-mode-skill/skills/teach-mode --category education --yes --force' "$FAKE_HERMES_LOG"
  grep -q 'skills install SaucePackets/bdk-dojo-skill/skills/bdk-dojo --category education --yes --force' "$FAKE_HERMES_LOG"

  local teach_line bdk_line
  teach_line=$(grep -n 'teach-mode-skill' "$FAKE_HERMES_LOG" | head -1 | cut -d: -f1)
  bdk_line=$(grep -n 'bdk-dojo-skill' "$FAKE_HERMES_LOG" | head -1 | cut -d: -f1)
  if [ "$teach_line" -ge "$bdk_line" ]; then
    echo "Teach Mode must install before BDK Dojo" >&2
    exit 1
  fi
}

run_override_case() {
  local tmp
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' RETURN
  make_fake_hermes "$tmp"
  export FAKE_HERMES_LOG="$tmp/hermes.log"
  HERMES_BIN="$tmp/hermes" HERMES_SKILL_CATEGORY="custom" bash "$INSTALLER" > "$tmp/out"
  grep -q -- '--category custom' "$FAKE_HERMES_LOG"
}

run_missing_hermes_case() {
  local tmp
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' RETURN
  if HERMES_BIN="$tmp/does-not-exist" bash "$INSTALLER" > "$tmp/out" 2> "$tmp/err"; then
    echo "installer should fail when Hermes is missing" >&2
    exit 1
  fi
  grep -q 'Hermes CLI not found' "$tmp/err"
}

run_success_case
run_override_case
run_missing_hermes_case

echo "OK: install-hermes.sh fake-Hermes tests passed"
