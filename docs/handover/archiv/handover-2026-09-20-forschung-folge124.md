<!--
  title: Handover — Forschung-Folge 124 (Stand 2026-09-20)
  session: Forschung-Folge 124
  class: handover
  date: 2026-09-20
  sha256: c258319760bceed7f11f62a972b80b9588871e33c9dd0d2171710df0741ff7a4
  status: live
-->
# Handover — Forschung-Folge 124 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 124)

- **HEAD** `84c1cd76` (== `origin/main`, Forschung-Folge 123). `git_safety`
  Snapshot `refs/safety/1789931467` bei Session-Beginn (Baum == HEAD).
- **Postfach** — kein neuer Eingang: letzter Ledger-Eintrag `1789930255`
  (`info@pine64.org`, Ox64 zugesagt); Antwort gesendet (`sent_ledger`
  `1789931195`). Keine neue forschung-eigene Post.
- **CI** — `ci_manage list` 2026-09-20 ~19:40Z: `te-gate` `35531196101`
  @`e6b6eec8` **in_progress** seit 19:06 (>2 h — hängt wie zuvor, Watchdog
  entscheidet); `ci-check` `35532930434` pending, `35531572974` @`8218f46a`
  in_progress; `tools-build` `35532906846` in_progress; `free-model-bench`
  `35532890353` / `free-model-agent-bench` `35532892161` laufen; `ps1-cdn`
  `35530153972` in_progress.

## Punkt 1 — Hyperscanning-Group-TE-Strang (still fallengelassen nach folge105)

Der ganze Strang fiel nach `folge105` lautlos aus der Übergabe (in keiner
folge106–123, kein Paper). Der letzte Lauf `hyperscanning-te` `35472171277`
@`1fd18b31` ist **failure** in Step 1 (`family_fn_gate`), nicht der Exit-2-Fall;
der Screen hat nie ein reales Verdikt getragen.

**In diesem Atom gemessen und gebaut:**
- **Defekt gefunden:** `surrogate_family_maxima` sortiert nur `maxima`
  (`:234`), nie `cell_distributions` (`:190/:225`); `per_cell_survivors` (`:244`)
  ruft `percentile` (sorted-Input, `:98–110`) auf die **ungesortierte**
  Draw-Reihenfolge → die Per-Zelle-Schwelle ist kein p95. Fix: jedes
  `cell_distributions[slot]` vor der Rückgabe sortieren (`hyperscanning_group_te.rs`).
- **Blindband gedruckt:** die `family_fn_gate`-Assertion trägt jetzt die
  C→D-Null-Verteilung (mean, sd, p95, excess in sd) + fam-max-Schwelle. Die
  Assertion selbst bleibt unangetastet.
- `cargo check -p omegaflow-measure --bin hyperscanning_group_te`: 0/0.

**Offen (härtester undatiert):**
- **Verdikt des Fixes** — `hyperscanning-te` nach Push dispatchen, einmal lesen.
  Grün ⇒ der Sort-Defekt war die Ursache. Rot ⇒ die Null ist zu breit
  (Riss 2), dann greift der Sort nicht. (Schritt: `gh workflow run
  hyperscanning-te` nach Push, `ci_manage view <id>` einmal.)
- **Riss 2 — zirkuläres FFT-Randartefakt** (`te.rs` `coherent_phase_surrogates`
  `:1593–1626`: zero-pad `n=600→1024`, circular phases, DC/Nyquist unrotiert, τ
  je Surrogat neu). Richtung plausibel FN, ungemessen. (Schritt: beim roten
  Verdikt `ci_manage log` lesen — das gedruckte Blindband entscheidet.)
