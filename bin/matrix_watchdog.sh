#!/usr/bin/env bash
# Der Wächter der Matrix: hält den Hidden-Lauf am Leben.
# Stirbt die Maschine (Absturz, SIGKILL, Neustart), startet der Wächter sie neu;
# das Gedächtnis /tmp/omegaflow_matrix_state.bin trägt den Stand, der warme
# Boot misst in Minuten weiter. Läuft die Maschine, wacht er nur.
# Start (überlebt opencode-Abstürze): setsid ./bin/matrix_watchdog.sh &
set -u
cd "$(dirname "$0")/.." || exit 1
LOG="${MATRIX_LOG:-/tmp/opencode/omegaflow_matrix.log}"
STATION="${MATRIX_STATION:-41001}"
while true; do
  if ! pgrep -x omegaflow >/dev/null; then
    echo "[watchdog] $(date -Is) start #station=$STATION" >>"$LOG"
    OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow "#station=$STATION" >>"$LOG" 2>&1
    code=$?
    echo "[watchdog] $(date -Is) exit $code — neustart in 5 s" >>"$LOG"
    sleep 5
  else
    sleep 30
  fi
done
