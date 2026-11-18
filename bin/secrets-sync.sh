#!/usr/bin/env bash
# secrets-sync.sh — spiegelt .secrets.local nach GitHub Actions Secrets
# (repo omegaflow/omegaflow).
#
# Modell (2026-09-21): GitHub hält
#   (1) EINEN vollständigen Spiegel `OMEGAFLOW_SECRETS_FILE` = base64(.secrets.local)
#   (2) nur die von Workflows gelesenen Einzel-Secrets (damit sie sie lesen).
# Das umgeht das harte 100-Secret-Cap pro Repo: der ganze lokale Bestand liegt
# in einem Secret, nicht in hundert.
#
# Der 0-Kanon: ein leerer Wert in .secrets.local ist `absent`, kein Secret —
# leere Werte werden übersprungen, nie als leeres Secret geschrieben.
#
# Usage:
#   bin/secrets-sync.sh            — trocken: nur berichten, nichts schreiben
#   bin/secrets-sync.sh --set      — schreiben (gh secret set)
set -u

REPO="${OMEGAFLOW_SECRETS_REPO:-omegaflow/omegaflow}"
SECRETS="${OMEGAFLOW_SECRETS_FILE:-.secrets.local}"
WRITE=0
[ "${1:-}" = "--set" ] && WRITE=1

[ -f "$SECRETS" ] || { echo "secrets-sync: $SECRETS fehlt" >&2; exit 2; }

set_secret() {
  local name="$1" value="$2"
  if [ "$WRITE" -eq 1 ]; then
    if printf '%s' "$value" | gh secret set "$name" --repo "$REPO" >/dev/null 2>&1; then
      echo "set   $name"
    else
      echo "fail  $name — gh secret set returned void" >&2
    fi
  else
    echo "would $name"
  fi
}

# 1. Der vollständige Spiegel (base64, damit Zeilenumbrüche erhalten bleiben).
set_secret OMEGAFLOW_SECRETS_FILE "$(base64 -w0 "$SECRETS")"

# 2. Die von Workflows gelesenen Einzel-Secrets, aus .secrets.local gespeist.
used="$(grep -rhoE 'secrets\.[A-Z0-9_]+' .github 2>/dev/null \
  | sed 's/^secrets\.//' | sort -u | grep -vx 'GITHUB_TOKEN' || true)"
for name in $used; do
  value="$(awk -F= -v k="$name" '$1==k {sub(/^[^=]*=/,""); print; exit}' "$SECRETS")"
  [ -n "$value" ] || continue
  set_secret "$name" "$value"
done

echo "---"
[ "$WRITE" -eq 0 ] && echo "secrets-sync: Trockenlauf — mit --set schreiben"
