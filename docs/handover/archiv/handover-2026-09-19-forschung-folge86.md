<!--
  title: Handover — Forschung-Folge 86 (Stand 2026-09-19)
  session: Forschung-Folge 86
  class: handover
  date: 2026-09-19
  sha256: dbd766036099935f2e5071e543f4522d436337db0aeb4931986853c253dfc512
  status: live
-->
# Handover — Forschung-Folge 86 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19)

- **HEAD** `cdbdc315` == `origin/main`; `git_safety` Snapshot `refs/safety/1789762019`.
- **Postfach** — die `An forschung`-Zeile (Entscheid-Folge 52: vC-Inversion +
  Silence-Map-Null FDR/BH) gefaltet, Zeile aus `post.md` gelöscht; kein neuer
  Ledger-Eingang seit `1789737560` (external-state-Eintrag, nicht fällig).
- **CI** (`ci_manage list`/`view`) — `measure-gates` `35389565681` @`e60aea10`
  **in_progress** (Re-Dispatch des silence-map-Fixes); `ci-check` `35389565276`
  in_progress; `te-gate` `35324015019` @`fb6b62b4` **completed failure** (s. u.);
  successes: `quake-feeds-cdn` `35387849782`, `swpc-mirror-cdn` `35386479836`.

## TE-Gate n=1000 Conditional-Null — red; Fix gebaut (härtester undatiert)

- **Befund:** te-gate `35324015019` @`fb6b62b4` completed failure:
  `gate_conditional_arx_fpr_fn_n1000` (Assert te.rs:4102). Die Arx-Null leckt die
  x–c-Kreuzkorrelation, FPR steigt mit a und rho — a=0.9 rho=0.5: 9/100,
  a=0.5 rho=0.9: 11/100, a=0.9 rho=0.9: 18/100 (Gitter 2/4/5/2/2/9/6/11/18).
  Grün im selben Lauf: shift/block/ksg/arx sweeps (≤6.9 %) + 6
  `gate_fpr_autocorrelation`-Arme. (Messung: `ci_manage log 35324015019`.)
- **Ursache (research-max, gemessen, read-only):**
  `arx_restricted_surrogate_conditional`/`_2` nullten die x-Lags in der
  Re-Simulation; die kollineare x–c-Struktur (`x = a·x + rho·c`) ging verloren,
  die Null wurde zu eng — der Rat-Einwand bestätigt.
- **Fix (eigene Datei `src/mathematikerin/te.rs`):** Re-Simulation mit
  `lagged_predict_nx`/`lagged_predict_2x` (x-Lags behalten);
  `lagged_predict_nx_zeroed`/`lagged_predict_2x_zeroed` gelöscht; Rename
  `arx_conditional_surrogate`/`arx_conditional_surrogate_2` (Name =
  Implementation). `cargo check -p omegaflow --all-targets` 0 Fehler/0 Warnungen.
- **Re-Dispatch:** `gh workflow run te-gate.yml` → `35425288111` @`e9e9ef63`
  (queued 2026-09-19); Trigger Run-Abschluss. Verdikte
  `gate_conditional_arx_fpr_fn_n1000`/`_reversed`/`_2` + FN-Arm
  `found/meas ≥ 0.5` lesen. (Schritt: `ci_manage view 35425288111`.) `wartend`
- **Folge:** grünes Verdikt ⇒ `211A→193A`-Conditional-Check frei
  (`docs/paper/solar-seconds-matrix.md:37,46`); bis dahin `pending`.

## Measure-Gates silence-map — `wartend`

- `35389565681` @`e60aea10` in_progress (Fix `expectation = ΣK·V`, committet in
  `e60aea10`). Trigger Run-Abschluss. (Schritt: `ci_manage view 35389565681` +
  Artefakt `measure-gates`.) Grün ⇒ 6 Gates + reale CDN-Messung.

## vC-Verteilung (Post Entscheid-Folge 52) — Richtung `operator-gebunden`, Verteilung messbar

- vC-Inversion belegt: Legacy `exp(-vC/(g+1/C))`
  (`archive-root/…/minkowski-field-permeability.md:162`) vs. heute
  `tanh(v_c/(g+PERM_GROUND))` (`omega.rs:1606`). Die Richtungs-Designfrage ist
  operator-gebunden; der erste Messschritt läuft ohne Wort: v_c-Verteilung
  (AR(1)-Null, Two-Cluster, Phasen-Surrogate) in CI. (Schritt:
  `docs/handover/handover-2026-09-18-entscheid-folge52.md` §Gremium+Wissenschaft;
  Probe in `tools/measure` + `measure-gates`.)

## Silence-Map-Null FDR/BH (Post Entscheid-Folge 52) — `pending`

- Die Silence-Map-Null soll auf FDR/BH (Benjamini-Hochberg 1995, DOI
  10.1111/j.2517-6161.1995.tb02031.x) umgestellt werden. (Schritt:
  `tools/measure/src/bin/silence_map_probe.rs` Null-Korrektur + Gate-Test; erst
  nach grünem `35389565681`.)

## MAG-Asset — `blockiert` (ernte)

- `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (853331 B, sha256
  `6f3724f7…`, valides ZIP, PDS4-`.tab` 4.83 MB). (Schritt:
  `tools/harvest/src/bin/bc_mpo_mag_compiler.rs` nach `voyager_odr_compiler.rs`,
  dann `sources.φ`-Eintrag + CI-Manifestation.)

## BepiColombo MORE — `termin`

- Cruise-Daten erst zur Wissenschaftsphase (~April 2027) freigegeben; PI Iess und
  PSA/Bentley bestätigt. (Schritt: kein TAP-Abruf vorher.)

## NSE/Haug — `wartend`

- Keller-Antwort (17.09.), „in einigen Tagen"; TRISP/MLZ-Anfrage (`214`) läuft.
  Trigger = Dateieingang. (Schritt: bei Eingang `nse_haug_trisp`-Quelle +
  Compiler + `sources.φ`.)

## Paper / Präregistrierung — `termin`

- Flyby Path 2 datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Diagnose TE-Null: `research-max` (pro/max) — hartes Atom (TE-/Null-Konstruktion),
  read-only, lieferte Mechanismus + exakten Diff. Kein flash-Äquivalent für dieses
  Atom (vom Operator für harte Diagnose benannt); kein Doppellauf. Burn: `session_burn`.
- Routine-Klasse geschlossen (2026-09-16): kein flash-Spiegel nötig.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (Fix + Rename)
- `docs/zustand/external-state.md` (TE-Gate-Zeile)
- `docs/handover/post.md` (`An forschung`-Zeile gelöscht, `An bau`-Zeile ergänzt)
- `docs/handover/handover-2026-09-19-forschung-folge86.md` (neu)
- `docs/handover/archiv/handover-2026-09-18-forschung-folge85.md` (Move)
- **Fremd (nicht anfassen):** `src/archivar/hdf5.rs`, `src/archivar/tests.rs`,
  `src/archivar/voyager_odr.rs`, `phi/sources.φ` (bau uncommittet),
  `docs/handover/handover-2026-09-18-{bau,ernte,entscheid}-folge*.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
