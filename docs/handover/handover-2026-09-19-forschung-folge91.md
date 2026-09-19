<!--
  title: Handover — Forschung-Folge 91 (Stand 2026-09-19)
  session: Forschung-Folge 91
  class: handover
  date: 2026-09-19
  sha256: 482ffff127bb0efc3f59fe208d70765fa8b5bbcf1b141c11759b96675036b369
  status: live
-->
# Handover — Forschung-Folge 91 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 91)

- **HEAD** `5cba9f70` == `origin/main`; eigener Commit folgt (Fast-Forward).
  `git_safety` Snapshot `refs/safety/1789834155` (Session-Beginn).
- **Postfach** — `post.md` war leer; kein `An forschung`-Eingang. Letzter
  `mail_ledger`-Eingang `1789795811` (Rubin-Forum AGN-Lightcurves, informativ),
  unverändert. Dieses Atom setzt die erste `An entscheid`-Zeile.
- **CI** — `ci_manage list`/`log` (~18:00Z): `ci-check` `35451506666` @`5cba9f70`
  **failure** — fremd: 5 clippy-Lints (`fugin.rs:43`, `zarr.rs:203`,
  `omega.rs:1626`, `te.rs:1972`, `tests.rs:4722`), 2 test-fails (`te.rs`
  `flare_envelope_conditional_keeps_true_coupling`, `synthetic_dag_recovers_known_direction`),
  fmt `tools/utils/src/bin/archive_search/{pdf,psychporta,pubmed,server,archive_search}.rs`;
  `silence_map_probe`/`main_flow` **0 Treffer** (forschung-eigenes ist nicht die
  Ursache). `measure-gates` `35445637806` **cancelled** → Re-Dispatch
  `35454958335` (dieses Atom). `te-gate` `35451283398` pending.
  `harvest-dispatch` `35451187565` failure (cassini dispatch HTTP 422
  unexpected inputs `format`/`timeout`). Die CI-Zeile in
  `docs/zustand/external-state.md` ist fortgeschrieben.

## Measure-Gates — Re-Dispatch `35454958335` — `wartend`

- Die forschung-eigene Verifikation (FDR/BH-`silence_map_probe` **und**
  `perm_target_probe` des vC-Hooks) läuft in `.github/workflows/measure-gates.yml`.
  Der Vorgänger `35445637806` wurde abgebrochen; dieses Atom hat `35454958335`
  dispatcht. Trigger Run-Abschluss. (Schritt: `ci_manage view 35454958335` +
  Artefakt `measure-gates`; grün ⇒ FDR/BH-Gate + vC-Hook bestätigt.)
- **Vor-Messung (bereits gelesen):** in `ci-check` `35448988400`/`35451506666`
  trägt der `test`-Step **keinen** `silence_map_probe`-Fail; der `format`-Diff @HEAD
  nennt `silence_map_probe` nicht. Das forschung-eigene ist grün; die rote CI ist fremd.

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

- CI-Log-Diagnose (`ci-check` `35451506666`/`35448988400`, `harvest-dispatch`
  `35451187565`) an `general` (flash) delegiert: vollständige Messung (Jobs,
  exakte Fehlerzeilen, Betroffenheit forschung-eigener Dateien), kein Doppellauf —
  die Routine-Agent-Klasse ist geschlossen (flash-Sieger, 2026-09-16).
- Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-19-forschung-folge91.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge90.md` (Move)
- `docs/zustand/external-state.md` (CI-Zeile, eigener Hunk)
- `docs/handover/post.md` (eigene `An entscheid`/`An bau`-Zeilen)
- **Fremd (nicht angefasst):** `src/mathematikerin/te.rs`,
  `src/archivar/{fugin,zarr,omega,tests}.rs`, `tools/utils/src/bin/archive_search/*`,
  die drei `handover-2026-09-16-*`-Renames nach `archiv/`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
