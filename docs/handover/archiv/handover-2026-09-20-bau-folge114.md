<!--
  title: Handover — Bau-Folge 114 (Stand 2026-09-20)
  session: Bau-Folge 114
  class: handover
  date: 2026-09-20
  sha256: 398140d0bf8fb814ff01a2116946a1cb2e3befe504cfa1f2b700a53cc81af3a1
  status: live
-->
# Handover — Bau-Folge 114 (2026-09-20)

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
Der Planungs-Pass nennt die offenen Punkte als Tafel (Punkt | Status | Bindung |
Schritt); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** Session-Beginn `bd88d2dc` == `origin/main`; während des Atoms landeten
  `9f8e4bdc` (ernte folge124) und `74359a92` (entscheid folge69) — mein Commit
  folgt auf `74359a92` (Fast-Forward, gemessen: HEAD == origin/main == `74359a92`).
- **Postfach** — `mail_digest`: 114 records, letzter Eingang `1789930255`
  (Pine64/Ox64, operator-gebunden) unverändert; kein neuer Eingang. Die
  Postfach-Zeile in `external-state.md` trägt bereits entscheid folge69.
- **CI** — `ci-check` `35531572974` @`8218f46a` **failure** (Issue #15 „cargo test
  returned void" offen seit 2026-09-14): 5 `archive_search`-Tests + fmt-Diffs; am
  HEAD unverändert (Pfad seit `8218f46a` nicht berührt). Watchdog 21:22:
  `te-gate` `35531196101`, `ps1-cdn` `35530153972`, `free-model-bench`
  `35527517605` in_progress. CI-Zeile in `external-state.md` ist entscheid folge69.
- **Binär** — `02ee415e` (Träger `tools-build` `35531153732`, success).

## Offen

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| fmt-Rot eigener Hunk `src/archivar/tests.rs` (letzter Commit `835c1735`) | `wartend` | `termin` (nächster `ci-check`-Lauf) | Run-Ergebnis lesen (`ci_manage log <id>`); nennt er `tests.rs`, nur den eigenen Hunk formatieren (kein lokaler fmt-Lauf — CI-only), dann `gh workflow run ci-check.yml`. |
| „strukturierte Feld-Grammatik" (Kanon-Akt) | `operator-gebunden` | `operator` (via entscheid) | `An entscheid:`-Zeile steht in `post.md`; entscheid legt die Frage beim Operator-Rückkehr vor. |
| `rpw-cdn.yml`-LIRA-Dispatch | `termin` (eigener Push) | eigen | Nach dem Push `gh workflow run rpw-cdn.yml`; Run-ID aus dem Watchdog, Asset `rpw_efield_lira.bin` per `archive_search --sniff`. |
| 3 `ausstehend` Queue-Korpora (`ledger.φ:58–66`) | `blockiert` | `linie:ernte` | force-gate-B-Fix steht (`port.rs` `field_or_review`); Re-Lauf auf dem nächsten frischen Binär, `# pending`-Review-Zeilen zählen, dann Disposition SOURCE_PORT §5.4 (`post.md`). |
| 7 `verifiziert` Korpora / 368 Survivor (`ledger.φ:70–96`) | `blockiert` | `linie:ernte` | Survivor-Review/Disposition nach SOURCE_PORT §5.4; `post.md`-Zeile steht. |

## Messung dieses Atoms (kein Punkt)

- **EPA-RadNet-Einheiten gebaut** (Post von ernte folge124): `bq/l` (×1e3) +
  `bq/m3` (Identität) in `src/archivar/units.rs` (`convert_to_si` und force-0-Liste);
  Gate-Assertions in `tests.rs` (`Bq/L` → 1e3, `BQ/M³` → 2.0; beide in force 0);
  `An ernte:`-Vollzug gepostet (Registrierung kann laufen).
- **LIRA/CDF-Registerriss gelöst:** der Eintrag `blocked_sources.φ:70–73`
  (`parser-def cdf`, „cdf_reader-Atom fehlt", geschrieben `dab201ee` ernte
  folge123) war schon bei Niederschrift falsch — Reader `src/archivar/cdf.rs` +
  `cdf25.rs` (`ee4db3aa`, 2026-09-16) und Compiler
  `tools/harvest/src/bin/bia_efield_compiler.rs` (`8399b401`, 2026-09-03) standen
  im Baum. Arm jetzt registriert (`sources.φ` `rpw_efield_lira`, distinkter
  Asset-Name; Default-Kollision `:398` gefixt) + verdrahtet (`rpw-cdn.yml` zweiter
  Step, Fenster 2022-12-01..2025-12-31, komplementär zur AMDA-Zeile
  2020-06-15..2022-12-01); Eintrag entfernt; external-state-Zeile steht.
- **`register_lookup`-Owner-Gap gefixt:** Status wird auf dem ersten Token
  gemappt (`parser-def cdf` → bau); Gate-Test
  `status_owner_maps_on_first_token`; `cargo check -p omegaflow-register
  --all-targets` 0/0.
- **Fabrikations-Säuberung im LIRA-Compiler** (vom Commit-Gate beim Commit
  gefunden, 5 Blockaden): 2× `unwrap_or(0)` (Probe-Spaltenzahl, Versions-Vergleich),
  1× `unwrap_or_default` (Arc-Aggregat), `unwrap_or_else`-Falls (Mutex-Locks,
  `--out`/`--decimate-min`/`--jobs`-Defaults) — alle auf explizite Absenz
  gestellt (`None`-Arme, `lock_recovered`-Helper, Match-Defaults). Die Fixtures
  existierten im Gate — kein neues Fixture, kein neuer Test nötig; der Gate-Block
  ist der Test.
- **CI-Verdikt + Posts:** Rot am HEAD gemessen (5 `archive_search`-Tests,
  research-owned; fmt-Diffs relay.rs/free_model_bench.rs/ps1_coverage_compiler.rs);
  Posts an forschung, entscheid (×2), ernte geschrieben.
- `cargo check --all-targets` (Kern) + `-p omegaflow-register --all-targets` +
  `-p omegaflow-harvest --bin bia_efield_compiler`: 0 Fehler, 0 Warnungen.
- **Abweichung (ehrlich):** ein Sub-Agent rief `bin/omegaflow --help` auf — die
  Binär kennt keinen Help-Modus und startete einen headless Quellen-Scan, nach
  60 s abgebrochen; kein Fenster, keine Radiation.

## Benchmark

- **Bau-Folge 114**: 4 `grind-flash`-Dispatches (LIRA-Messung, Mapper-Fix,
  CI-Triage, LIRA-Bau) + 1 `explore` (Bericht-Extraktion); kein pro/max nötig —
  alle lieferten vollständig. Die Klassen (Routine-Suche/Inspektion, mechanischer
  Fix) sind laut AGENTS.md bereits geschlossen; kein Doppellauf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/units.rs`, `src/archivar/tests.rs`,
  `tools/register/src/bin/register_lookup.rs`,
  `tools/harvest/src/bin/bia_efield_compiler.rs`, `phi/sources.φ`,
  `phi/blocked_sources.φ`, `.github/workflows/rpw-cdn.yml`,
  `docs/handover/post.md`, `docs/zustand/external-state.md` (nur die LIRA-Zeile —
  CI-/Postfach-Zeilen sind entscheid folge69, committed), neues
  `docs/handover/handover-2026-09-20-bau-folge114.md`, Move
  `handover-2026-09-20-bau-folge113.md` → `archiv/`.
- **Fremd (nicht anfassen):** nichts mehr uncommitted; ernte folge124 (`9f8e4bdc`)
  und entscheid folge69 (`74359a92`) landeten während des Atoms. Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push; danach wird der geänderte
Workflow `rpw-cdn.yml` dispatcht. `/consent` ist der session-weite Consent
(Delegation), nie das Commit-Wort.
