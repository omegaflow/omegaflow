#!/usr/bin/env bash
# The CI watchdog: reads the repo's GitHub Actions runs through bin/ci_manage
# (gh API, network-only — never the DB, never the opencode process, never a
# tracked doc), cancels only what does not flow, and reruns only a measured
# transient failure. Its state lives in its own log and seen-file; the
# CI-Status line stays the session's (one measurement, one author).
# Autostart: the systemd user timer ci-watchdog.timer runs this with --once
# every 64 min (install: ./bin/ci_watchdog.sh --install, then
# systemctl --user enable --now ci-watchdog.timer). A manual run is the same
# --once; without an argument the script loops itself.
set -u
cd "$(dirname "$0")/.." || exit 1
LOG="${CI_WATCHDOG_LOG:-/tmp/opencode/ci_watchdog.log}"
SEEN="${CI_WATCHDOG_SEEN:-/tmp/opencode/ci_watchdog.seen}"
POLL_S="${CI_WATCHDOG_POLL:-3840}"
CI="${CI_MANAGE:-./bin/ci_manage}"
SNAP="${CI_WATCHDOG_SNAPSHOT:-/tmp/opencode/ci_status.md}"
UNITSRC="$PWD/bin"
UNITDIR="$HOME/.config/systemd/user"
mkdir -p "$(dirname "$LOG")"
touch "$SEEN"

log() { echo "[ci_watchdog] $(date -Is) $*" >>"$LOG"; }
seen() { grep -qx "$1" "$SEEN" 2>/dev/null; }
mark() { echo "$1" >>"$SEEN"; }

# Median duration (s) of the last <=10 *successful* runs of one workflow,
# read from the single poll table. Failures are fast and would poison the
# floor; a run past 2x the success median is the one that does not flow.
# Fewer than two successes carry no median basis: one sample has no spread,
# and a single empty green run must not set the floor for every later run.
median_duration() {
  local wf="$1"
  printf '%s\n' "$rows" | awk -F'\t' -v w="$wf" '
    $5==w && $3=="success" && $6!="" && $6!="?" && $7!="" && $7!="?" {
      cmd="date -d \""$6"\" +%s"; cmd|getline s; close(cmd);
      cmd="date -d \""$7"\" +%s"; cmd|getline e; close(cmd);
      if (e>s) print e-s
    }' | head -10 | sort -n |
  awk '{a[NR]=$1} END{if(NR>=2) print a[int((NR+1)/2)]}'
}

poll_once() {
  local now
  now=$(date +%s)
  rows=$($CI list --limit 60 2>/dev/null) || {
    {
      printf '# CI status — %s\n' "$(date -Is)"
      printf '# list void — the gh-API answered void (rate limit or unreachable); no action\n'
    } > "$SNAP"
    log "list void — no action"
    return
  }

  # 1. cancel: an in_progress run past 2x its workflow's successful median.
  #    No successful history -> report, never guess a floor.
  printf '%s\n' "$rows" | awk -F'\t' '$2=="in_progress"{print $1"\t"$5"\t"$6}' |
  while IFS=$'\t' read -r id wf started; do
    [ -n "$id" ] && [ -n "$wf" ] && [ -n "$started" ] && [ "$started" != "?" ] || continue
    seen "$id" && continue
    local s med dur
    s=$(date -d "$started" +%s 2>/dev/null) || continue
    med=$(median_duration "$wf")
    if [ -z "$med" ]; then
      log "run $id in_progress ($wf), no median basis — fewer than two successful runs — no action"
      mark "$id"
      continue
    fi
    dur=$(( now - s ))
    if [ "$dur" -gt $(( 2 * med )) ]; then
      log "cancel $id ($wf): ${dur}s > 2x successful median ${med}s"
      $CI cancel "$id" >>"$LOG" 2>&1
      mark "$id"
      sleep 1
    fi
  done

  # 2. queued runs: report only. A queued follower is indistinguishable from
  #    a runner/concurrency queue in the list API — a cancel here would kill
  #    a correctly waiting run, so the measured state is logged, not acted on.
  printf '%s\n' "$rows" | awk -F'\t' '$2=="queued"{print $1"\t"$5"\t"$6}' |
  while IFS=$'\t' read -r id wf started; do
    [ -n "$id" ] || continue
    seen "$id" && continue
    log "queued $id ($wf) since ${started:-?} — reported, no action"
    mark "$id"
  done

  # 3. rerun: a failure at attempt 1 with a measured transient cause only —
  #    never an assertion (red is the null holding not), never a cap class.
  printf '%s\n' "$rows" | awk -F'\t' '$3=="failure" && $4=="1"{print $1"\t"$5}' |
  head -8 |
  while IFS=$'\t' read -r id wf; do
    [ -n "$id" ] || continue
    seen "$id" && continue
    local txt
    txt=$(gh run view "$id" --log-failed 2>/dev/null)
    if printf '%s' "$txt" | grep -qiE 'assertion|panicked at|test failed|test result: FAILED'; then
      log "no rerun $id ($wf): assertion-red"
      mark "$id"
      continue
    fi
    if printf '%s' "$txt" | grep -qiE 'runner has received a shutdown signal|timed out|could not resolve host|connection reset|curl: \('; then
      log "rerun $id ($wf): measured transient cause"
      $CI rerun "$id" >>"$LOG" 2>&1
      mark "$id"
      sleep 1
    else
      log "no rerun $id ($wf): cause not measured transient"
      mark "$id"
    fi
  done

  # The snapshot is the watchdog's sensor buffer: a session reads this file at
  # the planning pass (no API call, no polling). The tracked CI-Status line in
  # docs/zustand/external-state.md (the tracked pointer) names the ledgers true
  # home `state/zustand/external-state.md`; the session writes there.
  {
    printf '# CI status — %s (ci_watchdog poll, every %ss)\n' "$(date -Is)" "$POLL_S"
    printf '# read by sessions at the planning pass; never written by a session\n'
    printf '## active (in_progress/queued)\n'
    printf '%s\n' "$rows" | awk -F'\t' '$2=="in_progress"||$2=="queued"{print $1"\t"$2"\t"$5"\t"$6}'
    printf '## failed (attempt 1)\n'
    printf '%s\n' "$rows" | awk -F'\t' '$3=="failure"&&$4=="1"{print $1"\t"$5"\t"$6}'
  } > "$SNAP"
}

install_unit() {
  mkdir -p "$UNITDIR"
  cp "$UNITSRC/ci-watchdog.service" "$UNITDIR/ci-watchdog.service"
  cp "$UNITSRC/ci-watchdog.timer" "$UNITDIR/ci-watchdog.timer"
  systemctl --user daemon-reload >/dev/null 2>&1 || true
}

case "${1:-}" in
  --once)
    poll_once
    ;;
  --install)
    install_unit
    echo "ci-watchdog units installed — enable: systemctl --user enable --now ci-watchdog.timer"
    ;;
  *)
    while true; do
      poll_once
      sleep "$POLL_S"
    done
    ;;
esac
