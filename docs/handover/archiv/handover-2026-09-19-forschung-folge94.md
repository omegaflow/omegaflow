<!--
  title: Handover — Forschung-Folge 94 (Stand 2026-09-19)
  session: Forschung-Folge 94
  class: handover
  date: 2026-09-19
  sha256: ab92e2b8fe2777d9dc8a7dc86ecfba694aae3d0580a8e629039a6e5313a77452
  status: live
-->
# Handover — Forschung-Folge 94 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 94)

- **HEAD** `a786e208` == `origin/main`; `git_safety` Snapshot `refs/safety/1789840926`
  (Session-Beginn). Seit folge93 (`88fb27ad`): `2c13f7da` (Re-Dispatch-Registrierung),
  `1b0c3303` (bau: TE-Gates/Betti-Null), `a786e208` (pipeline: vier OpenNeuro-Datasets
  ds007822/ds007471/ds004103/ds008192 in `ledger.φ`). Fremde uncommittete Arbeit:
  drei Handover-Moves (`entscheid-folge24`, `forschung-folge44/51` → `archiv/`) —
  nicht angefasst.
- **Postfach** — kein `An forschung`-Eingang (`post.md` leer). `mail_ledger.φ`
  neuester Eintrag `1789715019` (Brave-Alert); keine neue forschungsrelevante
  Antwort. NSE/Haug-Keller (17.09.: „in einigen Tagen") bleibt Trigger.
- **CI** — `ci_manage list` (2026-09-19 ~18:1x): `hyperscanning-te` `35457694014`
  **in_progress** @`9806bb52` (Loader-Fix-Verifikation); `measure-gates`
  `35457737916` pending, `35456056999` **in_progress** @`f2a30682` (silence_map
  FN-Gate). Die CI-Zeile in `docs/zustand/external-state.md` wird zitiert, nicht kopiert.

## Hyperscanning-TE — kohärente Phasen-Null gebaut, CI-Verifikation ausstehend (härtester undatiert)

- **Gebaut (dieses Atom, in-session):** `TeNull::CoherentPhase` + `coherent_phase_surrogates`
  (`src/mathematikerin/te.rs`) — ein gemeinsamer φ-Vektor über alle Serien eines Sets;
  S_xy bleibt exakt (e^{iφ}·e^{-iφ}=1), die Struktur zweiter Ordnung inkl. der gerichteten
  Lag-Phasen bleibt, das Bispektrum fällt. Dispatched in `conditional_te_surrogates_n`
  (rotiert x UND y). `hyperscanning_group_te --null phase|coherent-phase` (Default
  `phase` — die p95/200-Reihe bleibt byte-identisch). Rat-Entscheidung (council,
  pro/max): die gemeinsame Rotation ist der korrekte Paar-Null für „TE über die lineare
  Kreuzkorrelation hinaus"; **Begriffsrichtigstellung:** die Einzel-Phasen-Null zerstört
  die Kreuzphasen-Struktur (erhält nur die Kohärenz) und liegt unter der kohärenten
  Nullverteilung — liberal, nicht konservativ (folge93 formulierte „konservative Obernull").
- **Kalibrier-Gate gebaut** (te.rs `#[cfg(test)]`): FP-Arm
  `gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200` (+ n=1000 ignoriert,
  für `te-gate.yml`), Linear-Kreuz-Absorption `coherent_phase_null_absorbs_linear_cross_coupling`
  (FP-Beweis + Komplementär unter `TeNull::Phase`), nichtlinearer TP-Arm
  `coherent_phase_null_detects_nonlinear_transfer`. `cargo check --workspace --all-targets`
  0 Fehler/0 Warnungen. (Schritt: `gh workflow run te-gate.yml`; `ci_manage view <id>`
  liest das Verdikt — die drei Arme + der n=1000-Arm.)
- **Loader-Fix-Verifikation** — Lauf `35457694014` @`9806bb52` in_progress (p95/200).
  (Schritt: `ci_manage view 35457694014` + Artefakt `hyperscanning-te-report`; ein
  lesbares [Fz]-Series = die Messung.)
- **Kohärenten Paar-Null auf den Screen anwenden** — nach grünem Loader-Lauf.
  (Schritt: `hyperscanning_group_te --null coherent-phase` im Workflow oder als zweiter
  Dispatch; die Differenz der Survivor-Zellen ist die Messung „über das Lineare hinaus".)
- **Risse (Rat, ungeglättet):** Gauss-Blindheit (CoherentPhase kann Gauss-linearen
  Transfer prinzipbedingt nie detektieren — zwei Nulls, zwei Fragen); nicht-Gauss-Marginale;
  Familien-Max-Interaktion unter CoherentPhase (stärkstes lineares Paar verdeckt schwächere
  nichtlineare → FN-Risiko; das Gate testet paarweise, nicht die Familien-Statistik);
  gemeinsamer zirkulärer Randartefakt; Ksg-Verhalten `pending`.
- **Bestätigungsstufe** — Screening p95/200, Überlebende p99/1000. (Schritt:
  `hyperscanning_group_te --percentile 99 --surrogates 1000` auf den Survivor-Zellen.)
- **Eigen-Historie als Konditionierer** — Apparat existiert (`conditional_te_*_n` mit
  `conds`). (Schritt: `LaggedCond` auf die Zielserie statt `&[]`.)
- **Mehrere Frontalkanäle** — `--channel` existiert. (Schritt: F3/F4 als getrennte
  Workflow-Matrix-Läufe.)
- **Topologische TE** (Takens, `te_compute`) als Upgrade. (Schritt:
  `topological_te_phase` statt `transfer_entropy_binned` im Gruppen-Werkzeug.)

## silence_map_probe FN-Gate — Verifikation (aus Folge 92)

- `35456056999` @`f2a30682` in_progress; der Fix `1056bc88` liegt im Lauf-Baum.
  `35457737916` pending. (Schritt: `ci_manage view 35456056999`/`35457737916` +
  Artefakt `measure-gates.txt`; grün ⇒ FN-Gate frei.)

## vC-Permeabilitäts-Karte — `operator-gebunden` (an entscheid gepostet)

- **Messakt**: `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad>
  ./target/release/omegaflow` lokal nach Core-Release-Build; danach `perm_target_probe
  --live <pfad>`. (Schritt: Operator-Wort + Release-Build.)
- **Richtungssemantik**: Legacy `exp(-vC/(g+1/C))` vs. heute `tanh(v_c/(g+PERM_GROUND))`.
  (Schritt: Operator-Wort.)

## TE-Gate n=1000 Conditional-Null — `wartend` (bau-eigen)

- `35427414837` @`fe6bdb2b`; neuer `te-gate` `35451283398` pending. (Schritt:
  `ci_manage view 35451283398`; grün ⇒ `211A→193A`-Conditional-Check frei,
  `docs/paper/solar-seconds-matrix.md:37,46`.)

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

- Der Rat (council, pro/max) entschied das Null-Modell — die TE-/Null-Konstruktion
  ist ein benanntes hartes Atom, kein flash-Mirror. Der Bau lief in-session (line).
  Burn (gemessen, `session_burn`): siehe Abschluss.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (`CoherentPhase`, `coherent_phase_surrogates`, Dispatch, 3 Gate-Arme)
- `tools/measure/src/bin/hyperscanning_group_te.rs` (`--null`, kohärenter Zweig)
- `tools/measure/src/bin/pcmci_class_benchmark.rs` (Anzeigename CoherentPhase)
- `tools/utils/src/bin/hdf5_reader.rs` (Fremd-Baum-Unblock: `ReadLength`/`FetchBudget`-Arme
  für baus committete `Hdf5Note`-Erweiterung — `cargo check --workspace` war bei HEAD rot)
- `docs/handover/handover-2026-09-19-forschung-folge94.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge93.md` (Move)
- **Fremd (nicht angefasst):** `docs/zustand/external-state.md`, `docs/handover/post.md`,
  `phi/pipeline/ledger.φ`, die fremden Handover-Moves, `?? bin/register_lookup`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
