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

# Ein VACUUM braucht eine exklusive Sperre auf der ganzen DB und hält sie
# für die Dauer des Umschreibens (~eine Minute bei 1 GB). Läuft opencode
# dabei (es hält die DB im WAL-Modus offen), blockieren dessen Schreibvorgänge
# hinter der Sperre und brechen als "Failed to execute statement" ab. Deshalb:
# VACUUM nur, wenn niemand die DB offen hält (Wochenende/Schließen der App),
# niemals neben einem laufenden opencode.
db_in_use() {
  [ -f "$DB" ] || return 1
  if command -v fuser >/dev/null 2>&1; then
    fuser -s "$DB" 2>/dev/null
    return $?
  fi
  pgrep -f 'ai.opencode.desktop|opencode serve' >/dev/null 2>&1
}

vacuum_if_bloated() {
  [ -f "$DB" ] || return 0
  if db_in_use; then
    echo "[vacuum] $(date -Is) übersprungen — opencode hält die DB offen" >>"$LOG"
    return 0
  fi
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
