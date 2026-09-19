<!--
  title: Handover — Forschung-Folge 89 (Stand 2026-09-19)
  session: Forschung-Folge 89
  class: handover
  date: 2026-09-19
  sha256: 2fc6d46e8a39317de0ad4f2e78de2e0c18d39a1f25e1c6c518bb1b81673e7717
  status: live
-->
# Handover — Forschung-Folge 89 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 89)

- **HEAD** `1c728300` == `origin/main`; eigener Commit folgt (Fast-Forward).
  `git_safety` Snapshot `refs/safety/1789824286` (Session-Beginn).
- **Postfach** — `post.md` trägt keine `An forschung`-Zeile; letzter
  `mail_ledger`-Eingang `1789795811` (Rubin-Forum AGN-Lightcurves, informativ),
  kein neuer forschungs-relevanter Eingang. `state/mail/mail_ledger.φ` unverändert.
- **CI** — HEAD hat seit dem Zustand-Eintrag gewechselt (`715219db` → `1c728300`),
  der Eintrag war fällig; gemessen (`ci_manage view`): `measure-gates`
  `35445637806` @`1c728300` **in_progress** (13:23Z), `te-gate` `35427414837`
  **in_progress**. Die CI-Zeile in `docs/zustand/external-state.md` ist
  fortgeschrieben.

## vC-Permeabilitäts-Karte — Live-Feld-Verteilung: Hook gebaut (dieses Atom)

- **Rat-Verdikt (council, pro/max):** die Sache selbst der „Live-Feld-Verteilung"
  ist die realisierte `(g, v_c)`-Eingangsverteilung der no-TE-Permeabilitäts-Karte
  auf dem **sensor-getriebenen** Feld des Operators; der versteckte Lauf
  (`OMEGAFLOW_HIDDEN=1`) mit Sensoren ist der Messakt. Ein selbst-getriebener
  CI-Lauf ist der analytische Ruhepunkt `(0,0,0,PERM_GROUND)` — **descoped mit
  Messung**, trägt null Verteilungsinformation.
- **Gebaut (dieses Atom):** Logging-Hook im no-TE-Zweig (`omega.rs:1626-1635`),
  env-gated über `OMEGAFLOW_PERM_LOG=<pfad>` (`OpenOptions append`, still,
  Fehler geschluckt, kein Fallback); **fresh-data-Gate** `ring_gen != perm_log_gen`
  — genau ein Eintrag je Readback, nie die wiederholten `v_c=0`-Zwischenticks.
  CSV: `ring_gen,omega_sum,g,v_c,target,alpha,field_permeability`. Gate-Test
  `the_no_te_branch_logs_one_line_per_fresh_field_sample` (`tests.rs`, drei Beine:
  exakte Gleichheit, `None` → keine Zeile, gleicher `ring_gen` → eine Zeile).
  `perm_target_probe --live <pfad>` (`tools/measure/src/bin/perm_target_probe.rs`,
  `live_mode`): sha256 + `sensors=yes|no`, Quantile, ε-Boden-Anteil,
  `TE-branch live distribution: pending`; leer → `live dump absent` exit 2;
  `sensors=no` → `refused` exit 2. `cargo check` + `cargo check --all-targets` +
  Probe-Bin: 0 Fehler/0 Warnungen (Testsuite-Verifikation läuft in `ci-check`).
- **Offen — der Messakt selbst** (`operator-gebunden`): der Hook ist jetzt gebaut,
  der Lauf braucht die Sensoren des Operators. (Schritt:
  `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> ./target/release/omegaflow` lokal
  nach Core-Release-Build; danach Dump als CDN-Asset registrieren, dann
  `perm_target_probe --live <pfad>`.)
- **Offen — Gate-Verifikation** (`pending`): der neue Test läuft in `ci-check`;
  nach dem Push dispatchen. (Schritt: `gh workflow run ci-check.yml` +
  `ci_manage view <id>`.)
- **`operator-gebunden`** — die Richtungssemantik (Legacy `exp(-vC/(g+1/C))` vs.
  heute `tanh(v_c/(g+PERM_GROUND))`). Kein CI-Lauf entscheidet sie. (Schritt:
  Operator-Wort zur Richtung.)

## Silence-Map-Null FDR/BH — `pending`

- Umstellung der Silence-Map-Null auf FDR/BH (Benjamini-Hochberg 1995, DOI
  10.1111/j.2517-6161.1995.tb02031.x). (Schritt:
  `tools/measure/src/bin/silence_map_probe.rs` Null-Korrektur + Gate-Test; erst
  nach grünem `measure-gates`.)

## Measure-Gates — Re-Dispatch (eigener Punkt) — `wartend`

- `35445637806` @`1c728300` **in_progress** (13:23Z; der Re-Dispatch aus
  Folge 88 läuft). Trigger Run-Abschluss. (Schritt: `ci_manage view 35445637806` +
  Artefakt `measure-gates`; grün ⇒ 6 Gates + reale CDN-Messung + Karte.)

## TE-Gate n=1000 Conditional-Null — `wartend` (bau-eigen)

- `35427414837` @`fe6bdb2b` **in_progress** (~6,7 h). Bau-folge88 hat die
  per-Lag-Kalibrierkurve (`te_c/thr_c`, FPR-vs-Bin) in `te-gate.yml` registriert.
  (Schritt: `ci_manage view 35427414837`; grün ⇒ `211A→193A`-Conditional-Check
  frei, `docs/paper/solar-seconds-matrix.md:37,46`.)

## multi_force_te_probe kompiliert nicht (Baum-Fund, bau) — `blockiert`

- `cargo check -p omegaflow-measure --bin multi_force_te_probe` → E0308:
  `conditional_te_stats_lagged_n`/`transfer_entropy_conditional_binned_n` erwarten
  `&[LaggedCond]`, der Bin übergibt `&[&[f32]]`
  (`tools/measure/src/bin/multi_force_te_probe.rs:67`). Post an bau liegt in
  `post.md`. (Schritt: Post-Zeile abholen; fremde Datei, nicht angefasst.)

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

- Rat: `council` (pro/max) für die Architektur-Entscheidung „was ist die Sache
  selbst der Live-Feld-Verteilung" — lieferte das Verdikt (A + descope von B).
  Bau: `grind-pro` (Urteilsatom am Feldpfad). Kein Doppellauf.
- Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/omega.rs` (`perm_log`/`perm_log_gen` + Hook)
- `src/mathematikerin/tests.rs` (`the_no_te_branch_logs_one_line_per_fresh_field_sample`)
- `tools/measure/src/bin/perm_target_probe.rs` (`--live` / `live_mode`)
- `docs/zustand/external-state.md` (CI-Zeile, eigener Hunk)
- `docs/handover/handover-2026-09-19-forschung-folge89.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge88.md` (Move)
- **Fremd (nicht angefasst):** `src/mathematikerin/te.rs`,
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `src/archivar/weberin_verdicts.rs` (bau), die drei
  `handover-2026-09-16-*`-Renames, die `post.md`-Zeilen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
