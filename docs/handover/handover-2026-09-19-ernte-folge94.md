<!--
  title: Handover — Ernte-Folge 94 (Stand 2026-09-19)
  session: Ernte-Folge 94
  class: handover
  date: 2026-09-19
  sha256: 180f70a0187ffdb225093d92b5544da9e81617209aa62cef2898ed3c7117c2cf
  status: live
-->
# Handover — Ernte-Folge 94 (2026-09-19)

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

- **HEAD** `88fb27ad` (== origin/main, `git_safety --close` am Atom-Ende; während der
  Session von anderen Linien vorangeschoben, Start `778c68db`). Safety-Net
  `refs/safety/1789836703` (Ernte-Folge 94 Start).
- **Postfach** — `docs/zustand/external-state.md:20`: jüngster Ledger-Eingang
  `1789795811` (Rubin-Forum AGN-DP2-Lightcurves, informativ), keine Sonden-Antwort.
  `post.md` trägt nur bau-Zeilen, keine ernte-Zeile. Unverändert.
- **CI** — Watchdog-Snapshot `2026-09-19T17:56Z` + `ci_manage list` (18:5xZ):
  `cassini-odf-cdn` `35455805362` / `cassini-rsr-cdn` `35455807084` **in_progress**,
  dazu viele `cancelled`/`pending` derselben Workflows (`35455835693`/`35455838104`)
  — Auto-Dispatch-Churn, kein stabiles Ergebnis; `measure-gates` `35456056999`,
  `hyperscanning-te` `35456288477`, `allwise-cdn` `35456069108` in_progress;
  `ci-check` `35456216825` pending. `external-state.md:22` @b374736c veraltet —
  zitiert, nicht kopiert (fremd).

## Source-Port — offene Arme (härtester undatiert zuerst)

- **lmd.jussieu TAP — Backend erholt, Port offen** `phi/pipeline/ledger.φ:14-16`.
  Re-Messung 2026-09-19: sync-QUERY 200, gültiges VOTable (tap_schema.tables
  beantwortet, QUERY_STATUS=OK). **Schritt:** Schema-Discovery
  (`http://vo.lmd.jussieu.fr/tap/sync?…TAP_SCHEMA.tables`), dann Kraft-Gate +
  Port über `docs/SOURCE_PORT.md`; bei Aufnahme `phi/sources.φ`-Block + CDN.
- **gedi_l2a** `phi/harvest.φ:57-65` — `blockiert` (Code). `src/archivar/hdf5.rs`
  ist im Working Tree fremd geändert (bau, gedi-Fix). Ernte-Schritt: keiner (an bau).
- **icesat2_atl03** `phi/harvest.φ:75-83` — `blockiert` (Budget).
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs` ist im Working Tree fremd
  geändert (bau). Ernte-Schritt: keiner (an bau).
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-12,18-20`
  — Re-Messung 2026-09-19: sync-QUERY VOTable-Error `localhost:5432 connection
  refused`; Root 200. **Schritt:** Re-Check (Backend-Erholung) via
  `archive_search --verdict` + sync-QUERY.
- **FUGIN Bulk** `phi/blocked_sources.φ:39-41` — 269 Cube-Assets nicht einzeln
  verifiziert. (Schritt: `gh release view fugin.nao.ac.jp` / Sniff.) `wartend`.

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — Re-Messung 2026-09-19:
  maxTime unverändert `2026-07-30T23:30:00Z`, Familie-Frontier `2026-07-31T00:00:00Z`
  (kein lebendes Geschwister). `termin`: nächste Re-Messung **2026-10-19**.

## Wartend (kein Auswahlpunkt)

- **Cassini ODF+RSR** `phi/harvest.φ:10-29`, `phi/sources.φ:6919-6936` — Re-Dispatch
  nach Push lief (`cassini-odf-cdn` `35455805362`, `cassini-rsr-cdn` `35455807084`),
  aber in_progress/Churn; kein Asset. Auslöser: Run-Abschluss. **Schritt:** Ergebnis
  aus Watchdog-Snapshot / `ci_manage view <id>` lesen; bei Erfolg Größe/sha256 messen
  und `sources.φ` (sha256-Zeile) + `harvest.φ` (`asset present`) fortschreiben.
  `wartend`.
- **Lasair-LSST** `external-state.md:23` — direct + Proton 502, Wayback 200 ohne
  Snapshot; nächste Wiedervorlage / Banner-Wechsel. `wartend`.
- **Voyager/Mariner/Viking/Juno/Cassini-Antworten** `blocked_sources.φ` — Anfragen
  offen. `wartend`.
- **BepiColombo bc_mpo_more** `blocked_sources.φ:30-33` — Freigabe ~April (PSA/Iess
  bestätigt). `wartend`.
- **Limadou PI-Freigabe** `ledger.φ:26-28` — per-act consent (Operator).
  `operator-gebunden`.
- **Queue-Korpora** `ledger.φ:82-120` — `--port` braucht das Operator-Wort.
  `operator-gebunden`.

## Benchmark

- **flash-first, Klasse geschlossen:** die vier Messungen (Voyager-Arm/Shards,
  CDN-Sniff goes16/ulysses, TAP-Re-Check, HFRADAR-maxTime) liefen über
  `grind-flash` — Routine-Messklasse ist geschlossen (2026-09-16: flash
  $0.0008–0.0017 gegen pro/max $0.0041–0.0090, identisches Ergebnis; Sieger
  `grind-flash`). Kein pro/max-Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/blocked_sources.φ`, `phi/pipeline/ledger.φ`, die neue
  `handover-2026-09-19-ernte-folge94.md`,
  `docs/handover/archiv/handover-2026-09-19-ernte-folge93.md` (verschoben).
- **Fremd (nicht anfassen):** `src/archivar/hdf5.rs`, `src/mathematikerin/te.rs`,
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs`, `tools/measure/src/eeglab.rs`,
  `tools/measure/src/bin/hyperscanning_te_matrix.rs`, `docs/handover/post.md`,
  `docs/zustand/external-state.md`,
  `docs/handover/handover-2026-09-19-entscheid-folge53.md`, die
  forschung/bau-Handover-Moves, `bin/register_lookup`. Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
