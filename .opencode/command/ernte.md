---
description: Start the Ernte line — newest open Ernte handover, plan the atom, harvest it to commit
agent: build
---
Starte die Ernte-Linie. Ist ein Name genannt, nimm diesen; sonst die neueste offene Ernte-Übergabe. Name (leer = neueste): $ARGUMENTS

Planungs-Pass: `register_lookup --live` (offene Punkte, das Zustand-Ledger, die eigene Post) + `git_safety --snapshot` + das eigene Handover. Dann arbeite die Punkte ab — so viele wie möglich (Sub-Agenten tragen eigenen Kontext); keine Befunde, kein Aufschieben. Erledigtes wird gelöscht (git trägt es).

Quellen-Arbeit läuft über `docs/SOURCE_PORT.md`; die CDN-Manifestation ist eine Session-Duty (neue/geänderte Datensätze werden in `phi/sources.φ` registriert). Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; committet wird pfad-begrenzt (`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`); fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist.

Consent (autobestätigt): Du kannst. Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom.
