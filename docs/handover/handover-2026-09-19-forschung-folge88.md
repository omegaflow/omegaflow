<!--
  title: Handover — Forschung-Folge 88 (Stand 2026-09-19)
  session: Forschung-Folge 88
  class: handover
  date: 2026-09-19
  sha256: 5d5494799ae0c61e161fa5adf6dabcca1de3ecf6478d2d73ce07aa8237c36f16
  status: live
-->
# Handover — Forschung-Folge 88 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 88)

- **HEAD** `715219db` == `origin/main`; eigener Commit folgt (Fast-Forward).
  `git_safety` Snapshot `refs/safety/1789798922` (Session-Beginn).
- **Postfach** — `post.md` trug bei Session-Beginn keine `An forschung`-Zeile; die
  `An bau`-Zeile (Ernte-Folge 91) und die `An bau`-Zeile dieser Session (siehe
  Geteilter Baum) stehen am Baum. `state/mail/mail_ledger.φ` kein neuer
  forschungs-relevanter Eingang.
- **CI** — der Zustand-Ledger `docs/zustand/external-state.md` trägt die frische
  CI-Zeile (Ernte-Folge 91, ~06:48Z); diese Session zitiert sie. Eigene Messung
  für den Atom: `measure-gates` `35426091713` @4e43856e **cancelled** (nicht rot);
  `te-gate` `35427414837` @fe6bdb2b **in_progress** (~4,6 h); `ci-check`
  `35426189981` @e60aea10 und `35427572159` @715219db **failure**.

## vC-Permeabilitäts-Karte — Live-Feld-Verteilung `pending` (härtester undatierter Punkt)

- **Erster Messschritt gebaut (dieses Atom).** Rat-Verdikt (council, pro/max):
  die Sache selbst ist die realisierte Eingangsverteilung der no-TE-Permeabilitäts-
  Karte `x = v_c/(g + PERM_GROUND)`, `target = tanh(x)` in f32, gemessen auf den real
  gehaltenen SWPC/GOES-Reihen (Proxy benannt im Probe-Header) plus AR(1)/Two-Cluster/
  Phasen-Surrogat-Kalibrierung. Umgesetzt: `perm_target(g,v_c)` aus `omega.rs`
  extrahiert (der Ast ruft sie), Tick-Test
  `the_no_te_tick_maps_the_silence_signal_to_the_epsilon_floor`, Probe
  `tools/measure/src/bin/perm_target_probe.rs`, Registrierung in `measure-gates.yml`
  (+ SWPC-Fetch). `cargo check` 0 Fehler/0 Warnungen.
- **Offen — das lebende Feld ist in CI nicht erreichbar** (keine Sensoren, keine
  Radiatoren, kein Readback): die realisierte v_c/g-Verteilung des Feldes braucht
  einen **Logging-Hook** auf dem vorhandenen `probe_readback` (`omega.rs`), der
  `probe_omega` + `field_permeability` je Readback schreibt, und einen **versteckten
  Feldlauf** (`OMEGAFLOW_HIDDEN=1`). (Schritt: Hook bauen + `OMEGAFLOW_HIDDEN=1`-Lauf;
  nie als gemessen fabrizieren.) Das ist ein benanntes, ungebautes Asset.
- **`operator-gebunden`** — die Richtungssemantik (Legacy `exp(-vC/(g+1/C))` vs.
  heute `tanh(v_c/(g+PERM_GROUND))`). Kein CI-Lauf entscheidet sie. (Schritt:
  Operator-Wort zur Richtung.)

## Silence-Map-Null FDR/BH — `pending`

- Umstellung der Silence-Map-Null auf FDR/BH (Benjamini-Hochberg 1995, DOI
  10.1111/j.2517-6161.1995.tb02031.x). (Schritt:
  `tools/measure/src/bin/silence_map_probe.rs` Null-Korrektur + Gate-Test; erst
  nach grünem `measure-gates`.)

## Measure-Gates — Re-Dispatch (eigener Punkt) — `wartend`

- `35426091713` @4e43856e **cancelled** (nicht rot). Dieses Atom dispatcht
  `measure-gates` auf dem neuen HEAD erneut; der Lauf trägt jetzt betti0 +
  silence-map + `perm_target_probe`. Trigger Run-Abschluss. (Schritt:
  `ci_manage view <id>` + Artefakt `measure-gates`; grün ⇒ 6 Gates + reale
  CDN-Messung + Karte.)

## TE-Gate n=1000 Conditional-Null — `wartend` (bau-eigen)

- `35427414837` @fe6bdb2b **in_progress** (~4,6 h). Bau-folge88 hat die
  per-Lag-Kalibrierkurve (`te_c/thr_c`, FPR-vs-Bin) in `te-gate.yml` registriert.
  (Schritt: `ci_manage view 35427414837`; grün ⇒ `211A→193A`-Conditional-Check
  frei, `docs/paper/solar-seconds-matrix.md:37,46`.)

## multi_force_te_probe kompiliert nicht (Baum-Fund, bau) — `blockiert`

- `cargo check -p omegaflow-measure --bin multi_force_te_probe` → E0308:
  `conditional_te_stats_lagged_n`/`transfer_entropy_conditional_binned_n` erwarten
  seit `te.rs` `fe6bdb2b` `&[LaggedCond]`, der Bin übergibt
  `&[&[f32]]` (`tools/measure/src/bin/multi_force_te_probe.rs:67`). (Schritt:
  Post an bau — Bin auf `LaggedCond` umstellen; fremde Datei, nicht angefasst.)

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

- Stehender Pass: kein Dispatch (lokal, Routineklasse geschlossen).
- Messdesign vC: `council` (pro/max) für die Architektur-Entscheidung „was ist die
  Sache selbst" — lieferte das Verdikt (Extraction + f32-Kalibrierprobe + benanntes
  `pending` fürs Live-Feld). Bau: `grind-pro` (Urteilsatom). Kein Doppellauf.
  Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/omega.rs` (`perm_target`)
- `src/mathematikerin/tests.rs` (Tick-Test)
- `tools/measure/src/bin/perm_target_probe.rs` (neu)
- `.github/workflows/measure-gates.yml` (SWPC-Fetch + `perm_target_probe`)
- `docs/handover/handover-2026-09-19-forschung-folge88.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge87.md` (Move)
- `docs/handover/post.md` — `An bau`-Zeile (`multi_force_te_probe`) angehängt;
  **nicht committet**, weil die aktive Ernte-Folge-91-Session post.md hält (ihr
  uncommitteter Hunk liegt daneben) — ernte-91s Commit trägt die Zeile mit, sonst
  committet sie die nächste Session.
- **Fremd (nicht angefasst):** `src/archivar/weberin_verdicts.rs`,
  `phi/{blocked_sources,harvest,sources}.φ`, `src/mathematikerin/te.rs` (bau),
  die drei `handover-2026-09-16-*`-Renames, `docs/zustand/external-state.md`
  (aktive Ernte-Folge-91-Session hält sie).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
