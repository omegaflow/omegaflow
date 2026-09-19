<!--
  title: Handover — Forschung-Folge 87 (Stand 2026-09-19)
  session: Forschung-Folge 87
  class: handover
  date: 2026-09-19
  sha256: 784ad4f54d630a419efc8655a08d827192f383c936c29df005f75bcae0e6cb92
  status: live
-->
# Handover — Forschung-Folge 87 (2026-09-19)

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

- **HEAD** `4e43856e` == `origin/main`; `git_safety` Snapshot `refs/safety/1789797494`.
- **Postfach** — `post.md` trug zwei `An bau`-Zeilen (von bau-folge87 08:13
  eingefaltet); kein `An forschung`. `state/mail/mail_ledger.φ` kein neuer Eingang.
- **CI** — `te-gate` `35425288111` @`e9e9ef63` **in_progress**; `ci-check`
  `35389926289` @`e60aea10` **failure** (8 rot / 1374 grün); `measure-gates`
  `35389565681` @`e60aea10` **cancelled** → re-dispatch `35426091713`.

## te.rs/Gate-Atom gehört der bau-Linie — kein Forschung-Atom (geklärt)

- **Befund:** der uncommittete Fix an `src/mathematikerin/te.rs` (pcmci-Konditions-
  set dedupliziert + um getesteten Treiber/Ziel gefiltert) + Gate-Fixture
  `pcmci_cond_endpoint_series` + Unit-Test gehört **bau-folge87**
  (`docs/handover/handover-2026-09-19-bau-folge87.md:65-69,145-147`); bau hat
  `0.6→1.2` und `RESTRICTED_PERMUTATION_BINS 32→64` als fabrication-risk
  **verworfen** (Rat pro/max, Z. 73-90).
- **Unabhängige research-max-Messung** (pro/max, read-only): `1.2` = Tuning
  (löscht den gemessenen FN `0.1994 < 0.2057` statt die Frontier zu messen);
  `64` = n-Floor ungeschützt (~2.3 Samples/Bin bei n=150); pcmci-Fix korrekt
  (MCI konditioniert nicht auf den Treiber); Fixture korrekt (0 False-Positives
  im Baum). Deckt sich mit dem bau-Rat-Verdikt.
- **Folge:** Forschung committet das Atom **nicht**; ein versehentlicher Eingriff
  (n_surr 10→200 im flare-Test) wurde zurückgesetzt. Der frühere forschung-Punkt
  „TE-Gate-Fix" entfällt an bau. (Schritt: keine Aktion — bau committet.)

## Measure-Gates silence-map — re-dispatch (eigener Punkt)

- `35389565681` @`e60aea10` **cancelled** (nicht rot; Fix `expectation = ΣK·V`
  in `e60aea10`). Re-dispatch **`35426091713`** (2026-09-19). Trigger
  Run-Abschluss. (Schritt: `ci_manage view 35426091713` + Artefakt
  `measure-gates`.) Grün ⇒ 6 Gates + reale CDN-Messung.

## TE-Gate n=1000 Conditional-Null — `wartend` (Kalibrierung jetzt bau-eigen)

- `35425288111` @`e9e9ef63` in_progress — ohne bau's pcmci-Fix/K-Frage. Bau-folge87
  registriert die ehrliche Kalibrierkurve (per-Lag-Konditionsdesign; Kopplungsscan
  `te_c/thr_c` über {0.3,0.6,0.9,1.2,2.0}; FPR-vs-Bin über {16,32,64,128} ×
  a ∈ {0,0.5,0.9}, trials ≥ 2000) in `te-gate.yml`. (Schritt: te-gate.yml-Verdikt
  lesen; grün ⇒ `211A→193A`-Conditional-Check frei, `docs/paper/solar-seconds-matrix.md:37,46`.)

## vC-Verteilung (Post Entscheid-Folge 52) — `operator-gebunden`

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
  nach grünem `35426091713`.)

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
  read-only; lieferte das unabhängige Verdikt, das bau's Rat-Verdikt bestätigt
  (1.2 Tuning, 64 fabrication-risk, pcmci-Fix korrekt). Kein Doppellauf. Burn:
  `session_burn`.
- Routine-Klasse geschlossen (2026-09-16): kein flash-Spiegel nötig.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-19-forschung-folge87.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge86.md` (Move)
- **Fremd (nicht anfassen):** `src/mathematikerin/te.rs`, `src/gate/commit_gate.rs`,
  `src/gate/commit_gate_vocab.json` (bau-folge87, aktiv), `phi/sources.φ`,
  `src/archivar/{hdf5,tests,voyager_odr,extract,parse}.rs`,
  `docs/handover/handover-2026-09-19-{bau,ernte}-folge*.md`, `post.md`,
  `phi/{harvest,pipeline/ledger,blocked_sources}.φ`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
