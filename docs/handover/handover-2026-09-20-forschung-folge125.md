<!--
  title: Handover — Forschung-Folge 125 (Stand 2026-09-20)
  session: Forschung-Folge 125
  class: handover
  date: 2026-09-20
  sha256: 852797e34fb8cb8ab49fc91a12c5355fada104f3943eab4df7064cba6ee5f214
  status: live
-->
# Handover — Forschung-Folge 125 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein
Auswahlpunkt — sie nennen nur ihren Auslöser. Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 125)

- **HEAD** `9f8e4bdc` (== `origin/main`, Ernte-Folge 124; Session-Beginn `bd88d2dc`,
  Forschung-Folge 124). `git_safety` Snapshot `refs/safety/1789935069` bei
  Session-Beginn.
- **Postfach** — kein neuer Eingang: letzter Ledger-Eintrag `1789930255`
  (`info@pine64.org`, Ox64 zugesagt, operator-gebunden); Antwort gesendet
  (`sent_ledger` `1789931195`). Keine neue forschung-eigene Post.
- **CI** — `te-gate` `35531196101` hing seit 19:06 (updated 19:06:28, >3 h) →
  `ci_manage cancel` + neu dispatcht `35534898200`. `hyperscanning-te`
  `35533691654` @`fbd0f153` **failure** (Blindband unten). Watchdog-Snapshot
  21:22:46: `ci-check` `35531572974` in_progress, `ps1-cdn` `35530153972`
  in_progress, `free-model-bench` `35527517605` in_progress; `ci-check`
  `35527911970`/`35526010713` failure.

## Punkt 1 — Hyperscanning-Group-TE: Naht-Fix gebaut, Verdikt offen

**Verdikt geschlossen:** Run `35533691654` lief auf `fbd0f153` — dem
Sort-Fix-Commit (folge124) — und ist **failure** in `family_fn_gate`.
Blindband: `TE 8.7004e-2 | null mean 2.7248e-1 sd 5.6243e-2 p95 3.6916e-1 |
excess -1.8547e-1 (-3.30 sd) | fam-max 4.8581e-1`. Der Sort-Fix war korrekt,
heilte aber nicht: das Signal liegt 3.30 sd **unter** der Surrogat-Null.
`family_fp_gate` grün.

**Diagnose (gemessen, grind-max):** Zero-Padding-Randartefakt in
`coherent_phase_surrogates` + `phase_randomized_surrogate`
(`src/mathematikerin/te.rs`): n=600→1024 ohne Endpunkt-Behandlung; die
Wert-Diskontinuität an der Pad-Kante erzeugt FFT-Leakage-Boden ≈0.29 vs.
Eigenrauschen 0.013. Bei exakter Periodizität kürzen sich die
Echo-Gitterterme im Original (Boden ≈+0.05 nat), beim Surrogat fuzzt der
Leakage-Boden sie → Null springt auf ≈+0.3 nat. τ-Instabilität trägt nicht
(MI-Kurvenform durch Phasenrotation erhalten, τ≈4/7 auf beiden Pfaden);
Bandbreiten tragen nicht (skaleninvariant). Historisch byte-identisch zu
`35472171277` @`1fd18b31` (TE 8.7004e-2) — deterministisch.

**Gebaut:** `endpoint_matched` (Ramp-Subtraktion vor FFT, Rückaddierung nach
IFFT) in beiden Generatoren (`src/mathematikerin/te.rs`, 23+/6−). `cargo
check -p omegaflow-measure --all-targets`: 0/0.

**Offen:**
- **Verdikt des Naht-Fixes** — nach Commit+Push `gh workflow run
  hyperscanning-te.yml`, `ci_manage view <id>` einmal. Prognose: Null-Mittel
  fällt auf den gemeinsamen Schätzer-Boden (~5-9e-2); ob das Excess über p95
  trägt, entscheidet der Lauf. (Schritt: dispatch nach Push, einmal lesen.)
- **τ-Einfrieren** — bei fortbestehender Inversion: `find_mi_lag` einmal auf
  der wahren Serie, dann `transfer_entropy_embedded` mit gefrorenem Paar auf
  den Surrogaten (Rat: `te.rs:2065` nimmt τ explizit). (Schritt: `te.rs`
  verdrahten, Lauf.)
- **Kohärente-Null-FP-Gate** — fehlender Symmetrie-Arm (Rat): unabhängige,
  spektral reiche Serien, kohärente Null, FP ≤ Zufall. (Schritt: Gate-Fixture
  in `hyperscanning_group_te.rs`.)

## Punkt 2 — Bestätigungsstufe `nominees` (gebaut)

`--nominees-out <file>` (Screen: TSV der Per-Zelle-Survivors) + `--nominees
<file>` (Bestätigung: jede nominierte Zelle gegen ihre eigene Per-Zelle-Null,
frischer Seed, p99/1000) in `tools/measure/src/bin/hyperscanning_group_te.rs`;
Workflow-Steps + Artefakt in `.github/workflows/hyperscanning-te.yml`. Neue
Tests: `nominees_round_trip`,
`confirmation_confirms_the_strong_pair_against_its_own_null` (Validierung am
starken Paar A→B). `cargo check`: 0/0.

**Offen:** die neuen Tests sind bewusst nicht im gefilterten CI-Teststep
(`family_fn_gate family_fp_gate`) — Entscheidung, ob sie dort laufen sollen;
Lauf des nominees-Pfads kommt mit dem nächsten `hyperscanning-te`-Run.

