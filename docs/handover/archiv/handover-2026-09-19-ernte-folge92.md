<!--
  title: Handover — Ernte-Folge 92 (Stand 2026-09-19)
  session: Ernte-Folge 92
  class: handover
  date: 2026-09-19
  sha256: 51b31ac2a0c776f675dcee2d7b8c517c77aa6822bd0c67b416b642f5a9517c46
  status: live
-->
# Handover — Ernte-Folge 92 (2026-09-19)

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
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-19)

- **HEAD** `b374736c` (== origin/main, Forschung-Folge 89, während dieser Session
  von einer fremden Linie gesetzt). Safety-Net `refs/safety/1789824224`.
- **Postfach** — `docs/zustand/external-state.md:20` (Ernte-Folge 91): jüngster
  Ledger-Eingang `1789795811` (Rubin-Forum, informativ), keine Sonden-Antwort.
  Kein neuer Eingang seit folge91 gemessen.
- **CI** — `docs/zustand/external-state.md:22` (Forschung-Folge 90, `ci_manage
  list`/`view` 14:47Z): in_progress `measure-gates` `35445637806` @1c728300,
  `te-gate` `35427414837` @fe6bdb2b; pending `ci-check` `35448988400` @b374736c;
  failure `ci-check` `35445591031` @75b88db6 (fmt-Drift
  `tools/utils/src/bin/archive_search.rs`, fremde Linie). Eintrag aktuell — zitiert,
  nicht kopiert.

## Source-Port — offene Arme (härtester undatiert zuerst)

- **Cassini RSS Arme — Erstlauf offen** `phi/harvest.φ:10-29` (cassini_odf,
  cassini_rsr `asset fehlt`). Gemessen 2026-09-19: RSR-Record 4260 B (SFDU C997,
  agg CHDO 1/232, sec CHDO 104/220, data CHDO 10/4000), 1000 Wörter @[260..4260)
  (Q high16/I low16); **SAMPLE RATE u16 BE @[70..72) ksps**, SAMPLE RESOLUTION
  @[68] bit, SFDU-SECOND f64 @[80..88) = **erster Sample** (Folge +1.0 s bei
  1 ksps). ODF TRK-2-18 (36 B) in `scc*_ddd/odf/`; RSR `.3a1/.2a1/.3b1/.2b1` in
  `scc*_ddd/rsr/`. Arme gebaut: `src/archivar/cassini_rsr.rs` (+extract-Dispatch),
  `tools/harvest/src/bin/cassini_{odf,rsr}_compiler.rs`,
  `.github/workflows/cassini-{odf,rsr}-cdn.yml`. **Schritt:** nach Push
  `gh workflow run cassini-odf-cdn.yml` + `gh workflow run cassini-rsr-cdn.yml`;
  dann Größe/sha256 aus dem Run messen und die `sources.φ`-Blöcke (`Asset fehlt`
  → present) + `harvest.φ` auf `asset present` fortschreiben.
- **gedi_l2a** `phi/harvest.φ:36-44` — `blockiert` (Code). Hang in
  `src/archivar/hdf5.rs` `gather_messages` (v1-Cont-Loop `:652`, v2 `:715`);
  Post an bau in `docs/handover/post.md`. (Schritt: bau-Fix.)
- **icesat2_atl03** `phi/harvest.φ:54-62` — `blockiert` (Budget). ci_watchdog
  cancelt (1598 s > 2× Median 71 s); 215 Granules à ~100 min, `--limit 2` seriell
  ohne Offset. Post an bau. (Schritt: `--limit 1` + `--skip <k>` in
  `icesat2_atl03_compiler.rs`.)
- **Voyager RSS PPI** `phi/blocked_sources.φ:35-37` — `pending`; `.ODR` present als
  `voyager_odr`, Reihen-Leser-Arm + sample_count-Semantik offen. (Schritt: Arm
  bauen, PDS3-Label START_TIME → Jahr.)
- **FUGIN Bulk** `phi/blocked_sources.φ:43-45` — 269 Cube-Assets nicht einzeln
  verifiziert. (Schritt: `gh release view fugin.nao.ac.jp` / Sniff.) `wartend`.

## Wartend (kein Auswahlpunkt)

- **Lasair-LSST** `external-state.md:23` — direct + Proton 502, Wayback 200 ohne
  Snapshot; nächste Wiedervorlage / Banner-Wechsel. `wartend`.
- **Voyager/Mariner/Viking/Juno/Cassini-Antworten** `blocked_sources.φ` — Anfragen
  offen. `wartend`.
- **BepiColombo bc_mpo_more** `blocked_sources.φ:30-33` — Freigabe ~April (PSA/Iess
  bestätigt). `wartend`.
- **Limadou PI-Freigabe** `ledger.φ` — per-act consent (Operator). `operator-gebunden`.
- **Queue-Korpora** `ledger.φ:82-120` — `--port` braucht das Operator-Wort für den
  lokalen Release-Binär-Lauf. `operator-gebunden`.

## Benchmark

- **flash-first:** ODF-Arm-Bau über `grind-flash` (cargo check 0/0, kein Doppel-Lauf).
  RSR-Zeitbasis über `grind-pro` (Format-/Label-Urteil, vier Deliverables geliefert);
  `research-max` scheiterte am DeepSeek-API-Timeout (transient), kein inhaltlicher
  Fehlschlag. Kein pro/max-Doppel-Lauf nötig.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/archivar/cassini_rsr.rs`, `src/archivar/mod.rs`,
  `src/archivar/extract.rs`, `tools/harvest/src/bin/cassini_odf_compiler.rs`,
  `tools/harvest/src/bin/cassini_rsr_compiler.rs`,
  `.github/workflows/cassini-odf-cdn.yml`,
  `.github/workflows/cassini-rsr-cdn.yml`, `phi/blocked_sources.φ`, `phi/harvest.φ`,
  `phi/sources.φ`, die neue `handover-2026-09-19-ernte-folge92.md`,
  `docs/handover/archiv/handover-2026-09-19-ernte-folge91.md` (verschoben).
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md` (Forschung-Folge 90),
  `docs/handover/post.md`, `src/archivar/weberin_verdicts.rs`, `src/gate/commit_gate.rs`,
  `src/gate/commit_gate_vocab.json`, `src/mathematikerin/te.rs`,
  `tools/measure/src/bin/{corona_conditional_probe,multi_force_te_probe,placebo_pair_eeg_probe,silence_map_probe}.rs`,
  die bau/forschung-Handover-Moves. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Neue Workflows
(`cassini-odf-cdn.yml`, `cassini-rsr-cdn.yml`) → nach dem Push dispatchen
(`gh workflow run`), Ergebnis nie pollen (Watchdog-Snapshot).