- **Riss 3 — τ-Instabilität** (Gate `gate_mi_lag_stability` in `te-gate.yml`).
- **Riss 4 — Ksg off-path** — `operator-gebunden`: der Familien-Screen ruft nur
  `topological_te_estimate`, nie Ksg. (Schritt: Operator entscheidet verdrahten
  oder descopen mit gemessenem „nicht auf dem Pfad".)
- **Bestätigungsstufe falsch verdrahtet** — `hyperscanning-te.yml` re-testet bei
  p99/1000 die **ganze Familie** → der Masken-Effekt kehrt zurück; die Nominierten
  müssen gegen **ihre eigenen** Per-Zelle-Nulls mit frischem Seed getestet werden.
  (Schritt: Workflow + Tool um `nominees`-Pfad erweitern, Rat-Verdikt folge124.)
- **Frontalkanäle F3/F4** — `--channel` trägt der Workflow. (Schritt: nach der
  Bestätigungsstufe getrennte Läufe.)
- **Takens-Screen Wandzeit + Watchdog-Floor** — erste O(n²)-Messung. (Schritt:
  beim ersten grünen Screen die Wandzeit lesen, dann Floor-Entscheid.)
- **Eigen-Historie als Konditionierer** — `conditional_te_*_n` existiert;
  `LaggedCond` auf die Zielserie statt `&[]`. (Schritt: Apparat verdrahten.)

## Punkt 2 — `register_lookup --dropped` (Diff-Gate über alle Linien)

Gebaut (dieses Atom): `register_lookup --dropped [<line>]` diffed je Linie
aufeinanderfolgende Übergaben und meldet jeden offenen Punkt aus N, der in N+1
fehlt, mit `git: resolved|none` über `git log --grep`. Deckt **alle** Linien ab
(kein Arg = alle). `cargo check -p omegaflow-register --bin register_lookup`: 0/0.

**Sweep gelaufen (frisches CI-Binär nach `tools-build` `35533686434`/`35533774541`
success):** `433 Paare, 3299 Kandidaten, 1748 dropped, 164 commit-resolved`.
Triage (read-only) über die 96 strukturellen Ketten: **~70 Matcher-Rauschen**
(Container-Überschriften, CI-Run-ID-Churn, Meta-Sätze), ~25 zentral in
`external-state.md`/Registern getrackt (kein Drop — relokiert). **Nur zwei klar
still-offen:** bau „strukturierte Feld-Grammatik" (Kanon-Akt, operator-gebunden,
letzte Nennung `bau-folge74`) und ernte „Rosetta ungelaufene Pfade /
Idempotenz-Gate" (letzte Nennung `ernte-folge75`) — beide als Post-Zeile an die
Eigentümerlinie getragen. Vier ambigu (betti0/fasy, Watchdog-Floor, Riss 4,
entscheid-`65f9243b`-install). Der Hyperscanning-Strang wäre vom Werkzeug
gefangen worden — Validierung.

**Gebaut (dieses Atom):** `--persist <n>` (meldet nur Punkte, die ≥n Übergaben
überleben, dann verschwinden) + Container-Ausschluss. `cargo check`: 0/0.

**Offen:** `--persist` ist noch nicht im CI-Binär (neuer Commit → `tools-build`).
(Schritt: nach Push `register_lookup --dropped --persist 2` einmal lesen.)
**Noch zu entscheiden:** als CI-Gate verdrahten, damit ein stiller Drop rot wird
(Rat/Architektur).

## Punkt 3 — `te-gate`-Verdikt @`e6b6eec8`

`te-gate` `35531196101` in_progress seit 19:06 (>2 h, hängend). Der n=1000-FPR-Boden
bleibt ungemessen. (Schritt: Watchdog/`ci_manage view 35531196101` einmal; bei
Hänger `ci_manage cancel` + Re-Dispatch.)

## Riss (getragen, nicht geglättet)

Der Rat (folge124): die Per-Zelle-Stufe ist **unabhängig** vom Familien-Max (sie
liest nur die eigene Slot-Verteilung) — die Zelle stirbt an ihrer eigenen
Null-Untergrenze, nicht an der Maske. Zwei wahre Sätze stehen gegeneinander:
*der Familien-Max trägt die FWER-Pflicht* und *die Per-Zelle-Stufe darf schwachen
Transfer nicht verpassen*. Per-Zelle-p95 über m Zellen erzeugt 0.05·m FP per
Konstruktion — die FN-Pflicht wird mit FP gekauft, legitim nur, solange die
Bestätigungsstufe die Inferenz trägt. Bis dahin gilt: **die Per-Zelle-Stufe
nominiert, die Bestätigung bestätigt** — die Ausgabe ist eine Nominierungsliste
und muss so gelesen werden. Die Assertion bewegt sich nicht; unter dem
Schätzer-Boden ist das Instrument blind, und der Boden wird als gemessene
Konstante gedruckt, nie durch Lockern versteckt.

