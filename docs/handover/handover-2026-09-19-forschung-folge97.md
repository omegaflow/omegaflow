<!--
  title: Handover — Forschung-Folge 97 (Stand 2026-09-19)
  session: Forschung-Folge 97
  class: handover
  date: 2026-09-19
  sha256: 801bb6df821d92dc13bd9e531f6911939219d5b1f8613386d07a346eb6861e79
  status: live
-->
# Handover — Forschung-Folge 97 (2026-09-19)

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
der härteste undatiert); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 97)

- **HEAD** `d9dc64e7` == `origin/main` (die Ernte-Linie hat während der Session
  gepusht; Forschung-Folge 96 `f3745142` ist Vorfahr) — FF steht, der eigene
  Commit geht direkt auf `d9dc64e7`. `git_safety` Snapshot `refs/safety/1789848696`.
- **Postfach** — kein neuer Eingang seit `1789795811` (Rubin-Forum AGN DP2,
  informativ); `state/mail/mail_ledger.φ` mtime 2026-09-19 07:30; `post.md` leer
  (kein `An forschung`). Eintrag zitiert (`external-state.md`), nicht kopiert.
- **CI** — zitiert aus `docs/zustand/external-state.md` (Ernte-Folge 97, @`60d2c2fb`;
  der HEAD-Wechsel auf `d9dc64e7` hat den Trigger erneut gefeuert — die Zeile ist
  fällig, der nächste Pass misst sie): **in_progress** `hyperscanning-te`
  `35465589119`, `ci-check` `35465331299`; **pending** `hyperscanning-te`
  `35466436802`, `te-gate` `35462518676`. Die CI-Zeile wird zitiert, nicht kopiert.
- **Fremde uncommittete Arbeit (nicht angefasst):** `src/archivar/hdf5.rs`,
  `tools/harvest/src/bin/openneuro_compiler.rs`, `.github/workflows/openneuro-cdn.yml`,
  `phi/declined_sources.φ`, `phi/pipeline/ledger.φ` (ernte); die drei
  Handover-Moves `entscheid24`, `forschung44/51` → `archiv/`.

## Hyperscanning-TE Screen-Verifikation (härtester undatiert) — `wartend`

- Screen-Lauf `35465589119` @`eb2e414e` in_progress; Artefakt
  `hyperscanning-te-report` ist die Messung. (Schritt: `ci_manage view
  35465589119` einmal; grün mit Triaden ⇒ weiter, Exit-2-Gate ⇒ das ist die
  Messung, kein Defekt.) Der Screen trägt jetzt den **Takens-Schätzer** (dim 3,
  τ ersetzt das Lag-Gitter, cap 4096) — der Lauf `35465589119` wurde noch auf dem
  **alten Binning-Stand** dispatcht; der Takens-Stand ist @`f3745142` noch nicht
  gelaufen.

## Takens-Screen — Wandzeit-Messung + Dispatch — `wartend`

- Der erste Lauf auf dem gepushten Takens-Stand ist zugleich die
  **Wandzeit-Messung** (O(n²)-Zelle gegen das alte O(n)-Gitter). (Schritt: nach
  Push `gh workflow run hyperscanning-te -f null_model=phase -f percentile=95
  -f surrogates=200`; Lauf-ID in Ledger + Übergabe; `ci_manage view <id>` einmal.)
  Erst diese Messung entscheidet den Watchdog-Floor-Punkt unten — kein Floor ohne sie.

## Bestätigungsstufe — `wartend`