## Punkt 3 — `te-gate`-Verdikt @`e6b6eec8`

`35531196101` hing (>3 h, kein Update seit 19:06:28) → gecancelt, neu
dispatcht `35534898200`. Der n=1000-FPR-Boden bleibt ungemessen. (Schritt:
`ci_manage view 35534898200` einmal.)

## Punkt 4 — `register_lookup --dropped --persist 2` (gelesen)

435 pairs, 3317 candidates, 1760 dropped, 164 commit-resolved. Der Filter läuft
im CI-Binär; die Liste bleibt groß (1761 Zeilen), Triage-Stand folge124
unverändert (2 echte Fälle als Post an bau/ernte getragen). (Offen: `--dropped`
als CI-Gate verdrahten — Rat/Architektur.)

## Riss (getragen, nicht geglättet)

Der Rat ist uneinig über den dominanten Mechanismus der Inversion —
Naht/gemeinsame Hüllkurve (Sinne), τ-Mischung (Fluss), KDE-Boden (Berg); durch
Lesen nicht entscheidbar. Die Reparatur-Reihenfolge misst es: der Naht-Fix ist
gebaut und wird zuerst gemessen; bei fortbestehender Inversion folgt
τ-Einfrieren. Die Assertion bewegt sich nicht; der Boden wird als gemessene
Konstante gedruckt, nie durch Lockern versteckt.

## Wartend / operator-gebunden / termin

- Riss 4 Ksg off-path — `operator-gebunden`: als Post-Zeile `An entscheid:`
  getragen (verdrahten oder descopen).
- Cookie-Editor-Export — `wartend`/`operator` (Host fehlt). (Schritt: Operator
  nennt Host.)
- Hardware-Sponsoring Framework/Tuxedo — `wartend`/`dritter` (Trigger Antwort).
- Flyby-Path-2-Kette — `termin:2026-09-28` (Zellen ab Perigäum füllen).
- NSE/Haug — `wartend`/`dritter` (Trigger Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe Wissenschaftsphase).
- Buster-Store-„Updated"-Datum — `wartend`/`pending`: CWS-Listing nur im
  echten Browser lesbar. (Schritt: im echten Browser lesen.)

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| 1. Naht-Fix-Verdikt (Endpunkt-Matching) | wartend | eigen | nach Push `gh workflow run hyperscanning-te.yml`, `ci_manage view` einmal |
| 2. τ-Einfrieren (bei fortbestehender Inversion) | wartend | eigen | `te.rs` verdrahten, Lauf |
| 3. Kohärente-Null-FP-Gate | wartend | eigen | Gate-Fixture `hyperscanning_group_te.rs` |
| 4. nominees-Tests in CI-Teststep? | wartend | eigen | Entscheidung + Workflow-Zeile |
| 5. `te-gate` `35534898200` | wartend | eigen | `ci_manage view 35534898200` einmal |
| 6. `--dropped` als CI-Gate | wartend | eigen | Rat: Gate-Verdrahtung |
| 7. Frontalkanäle F3/F4 | wartend | eigen | getrennte Läufe nach grünem Screen |
| 8. Takens-Wandzeit + Watchdog-Floor | wartend | eigen | beim grünen Screen Wandzeit lesen |
| 9. Eigen-Historie Konditionierer | wartend | eigen | `LaggedCond` auf Zielserie |
| 10. Riss 4 Ksg off-path | operator-gebunden | operator | entscheid-Post (verdrahten/descopen) |
| 11. Cookie-Editor-Export | wartend | operator | Operator nennt Host |
| 12. Hardware Framework/Tuxedo | wartend | dritter | Trigger Antwort |
| 13. Flyby-Path-2 | termin:2026-09-28 | termin | Zellen ab Perigäum |
| 14. NSE/Haug | wartend | dritter | Trigger Dateieingang |
| 15. BepiColombo MORE | termin:2027-04 | termin | Freigabe Wissenschaftsphase |
| 16. Buster-Store-„Updated"-Datum | wartend | eigen | im echten Browser lesen |

## Benchmark

- **Hyperscanning-Null-Diagnose (max):** `grind-max` fand den Leakage-Boden
  (Faktor ~20 über Eigenrauschen), schloss τ und Bandbreiten als Träger aus und
  baute den Naht-Fix — hartes Atom (TE-/Null-Konstruktion), kein
  flash-Doppellauf.
- **Bestätigungsstufe (flash):** `grind-flash`, 432 Zeilen Tool + Workflow,
  `cargo check` 0/0. Routine-Klasse, flash-Sieger.
- **Rat (pro/max):** Architektur — Nominees jetzt, Drei-Klassen-Disziplin
  (Schwelle/Instrument/Fixture), τ-Reihenfolge. Architektur-Klasse, kein
  Benchmark-Doppel.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (Naht-Fix, beide Generatoren)
- `tools/measure/src/bin/hyperscanning_group_te.rs` (`--nominees-out`/`--nominees` + Tests)
- `.github/workflows/hyperscanning-te.yml` (Nominees-Steps + Artefakt)
- `docs/handover/handover-2026-09-20-forschung-folge125.md` (neu)
- Move `handover-2026-09-20-forschung-folge124.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (CI-Zeile)
- `docs/handover/post.md` (An entscheid)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Nach dem Push: `gh workflow run
hyperscanning-te.yml` (Naht-Fix-Verdikt) + `gh workflow run te-gate.yml` ist
bereits dispatcht (`35534898200`).
