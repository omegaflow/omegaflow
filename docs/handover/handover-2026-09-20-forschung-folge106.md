<!--
  title: Handover — Forschung-Folge 106 (Stand 2026-09-20)
  session: Forschung-Folge 106
  class: handover
  date: 2026-09-20
  sha256: cfc59be2e61e0e14da752c775f6a745cb15825eb7d21876994705caa6cc0bccc
  status: live
-->
# Handover — Forschung-Folge 106 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 106)

- **HEAD** `ebd0cfb9` (== `origin/main`, „ernte: register icesat2_atl03 dispatch
  run 35515194248 in handover 106"); `git_safety` Snapshot bei Session-Beginn.
  Der Baum trug bei Beginn fremde uncommittete Arbeit (ernte `phi/harvest.φ` +
  `handover-…-ernte-folge106.md`) — nicht angefasst; ernte hat sie im Verlauf
  selbst committet.
- **Postfach** — letzter Ledger-Eingang `1789906306` (Weiterleitung des
  Sotgiu-Limadou-Threads), davor `1789902016` (Tuxedo Ticket), `1789900094`
  (Framework), die Key-/Welcome-Eingänge. Keine neue forschung-eigene Post; die
  Zeile steht gemessen in `docs/zustand/external-state.md` (Entscheid-Folge 61).
- **CI** — `ci_manage list` (2026-09-20 ~14:05Z): **pending** `ci-check`
  `35515221806`; **in_progress** `harvest` `35515194248`, `te-gate` `35513982359`,
  `ci-check` `35513190719` @`7d0a1272`; **failure** `ci-check` `35510151014`,
  `release-build` `35513611936`; **success** `harvest-dispatch`, `quake-feeds-cdn`,
  `ned-cdn`, `ps1-cdn`, `allwise-cdn`; cancelled die übrige ci-check-Kette
  (per-ref-Concurrency). Die CI-Zeile in `external-state.md` fortgeschrieben.
- **Post** — die Zeile `An forschung` (force-gate B + Kernel-Riss) aus
  `post.md` abgeholt und hierher gefaltet (Zeile gelöscht).

## Kernel-Riss (force-gate B) — Messung offen, `wartend`-entschieden

Der Operator hat force-gate **B** entschieden (skip + Review, kein Default). Der
**Kernel-Riss** ist als Widerspruch zu **messen**, nie zu glätten:
`default_kernel_for` (`src/mathematikerin/force.rs:57-70`) und
`kernel_id_for_force` (`force.rs:33-40`) widersprechen sich für seismic-surface
(`erfc` vs. Kernel `1`), diffusion (`gaussian-inverse-square` vs. Kernel `3`) und
advective (`patch-levy` vs. Kernel `1`); live `sources.φ` folgt erfc.
`port.rs:1357/1509` trägt `UNCERTAIN`→Review. (Schritt: Messung/Probe bauen, den
Riss als `VerdictWord::Riss` mit beiden Zeugenlinien tragen — `grind-max`; die
Kernel-ID-Zuordnung zuerst gegen die Registry messen.)

## Datenbank-Ausbau — offene Kandidaten

`--reactome` und `--interpro` in diesem Atom gebaut: Module
(`tools/utils/src/bin/archive_search/{reactome,interpro}.rs`), Verdrahtung
(`archive_search.rs` arg-Parse + usage, `net.rs` `QUERY_MODES` + `run_lines`,
`web.rs` `MODES`, `server.rs` `MODES`), `docs/concepts/tools-map.md`;
`cargo check -p omegaflow-utils --all-targets` 0/0. JSON-Formen live gemessen
(2026-09-20): Reactome `results[].entries[].{stId,name,type,species[],databaseName,referenceIdentifier}`
(Highlight-Markup im `name` gefällt), InterPro
`results[].metadata.{accession,name,source_database,type,integrated}` (URL über
`source_database`). Noch offen: `--alphafold`
(`alphafold.ebi.ac.uk/api/prediction/<acc>`), `--entrez` (GenBank/SRA/GEO,
`eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi`), `--ena`, `--biomodels`,
`--nist`, `--cod` — alle HTTP 200 direct außer KEGG (Lizenz). (Schritt: nächsten
Kandidaten nach `docs/SOURCE_PORT.md` portieren, `grind-flash`.)

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` (in_progress, dispatcht 2026-09-20 13:36Z) — die beiden
vorigen Läufe des 3σ-Fixes endeten **cancelled** (`35475226890` @`5b406e16`,
`35473941500` @`9f48b5bd`), kein Verdikt; der n=1000-FPR-Boden ist damit
ungemessen. (Schritt: `ci_manage view 35513982359`; bei rot `ci_manage log`.)

## Flyby Path 2 — `termin`

Das versiegelte Blatt (`docs/paper/flyby-path-2-preregistration.md`, Seal
2026-08-22; `-falsification-metric-addendum.md`, Seal 2026-08-28) ist kein
Auftrag. Die präregistrierte Kette (plasma-pressure gradient, IMF-Bz, Kp, Swarm)
muss transit-time-korrigiert am Perigäum-Tubus gefüllt und als Auftrag
registriert werden — nach dem Ereignis (28./29.09.) ist die Vorhersage post-hoc.
Kanäle leben: `phi/sources.φ` RTSW mag/wind (`:158,164`), Kp (`:179`), Swarm
(`:6062-6078`). (Schritt: Auftrag in `docs/auftrag/`, dann die Kette füllen —
`research-max`.)

## API-Keys ablegen — `operator-gebunden`

Die Modi `--core` (keyless nutzbar), `--semanticscholar`, `--materialsproject`
sind gebaut und committet; ohne Key melden sie `pending` bzw. den 429. (Schritt:
`CORE_API_KEY` und `S2_API_KEY` aus `state/mail/mail_ledger.φ`, `MP_API_KEY` aus
`next-gen.materialsproject.org/api` in `.secrets.local`; dann
`archive_search --core/--semanticscholar/--materialsproject <q>` messen.) Gehört
zur `entscheid`-Linie.

## ernte / termin

- **MAG-Asset** — `blockiert` (ernte): Compiler nach `voyager_odr_compiler.rs`,
  dann `sources.φ` + CI-Manifestation.
- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027). **Flyby-Path-2-Abruf** —
  `termin` (28.09., s. o.).

## Benchmark (dieses Atom)

- Kein Doppellauf: das Atom war ein Routine-Port (reactome + interpro) plus ein
  CI-Dispatch — kein Architektur-Urteil. Die Klasse „Routine-Port" ist
  geschlossen (2026-09-16: `grind-flash` $0.0008, Sieger gegen pro/max); der
  Port lief mit `grind-flash`.

## Geteilter Baum — eigener Pfad-Satz

- `tools/utils/src/bin/archive_search.rs`, `…/archive_search/net.rs`,
  `…/archive_search/server.rs`, `…/archive_search/web.rs`,
  `…/archive_search/reactome.rs` (neu), `…/archive_search/interpro.rs` (neu),
  `docs/concepts/tools-map.md`, `docs/zustand/external-state.md`,
  `docs/handover/post.md`, `docs/handover/handover-2026-09-20-forschung-folge106.md`
  (neu), `docs/handover/archiv/handover-2026-09-20-forschung-folge105.md` (Move).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
