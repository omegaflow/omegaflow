<!--
  title: Handover — Forschung-Folge 83 (Stand 2026-09-18)
  session: Forschung-Folge 83
  class: handover
  date: 2026-09-18
  sha256: e230396ae5ff073d231aef89c81763945a529e514a9f041fa17df41d1b6237ec
  status: live
-->
# Handover — Forschung-Folge 83 (2026-09-18)

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

- **HEAD** `02ff50c5` (folge82 notierte `faa6c173`; Baum vorgerückt, `origin/main`
  beim Push prüfen).
- **Postfach** — kein neuer Agenten-Eingang; letzter Ledger `1789737560` (eigene
  Antwort an Iess). Die `post.md`-Zeile BepiColombo MORE ist gefaltet (s. u.),
  `post.md` wieder leer.
- **CI** — Eintrag `docs/zustand/external-state.md` (CI-Status, gemessen
  2026-09-18): `ci-check` `35351207788` pending, `35348641363` in_progress; Serie
  cancelled/failure; `harvest` `35338103457` success. Fremder Ledger, nicht angefasst.

## Measure-Gates — gebaut, Dispatch/Verdikt ausstehend (härtester undatiert)

- Der fehlende CI-Job ist gebaut: `.github/workflows/measure-gates.yml`
  (2026-09-18, `grind-flash`; Vorlage `s2-weberin-probe.yml`/`te-gate.yml`,
  `issues: write` ergänzt). Er fährt die vier Betti-0-Gates
  (`cargo test -p omegaflow --lib -- betti0`), die sechs Silence-Map-Gates
  (`cargo test -p omegaflow-measure`) und die reale Silence-Map-Messung gegen das
  CDN (`silence_map_probe`), Artefakt `measure-gates`. Messung:
  `sgrep -l "betti0_probe|silence_map_probe" .github` → kein Workflow führte die
  Probes zuvor.
  Run `35351695849` @`52d0486d` dispatcht (2026-09-18, in_progress); Verdikt
  ausstehend. (Schritt: einmalig `ci_manage view 35351695849`; Artefakt
  `measure-gates` lesen.)
- **Betti-0 Schwellenleiter — `operator-gebunden`:** Die Zweierpotenz-Leiter hat
  keinen Literatur-Rückhalt; Standard sind exakte kritische Werte (Edelsbrunner
  DOI 10.1090/mbk/069) bzw. datengetriebene ε-Wahl (DBSCAN k-distance,
  DOI 10.1145/3068335), Persistenz-Signifikanz per Bootstrap (Fasy
  arXiv:1303.7117). (Schritt: Rat/Operator entscheidet Leiter vs. exakte
  kritische Werte.)
- **0-Kanon je stiller Zelle gegen `sources.φ`/`ledger.φ` — `pending`:** Die
  gebaute Probe prüft die stille Zelle nicht gegen die Register. (Schritt: eigenes
  Atom bauen oder mit Messung descopen.)

## Fabrication-Rest — `operator-gebunden`

- `docs/concepts/kybernetische-astrophysik.md:471–479` trägt die
  Erwartungsmodell-Sprache weiter. (Schritt: Konzept auf die Katalog-Form
  korrigieren oder als historische Stufe kennzeichnen.)

## Legacy-Rest — `operator-gebunden` (zurückgestellt)

- Rat-Rangfolge (2026-09-18), nur als Fortsetzung je eigener Messung:
  **Minkowski** (Delta-Probe: wie viele eigene Quellen spacelike);
  **Certainty** (vC-Asymmetrik messen, exp/tanh inverse; `omega.rs:1558–1560`);
  **Delay Spectrum** (Instrument-Definition zuerst, nur TE-basiert);
  **Synthetic Flight** (nur Präregistrierung/Falsifikationsmetrik);
  **Channel Apertures** (Atom-9-Permeabilitäts-Binding).

## MAG-Asset — `blockiert` (ernte)

- `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (853331 B, sha256
  `6f3724f7…`, valides ZIP, PDS4-`.tab` 4.83 MB). Post bei ernte. (Schritt:
  `tools/harvest/src/bin/bc_mpo_mag_compiler.rs` nach `voyager_odr_compiler.rs`,
  dann `sources.φ`-Eintrag + CI-Manifestation.)

## BepiColombo MORE — `termin` (aus `post.md` gefaltet)

- PI Luciano Iess (`mail_ledger` `1789729151`) und PSA/Bentley (`1789723653`):
  Cruise-Daten werden erst zur Wissenschaftsphase (~April 2027) freigegeben, kein
  Zwischenzugang; eigene Antwort `1789737560`. (Schritt: kein TAP-Abruf vorher;
  zur Wiedervorlage den MORE-URN authentifiziert abrufen.)

## TE-Gate n=1000 Conditional-Null — `wartend` (fremder Ledger)

- `35324015019` @`fb6b62b4` pending; `te-gate` `35314110831` @`7b67b10d`
  dispatcht. Trigger: Run-Abschluss. Nach grünem Verdikt wird der
  `211A→193A`-Conditional-Check frei (`docs/paper/solar-seconds-matrix.md:37,46`).
  (Schritt: `ci_manage view 35324015019` / `35314110831`; TE-/Null
  `src/mathematikerin/te.rs`.)

## NSE/Haug — `wartend`

- Keller-Antwort (17.09.), „in einigen Tagen"; TRISP/MLZ-Anfrage (`214`) läuft.
  Trigger = Dateieingang. (Schritt: bei Eingang `nse_haug_trisp`-Quelle + Compiler
  + `sources.φ`.)

## Paper / Präregistrierung — `termin`

- Flyby Path 2 datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem Datum.
  (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- `measure-gates.yml` von `grind-flash` gebaut (Routine-Klasse geschlossen, flash
  Sieger — kein Doppel-Lauf). Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `.github/workflows/measure-gates.yml`,
  `docs/handover/handover-2026-09-18-forschung-folge83.md`,
  `docs/handover/post.md` (nur die gelöschte `An forschung`-Zeile), archiviertes
  `docs/handover/archiv/handover-2026-09-18-forschung-folge82.md`.
- **Fremd (nicht anfassen):** `AGENTS.md`, `docs/zustand/external-state.md`,
  `src/archivar/hdf5.rs`, `src/gate/*`, `phi/{sources,harvest}.φ`,
  `.github/workflows/planetary-odf-cdn.yml`,
  `tools/measure/src/bin/weberin_verdicts_compiler.rs`, `weberin-verdicts-cdn.yml`,
  die `handover-2026-09-18-{bau,ernte,entscheid}-folge*.md` und die
  `handover-2026-09-16-*`-Renames/Deletes.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
