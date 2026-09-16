<!--
  title: Handover — Bau-Folge 56 (Stand 2026-09-16)
  session: Bau-Folge 56
  class: handover
  date: 2026-09-16
  sha256: a1804d661af2ccc5fc03987b86cfdce7a5de64a29c9f28200ee132f2ef7e64d1
  status: live
-->
# Handover — Bau-Folge 56 (2026-09-16)

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

## TE-Gate-Arx — der Lauf trägt die konditionalen Gates; pending

- te-gate-Lauf `35139361939` @ `e28abbdf` ist `pending` (no jobs, gemessen
  2026-09-16); der Vorgänger `35139346318` ist `cancelled`. Er trägt
  `gate_fpr_autocorrelation_phase_null_binned_n_1000`, `block_sweep_n1000` und die
  konditionalen Arx-Gates. (Schritt: `gh run view 35139361939` — grün: Punkte zu;
  rot: der Assert nennt die Zelle.)
- Block-Null: hält eine Blocklänge den FPR-Anstieg ≤ 2pp → sie bleibt; hält keine →
  `multi_force_te_probe.rs:74` auf Arx, Block ausmustern. (Schritt: den
  `block_sweep_n1000`-Abschnitt des Laufs lesen.)

## ODR — Voyager-Serie sharded, Lauf in_progress

- Lauf `35143340703` lädt die 484 `.ODR`-Dateien in ~1-GiB-Shards
  (`voyager_odr_s{ord}.bin`); der Serien-Job ist `in_progress` (gemessen
  2026-09-16), noch keine Shard hochgeladen. (Schritt: `gh run view 35143340703`,
  dann Shard-Zeilen in `phi/sources.φ`.)
- Offen bleiben: Galileo 12-bit kein Record im PPI-Archiv; `year_full` 00–89
  `None` (Jahrhundert ungemessen); Voyager Decimation>1 unverifiziert. (Schritt:
  `sgrep "year_full\|decimation" src/archivar/galileo_odr.rs src/archivar/voyager_odr.rs`.)

## las — kein Asset, kein Konsument; die Registrierung war eine falsche Prämisse

- Gemessen 2026-09-16 (grind-pro): `ff69e51f` trägt nur den CRS-Dekoder
  (`src/archivar/las/mod.rs`, `projection_crs`); es gibt **keinen** las-Compiler,
  **kein** `las-cdn.yml`, **kein** CDN-Release und **keine** Format-Verzweigung
  (`geo.rs` `magic_of`/`comp_max`, `extract.rs` `series_parse_bin` tragen kein
  `las`-Arm) — eine `format las`-Zeile würde null Kanäle liefern. (Schritt: den
  Konsumenten benennen und den `las`-Arm + Compiler/Workflow bauen; oder
  descopen mit Befund.)

## nexrad_level2 — Site-Anker gebaut, Re-Manifestation dispatcht

- Der Site-Anker (29-B-Block, `present`-Flag, stid+lat/lon/alt) ist gebaut
  (`ff69e51f`); das manifestierte `nexrad_level2.bin` trägt noch den alten
  8-B-Kopf → stale. Re-Manifestation dispatcht: nexrad-cdn run `35144618354`
  (queued). (Schritt: `gh run view 35144618354`; nach grünem Lauf die
  `standort`-Note in `phi/sources.φ:2451` entfernen.)

## CI-Format-Gate — fremde Linien

- Fremd unformatiert bleiben `src/archivar/vtscat.rs`,
  `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/src/bin/archive_search.rs` und die ernte-eigenen harvest-Compiler;
  `src/gate/commit_gate.rs` ist konform (gemessen 2026-09-16). (Schritt: die
  jeweilige Linie formatiert ihre eigene Datei; Post-Zeile in `post.md`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
