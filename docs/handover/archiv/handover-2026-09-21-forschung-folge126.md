<!--
  title: Handover — Forschung-Folge 126 (Stand 2026-09-21)
  session: Forschung-Folge 126
  class: handover
  date: 2026-09-21
  sha256: 26e0e4ccacdd6ba27cc3cad7851e66f8083416a2b768f10cefb0523355a2e393
  status: live
-->
# Handover — Forschung-Folge 126 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein
Auswahlpunkt — sie nennen nur ihren Auslöser. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 126)

- **HEAD** `0fd1c5a9` (== `origin/main`, Ernte-Folge 125) bei Session-Beginn.
  Mid-Session advanced entscheid-folge71 auf `5a33779b` (zwei Commits, gepusht) —
  der eigene Commit steht darauf. `git_safety` Snapshot bei Beginn: „working tree
  equals HEAD — nothing to record".
- **CI** — Watchdog-Snapshot 06:58:54: `health-check` `35543181033` in_progress;
  `ci-check` `35537130867` @`5894b345` **failure**; `hyperscanning-te` `35537110267`
  @`16dd020c` **failure**; `rpw-cdn` `35536005969`/`35536016815` failure; `ci-check`
  `35566258372` @`0fd1c5a9` in_progress (seit 05:53, kein Update 07:0x).
- **Naht-Fix-Verdikt (gemessen, geschlossen):** `hyperscanning-te` `35537110267`
  @`16dd020c` (enthält den Naht-Fix `4e3ddb13`) **failure** in `family_fn_gate`.
  Blindband: `TE 8.7004e-2 | null mean 2.2945e-1 sd 7.0663e-2 p95 3.3960e-1 |
  excess -1.4244e-1 (-2.02 sd) | fam-max 5.7915e-1`. Vor dem Fix (`35533691654`
  @`fbd0f153`): `null mean 2.7248e-1 | excess -3.30 sd`. Der Naht-Fix senkte die
  Null (0.272→0.229) und hob das Excess um 1.28 sd (−3.30 → −2.02), **heilt aber
  nicht** — die Prognose „Null fällt auf den gemeinsamen Boden 5–9e-2" ist
  widerlegt. Die Reparatur-Reihenfolge des Risses geht weiter: τ-Einfrieren.

## Punkt 1 — τ-Einfrieren gebaut, Verdikt offen

**Gebaut** (Atom folge126): der Surrogat-Null-Pfad des Familien-Screens friert τ
auf den Wert der wahren Serie ein, statt es je Surrogat neu zu suchen.
- `src/mathematikerin/te.rs`: `topological_te_estimate_frozen(x, y, dim, tau_x,
  tau_y)` — derselbe Schätzer, aber ohne `find_mi_lag` (τ explizit, wie
  `transfer_entropy_embedded` es nimmt).
- `tools/measure/src/bin/hyperscanning_group_te.rs`: `member_taus` (τ je
  Triaden-Mitglied aus der wahren Serie), `frozen_estimate`; `surrogate_family_maxima`
  und `confirmation_cell_nulls` rufen nur noch den eingefrorenen Schätzer auf den
  randomisierten Serien (kein `topological_te_estimate` mehr auf `randomized[...]`).
  Unabhängige Prüfung (general/flash): Mapping `tau_x=τ(series[j])`,
  `tau_y=τ(series[i])` **korrekt**, kein Restsuch-Pfad.
- `src/mathematikerin/te.rs` Test `topological_estimate_frozen_matches_searched_tau`:
  der eingefrorene Schätzer bei den gesuchten τ ist **bit-identisch** zum kanonischen
  (A = A zwischen den zwei Pfaden; Rat-Auflage).
- `cargo check -p omegaflow --all-targets` + `cargo check -p omegaflow-measure
  --all-targets`: 0/0.

**Offen:** Verdikt — **dispatcht** `hyperscanning-te` `35567708611` @`f0ca4026`
(in_progress, post-push). Prognose: die eingefrorene Null fällt auf den
gemeinsamen Schätzer-Boden; ob das Excess über p95 trägt, entscheidet der Lauf.
(Schritt: `ci_manage view 35567708611` einmal — nie pollen.)