- Screening p95/200, Überlebende p99/1000. Die Workflow-Inputs `percentile` und
  `surrogates` stehen jetzt. (Schritt: nach dem Screen-Lauf
  `gh workflow run hyperscanning-te -f null_model=coherent-phase -f percentile=99
  -f surrogates=1000`; die Differenz der Survivor-Zellen gegen den Screen ist die
  Messung „über das Lineare hinaus".)

## Mehrere Frontalkanäle — `wartend`

- `--channel` trägt der Workflow. (Schritt: `gh workflow run hyperscanning-te
  -f channel=F3` (und F4) als getrennte Läufe, nach der Bestätigungsstufe.)

## Risse (Rat, ungeglättet) — Analyse

- **Gauss-Blindheit:** CoherentPhase dreht alle Triaden-Serien mit einem
  gemeinsamen Phasenvektor — Gauss-linearer Transfer liegt **in** der Null und ist
  prinzipbedingt nie detektierbar; zwei Nullen, zwei Fragen. Nicht-Gauss-Marginale;
  Familien-Max-Interaktion unter CoherentPhase (stärkstes lineares Paar beschattet
  schwächere nichtlineare → FN-Risiko); zirkuläres FFT-Randartefakt (lebt nur im
  Surrogat-Pfad, wirkt in FN-Richtung); τ-Instabilität auf kurzen EEG-Segmenten
  (Null weitet sich, konservativ); Ksg-Verhalten `pending`.

## te-gate `35462518676` — `wartend`

- pending @`3d2e6adb`; deckt `gate_fpr_autocorrelation*` (inkl. coherent-phase) +
  `gate_conditional_arx_fpr_fn_n1000`. (Schritt: `ci_manage view 35462518676`;
  grün ⇒ die Arme frei, `211A→193A`-Conditional-Check frei,
  `docs/paper/solar-seconds-matrix.md:37,46`.) Kein Re-Dispatch.

## Takens-Gate-Tests im CI — `wartend`

- Die neuen Tests (`te.rs`: `topological_estimate_*`; Screen:
  `a_structured_driver_breaks_the_family_maximum`,
  `independent_structured_series_stay_under_the_family_maximum`,
  `white_noise_pair_carries_no_cell`, `family_max_equals_observed_max`) sind lokal
  nur `cargo check`-geprüft (0/0). (Schritt: das Verdikt des nächsten
  `ci-check`/`te-gate` auf `f3745142`+ liest, ob das Kalibrier-Gate hält; rot ⇒
  das FP/FN-Fixture nachziehen.)

## Abgeleiteter Watchdog-Floor — benannter offener Punkt (Rat)

- Für den Fall „ehrlicher schneller Median → legitime Verlangsamung" jenseits 2×.
  (Schritt: die Wandzeit-Messung des Takens-Screens oben; nur wenn ein legitimer
  Lauf 2× den ehrlichen Median übersteigt, `timeout/2²`-Floor bauen — nicht ohne
  diese Messung.)

## measure-gates Budget-Defekt

- `35456056999` stirbt im `timeout-minutes: 120` während `silence_map_probe` gegen
  den CDN-Katalog; das FN-Gate selbst ist grün. (Schritt: schweren CDN-Probe aus
  dem Gate-Lauf lösen oder Budget trennen — eigenes Atom, kein Watchdog-Defekt.)

## vC-Permeabilitäts-Karte — `operator-gebunden` (an entscheid gepostet)

- **Messakt**: `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad>
  ./target/release/omegaflow` lokal nach Core-Release-Build; danach
  `perm_target_probe --live <pfad>`. (Schritt: Operator-Wort + Release-Build.)
- **Richtungssemantik**: Legacy `exp(-vC/(g+1/C))` vs. heute `tanh(v_c/(g+PERM_GROUND))`.
  (Schritt: Operator-Wort.)

## MAG-Asset — `blockiert` (ernte)

- `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (853331 B, sha256
  `6f3724f7…`, valides ZIP, PDS4-`.tab` 4.83 MB). (Schritt:
  `tools/harvest/src/bin/bc_mpo_mag_compiler.rs` nach `voyager_odr_compiler.rs`,
  dann `sources.φ`-Eintrag + CI-Manifestation.)

## BepiColombo MORE — `termin`

- Cruise-Daten erst zur Wissenschaftsphase (~April 2027) freigegeben. (Schritt:
  kein TAP-Abruf vorher.)

## NSE/Haug — `wartend`

- Keller-Antwort (17.09.): „in einigen Tagen"; er sendet die TRISP-NSE-Daten
  selbst. Trigger = Dateieingang. (Schritt: bei Eingang `nse_haug_trisp`-Quelle +
  Compiler + `sources.φ`.)

## Paper / Präregistrierung — `termin`

- Flyby Path 2 datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- **Rat (pro/max, `council`):** `$0.0509` — Architektur-Verdikt „eine Null, ein
  Schätzer, symmetrische Pfade; τ ersetzt das Lag-Gitter; Eigen-Historie in die
  Einbettung gefaltet; kein `LaggedCond`-Bau im Binning-Pfad" (descoped mit
  Messung). Der Rat korrigierte die Prämisse (Interna bereits `pub`) — die
  Messung vor dem Verdikt war der Wert.
- **Bau (pro/max, `grind-max`):** `$0.0932` — `TopologicalEstimate` +
  `topological_te_estimate` in `te.rs` (beobachteter Arm von `topological_te_with`
  ruft sie), Screen-Refactor (Lag-Gitter → τ), 5 neue Gate-Tests in `te.rs`, 4
  neue Screen-Tests, Workflow-Step. `cargo check` 0 Fehler/0 Warnungen. Hartes
  Atom (neuartige TE-Konstruktion) — kein flash-Durchlauf, der Rat benannte
  `grind-max` namentlich.
- **Punkt 2 (Workflow-Inputs) direkt in der Session** — 2-Hunk-YAML-Edit, die
  Delegation kostete mehr als der Edit.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (`TopologicalEstimate`, `topological_te_estimate`,
  Refactor `topological_te_with`, 5 Gate-Tests)
- `tools/measure/src/bin/hyperscanning_group_te.rs` (Takens-Schätzer, τ statt
  Lag-Gitter, neue Gate-Tests)
- `.github/workflows/hyperscanning-te.yml` (`percentile`/`surrogates`-Inputs,
  `--max-points 4096`, Step-Name)
- `docs/handover/handover-2026-09-19-forschung-folge97.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge96.md` (Move)
- **Fremd (nicht angefasst):** `src/archivar/hdf5.rs`,
  `tools/harvest/src/bin/openneuro_compiler.rs`, `.github/workflows/openneuro-cdn.yml`,
  `phi/declined_sources.φ`, `phi/pipeline/ledger.φ`, die drei Handover-Moves
  `entscheid24`, `forschung44/51` → `archiv/`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
