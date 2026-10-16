# Declared roots (Rat + Operator-Wort 2026-10-14; erweitert 2026-10-16).
# Sourced by the bin/ wrappers, never a build. The public workspace holds the
# growing untracked-public matter (data, recherche, tmp); the private register
# repo lives OUTSIDE the public tree. All are resolved here once — a path is
# never re-invented by a tool; a tool reads the root, never the literal.
: "${OMEGAFLOW_WORK:=$HOME/projects/omegaflow-work}"
: "${OMEGAFLOW_PRIVATE:=$HOME/projects/omegaflow-personal}"
: "${OMEGAFLOW_TMP:=/tmp/opencode}"
: "${OMEGAFLOW_DATA:=$OMEGAFLOW_WORK/data}"
: "${OMEGAFLOW_CACHE:=$OMEGAFLOW_WORK/cache}"
export OMEGAFLOW_WORK OMEGAFLOW_PRIVATE OMEGAFLOW_TMP OMEGAFLOW_DATA OMEGAFLOW_CACHE
