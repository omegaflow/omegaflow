# Declared roots (Rat + Operator-Wort 2026-10-14; erweitert 2026-10-19).
# Sourced by the bin/ wrappers, never a build. Die Werkstatt haelt zwei Speicher:
# `sources/` (CDN-gebunden, rekonstruierbar) und `experiments/` (nur lokal).
# Das private Register-Repo liegt AUSSERHALB des oeffentlichen Baums. Alle Wurzeln
# werden hier einmal aufgeloest — kein Werkzeug erfindet einen Pfad.
: "${OMEGAFLOW_WORK:=$HOME/projects/omegaflow-work}"
: "${OMEGAFLOW_PRIVATE:=$HOME/projects/omegaflow-private}"
: "${OMEGAFLOW_TMP:=$OMEGAFLOW_WORK/scratch}"
: "${OMEGAFLOW_ZUSTAND:=$OMEGAFLOW_WORK/state}"
: "${OMEGAFLOW_EXPERIMENTS:=$OMEGAFLOW_WORK/experiments}"
: "${OMEGAFLOW_SOURCES:=$OMEGAFLOW_WORK/sources}"
: "${OMEGAFLOW_CACHE:=$OMEGAFLOW_SOURCES}"
: "${OMEGAFLOW_SPILL_DIR:=$OMEGAFLOW_WORK/research/tmp/archive_search/search}"
export OMEGAFLOW_ZUSTAND OMEGAFLOW_WORK OMEGAFLOW_PRIVATE OMEGAFLOW_TMP OMEGAFLOW_EXPERIMENTS OMEGAFLOW_SOURCES OMEGAFLOW_CACHE OMEGAFLOW_SPILL_DIR
