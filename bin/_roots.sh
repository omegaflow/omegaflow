# Declared roots (Rat + Operator-Wort 2026-10-14). Sourced by the bin/ wrappers,
# never a build. The public workspace holds the growing untracked-public matter
# (data, recherche); the private register repo lives OUTSIDE the public tree.
# Both are resolved here once — a path is never re-invented by a tool.
: "${OMEGAFLOW_WORK:=$HOME/projects/omegaflow-work}"
: "${OMEGAFLOW_PRIVATE:=$HOME/projects/omegaflow-personal}"
export OMEGAFLOW_WORK OMEGAFLOW_PRIVATE
