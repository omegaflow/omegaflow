<!--
  title: Handover — Register-Digest (Stand 2026-09-15)
  session: Register-Digest
  class: handover
  date: 2026-09-15
  sha256: 58654909ac256d5881c21545680dee66097a7f5409a88a74f0f9cbb8b684b933
  status: live
-->
# Handover — Register-Digest (2026-09-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## Den jetzt sichtbaren Überseh-Korpus abarbeiten

- `register_lookup --live` gebaut und in AGENTS verdrahtet: es liest alle lebenden
  Doc-Klassen (handover ohne `archiv/`, surveys, plans, auftrag, blatt, concepts,
  paper), druckt je Dokument Kopf (class/date/status) und jede offene Zeile per
  Marker; `descoped` ist eigene Klasse `released`, headerlose Dokumente
  `unverifiable`, Namens-Kollision live↔archiv wird geflaggt. Der Session-Start
  fährt `--live`, der Abschluss `--history`. Gemessen beim Bau: 106 Dokumente,
  638 offene Zeilen, 2 Namens-Kollisionen. (Schritt: den Korpus Zeile für Zeile in
  die jeweilige Linie arbeiten — je Punkt ein Atom, keine Ablage.)
- Die Historie-Suche (Repo + Legacy) hat den Umfang gemessen: ~25 konkrete offene
  Pflichten + ~28 weitere in surveys/auftrag/blatt/concepts/papers, die keine
  lebende Übergabe trägt; ~13 „in history, absent from tree" im aktuellen Repo
  (Nadeln Ⅷ–Ⅻ, AllWISE-Coverage, ned-Crawl, SPICE-`.bc`, Gaia-XP ring↔nest,
  Katalog-Lücken RAVE/APOGEE/HyperLEDA/TGSS/GLADE+, 101 NODD-Dispositionen,
  `vo-tap`-Push); ~8 im Legacy-Repo (Stigmergic Mycelium/Nostr, Retro-Manifestation,
  vertex-splat, NED-Totals, MIMIC-III-Waveform, Sample-Budget, Aberration).
  (Schritt: die drei verlorenen Wiedervorlagen 09-22/09-28/12-03 sind in
  `entscheid-folge6` wieder eingesetzt; die übrigen je Linie aufnehmen.)
- `--history`-Blindfleck: offene Zeilen, die in einer *umgeschriebenen* (nicht
  gelöschten) Datei verschwanden, sind für den `--diff-filter=D`-Scan unsichtbar
  (im Output als gemessene Grenze benannt). (Schritt: bei Bedarf ein
  `git log -S`-Modus je Thema.)
- Legacy-Repo hat 80 archivierte/gelöschte `docs/handover/*.md` ohne Gegenstück
  im aktuellen `docs/handover/archiv/`. (Schritt: `register_lookup --history
  --legacy $HOME/backup/archive/omegaflow/omegaflow-legacy`.)
- `--live`-Namensstimme (Rat): das Wort „live" reibt an der Quellen-Registerklasse
  `live`. (Schritt: Operator entscheidet Umbenennung `--open` oder bleibt.)

## repo_surveillance — Branch-Verdict fehlt

- `session-protokoll.md` §Wächter verlangt zusätzlich den Branch-Namen als eigene
  Verdict-Zeile; `tools/register/src/bin/repo_surveillance.rs:36-47` prüft nur den
  Remote. (Schritt: Branch in `git_verdict()` aufnehmen.)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
