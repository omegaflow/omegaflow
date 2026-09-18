<!--
  title: Handover — Forschung-Folge 84 (Stand 2026-09-18)
  session: Forschung-Folge 84
  class: handover
  date: 2026-09-18
  sha256: 0c0c2cefc2fe307359eb56647a6b79c4bc33a33c762b45bf5bbf972f72b2d4c0
  status: live
-->
# Handover — Forschung-Folge 84 (2026-09-18)

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
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-18)

- **HEAD** `9f75056c` (Session-Start `533c4b2a`; die Ernte-Linie rückte den Baum
  während der Session vor; `9f75056c` == `origin/main`).
- **Postfach** — kein neuer Agenten-Eingang; letzter Ledger `1789737560` (eigene
  Antwort an PI Iess). `post.md` trägt die eigene `An entscheid`-Zeile; die zwei
  fremden `An bau`-Zeilen hat die bau-Linie noch während dieser Session gefaltet
  (`bau-folge85.md:85`).
- **CI** — `measure-gates` `35351695849` @`52d0486d` **completed failure**
  (diagnostiziert, s. u.); `ci-check` `35351962459` pending;
  `harvest`/`harvest-long`/`planetary-odf-cdn` in_progress/pending
  (`ci_manage list` 2026-09-18). Der Zustand-Ledger-Eintrag steht auf `533c4b2a`
  (um einen Commit veraltet) — fremde Datei, nicht angefasst.

## Measure-Gates — Scope-Fix, Re-Dispatch ausstehend (härtester undatiert)

- Run `35351695849` @`52d0486d` **failure**: Schritt 2 lief
  `cargo test -p omegaflow-measure` über die ganze Kiste;
  `deredden_baseline_probe` schreibt nach `/tmp/opencode/db_probe_*.be19`
  (in CI absent) → 2 Test-Fehler → cargo brach ab, **bevor** die
  Silence-Map-Gates liefen. Der Betti-0-Gate selbst war grün (4 Tests,
  `measure-gates.txt:143–149`).
- Fix (eigene Datei): `.github/workflows/measure-gates.yml:27` auf
  `cargo test -p omegaflow-measure --bin silence_map_probe` gescoped.
  (Schritt: nach Commit `gh workflow run measure-gates.yml`, Run-ID registrieren,
  einmalig `ci_manage view <id>` + Artefakt `measure-gates`; erst dann stehen die
  sechs Silence-Map-Gates + die reale CDN-Messung auf dem Prüfstand.)
- Der `deredden`-Test-Fehler ist von der bau-Linie gefaltet (`bau-folge85.md:85`;
  fremde Probe, `tools/measure/src/bin/deredden_baseline_probe.rs`).

## Betti-0-Schwellenleiter — `operator-gebunden`

- Rat-Verdikt (2026-09-18): Zweierpotenz-Leiter **behalten** — sie ist ein
  Anzeige-Gitter (`te.rs:2062–2071`), der Verdikt `persistence` leiter-unabhängig;
  Edelsbrunner/Fasy prüfen Intervall-Signifikanz, DBSCAN wählt ein ε — andere
  Frage. Als `An entscheid`-Post. (Schritt: Operator-Wort.)

## Fasy-Bootstrap-Signifikanz — `pending` (Rat-Zukunft)

- Eigener Gate, nur wenn ein Konsument eine Signifikanz-Aussage braucht; keine
  Leiter-Ablösung. (Schritt: ersten Konsumenten messen, sonst schweigt der Punkt.)

## Fabrication-Rest — `operator-gebunden`

- `docs/concepts/kybernetische-astrophysik.md:471–479` trägt die
  Erwartungsmodell-Sprache. Als `An entscheid`-Post. (Schritt: Konzept auf die
  Katalog-Form korrigieren oder als historische Stufe kennzeichnen.)

## Legacy-Rest — `operator-gebunden`

- Rat-Rangfolge (2026-09-18), nur als Fortsetzung je eigener Messung:
  **Minkowski** (Delta-Probe), **Certainty** (vC-Asymmetrie), **Delay Spectrum**,
  **Synthetic Flight**, **Channel Apertures**. Als `An entscheid`-Post.
  (Schritt: Operator-Wort; danach je eigene Messung.)

## MAG-Asset — `blockiert` (ernte)

- `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (853331 B, sha256
  `6f3724f7…`, valides ZIP, PDS4-`.tab` 4.83 MB). (Schritt:
  `tools/harvest/src/bin/bc_mpo_mag_compiler.rs` nach `voyager_odr_compiler.rs`,
  dann `sources.φ`-Eintrag + CI-Manifestation.)

## BepiColombo MORE — `termin`

- Cruise-Daten werden erst zur Wissenschaftsphase (~April 2027) freigegeben; PI
  Iess und PSA/Bentley bestätigt. (Schritt: kein TAP-Abruf vorher; zur
  Wiedervorlage den MORE-URN authentifiziert abrufen.)

## TE-Gate n=1000 Conditional-Null — `wartend` (fremder Ledger)

- `35324015019` @`fb6b62b4` in_progress. Trigger: Run-Abschluss. Nach grünem
  Verdikt wird der `211A→193A`-Conditional-Check frei
  (`docs/paper/solar-seconds-matrix.md:37,46`). (Schritt:
  `ci_manage view 35324015019`; TE-/Null `src/mathematikerin/te.rs`.)

## NSE/Haug — `wartend`

- Keller-Antwort (17.09.), „in einigen Tagen"; TRISP/MLZ-Anfrage (`214`) läuft.
  Trigger = Dateieingang. (Schritt: bei Eingang `nse_haug_trisp`-Quelle +
  Compiler + `sources.φ`.)

## Paper / Präregistrierung — `termin`

- Flyby Path 2 datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Released — descoped mit Messung (2026-09-18)

- **0-Kanon je stiller Zelle gegen `sources.φ`/`ledger.φ`** — die Register tragen
  **keinen Himmels-Footprint** (82 `ra`/`dec`-Punktspalten, 0
  coverage/healpix/polygon; `ledger.φ` ohne Himmelsposition; `dr3_stars.bin`
  registriert `sources.φ:7762`). Eine stille Zelle lässt sich nicht an eine
  registrierte Quelle binden — der Befund steht am Messort
  (`tools/measure/src/bin/silence_map_probe.rs`, `0-Kanon:`-Zeile). Ein
  Footprint-Feld wäre ein eigener Architektur-Akt; kein Konsument verlangt ihn.

## Benchmark

- Register-Abdeckung: `grind-flash` (Routine-Klasse, flash-first; kein
  pro/max-Spiegel nötig). Betti-0: Rat (pro/max, Architektur). Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `.github/workflows/measure-gates.yml`,
  `tools/measure/src/bin/silence_map_probe.rs`, `docs/handover/post.md` (nur die
  eigene `An entscheid`-Zeile + Header-sha), dieses Handover, archiviertes
  `docs/handover/archiv/handover-2026-09-18-forschung-folge83.md`.
- **Fremd (nicht anfassen):** `AGENTS.md`, `docs/zustand/external-state.md`,
  `phi/sources.φ`, `src/gate/*`, `src/archivar/main_flow.rs`,
  `src/mathematikerin/omega.rs`, `src/mathematikerin/tests.rs`,
  `tools/measure/src/bin/weberin_verdicts_compiler.rs`,
  `.github/workflows/weberin-verdicts-cdn.yml`, die
  `handover-2026-09-18-{bau,ernte,entscheid}-folge*.md` und die
  `handover-2026-09-16-*`-Renames/Deletes.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
