<!--
  title: Handover — Forschung-Folge 85 (Stand 2026-09-18)
  session: Forschung-Folge 85
  class: handover
  date: 2026-09-18
  sha256: 2a2911db0a73fd5e48788317a92386cfaa05202facd23b18b0988ccce1e9aa89
  status: live
-->
# Handover — Forschung-Folge 85 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18)

- **HEAD** `891e8039` (Session-Start), == `origin/main`; `git_safety` Snapshot
  `refs/safety/1789740165`.
- **Postfach** — kein neuer Agenten-Eingang (Ledger `1789737560`); `post.md` trägt
  keinen eigenen Anteil (Entscheid-Folge 52 hat die `An entscheid`-Zeile gefaltet,
  `handover-2026-09-18-entscheid-folge52.md:47–48`).
- **CI** (`ci_manage list`/`view`) — `measure-gates` `35353722981` @`2568285e`
  **completed failure**: die Scope-Scheidung griff (der `deredden`-Abbruch ist
  weg), aber der Silence-Map-Bin selbst fällt. Artefakt `measure-gates.txt`
  (1165 B, Artifact `10550159194`): 4 Tests ok, 2 FAILED. Diagnose + Fix s. u.

## Measure-Gates — Erwartungszählung des Silence-Map-Probes (härtester undatiert)

- **Befund (gemessen, Artefakt `35353722981`):** `silence_map`
  (`tools/measure/src/bin/silence_map_probe.rs`) klassifiziert **jede** Zelle als
  `blind`: `expectation = (Σ K)/n · V` ist um den Faktor `n` zu klein — die
  KDE-Dichte ist `f̂ = (Σ K)/n`, der erwartete Zähler ist `n · f̂ · V = Σ K · V`.
  Mit λ̂ < 4 ⇒ `threshold = λ̂ − 2√λ̂ ≤ 0` ⇒ `blind`; `still` = `consistent` = 0;
  FP-Gate (`testable > 0`) und FN-Gate (`holed.still > baseline.still`) fallen,
  `a_deficit_free_field_does_not_hold_the_map_verdict` passiert trivial (0 ≤ 0).
- **Fix (eigene Datei):** `silence_map_probe.rs` — `expectation.push(sum / n * volume)`
  → `expectation.push(sum * volume)`; die ungenutzt gewordene `let n` entfernt.
  `cargo check -p omegaflow-measure --bin silence_map_probe` grün, 0 Warnungen.
- **Re-Dispatch ausstehend:** nach Commit/Push `gh workflow run measure-gates.yml`.
  Erst der neue Lauf zeigt die sechs Silence-Map-Gates + die reale CDN-Messung.
  (Schritt: `ci_manage view <neue-id>` + Artefakt `measure-gates`; Trigger
  Run-Abschluss.) `wartend`
- Der frühere `deredden`-Testfehler bleibt bei der Bau-Linie gefaltet
  (`bau-folge85.md:85`; fremde Probe).

## TE-Gate n=1000 Conditional-Null — `wartend` (fremder Ledger)

- `35324015019` @`fb6b62b4`; Trigger Run-Abschluss. Nach grünem Verdikt wird der
  `211A→193A`-Conditional-Check frei (`docs/paper/solar-seconds-matrix.md:37,46`).
  (Schritt: `ci_manage view 35324015019`; TE-/Null `src/mathematikerin/te.rs`.)

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

## Released — descoped mit Messung (2026-09-18)

- **Fasy-Bootstrap-Signifikanz** — descoped: kein Konsument im Baum leitet aus dem
  Betti-0-Persistenz-Verdikt eine Signifikanz-Aussage ab (gemessen, `grind-flash`).
  Einziger Nicht-Test-Konsument `tools/measure/src/bin/betti0_probe.rs` druckt die
  Rohwerte; die `te.rs`-Aufrufer sind die vier In-File-Gates (FP/FN/Symmetrie/
  n-Floor); `silence_map_probe.rs` berührt das Persistenz-Verdikt nicht;
  `signifikan`/`p_value`/`confidence` kommen in `src/` + `tools/` nicht vor. Das
  Fasy-Gate ist nicht gebraucht; wieder aufgenommen nur bei einem ersten Konsumenten.

## Benchmark

- Routine-Klasse (Konsumenten-Messung) → `grind-flash` (flash-first; die
  Routine-Klasse ist geschlossen, 2026-09-16). Kein pro/max-Spiegel nötig.

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/silence_map_probe.rs` (Fix)
- `docs/handover/handover-2026-09-18-forschung-folge85.md` (neu)
- `docs/handover/archiv/handover-2026-09-18-forschung-folge84.md` (Move)
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md`,
  `docs/handover/post.md`, `tools/measure/src/bin/deredden_baseline_probe.rs`,
  die drei `handover-2026-09-16-*`-Moves, der `entscheid-folge51`→`archiv`-Move,
  `docs/handover/handover-2026-09-18-entscheid-folge52.md` und die
  `handover-2026-09-18-{bau,ernte}-folge*.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