## Punkt 2 — Kohärente-Null-FP-Gate gebaut (Symmetrie-Arm)

`coherent_null_fp_gate` in `hyperscanning_group_te.rs`: unabhängige, spektral
reiche Serien (`rich_series`, fünf inkommensurable Sinus + AR-Rauschen), kohärente
Null, per-Zelle-Survivor-FP ≤ Zufall. Bislang fehlte dieser Arm (der kohärente
Pfad war nur im FN-Gate, nie im FP-Gate). Der CI-Teststep ist um
`coherent_null_fp_gate` erweitert (`.github/workflows/hyperscanning-te.yml:41`).
**Offen:** Verdikt im nächsten `hyperscanning-te`-Lauf. (Schritt: Run lesen.)

## Punkt 3 — `te-gate` n=1000-FPR-Boden

`te-gate` `35534898200` @`bd88d2dc` **cancelled** (2026-09-21T02:15) — kein
Verdikt; der n=1000-FPR-Boden bleibt ungemessen. Neu **dispatcht** `te-gate`
`35567711055` @`f0ca4026` (in_progress, post-push). (Schritt:
`ci_manage view 35567711055` einmal.)

## Punkt 4 — nominees-Tests in den CI-Teststep?

Die neuen Tests (`nominees_round_trip`,
`confirmation_confirms_the_strong_pair_against_its_own_null`) sind bewusst nicht
im gefilterten Teststep (`family_fn_gate family_fp_gate`). Entscheidung, ob sie
dort laufen sollen. (Schritt: Entscheidung + Workflow-Zeile.)

## Punkt 5 — ci-check-Rot

`ci-check` `35537130867` @`5894b345` failure: 5 Tests (case-flag
`archive_search.rs:1533`; cod Entry-Feld-Pflicht `cod.rs:180`; entrez `term=`
`entrez.rs:161`; materialsproject Float-Skalar `materialsproject.rs:87`; pdf ObjStm
`/First` `pdf.rs:1850`) + fmt `relay.rs:1107`. Neuer Lauf `35566258372` @`0fd1c5a9`
in_progress — zuerst lesen, dann heilen oder als fremd (bau/ernte) benennen.
(Schritt: `ci_manage view 35566258372` einmal.)

## Punkt 6 — `--dropped` als CI-Gate

435 pairs, 3317 candidates, 1760 dropped, 164 commit-resolved; Liste groß.
(Offen: `--dropped` als CI-Gate verdrahten — Rat/Architektur.) (Schritt: Rat.)

## Punkt 7 — Frontalkanäle F3/F4

Getrennte Läufe nach grünem Screen. (Schritt: nach grünem `family_fn_gate`.)

## Punkt 8 — Takens-Wandzeit + Watchdog-Floor

Beim grünen Screen die Wandzeit lesen und den Watchdog-Floor daraus ableiten.
(Schritt: Wandzeit im grünen Lauf messen.)

## Punkt 9 — Eigen-Historie Konditionierer

`LaggedCond` auf Zielserie. (Schritt: `te.rs` verdrahten.)

## Riss (getragen, nicht geglättet)

Der Rat ist uneinig über den dominanten Mechanismus der Inversion —
Naht/gemeinsame Hüllkurve (Sinne), τ-Mischung (Fluss), KDE-Boden (Berg); durch
Lesen nicht entscheidbar. Die Reparatur-Reihenfolge misst es: der Naht-Fix ist
gemessen und war **nicht** ausreichend (excess −2.02 sd); der τ-Einfrier ist
gebaut und wird als nächster gemessen. Die Assertion bewegt sich nicht; der Boden
wird als gemessene Konstante gedruckt, nie durch Lockern versteckt.

## Wartend / operator-gebunden / termin

- Riss 4 Ksg off-path — `operator-gebunden`: als Post-Zeile `An entscheid:`
  getragen (verdrahten oder descopen).
