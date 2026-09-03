#!/usr/bin/env bash
# Der Vacuum-Wächter: holt die freien Seiten der opencode.db zurück, die
# opencode beim Session-Räumen hinterlässt (opencode VACUUMt nicht selbst,
# siehe Issue #33356). Tokenfrei: nur lokale sqlite3-Queries, kein Netz,
# keine LLM-API. Läuft still im Hintergrund.
# Signal ist die Freiliste: Löschungen geben Seiten frei, normales
# Session-Schreiben nicht. Steigt die Freiliste über die Schwelle, wird
# VACUUM gezogen; ist die DB gerade beschäftigt, bleibt sie und der
# nächste Poll versucht es erneut.
set -u
DB="${OPENCODE_DB:-$HOME/.local/share/opencode/opencode.db}"
POLL_S="${OPENCODE_VACUUM_POLL:-10}"
FLOOR_MB="${OPENCODE_VACUUM_FLOOR_MB:-4}"
FLOOR=$((FLOOR_MB * 1024 * 1024))
LOG="${OPENCODE_VACUUM_LOG:-/tmp/opencode/opencode_vacuum.log}"

vacuum_if_bloated() {
  [ -f "$DB" ] || return 0
  local out ps fc free
  out=$(sqlite3 "$DB" "PRAGMA page_size; PRAGMA freelist_count;" 2>/dev/null) || return 0
  ps=$(printf '%s\n' "$out" | sed -n '1p')
  fc=$(printf '%s\n' "$out" | sed -n '2p')
  [ -n "$ps" ] && [ -n "$fc" ] || return 0
  [ "$fc" -gt 0 ] || return 0
  free=$((fc * ps))
  [ "$free" -ge "$FLOOR" ] || return 0
  local mb=$((free / 1024 / 1024))
  echo "[vacuum] $(date -Is) $mb MiB freie Seiten -> VACUUM" >>"$LOG"
  if sqlite3 -cmd ".timeout 3000" "$DB" "VACUUM;" >>"$LOG" 2>&1; then
    echo "[vacuum] $(date -Is) fertig" >>"$LOG"
  else
    echo "[vacuum] $(date -Is) verschoben (db beschäftigt) — nächster Poll versucht es erneut" >>"$LOG"
  fi
}

mkdir -p "$(dirname "$LOG")"
vacuum_if_bloated
while true; do
  sleep "$POLL_S"
  vacuum_if_bloated
done
