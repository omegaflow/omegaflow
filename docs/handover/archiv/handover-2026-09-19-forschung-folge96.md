<!--
  title: Handover — Forschung-Folge 96 (Stand 2026-09-19)
  session: Forschung-Folge 96
  class: handover
  date: 2026-09-19
  sha256: f47a0fa3a81af6b8c13201fc9e0145c784fd11146063dfac90628bd398937757
  status: live
-->
# Handover — Forschung-Folge 96 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 96)

- **HEAD** `eb2e414e` == `origin/main` (Merge: fmt-apply-Bot `d63e2a11` +
  forschung `f3b8684b` + entscheid `8ce7caa0`); `git_safety` Snapshot
  `refs/safety/1789848061`. Fremde uncommittete Arbeit: die drei Handover-Moves
  `entscheid24`, `forschung44/51` → `archiv/` — nicht angefasst.
- **Postfach** — kein neuer Eingang seit `1789795811` (Rubin-Forum AGN DP2,
  informativ); `post.md` leer (kein `An forschung`); jüngster `sent_ledger`
  `1789725472`. NSE/Haug bleibt Trigger. Eintrag zitiert (`external-state.md`),
  nicht kopiert.
- **CI** — **`hyperscanning-te` `35465589119` in_progress** @`eb2e414e` (der
  Screen-Re-Dispatch, 21:50 lokal nach dem Merge); der kohärente Paar-Null
  `35466436802` **pending** @`e8988ae3` (dieses Atom, dispatcht); `te-gate`
  `35462518676` **pending** @`3d2e6adb`; `ci-check` churn; `fmt-apply`
  `35464780631` success. Die CI-Zeile in `docs/zustand/external-state.md` wird
  zitiert, nicht kopiert.

## Hyperscanning-TE — Screen-Verifikation (härtester undatiert)

- **Screen-Lauf `35465589119`** in_progress @`eb2e414e`; Artefakt
  `hyperscanning-te-report` ist die Messung. (Schritt: `ci_manage view
  35465589119` einmal; grün mit Triaden ⇒ weiter, Exit-2-Gate ⇒ das ist die
  Messung, kein Defekt.)
- **Der Messpfad trägt die Null-Wahl und den Kanal** —
  `.github/workflows/hyperscanning-te.yml` hat die Inputs `null_model`
  (`phase|coherent-phase`) und `channel` sowie `set -o pipefail`. **Gemessener
  Defekt (dieses Atom):** `{ … } | tee report.txt` ohne pipefail gab immer tees
  0 zurück — das Exit(2)-Gate des Werkzeugs konnte die Conclusion **nicht** rot
  machen; der Screen-Verifikationspfad war unwirksam. (Schritt: nach Push
  dispatch, siehe nächster Punkt.)
- **Kohärenten Paar-Null auf den Screen anwenden** — Lauf `35466436802`
  pending @`e8988ae3`, dispatcht (`-f null_model=coherent-phase`); die
  Concurrency-Gruppe (`cancel-in-progress: false`) reiht hinter dem Screen
  `35465589119` ein. (Schritt: `ci_manage view 35466436802` einmal; die Differenz
  der Survivor-Zellen gegen den Screen ist die Messung „über das Lineare hinaus".)
- **Bestätigungsstufe** — Screening p95/200, Überlebende p99/1000. (Schritt:
  Workflow um `percentile`/`surrogates`-Inputs erweitern, dann
  `-f null_model=coherent-phase` mit p99/1000 auf die Survivor-Zellen.)
- **Mehrere Frontalkanäle** — `--channel` trägt nun der Workflow. (Schritt:
  `gh workflow run hyperscanning-te -f channel=F3` (und F4) als getrennte
  Matrix-Läufe.)
- **Eigen-Historie als Konditionierer** — Apparat existiert
  (`conditional_te_*_n`). (Schritt: `LaggedCond` auf die Zielserie statt `&[]`.)
- **Topologische TE** (Takens, `te_compute`) als Upgrade. (Schritt:
  `topological_te_phase` statt `transfer_entropy_binned` im Gruppen-Werkzeug.)
- **Risse (Rat, ungeglättet):** Gauss-Blindheit (CoherentPhase kann Gauss-linearen
  Transfer prinzipbedingt nie detektieren — zwei Nulls, zwei Fragen); nicht-Gauss-
  Marginale; Familien-Max-Interaktion unter CoherentPhase (stärkstes lineares Paar
  verdeckt schwächere nichtlineare → FN-Risiko); gemeinsamer zirkulärer
  Randartefakt; Ksg-Verhalten `pending`.

## te-gate `35462518676` — `wartend`

- pending @`3d2e6adb`; deckt `gate_fpr_autocorrelation*` (inkl. coherent-phase) +
  `gate_conditional_arx_fpr_fn_n1000`. (Schritt: `ci_manage view 35462518676`;
  grün ⇒ die Arme frei, `211A→193A`-Conditional-Check frei,
  `docs/paper/solar-seconds-matrix.md:37,46`.) Kein Re-Dispatch.

## Abgeleiteter Watchdog-Floor — benannter offener Punkt (Rat)

- Für den Fall „ehrlicher schneller Median → legitime Verlangsamung" jenseits 2×.
  (Schritt: messen, ob ein legitimer `hyperscanning-te`-Lauf 2× den ehrlichen
  Median übersteigt; nur dann `timeout/2²`-Floor bauen — nicht ohne diese Messung.)

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

## TE-Gate n=1000 Conditional-Null — `wartend` (bau-eigen)

- Läuft im `te-gate` `35462518676` (siehe oben). (Schritt: dessen Verdikt.)

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

- Das Atom (Workflow-Inputs `null_model`/`channel` + `set -o pipefail`) war
  Routine-Bau und lief direkt in der Session — ein 3-Hunk-YAML-Edit, die
  Delegation kostete mehr als der Edit; kein Sub-Agent, kein pro/max-Atom. Burn
  (gemessen, `session_burn`): `line` `deepseek-v4-flash` ≈ $0.038 (1 Lauf).

## Geteilter Baum — eigener Pfad-Satz

- `.github/workflows/hyperscanning-te.yml` (Inputs `null_model` + `channel`,
  `set -o pipefail`)
- `docs/zustand/external-state.md` (CI-Zeile auf `eb2e414e`)
- `docs/handover/handover-2026-09-19-forschung-folge96.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge95.md` (Move)
- **Fremd (nicht angefasst):** die Handover-Moves `entscheid24`, `forschung44/51`
  → `archiv/`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
