<!--
  title: Handover — Forschung-Folge 92 (Stand 2026-09-19)
  session: Forschung-Folge 92
  class: handover
  date: 2026-09-19
  sha256: 321e198950746f969bdb5026fb8ce13cccecb4672ae544d3af86f200197e63d2
  status: live
-->
# Handover — Forschung-Folge 92 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 92)

- **HEAD** `1eef6949` == `origin/main`; `git_safety` Snapshot `refs/safety/1789835419`
  (Session-Beginn). Fremde uncommittete Arbeit im geteilten Baum (s. u.) — nicht
  angefasst.
- **Postfach** — `state/mail/mail_ledger.φ` leer (0 Zeilen); kein `An forschung`-Eingang.
  `post.md` trägt nur `An entscheid` (folge91-eigen) und `An bau` (fremd). Kein neuer Eingang.
- **CI** — `ci_manage list`/`view`/`log` + Artefakt `measure-gates.txt`
  (`gh run download 35454958335 -n measure-gates`, 2026-09-19): **failure**
  `measure-gates` `35454958335` @`5cba9f70` — `silence_map_probe`
  `an_inserted_hole_is_detected` FN-Gate; **in_progress** `ci-check` `35455108694`
  @`1eef6949`; **pending** `health-check` `35451454869`, `te-gate` `35451283398`;
  **failure** `harvest-dispatch` `35451187565` (cassini 422). Die CI-Zeile in
  `docs/zustand/external-state.md` (@`1eef6949`, Entscheid-Folge 53) ist aktuell —
  zitiert, nicht kopiert.

## silence_map_probe FN-Gate — Fix gebaut, Verifikation ausstehend (härtester undatiert)

- **Messung** (Artefakt `measure-gates.txt`): `an_inserted_hole_is_detected` rot —
  „0 still vs 0 baseline". Ursache: die KDE-Null (`silverman` h ≈ 0,369 < Loch) wird
  vom Loch selbst kontaminiert; λ̂ fällt 25 → ≈ 6, p = exp(−6) ≈ 2,5e−3 steht über dem
  BH rank-1 (5e−5) → BH sieht das Loch nicht. Der alte per-Zelle-2σ-`still`
  (observed < λ̂ − 2√λ̂) sah es; der folge90-BH-Umbau ist unterpowert, nicht falsch.
- **Fix** (Rat, pro/max): `bandwidths(points, cell)` flort jede Achse auf die Zellkante
  (`silverman(&xs)?.max(cell)`); die Null ist glatt auf der Zell-Skala, das Loch wird
  sichtbar (λ̂ ≈ 18,9, p ≈ 6e−9). `silverman` in `src/mathematikerin/te.rs` unberührt;
  Print-Label „null bandwidth h (Silverman floored at the cell edge)".
  `cargo check -p omegaflow-measure --all-targets` 0 Fehler/0 Warnungen.
- **Riss (benannt, nicht geglättet):** (1) ein Loch kleiner als die Zellkante bleibt
  konstruktiv unsichtbar — die Auflösungsgrenze, nicht der Test; (2) die Null ist die
  katalog-eigene Dichte — es gibt kein Selektionsfunktions-Modell (Footprint,
  Magnitude-Limit) im Baum. Re-Dispatch nach dem Push: `measure-gates`
  `35456056999` @`f2a30682` (in_progress). (Schritt: `ci_manage view 35456056999`
  + Artefakt `measure-gates.txt`; grün ⇒ FN-Gate frei.)

## vC-Permeabilitäts-Karte — `operator-gebunden` (an entscheid gepostet)

- **Messakt** (`operator-gebunden`): `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad>
  ./target/release/omegaflow` lokal nach Core-Release-Build; danach Dump als
  CDN-Asset registrieren, dann `perm_target_probe --live <pfad>`. (Schritt:
  Operator-Wort + Release-Build; siehe `post.md` `An entscheid`.)
- **Richtungssemantik** (`operator-gebunden`): Legacy `exp(-vC/(g+1/C))` vs. heute
  `tanh(v_c/(g+PERM_GROUND))`. Kein CI-Lauf entscheidet sie. (Schritt:
  Operator-Wort; siehe `post.md` `An entscheid`.)

## TE-Gate n=1000 Conditional-Null — `wartend` (bau-eigen)

- `35427414837` @`fe6bdb2b`; neuer `te-gate` `35451283398` pending. Bau-folge88 hat
  die per-Lag-Kalibrierkurve (`te_c/thr_c`, FPR-vs-Bin) in `te-gate.yml` registriert.
  (Schritt: `ci_manage view 35451283398`; grün ⇒ `211A→193A`-Conditional-Check
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

- Rat (`council`, pro/max) entschied den Null-Fix — die TE-/Null-Konstruktion ist
  ein benanntes hartes Atom (AGENTS.md Kostenleiter), kein flash-Mirror nötig. Der
  Rat lieferte Entscheidung + exakten Change + Riss; umgesetzt in-session.
- Burn (gemessen, `session_burn`): line $0.0649 + council ≈ $0.022 (deepseek-v4-pro);
  Gesamtburn $0.7580 / 46 Sessions.

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/silence_map_probe.rs` (Fix)
- `docs/handover/handover-2026-09-19-forschung-folge92.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge91.md` (Move)
- **Fremd (nicht angefasst):** `docs/zustand/external-state.md` (Entscheid-Folge 53),
  `docs/handover/post.md`, `phi/harvest.φ`, `phi/sources.φ`, `src/archivar/hdf5.rs`,
  `src/mathematikerin/te.rs`, `tools/harvest/src/bin/{cassini_odf,cassini_rsr,icesat2_atl03}_compiler.rs`,
  `tools/measure/src/eeglab.rs`, `.github/workflows/harvest-dispatch.yml`, die
  `handover-2026-09-1*`-Renames/Moves, `?? bin/register_lookup`,
  `?? handover-2026-09-19-entscheid-folge53.md`, `?? handover-2026-09-19-ernte-folge93.md`,
  `?? tools/measure/src/bin/hyperscanning_te_matrix.rs`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