- Cookie-Editor-Export — `wartend`/`operator` (Host fehlt).
- Flyby-Path-2-Kette — `termin:2026-09-28` (Zellen ab Perigäum füllen).
- NSE/Haug — `wartend`/`dritter` (Trigger Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe Wissenschaftsphase).
- Buster-Store-„Updated"-Datum — `wartend`/`operator`: CWS-Listing nur im echten
  Browser lesbar.

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| 1. τ-Einfrieren-Verdikt | wartend | eigen | `ci_manage view 35567708611` einmal |
| 2. Kohärente-Null-FP-Gate-Verdikt | wartend | eigen | im Lauf `35567708611` |
| 3. `te-gate` n=1000-FPR | wartend | eigen | `ci_manage view 35567711055` einmal |
| 4. nominees-Tests in CI-Teststep? | wartend | eigen | Entscheidung + Workflow-Zeile |
| 5. ci-check-Rot | wartend | eigen | `ci_manage view 35566258372`, heilen/benennen |
| 6. `--dropped` als CI-Gate | wartend | eigen | Rat: Gate-Verdrahtung |
| 7. Frontalkanäle F3/F4 | wartend | eigen | getrennte Läufe nach grünem Screen |
| 8. Takens-Wandzeit + Watchdog-Floor | wartend | eigen | Wandzeit im grünen Lauf |
| 9. Eigen-Historie Konditionierer | wartend | eigen | `LaggedCond` auf Zielserie |
| 10. Riss 4 Ksg off-path | operator-gebunden | operator | entscheid-Post (verdrahten/descopen) |
| 11. Cookie-Editor-Export | wartend | operator | Operator nennt Host |
| 13. Flyby-Path-2 | termin:2026-09-28 | termin | Zellen ab Perigäum |
| 14. NSE/Haug | wartend | dritter | Trigger Dateieingang |
| 15. BepiColombo MORE | termin:2027-04 | termin | Freigabe Wissenschaftsphase |
| 16. Buster-Store-„Updated"-Datum | wartend | operator | im echten Browser lesen |

## Benchmark

- **τ-Einfrieren (hartes Atom, TE-/Null-Konstruktion):** Kern im Session-Kontext
  (build) gebaut; die Route war vom Rat vorgegeben (kein Diagnose-Suchlauf). Die
  Klasse ist gemessen geschlossen (folge125: `grind-max` als Sieger, „kein
  flash-Doppellauf") — kein neuer Doppellauf.
- **Frozen-τ-Verifikation (Routine):** `general` (flash) — korrektes Mapping,
  kein Restsuch-Pfad; Klasse Routine, flash-Sieger, kein pro/max-Doppel.
- **Kohärente-FP-Fixture (Routine):** im Session-Kontext (build) — Test-Fixture,
  Routine-Klasse (flash-Sieger gemessen 2026-09-16).
- **Rat (pro/max, Abschluss-Blatthaltung):** hielt das fertige Atom — Verdikt
  einstimmig „faithful": Assertion unberührt, Boden gedruckt, das Mechanismus
  (τ-Selbstwahl in der Null) entfernt; eine Auflage, der Parity-Test
  `topological_estimate_frozen_matches_searched_tau` (gebaut). Architektur-Klasse,
  kein Benchmark-Doppel.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (`topological_te_estimate_frozen`)
- `tools/measure/src/bin/hyperscanning_group_te.rs` (frozen Null-Pfade +
  `coherent_null_fp_gate` + `rich_series`)
- `.github/workflows/hyperscanning-te.yml` (Teststep um `coherent_null_fp_gate`)
- `docs/handover/handover-2026-09-21-forschung-folge126.md` (neu)
- Move `handover-2026-09-20-forschung-folge125.md` → `archiv/` (eigene Linie, atomar)

Nicht im Pfad-Satz: `docs/zustand/external-state.md` — eine laufende
entscheid-folge71-Session hält dort uncommittete Hunks (CI-Zeile, Postfach
measured-at); der eigene Postfach-Hunk (`1789970277`, Framework-decline) wurde
zugunsten der fremden Arbeit **zurückgenommen** (geteilter Baum, nur eigene
Hunks). Der Framework-decline steht daher nur hier im Stehenden Pass; die
Zustand-Zeile folgt im nächsten freien Pass.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Commit `f0ca4026` gepusht (==
`origin/main`); dispatcht: `hyperscanning-te` `35567708611` (τ-Einfrier-Verdikt) +
`te-gate` `35567711055` (n=1000-FPR-Boden), beide @`f0ca4026`, in_progress — je
`ci_manage view <id>` einmal, nie pollen.
