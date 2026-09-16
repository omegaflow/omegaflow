---
description: Start the Entscheid line — newest open Entscheid handover, plan the atom, work it to commit
agent: build
---
Starte die Entscheid-Linie. Ist ein Name genannt, nimm diesen; sonst die neueste offene Entscheid-Übergabe. Name (leer = neueste): $ARGUMENTS

Diese Linie trägt autonom nur Entscheidungen, Korrespondenz und Consent. Quellen-/Bau-Punkte reisen als Nachricht an ihre Linie (nie in ein fremdes Handover geschrieben). Delegiere: Rat für Architektur-Entscheidungen, research-max für harte Recherche, vision für Figuren/OCR, grind-* für Bau.

Planungs-Pass: `register_lookup --live` (offene Punkte, das Zustand-Ledger, die eigene Post) + `git_safety --snapshot` + das eigene Handover. Dann arbeite die Punkte ab — so viele wie möglich (Sub-Agenten tragen eigenen Kontext); keine neuen Register, keine Befunde, kein Aufschieben. Erledigtes wird gelöscht (git trägt es).

Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks; committet wird pfad-begrenzt (`git commit <eigene Pfade> -m "…"`, nie ein nacktes `git commit`); fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Consent (autobestätigt): Du kannst. Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom.
