#!/usr/bin/env bash
# secrets-sync.sh — spiegelt die lokalen Provider-Keys von .secrets.local nach
# GitHub Actions Secrets (repo omegaflow/omegaflow).
#
# Der 0-Kanon: ein leerer Wert in .secrets.local ist `absent`, kein Secret.
# Absent wird NIE als leerer GitHub-Secret geschrieben — die fünf leeren
# Platzhalter (RUBIN_*, SUPERMAG_PASS, TOAR_*) bleiben lokal leer und werden
# übersprungen, statt einen leeren Secret anzulegen.
#
# Usage:
#   bin/secrets-sync.sh            — trocken: nur berichten, nichts schreiben
#   bin/secrets-sync.sh --set      — schreiben (gh secret set)
#
# Case-Falle: GitHub-Secret-Namen sind case-sensitiv. Trägt GitHub bereits
# einen Namen, der sich nur in Groß-/Kleinschreibung vom lokalen Key
# unterscheidet (tedp_* lokal vs TEDP_* auf GitHub), wird der bestehende Name
# wiederverwendet statt ein Duplikat anzulegen.
set -u

REPO="${OMEGAFLOW_SECRETS_REPO:-omegaflow/omegaflow}"
SECRETS="${OMEGAFLOW_SECRETS_FILE:-.secrets.local}"
WRITE=0
[ "${1:-}" = "--set" ] && WRITE=1

[ -f "$SECRETS" ] || { echo "secrets-sync: $SECRETS fehlt" >&2; exit 2; }

# Bestehende GitHub-Secret-Namen (einmal, für die Case-Falle).
gh_names="$(gh secret list --repo "$REPO" 2>/dev/null | awk '{print $1}')"

existing_case() {
  # Gibt den exakten GitHub-Namen zurück, falls einer nur in Groß-/Klein-
  # schreibung abweicht; sonst leer.
  local key="$1" gh
  for gh in $gh_names; do
    [ "${gh,,}" = "${key,,}" ] && { printf '%s\n' "$gh"; return 0; }
  done
  return 1
}

setted=0
skipped_empty=0
skipped_unchanged=0
reused_case=0

while IFS= read -r line; do
  case "$line" in
    ''|'#'*) continue ;;
  esac
  key="${line%%=*}"
  key="${key%"${key##*[![:space:]]}"}"
  [ -n "$key" ] || continue
  value="${line#*=}"
  value="${value#"${value%%[![:space:]]*}"}"

  if [ -z "$value" ]; then
    echo "skip  $key (leer — absent, kein Secret)"
    skipped_empty=$((skipped_empty + 1))
    continue
  fi

  target="$key"
  if ! printf '%s\n' "$gh_names" | grep -qx "$key"; then
    if case_name="$(existing_case "$key")"; then
      echo "reuse $key -> $case_name (nur Case-Abweichung)"
      target="$case_name"
      reused_case=$((reused_case + 1))
    fi
  fi

  if [ "$WRITE" -eq 1 ]; then
    if printf '%s' "$value" | gh secret set "$target" --repo "$REPO" >/dev/null 2>&1; then
      echo "set   $target"
      setted=$((setted + 1))
    else
      echo "fail  $target — gh secret set returned void" >&2
    fi
  else
    echo "would $target"
    setted=$((setted + 1))
  fi
done < "$SECRETS"

echo "---"
echo "secrets-sync: $setted $( [ "$WRITE" -eq 1 ] && echo gesetzt || echo zu setzen ), $skipped_empty leer übersprungen, $reused_case case-wiederverwendet"
[ "$WRITE" -eq 0 ] && echo "secrets-sync: Trockenlauf — mit --set schreiben"
