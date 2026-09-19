<!--
  title: Handover — Forschung-Folge 103 (Stand 2026-09-20)
  session: Forschung-Folge 103
  class: handover
  date: 2026-09-20
  sha256: 8355cd50699cb870d5fe75199fa34daa502ac538110062e89660ec33abca6716
  status: live
-->
# Handover — Forschung-Folge 103 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 103)

- **HEAD** `bfa204ae` (ernte folge103) == `origin/main` bei Session-Beginn; der
  Baum bewegte sich mid-Session (`34482126` → `bfa204ae`, fremde Commits der
  Ernte-Linie + bau98); meine te.rs-Hunks sitzen sauber darauf. `git_safety`
  Snapshot `refs/safety/1789858138`.
- **Postfach** — `post.md` sauber; keine `An forschung`-Zeile. `external-state.md`-
  Eintrag Postfach @2026-09-20, Intervall nicht abgelaufen → zitiert, nicht kopiert.
- **CI** — `external-state.md` CI-Eintrag @`bfa204ae` neu gemessen: **pending**
  `ci-check` `35474637619` @`bfa204ae`, `health-check` `35473916149`;
  **in_progress** `ci-check` `35473943046` @`9f48b5bd`, `te-gate` `35473941500`
  @`9f48b5bd`; **success** `openneuro-eeg-probe` `35474638076` (bau98 companion-fix
  `.vhdr`→`.eeg`; der `An bau`-Post erledigt), `harvest` `35474615876`/`35474641409`,
  `harvest-dispatch` `35474610602`/`35474630076`, `auto-dispatch` `35474630080`,
  `swpc-mirror-cdn` `35474661305`, `allwise-cdn` `35471283884`, `fmt-apply`
  `35472907001`; **cancelled** die ci-check-Kette (per-ref-Concurrency).
  Nach dem Push (`5b406e16`): `ci-check` `35475222662` (Push-Auto) + `te-gate`
  `35475226890` (dispatcht) pending.
- **TE-Gate n=1000 FPR-Boden** — Verdikt `35425288111` @`e9e9ef63` **failure**
  (gemessen, `ci_manage log`): `gate_conditional_arx_fpr_fn_n1000`, FPR rise
  4.00pp bei rho=0.5 (a=0: 2/100 → a=0.9: 6/100), größte Zelle 7/100 (unter dem
  8%-Boden). Ursache: die feste 2pp-Decke, nicht die Null. Rat folge103 streicht
  die feste Decke (fabriziert, unter 1σ bei neg≈100) und wendet
  `fpr_rise_sigma_test` (3σ Binomial-Differenz) auf die vier verbleibenden Gates
  an — **gebaut, committet** (`te.rs`); kalibriertes Verdikt ausstehend.

## n=1000 FPR-Gate — 3σ-Fix committet, kalibriertes Verdikt ausstehend — `wartend` (CI-Verdikt)

Die feste `fpr9 - fpr0 <= 2.0`-Decke in vier Gates (`te.rs:4490`
`fpr_bins_sweep_n2000`, `4795` `gate_conditional_arx_fpr_fn_n1000`, `4873`
`_reversed`, `4973` `_2`) ist durch `fpr_rise_sigma_test(fpr0, fpr9, neg0, neg9)`
ersetzt (3σ, bereits in `gate_fpr_autocorr_assert` seit `1934c012`); die 8%-Zell-
und FN-Äste unberührt. `cargo check --all-targets` 0 Fehler / 0 Warnungen.
Commit `5b406e16` gepusht; `te-gate` `35475226890` @`5b406e16` dispatcht (queued
hinter dem Prä-Fix-Lauf). (Schritt: `ci_manage view 35475226890` liest das
kalibrierte Verdikt; der Prä-Fix-Lauf `35473941500` @`9f48b5bd` bleibt als
Prä-Fix-Messung lesbar.)

## Flare-Gate — Power-Probe gebaut — `wartend` (CI-Verdikt)

Die print-only Probe `flare_envelope_power_probe` (`te.rs`, `#[ignore]`) + Step in
`.github/workflows/te-gate.yml` liegen in `9f48b5bd`. Der Lauf `35473941500`
@`9f48b5bd` trägt das `flare power probe`-Output. (Schritt: `ci_manage log
35473941500` liest n∈{400,600,1000} `found/meas/power`; danach das Gate an das n
setzen, dessen Power den 50%-Floor mit Marge trägt, oder Rat-Wort über das
Driver-Design.)

## ci-check-Gate — Timeout-Hunks bereit — `wartend` (CI-Verdikt)

Die vier `timeout-minutes`-Hunks in `ci-check.yml` (build 60 / format 15 / test
120 / clippy 120) liegen in `9f48b5bd`; bau98 fügte `tools/harvest/**` zu den
push-Pfaden. (Schritt: `ci_manage view` des ci-check auf dem neuen HEAD — die
Tests 1+2 `coherent_phase_null_*` entscheiden den folge98-Swap, `format`/`clippy`
den Rest.)

## CI-Verdikte am Gate-Fluss — `wartend` (nach dem ci-check/te-gate-Neulauf)

- Riss 2 (FFT-Pad-Randartefakt) + Riss 3 (τ-Instabilität): Messgates gebaut
  (folge100); Verdikt aus `ci-check`/`te-gate` auf dem neuen HEAD.
- `hyperscanning-te` `35472171277` @`1fd18b31` pending; `silence-map-probe`
  `35468942740` @`4638d7f3` in_progress (Wandzeit seit 20:57Z). (Schritt: Verdikte
  lesen, dann den jeweiligen Punkt schließen.)
- Takens-Screen Wandzeit; danach Bestätigungsstufe p99/1000 + Frontalkanäle F3/F4
  (`gh workflow run hyperscanning-te -f …`). Abgeleiteter Watchdog-Floor: nach der
  Takens-Wandzeit-Messung.

## ernte / termin

- **MAG-Asset** — `blockiert` (ernte): Compiler nach `voyager_odr_compiler.rs`,
  dann `sources.φ` + CI-Manifestation.
- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027). **Flyby-Path-2-Abruf** —
  `termin` (28.09.).

## Benchmark (dieses Atom)

- **Gate-Konsistenz-Entscheid** (die vier 2pp-Decken): `council` (pro/max) —
  Verdikt „apply" einstimmig; kein flash-Gegenlauf, die Klasse ist Architektur/Urteil,
  kein Routinelauf. Kein Doppel-Lauf in dieser Runde.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (vier Asserts → `fpr_rise_sigma_test`)
- `docs/zustand/external-state.md` (CI-Status + TE-Gate-Zeile)
- `docs/handover/handover-2026-09-20-forschung-folge103.md` (neu) +
  `archiv/handover-2026-09-20-forschung-folge102.md` (Move)
- **Fremd (nicht angefasst):** die uncommitteten Moves
  `entscheid-folge24`/`forschung-folge44`/`forschung-folge51` → `archiv/` (D + ??),
  `docs/handover/handover-2026-09-20-ernte-folge103.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
