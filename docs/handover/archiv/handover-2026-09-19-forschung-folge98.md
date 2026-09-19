<!--
  title: Handover — Forschung-Folge 98 (Stand 2026-09-19)
  session: Forschung-Folge 98
  class: handover
  date: 2026-09-19
  sha256: 31217a33d0440ab599478147a2117a437a97b70c13dfa1a06274a45e30fc66cd
  status: live
-->
# Handover — Forschung-Folge 98 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 98)

- **HEAD** `7aa5c23e` == `origin/main` („bau folge94: fold the ernte98 post …");
  FF steht. `git_safety` Snapshot `refs/safety/1789850579`. Die Planungs-Pass-Zeile
  `d1750fe0` ist überholt — bau/ernte haben während der Session gepusht.
- **Postfach** — kein neuer Eingang seit `1789795811` (Rubin-Forum AGN DP2,
  informativ); `post.md` trägt nur `An bau` (sniff-Partial-Hash), **keine**
  `An forschung`-Zeile. Eintrag zitiert (`external-state.md`), nicht kopiert.
- **CI** — zitiert aus `docs/zustand/external-state.md` (Zeile CI-Status,
  Ernte-Folge 99 @`7aa5c23e`; nicht kopiert). Für die eigene Messung: kein
  `ci-check`-Verdikt auf dem Takens-Baum (`5219db7e`+) vorhanden; der letzte
  abgeschlossene `ci-check` `35462513268` @`3d2e6adb` trug die zwei roten
  CoherentPhase-Tests (Werte unten).
- **Fremde uncommittete Arbeit (nicht angefasst):** die drei Handover-Moves
  `entscheid-folge24`, `forschung-folge44`, `forschung-folge51` → `archiv/`.

## TE-Kalibrier-Gate — CoherentPhase-Tests — `wartend`

- Zwei Tests in `src/mathematikerin/te.rs` maßen die **abwesende Richtung**:
  `transfer_entropy_conditional_binned_n(&x,&y)` statt `(&y,&x)` — der erste
  Argument-Slot ist das **Target** (`te.rs:519`, `map_full` = `x_{t+shift}, x_t, y_t`,
  `te.rs:647`; Vertrag `multi_force_te_probe.rs:63` `(target, driver, …)`). Gemessen
  @`3d2e6adb` (`ci-check` `35462513268`): `coherent_phase_null_absorbs_linear_cross_coupling`
  obs 0.03348 < thr_phase 0.04738; `coherent_phase_null_detects_nonlinear_transfer`
  obs 0.00490 < thr 0.03381 — beide obs sind Bias-Niveau der leeren Richtung.
  Fix: fünf Aufrufstellen getauscht (2 obs + 3 Threshold), `cargo check -p omegaflow
  --all-targets` 0 Fehler/0 Warnungen. (Schritt: das Verdikt des nächsten
  `ci-check` auf HEAD lesen; rot ⇒ Fixture/Schätzer nachziehen, grün ⇒ das
  Kalibrier-Gate hält.)

## measure-gates Budget — CDN-Probe abgetrennt — `wartend`

- Der schwere `silence_map_probe`-Lauf gegen den CDN-Katalog (`dr3_stars.bin`,
  `fetch_raw_bytes` bis 3600 s) sprengte das 120-min-Budget des Gate-Laufs
  (`35456056999` starb dort; das FN-Gate selbst ist grün). Neu: eigenes Workflow
  `.github/workflows/silence-map-probe.yml` (timeout 180); `measure-gates.yml`
  führt nur Betti-0 + silence-map-Gates + perm_target_probe. (Schritt: nach dem
  Push beide Workflows dispatchen; das Verdikt aus den Läufen lesen.)

## Takens-Screen — Wandzeit-Messung — `wartend`

- Lauf `35468144989` @`5219db7e` pending (`null_model=phase`, `percentile=95`,
  `surrogates=200`) — der erste Lauf auf dem Takens-Stand, zugleich die
  **Wandzeit-Messung** (O(n²)-Zelle gegen das alte O(n)-Gitter). (Schritt:
  `ci_manage view 35468144989` einmal; grün mit Survivor/Pending ⇒ Wandzeit lesen;
  Exit-2-Gate ⇒ das ist die Messung, kein Defekt.) Erst diese Messung entscheidet
  den Watchdog-Floor-Punkt — kein Floor ohne sie.

## Bestätigungsstufe — `wartend`

- Screening p95/200, Überlebende p99/1000; die Workflow-Inputs stehen. (Schritt:
  nach dem Screen-Lauf `gh workflow run hyperscanning-te -f null_model=coherent-phase
  -f percentile=99 -f surrogates=1000`; die Differenz der Survivor-Zellen ist die
  Messung „über das Lineare hinaus".)

## Mehrere Frontalkanäle — `wartend`

- `--channel` trägt der Workflow. (Schritt: `gh workflow run hyperscanning-te
  -f channel=F3` (und F4) als getrennte Läufe, nach der Bestätigungsstufe.)

## te-gate `35462518676` — `wartend`

- pending @`3d2e6adb`; deckt `gate_fpr_autocorrelation*` (inkl. coherent-phase) +
  `gate_conditional_arx_fpr_fn_n1000`. (Schritt: `ci_manage view 35462518676`;
  grün ⇒ die Arme frei, `211A→193A`-Conditional-Check frei,
  `docs/paper/solar-seconds-matrix.md:37,46`.) Kein Re-Dispatch.

## Abgeleiteter Watchdog-Floor — `wartend`

- Für den Fall „ehrlicher schneller Median → legitime Verlangsamung" jenseits 2×.
  (Schritt: die Wandzeit-Messung des Takens-Screens oben; nur wenn ein legitimer
  Lauf 2× den ehrlichen Median übersteigt, `timeout/2²`-Floor bauen.)

## Risse (Rat, ungeglättet) — Analyse

- **Gauss-Blindheit ist Eigenschaft, kein Defekt** (research-max, gemessen): CoherentPhase
  dreht beide Serien mit **einem** gemeinsamen Phasenvektor (`te.rs:1603–1636`), erhält
  das Kreuz-Spektrum — ein Gauss-linearer Transfer liegt damit prinzipbedingt in der
  Null; die zwei CoherentPhase-Tests prüfen genau das (nun richtungs-korrekt).
- Offen: Familien-Max-FN-Risiko (stärkstes lineares Paar beschattet schwächere
  nichtlineare); zirkuläres FFT-Randartefakt (nur Surrogat-Pfad, FN-Richtung);
  τ-Instabilität auf kurzen EEG-Segmenten (Null weitet sich, konservativ);
  Ksg-Verhalten `pending`.

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

- **Forschung-Folge 98, hard atom (TE-Schätzer-Diagnose):** `research-max`
  (pro/max) — Root Cause der zwei roten CoherentPhase-Tests: vertauschte
  `(target, driver)`-Ordnung; drei unabhängige Zeugen (Schätzer-Mathematik
  `te.rs:578–586/647`, Probe-Vertrag `multi_force_te_probe.rs:63`, die zwei
  Schwester-Tests mit korrekter Ordnung). Kein flash-Vorlauf — die Klasse
  „TE-/Null-Konstruktion" ist ein benanntes hartes Atom (`AGENTS.md`); ein
  flash-Vergleich ist offen. Burn `research-max` `$0.0565` (`session_burn`).

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (5 Aufrufstellen der zwei CoherentPhase-Tests)
- `.github/workflows/measure-gates.yml` (CDN-Probe-Schritt entfernt)
- `.github/workflows/silence-map-probe.yml` (neu)
- `docs/handover/handover-2026-09-19-forschung-folge98.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge97.md` (Move)
- **Fremd (nicht angefasst):** die drei Handover-Moves `entscheid-folge24`,
  `forschung-folge44`, `forschung-folge51` → `archiv/`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