## Wartend / operator-gebunden / termin

- Cookie-Editor-Export — `wartend`/`operator` (Host fehlt). (Schritt: Operator nennt Host.)
- Flyby-Path-2-Kette — `termin:2026-09-28` (Zellen ab Perigäum füllen).
- NSE/Haug — `wartend`/`dritter` (Trigger Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe Wissenschaftsphase).
- Buster-Store-„Updated"-Datum — `wartend`/`pending`: CWS-Listing nicht
  skriptbar (nur im echten Browser lesbar); aus folge116 still gefallen, jetzt
  zurückgetragen. (Schritt: im echten Browser lesen.)

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| 1. `family_fn_gate`-Verdikt (Sort-Fix) | wartend | eigen | `gh workflow run hyperscanning-te` nach Push, `ci_manage view` einmal |
| 2. Riss 2 FFT-Randartefakt | wartend | eigen | beim roten Verdikt `ci_manage log` (Blindband) |
| 3. Riss 3 τ-Instabilität | wartend | eigen | `gate_mi_lag_stability` in `te-gate.yml` |
| 4. Riss 4 Ksg off-path | operator-gebunden | operator | Operator: verdrahten oder descopen |
| 5. Bestätigungsstufe (nominees statt Familie) | wartend | eigen | Workflow+Tool um `nominees`-Pfad erweitern |
| 6. Frontalkanäle F3/F4 | wartend | eigen | getrennte Läufe nach Bestätigung |
| 7. Takens-Wandzeit + Watchdog-Floor | wartend | eigen | beim grünen Screen Wandzeit lesen |
| 8. Eigen-Historie Konditionierer | wartend | eigen | `LaggedCond` auf Zielserie |
| 9. `--dropped` gelaufen; `--persist`-Filter noch nicht im CI-Binär | wartend | eigen | nach Push `register_lookup --dropped --persist 2` einmal |
| 10. `--dropped` als CI-Gate | wartend | eigen | Rat: Gate-Verdrahtung entscheiden |
| 11. `te-gate`-Verdikt `35531196101` | wartend | eigen | `ci_manage view 35531196101` einmal |
| 12. Cookie-Editor-Export | wartend | operator | Operator nennt Host |
| 14. Flyby-Path-2 | termin:2026-09-28 | termin | Zellen ab Perigäum |
| 15. NSE/Haug | wartend | dritter | Trigger Dateieingang |
| 16. BepiColombo MORE | termin:2027-04 | termin | Freigabe Wissenschaftsphase |
| 17. Buster-Store-„Updated"-Datum | wartend | eigen | im echten Browser lesen |

## Benchmark

- **Hyperscanning-Diagnose (flash):** `general` (flash) fand den Sort-Defekt in
  einem Lauf (`cell_distributions` ungesortiert) — präzise, mit Zeilennummern.
  Kein pro/max-Doppellauf nötig: der Rat (pro/max) lief separat für die
  Architektur (FN/FP), nicht für dieselbe Aufgabe. Sieger der Diagnose: flash.
- **`--dropped` (flash):** `grind-flash`, ~473 Zeilen + 5 Tests, `cargo check` 0/0.
  Die Routine-Klasse ist geschlossen (flash-Sieger) — kein Doppellauf.
- **Rat (pro/max):** FN/FP-Architektur — Verdikt oben (Riss). Architektur-Klasse,
  kein Benchmark-Doppel.

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/hyperscanning_group_te.rs` (Sort-Fix + Blindband)
- `tools/register/src/bin/register_lookup.rs` (`--dropped`)
- `docs/handover/handover-2026-09-20-forschung-folge124.md` (neu)
- Move `handover-2026-09-20-forschung-folge123.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (CI-Zeile)
- `docs/concepts/tools-map.md` (`--dropped`-Zeile + `--live`→`--open`-Korrektur)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
