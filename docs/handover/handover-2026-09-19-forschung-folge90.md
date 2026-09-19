<!--
  title: Handover — Forschung-Folge 90 (Stand 2026-09-19)
  session: Forschung-Folge 90
  class: handover
  date: 2026-09-19
  sha256: aaa926a9d778b66f2d6941821c756af142cde1443f2b4771d9463b5b46921fce
  status: live
-->
# Handover — Forschung-Folge 90 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 90)

- **HEAD** `b374736c` == `origin/main`; eigener Commit folgt (Fast-Forward).
  `git_safety` Snapshot `refs/safety/1789828354` (Session-Beginn).
- **Postfach** — `post.md` trägt keine `An forschung`-Zeile; beide Zeilen gehen
  `An bau` (fremd). Letzter `mail_ledger`-Eingang `1789795811` (Rubin-Forum
  AGN-Lightcurves, informativ), kein neuer forschungs-relevanter Eingang.
- **CI** — HEAD hat seit dem Zustand-Eintrag gewechselt (`1c728300` → `b374736c`),
  der Eintrag war fällig; gemessen (`ci_manage list`/`view`, 14:47Z):
  `measure-gates` `35445637806` @`1c728300` **in_progress**, `te-gate`
  `35427414837` @`fe6bdb2b` **in_progress**, `ci-check` `35448988400` @`b374736c`
  **pending**. Die CI-Zeile in `docs/zustand/external-state.md` ist fortgeschrieben.

## Silence-Map-Null FDR/BH — gebaut (dieses Atom); Gate-Verifikation `wartend`

- **Gebaut (dieses Atom):** `tools/measure/src/bin/silence_map_probe.rs` — die
  Ensemble-Maximum-FWER-Null ersetzt durch Benjamini-Hochberg FDR (1995, DOI
  10.1111/j.2517-6161.1995.tb02031.x). Per testbarer Zelle lower-tail-Poisson-p-Wert
  `P(X <= c; lam)` exakt per logsumexp (`-lam + k*ln(lam) - ln(k!)`); BH über
  `omegaflow::te::benjamini_hochberg` bei `FDR_LEVEL = 0.05`; `still` = BH-signifikant
  (`cutoff > 0`, `p <= cutoff`). `ensemble_max`/`level`/`--surrogates`/`--seed`/
  Surrogat-Schleife entfernt; `poisson`/`next_rng` nach `mod tests` verschoben (nur
  test-genutzt, 0 Warnungen). `SilenceMap` trägt `fdr_cutoff`/`fdr_level`.
  Gate-Tests: FP (still-Anteil < 0.1), FN (Loch erkannt + `fdr_cutoff > 0`), blind,
  symmetry, n-floor, BH-Budget (`still <= FDR_LEVEL * testable`).
  `cargo check -p omegaflow-measure --bin silence_map_probe` +
  `cargo check -p omegaflow-measure --all-targets`: 0 Fehler/0 Warnungen.
- **Offen — Gate-Verifikation** (`wartend`): `cargo test`/fmt/clippy laufen in
  `ci-check`; nach Push dispatchen. Trigger Run-Abschluss. (Schritt:
  `gh workflow run ci-check.yml` + `ci_manage view <id>`.)
- **Baseline (gemessen):** `measure-gates` `35445637806` @`1c728300` ist
  SHA-gepinnt und damit die Vor-Änderungs-Messung; die Änderung im Arbeitsbaum
  korrumpiert sie nicht. Deshalb war der Register-Schritt „erst nach grünem
  measure-gates" erfüllbar, obwohl der Lauf noch offen ist.

## vC-Permeabilitäts-Karte — Live-Feld-Verteilung

- Hook gebaut (Folge 89): `OMEGAFLOW_PERM_LOG` im no-TE-Zweig, fresh-data-Gate,
  `perm_target_probe --live`.
- **Offen — Messakt selbst** (`operator-gebunden`): `OMEGAFLOW_HIDDEN=1
  OMEGAFLOW_PERM_LOG=<pfad> ./target/release/omegaflow` lokal nach Core-Release-Build;
  danach Dump als CDN-Asset registrieren, dann `perm_target_probe --live <pfad>`.
