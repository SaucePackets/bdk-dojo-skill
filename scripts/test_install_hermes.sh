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
  exit 0
fi
if [ "$1" = "skills" ] && [ "$2" = "list" ]; then
  case "${FAKE_LIST_MODE:-both}" in
    both)
      printf '│ teach-mode │ education │ url │ community │ enabled │\n'
      printf '│ bdk-dojo   │ education │ url │ community │ enabled │\n'
      ;;
    no-teach)
      printf '│ bdk-dojo   │ education │ url │ community │ enabled │\n'
      ;;
    no-bdk)
      printf '│ teach-mode │ education │ url │ community │ enabled │\n'
      ;;
    none)
      ;;
  esac
  exit 0
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
  export HERMES_BIN="$tmp/hermes"
  export TEACH_MODE_REF="test-teach"
  export BDK_DOJO_REF="test-bdk"
  bash "$INSTALLER" > "$tmp/out"

  grep -q 'teach-mode-skill/test-teach/skills/teach-mode/SKILL.md' "$FAKE_HERMES_LOG"
  grep -q 'bdk-dojo-skill/test-bdk/skills/bdk-dojo/SKILL.md' "$FAKE_HERMES_LOG"

  local teach_line bdk_line
  teach_line=$(grep -n 'teach-mode-skill' "$FAKE_HERMES_LOG" | head -1 | cut -d: -f1)
  bdk_line=$(grep -n 'bdk-dojo-skill' "$FAKE_HERMES_LOG" | head -1 | cut -d: -f1)
  if [ "$teach_line" -ge "$bdk_line" ]; then
    echo "Teach Mode must install before BDK Dojo" >&2
    exit 1
  fi
}

run_failure_case() {
  local mode="$1"
  local expected="$2"
  local tmp
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' RETURN
  make_fake_hermes "$tmp"
  export FAKE_HERMES_LOG="$tmp/hermes.log"
  export HERMES_BIN="$tmp/hermes"
  export FAKE_LIST_MODE="$mode"
  if bash "$INSTALLER" > "$tmp/out" 2> "$tmp/err"; then
    echo "installer should fail for FAKE_LIST_MODE=$mode" >&2
    exit 1
  fi
  grep -q "$expected" "$tmp/err"
}

run_missing_hermes_case() {
  local tmp
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' RETURN
  export HERMES_BIN="$tmp/does-not-exist"
  if bash "$INSTALLER" > "$tmp/out" 2> "$tmp/err"; then
    echo "installer should fail when Hermes is missing" >&2
    exit 1
  fi
  grep -q 'Hermes CLI not found' "$tmp/err"
}

run_success_case
run_failure_case no-teach 'Teach Mode was not visible after install.'
run_failure_case no-bdk 'BDK Dojo was not visible after install.'
run_missing_hermes_case

echo "OK: install-hermes.sh fake-Hermes tests passed"
