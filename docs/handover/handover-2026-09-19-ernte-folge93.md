<!--
  title: Handover — Ernte-Folge 93 (Stand 2026-09-19)
  session: Ernte-Folge 93
  class: handover
  date: 2026-09-19
  sha256: 70a8002ad8dc194f2793b5719efe92229c1948c0cd64197b6123d58bb0bf4422
  status: live
-->
# Handover — Ernte-Folge 93 (2026-09-19)

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

- **HEAD** `5cba9f70` (== origin/main). Safety-Net `refs/safety/1789834166`.
- **Postfach** — `docs/zustand/external-state.md:20` (Ernte-Folge 91): jüngster
  Ledger-Eingang `1789795811` (Rubin-Forum, informativ), keine Sonden-Antwort.
  `docs/handover/post.md` trägt eine fremde bau-Zeile (`ci-check` `35451506666`),
  nicht ernte.
- **CI** — Watchdog-Snapshot `2026-09-19T17:56Z`: in_progress `ci-check`
  `35451506666` (failure attempt 1), `health-check` `35436939441`, `te-gate`
  `35427414837`, `harvest` `35426127660`; failed `harvest-dispatch` `35451187565`
  (in dieser Session gefixt), `ci-check` `35448988400`/`35445628457`/`35445591031`,
  `harvest` `35426123577`. Der Eintrag `external-state.md:22` ist @b374736c veraltet
  — zitiert, nicht kopiert.

## Source-Port — offene Arme (härtester undatiert zuerst)

- **Cassini ODF+RSR — Erstlauf 2026-09-19 leer** `phi/harvest.φ:10-29`,
  `phi/sources.φ:6919-6936`. Läufe `cassini-odf-cdn` `35451207386` /
  `cassini-rsr-cdn` `35451218520` grün, aber 0 Volumes/0 Records (atmos-Fetch
  `curl: (56) Connection reset by peer`), kein Asset hochgeladen. Ursachen in
  dieser Session gefixt: Compiler-Fehlgrün (Fetch-Ausfall als „0 honored", exit 0)
  in `cassini_odf_compiler.rs`/`cassini_rsr_compiler.rs`; RSR-Crawl-Bug (absoluter
  `pds-app-bar`-Href als „Datei"); `harvest-dispatch`-422 (Ziel-Workflows ohne
  `format`-Input). Route jetzt offen (`archive_search --verdict
  https://atmos.nmsu.edu/pdsd/archive/data/` → stage 1 HTTP 200). **Schritt:**
  nach Push `gh workflow run cassini-odf-cdn.yml` + `gh workflow run
  cassini-rsr-cdn.yml`; Größe/sha256 aus dem Run messen und `sources.φ`
  (sha256-Zeile) + `harvest.φ` (`asset present`) fortschreiben.
- **gedi_l2a** `phi/harvest.φ:57-65` — `blockiert` (Code). `src/archivar/hdf5.rs`
  ist im Working Tree fremd geändert (bau, gedi-Fix). Ernte-Schritt: keiner (an bau).
- **icesat2_atl03** `phi/harvest.φ:75-83` — `blockiert` (Budget).
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs` ist im Working Tree fremd
  geändert (bau). Ernte-Schritt: keiner (an bau).
- **Voyager RSS PPI** `phi/blocked_sources.φ:35-37` — `pending`; `.ODR` present als
  `voyager_odr`, Reihen-Leser-Arm + sample_count-Semantik offen. (Schritt: Arm
  bauen, PDS3-Label START_TIME → Jahr.) Eigen (Code).
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
- **Queue-Korpora** `ledger.φ:82-120` — `--port` braucht das Operator-Wort. `operator-gebunden`.

## Benchmark

- **flash-first:** Compiler-Fix (Fetch-Ausfall-Hard-Abort + RSR-Crawl-Filter) und
  `harvest-dispatch`-Input-Fix über `grind-flash` (`cargo check` 0/0). Die Messung
  des Fehlgrüns (Cassini-Assets fehlen, Läufe leer) über `grind-flash`; die
  `harvest-dispatch`-Diagnose über `general` — beide flash, kein pro/max-Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/harvest/src/bin/cassini_odf_compiler.rs`,
  `tools/harvest/src/bin/cassini_rsr_compiler.rs`,
  `.github/workflows/harvest-dispatch.yml`, `phi/harvest.φ`, `phi/sources.φ`, die
  neue `handover-2026-09-19-ernte-folge93.md`,
  `docs/handover/archiv/handover-2026-09-19-ernte-folge92.md` (verschoben).
- **Fremd (nicht anfassen):** `src/archivar/hdf5.rs`, `src/mathematikerin/te.rs`,
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs`, `tools/measure/src/eeglab.rs`,
  `tools/measure/src/bin/hyperscanning_te_matrix.rs`, `docs/handover/post.md`,
  `docs/zustand/external-state.md`,
  `docs/handover/handover-2026-09-19-entscheid-folge53.md`, die
  entscheid/forschung/bau-Handover-Moves, `bin/register_lookup`. Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Nach dem Push die zwei
cassini-Workflows dispatchen (`gh workflow run`), Ergebnis nie pollen
(Watchdog-Snapshot).
