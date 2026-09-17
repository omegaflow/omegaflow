<!--
  title: Handover — Forschung-Folge 59 (Stand 2026-09-17)
  session: Forschung-Folge 59
  class: handover
  date: 2026-09-17
  sha256: 5a0ecb2ff09249ac893c30a2d65a122e13cd4755ed70303e62a4dbb61449c627
  status: live
-->
# Handover — Forschung-Folge 59 (2026-09-17)

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

## Stehender Pass (automatisch, keine Auswahl)

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu
Session-Beginn gemessen, bevor die Auswahl steht — ihr Ausgang verändert die
Auswahl, sie sind kein Auswahlpunkt. Ergebnis direkt in dieses Handover + den
Zustand-Ledger. Karte: `docs/concepts/tools-map.md` — bei Widerspruch gilt `--help`.

- **Postfach** — `state/mail/mail_ledger.φ` (91 Zeilen) gemessen: **neuer Eingang** —
  MPI-FKF/TRISP (über FRM-II/TUM weitergeleitet) sendet die NSE-I(q,t)-Rohdaten
  (Haug et al., TRISP/MLZ) direkt, „a few days"; Operator hat am 17.09. geantwortet.
  Die fünf Sonden-Anfragen (NSSDC/JPL-NAV), GitHub-GC #4761801, Privacy und Rubin
  bleiben ohne Antwort (Rubin = Forum-Summary, keine Antwort). `docs/zustand/
  external-state.md` trägt fremde uncommittete Änderungen → zitiert, nicht überschrieben.
- **CI-Status am HEAD `671f2bd9`** — der Watchdog-Snapshot (14:57Z) war vor dem
  Forschung-Push; `ci_manage list` am HEAD: **paper-check `35228278279` failure
  @ `0d8936b6`** (Ein-Blatt-Sheets-Gate, `blatt-der-grat` sha stale — s.u.),
  **ci-check `35228714608` pending @ `671f2bd9`** (Format-Gate), demeter-cdn
  `35228716483` in_progress / `35229646866` pending (Ernte), planetary-odf/allwise/
  maven-tnf/ned/ps1/physionet/gaia aktiv/queued. `docs/zustand/external-state.md`
  trägt fremde uncommittete Änderungen → zitiert, nicht überschrieben.

## Paper-Gate — Ein-Blatt-Sheets-Gate rot: `blatt-der-grat` sha stale (härtester undatiert)

- **Gemessen:** Der Paper-Fix `0d8936b6` (sechs Papiere) hat den ersten
  Export-Schritt grün gemacht; Lauf `35228278279` fällt nun im **zweiten** Schritt
  `Export gate over Ein-Blatt sheets (docs/blatt)` (`--check docs/blatt/*.md`):
  `blatt-der-grat` trägt `sha=d7b28783…/057db7fd…` → `header_sha ≠ body_sha` →
  `any_diff` → exit 1. Der Reference-Gate-Schritt (arXiv/DOI) lief weiterhin nie.
- **Umgesetzt:** `docs/blatt/blatt-der-grat.md` Kopf-`sha256` = `d7b28783…`
  (Body-Hash über den Body ohne Header; reproduziert mit `tail -n +9 … | sha256sum`).
- **Offen:** Der Push triggert `paper-check` automatisch (`on: push`, `docs/blatt/**`)
  → Lauf am neuen SHA messen (`ci_manage list` → `ci_manage view <id>`). Der
  Reference-Gate-Schritt (arXiv/DOI) ist erstmals zu messen; bei Rot den Befund als
  Handover-Zeile tragen. (Schritt: `ci_manage view <paper-check-run>@<sha>`.)

## Format-Gate — ci-check am gepushten SHA pending

- `ci-check` `35228714608` @ `671f2bd9` ist **pending**; der `format`-Job
  (`cargo fmt --check`) liefert den Nachweis. Der eigene Push (`docs/**`) triggert
  zusätzlich einen frischen ci-check am neuen SHA. Die fremden Pfade aus Lauf `35185912267`
  (`src/archivar/kcdc.rs:425` ernte, `odf.rs:425` bau,
  `tools/harvest/src/bin/{cassini,dart}_tnf_compiler.rs`, `las_compiler.rs`,
  `tools/register/src/bin/cdn_reconcile.rs:310`,
  `tools/utils/src/bin/archive_search.rs:437`) bei Rot als Post an ernte/bau, nicht in
  diese Linie. (Schritt: `ci_manage view` des ci-check-Laufs am neuen SHA beim
  nächsten Pass.)

## NSE/Haug — Route offen, Rohdaten unterwegs

- MPI-FKF/TRISP sendet die Rohdaten (Mail 2026-09-17, `state/mail/mail_ledger.φ`);
  Survey `survey-2026-09-14-warteliste-offene-alternativen.md:52` auf „Route offen"
  aktualisiert (+ Kopf-sha), die Zeile aus §Bleibt offen entfernt. Auf Eingang:
  `nse_haug_trisp`-Quelle + Compiler + `sources.φ` (0-Kanon: kein Asset, solange keine
  Datei da ist). (Schritt: bei Mail-Eingang die Dateien lesen, dann Compiler-Draft.)

## Docs-Pendings — Sichtung 2026-09-17

- Machbare Einzelne (je Survey-Zeile belegt): Mariner `PSPA-00316` CDN-Dispatch
  (`survey-2026-09-17-sonden-request-only.md:66–67`); Juno post-EFB OCRU ↔
  `juno_odf.bin` (`:38`); Pioneer ATDF dtype-12/13 (`src/archivar/atdf.rs:508`
  gated nur 1|2); MAVEN-TNF-Shards; GOES-16 ABI `sources.φ`+Workflow
  (`survey-2026-09-14-kapitulationen-pendings-inventur.md:51`); WWLLN
  (`survey-2026-09-13-weberin-quellen.md:188`); Ulysses/BepiColombo/LRO
  `sources.φ` (`survey-2026-09-16-sonden-flotte.md:57–59`). (Schritt: je Eintrag
  der genannten Survey-Zeile.)

## CDN-Concurrency Follow-up

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht
  (`grep -q "^mro_odf_"` übersieht ein halbes Set); `cancel-in-progress: false`
  bleibt bis dahin. (Schritt: auf die erwarteten Shard-Namen härten, messbar
  erst nach einem erfolgreichen `mro_odf`-Lauf.)

## Legacy-Konzepte (hintenangestellt — Operator-Wort)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als
  4.; Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt:
  bei Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/
  silence_map_probe.rs`, Null-Kalibrierung als Spiegel der FP/FN/Symmetrie-Gates
  von `te.rs`.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Die paper-check-Diagnose (ci_manage `list`/`view` + `gh run view --log-failed` →
  Ein-Blatt-Sheets-Gate, `blatt-der-grat` sha) lief direkt im Linien-Agenten, kein
  Dispatch nötig — Routine-Klasse, flash-first bestätigt. Kein pro/max.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/blatt/blatt-der-grat.md`,
  `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`,
  `docs/handover/handover-2026-09-17-forschung-folge59.md`
  (+ archiviertes `handover-2026-09-17-forschung-folge58.md`). Fremde uncommittete
  Arbeit (nicht anfassen): die gestagten Renames
  `handover-2026-09-16-{entscheid-folge24,forschung-folge44,forschung-folge51}` →
  `archiv/`, `docs/zustand/external-state.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