- **Offen — Hook-Gate-Verifikation** (`wartend`): `ci-check` `35448988400`
  @`b374736c` pending. Trigger Run-Abschluss. (Schritt: `ci_manage view 35448988400`.)
- **Offen — Richtungssemantik** (`operator-gebunden`): Legacy `exp(-vC/(g+1/C))`
  vs. heute `tanh(v_c/(g+PERM_GROUND))`. Kein CI-Lauf entscheidet sie. (Schritt:
  Operator-Wort zur Richtung.)
- **Post an entscheid zurückgestellt** (`wartend`): die zwei `operator-gebunden`-Punkte
  gehören als `An entscheid`-Zeile in `post.md`; `post.md` trägt jedoch fremde
  uncommittete Arbeit (eine andere Linie hat die beiden `An bau`-Zeilen konsumiert
  und gelöscht), eine eigene Zeile würde fremde Hunks in den Commit spülen. Trigger:
  fremde `post.md`-Änderung committet. (Schritt: `An entscheid: vC-Messakt +
  Richtungssemantik operator-gebunden` in `post.md` setzen.)

## Measure-Gates — Re-Dispatch (eigener Punkt) — `wartend`

- `35445637806` @`1c728300` **in_progress** (13:23Z; der Re-Dispatch aus Folge 88
  läuft). Trigger Run-Abschluss. (Schritt: `ci_manage view 35445637806` + Artefakt
  `measure-gates`; grün ⇒ 6 Gates + reale CDN-Messung + Karte.)

## TE-Gate n=1000 Conditional-Null — `wartend` (bau-eigen)

- `35427414837` @`fe6bdb2b` **in_progress** (~8 h). Bau-folge88 hat die
  per-Lag-Kalibrierkurve (`te_c/thr_c`, FPR-vs-Bin) in `te-gate.yml` registriert.
  (Schritt: `ci_manage view 35427414837`; grün ⇒ `211A→193A`-Conditional-Check
  frei, `docs/paper/solar-seconds-matrix.md:37,46`.)

## MAG-Asset — `blockiert` (ernte)

- `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (853331 B, sha256
  `6f3724f7…`, valides ZIP, PDS4-`.tab` 4.83 MB). (Schritt:
  `tools/harvest/src/bin/bc_mpo_mag_compiler.rs` nach `voyager_odr_compiler.rs`,
  dann `sources.φ`-Eintrag + CI-Manifestation.)

## BepiColombo MORE — `termin`

- Cruise-Daten erst zur Wissenschaftsphase (~April 2027) freigegeben; PI Iess und
  PSA/Bentley bestätigt. (Schritt: kein TAP-Abruf vorher.)

## NSE/Haug — `wartend`

- Keller-Antwort (17.09.): „in einigen Tagen"; er sendet die TRISP-NSE-Daten
  selbst. TRISP/MLZ-Anfrage (`214`) läuft. Trigger = Dateieingang. (Schritt: bei
  Eingang `nse_haug_trisp`-Quelle + Compiler + `sources.φ`.)

## Paper / Präregistrierung — `termin`

- Flyby Path 2 datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Kein Rat in diesem Atom. Bau: `grind-max` (harter Statistik-/Null-Atom) wurde
  dispatcht und kehrte **leer** zurück — keine Dateiänderung, leere Antwort; der
  Haupt-Agent (`build`) hat den Atom direkt gebaut. Ergebnis: `cargo check` 0/0.
  Kein Doppellauf.
- Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/silence_map_probe.rs` (FDR/BH-Umstellung)
- `docs/zustand/external-state.md` (CI-Zeile, eigener Hunk)
- `docs/handover/handover-2026-09-19-forschung-folge90.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge89.md` (Move)
- **Fremd (nicht angefasst):** `docs/handover/post.md` (fremde uncommittete
  Löschung der zwei `An bau`-Zeilen), `src/mathematikerin/te.rs`,
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `src/archivar/weberin_verdicts.rs`,
  `tools/measure/src/bin/{corona_conditional_probe,multi_force_te_probe,placebo_pair_eeg_probe}.rs`,
  die `handover-2026-09-16-*`-Renames, `docs/handover/handover-2026-09-19-bau-folge89.md`,
  `tools/harvest/src/bin/cassini_odf_compiler.rs`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
