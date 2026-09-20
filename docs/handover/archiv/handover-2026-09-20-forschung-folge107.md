<!--
  title: Handover — Forschung-Folge 107 (Stand 2026-09-20)
  session: Forschung-Folge 107
  class: handover
  date: 2026-09-20
  sha256: 41fd801db01257fc0898ff760566900a3602c766f6f301e543c6572027b91c4e
  status: live
-->
# Handover — Forschung-Folge 107 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 107)

- **HEAD** `e3c478af` (== `origin/main`, „ci: build the PATH tools in CI, refresh
  the wrappers from tools-latest"); `git_safety` Snapshot bei Session-Beginn. Der
  Baum trug bei Beginn fremde uncommittete Arbeit (ernte: staged
  `docs/zustand/external-state.md`, `phi/pipeline/ledger.φ`, der Handover-Move
  folge107→`archiv/` + neue folge108; worktree `src/archivar/bsp_reader/daf.rs`) —
  nicht angefasst.
- **Postfach** — letzter Ledger-Eingang `1789906306` (Weiterleitung des
  Sotgiu-Limadou-Threads durch den Operator); keine neue forschung-eigene Post;
  `post.md` leer. Die Zeile steht gemessen in `docs/zustand/external-state.md`.
- **CI** — `ci_manage list` (2026-09-20 ~14:31Z): **pending** `ci-check`
  `35516540173`; **in_progress** `ci-check` `35516232478`, `allwise-cdn`
  `35516186859`, `te-gate` `35513982359`; **success** `harvest` `35516529023`,
  `tools-build` `35516232599`, `paper-check`, `esp32-firmware`, `harvest-dispatch`;
  **failure** `release-build` `35513611936` @`7d0a1272`; **cancelled** die
  ci-check-Kette (per-ref-Concurrency). Die **CI-Zeile in `external-state.md` ist
  fällig** (HEAD `e3c478af` ≠ `ae0fce25`), aber die Datei ist von der ernte-Linie
  gehalten (staged, fremde Arbeit) — nicht angefasst; nächster Schritt: CI-Zeile
  fortschreiben, sobald die Datei frei ist.

## Praxis-Riss Kernel/Force in `phi/sources.φ` — Review offen

**Korrektur (Forschung-Folge 108, gemessen):** Die Behauptung „eine Handvoll
Zeilen weicht ab" ist falsch — A ≠ A. Gemessene Kernel×Force-Matrix (`awk` über
alle `field`/`last`/`first`/`url`-Zeilen, 2026-09-20):

```
    650 inverse-square em            41 gaussian-inverse-square advective
    371 inverse-square acoustic      27 erfc diffusion
    336 inverse-square advective     23 gaussian-inverse-square thermal
    220 inverse-square thermal       17 erfc thermal
     93 inverse-square gravity       14 erfc seismic-surface
     75 exponential-decay thermal    12 inverse-square electric
     73 gaussian-inverse-square acoustic   9 gaussian-inverse-square gravity
     61 gaussian-inverse-square diffusion  7 gaussian-inverse-square seismic-body
     60 gaussian-inverse-square em    4 erfc em
     48 patch-levy advective          1 gaussian-inverse-square seismic-surface
                                       1 erfc advective
```

Zwei Zeugenlinien: die alte Übergabe-Zeile („Handvoll") gegen die Matrix (hunderte
Abweichungen vom per-Force-Default). Das Rat-Verdikt (Forschung-Folge 108): der
Kontrakt-Satz war als **Konverter-Fallback für Zeilen OHNE Kernel-Token** gemeint
(`sources-v2-spec.md:70-71`), nicht normativ über deklarierten Zeilen; der Parser
ehrt den Token zuerst (`parse.rs:759/1167`), der Konverter materialisiert den
Default nur im Synthese-/Entwurfspfad (`port.rs:18/1214/1237`). Kein Riss zwischen
Register und Code, kein Glätten, keine Massen-Korrektur. Der Kontrakt ist geschärft
(`SOURCE_PORT.md:233-238`); die per-Klasse-Physikbewertung wandert als
Warteschlange in folge108 (erste Klasse `inverse-square acoustic`, 371).

## Datenbank-Ausbau — offene Kandidaten

`--alphafold` in diesem Atom gebaut: Modul
`tools/utils/src/bin/archive_search/alphafold.rs`, Verdrahtung in
`archive_search.rs`/`net.rs`/`web.rs`/`server.rs`, `docs/concepts/tools-map.md`;
`cargo check -p omegaflow-utils --all-targets` 0/0. JSON-Form live gemessen
(2026-09-20): Top-Level-Array mit `entryId`, `uniprotAccession`, `uniprotId`,
`uniprotDescription`, `gene`, `organismScientificName`, `sequenceVersionDate`,
`pdbUrl`, `globalMetricValue`. Noch offen: `--entrez` (GenBank/SRA/GEO,
`eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi`), `--ena`, `--biomodels`,
`--nist`, `--cod` — alle HTTP 200 direct außer KEGG (Lizenz). (Schritt: nächsten
Kandidaten nach `docs/SOURCE_PORT.md` portieren — `grind-flash`.)

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` (in_progress, dispatcht 2026-09-20 13:36Z) — die beiden
vorigen Läufe des 3σ-Fixes endeten **cancelled** (`35475226890` @`5b406e16`,
`35473941500` @`9f48b5bd`), kein Verdikt; der n=1000-FPR-Boden ist ungemessen.
(Schritt: `ci_manage view 35513982359`; bei rot `ci_manage log`.)

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

- Kein Doppellauf: das Atom war ein Architektur-Urteil (Kernel-Riss) + ein
  Routine-Port (`--alphafold`) + Gate-Fixture. Die Klasse „Routine-Port" ist
  geschlossen (2026-09-16: `grind-flash` Sieger, $0.0008); der Port lief mit
  `grind-flash`. Die Architektur trug der Rat (`council`, pro/max) — der
  Kernel-Riss ist das benannte harte Atom.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/force.rs`, `src/archivar/port.rs`,
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `tools/utils/src/bin/archive_search.rs`,
  `tools/utils/src/bin/archive_search/net.rs`,
  `tools/utils/src/bin/archive_search/server.rs`,
  `tools/utils/src/bin/archive_search/web.rs`,
  `tools/utils/src/bin/archive_search/alphafold.rs` (neu),
  `docs/concepts/tools-map.md`,
  `docs/handover/handover-2026-09-20-forschung-folge107.md` (neu),
  `docs/handover/archiv/handover-2026-09-20-forschung-folge106.md` (Move).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
