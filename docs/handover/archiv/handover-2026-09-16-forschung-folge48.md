<!--
  title: Handover — Forschung-Folge 48 (2026-09-16)
  session: Forschung-Folge 48
  class: handover
  date: 2026-09-16
  sha256: 1505e7c266661d9583e71e2ccc3d34da7bad4e0d79de0fef411e421600611738
  status: live
-->
# Handover — Forschung-Folge 48 (2026-09-16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Paper / §4.5 — Monats-`posfrac` (härtester undatiert)

- Der Monats-Block-Produzent der `aia_ladder_probe` lag **uncommittet** im Baum
  (gemessen: `git log -S "posfrac"` leer; der Lauf `35140498561`, headSha
  `43af521f`, trägt den Block darum nicht). Jetzt committet; `aia-ladder-probe`
  auf dem neuen HEAD neu dispatcht (Workflow `aia-ladder-probe.yml`). (Schritt:
  `gh run list --workflow=aia-ladder-probe.yml --limit 1` für die Run-Nr; bei
  success `gh run download <nr> --name aia-ladder-2015`, den Monats-Block
  `JJJJ-MM | N events | posfrac F (pos/tot) | mean D` lesen und `MIN–MAX` in
  `docs/paper/corona-heating-ladder.md:317–320` setzen; kein Polling.)

## neptune_c — Rift-Probe-CI-Lauf

- Beschaffung/Komposition sind gebaut (`neptune-c-spk-cdn.yml`; Artefakt
  `ephemeris_neptune_c.bin` CDN HTTP 200, 17 181 776 B). Offen war nur der
  Rift-Probe-Lauf — Workflow `neptune-center-rift.yml` neu gebaut und dispatcht.
  (Schritt: `gh run list --workflow=neptune-center-rift.yml --limit 1` für die
  Run-Nr; `gh run download <nr> --name neptune-center-rift`,
  `state/reports/neptune_center_rift.txt` lesen, den Befund ins Register/Papier
  tragen.)

## ODF-Bande-Split — Konsument

- Lauf `planetary-odf-cdn 35139594201` pending (queued, kein Job-Start; gemessen
  2026-09-16). (Schritt: `gh run view 35139594201 --log`, den gedruckten
  φ-Block nehmen und jeden ganzen 5-Zeilen-Block je Shard in
  `phi/sources.φ:6723–6733` ersetzen — `refuse_shard_overlaps` in
  `src/archivar/parse.rs` verweigert Überlappung gleichen `format`.)

## CI / geteilter Zustand

- CI-`format` rot — fremde unformatierte Dateien; die zwei forschung-eigenen
  Proben `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`
  bleiben pending (fmt ist ein funktionaler Lauf → CI). (Schritt:
  rustfmt-Diff aus dem `ci-check`-Lauf anwenden.)

## Paper / Präregistrierung

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: `docs/paper/flyby-path-2-preregistration.md`.)

## Delegation / Benchmark

- `neptune_c` an `research-max` (hartes Atom, mehrstufige Register-/Route-Frage):
  gemessen, dass die Kette bereits gebaut ist — die flash-Klasse ist geschlossen
  (2026-09-16), kein Re-Run. Rat (`council`) zur Abschluss-Entscheidung:
  einstimmig — den uncommitteten Probe-Hunk als eigenes Linien-Werk committen,
  pfad-begrenzt (der fremde gestagete Rename bleibt unberührt), neu dispatchen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
