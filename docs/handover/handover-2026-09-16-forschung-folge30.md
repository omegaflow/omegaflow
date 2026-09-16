<!--
  title: Handover — Forschung-Folge 30 (2026-09-16)
  session: Forschung-Folge 30
  class: handover
  date: 2026-09-16
  sha256: 481ff6f4b1c54fef8c66f7a6f7b1686342a40b2c3cddc1943866607836d75659
  status: live
-->
# Handover — Forschung-Folge 30 (2026-09-16)

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

## TE / Statistik

- **GIC causal driver** — PCMCI auf dem Minuten-Sturm-Ensemble, KDE-h,
  Rückkanal-Härtung. (Schritt: `docs/paper/gic-causal-driver.md`.)
- **Blatt 2/3 — Rest offen.** fam/max-T-Bound (Blatt 2), retro OMNI2-PCMCI-Zeile
  und KDE-h/`laic_probe` 0–72-h brauchen einen lokalen laic-Harvest (CI-Skala).
  (Schritt: `docs/concepts/blatt-papier-resultat.md`, `multi_force_te_probe`.)
- **Der Grat / AIA-Je-Zellen-Schwellen — Fix steht, Lauf offen.** Der
  `aia-ladder-probe`-Workflow setzt `--goes-dir` (GOES-15-Trigger, die
  §4.5-Matrix-Konfiguration), der Probe druckt die `thr`-Zeile (mean + 2σ je
  Paar×Lag). (Schritt: `gh workflow run aia-ladder-probe.yml`, Artefakt lesen,
  Je-Zellen-Schwellen ins Blatt; Reproduktions-Gate: trifft der Lauf die
  §4.5-Zahlen (524/1019/281 Ereignisse, fam 1.71/1.96/1.75e-1) nicht, ist das
  ein eigener Befund — `docs/blatt/blatt-der-grat.md`,
  `docs/paper/corona-heating-ladder.md` §4.5.)
- **304-Å-Trigger-Lauf (run 35081213805) — nicht vergleichbar.** 2013
  (273 Ereignisse, fam 2.1928e-1) und 2015 (4365 Ereignisse, fam 1.9542e-1)
  gelandet, keine Sprosse über fam; 2014-Job lief noch. Der Lauf nutzt den
  304-Å-Trigger, nicht den GOES-Trigger der §4.5-Matrix. (Schritt: nach
  2014-Landung als Ein-Satz-Trigger-Kreuzprüfung in §4.5 falten oder gemessen
  descopen.)
- **Solar 211A→193A** — conditional stage-2. (Schritt:
  `corona_conditional_probe`-Lauf, `docs/paper/solar-seconds-matrix.md`.)

## Bande-Split / Sonden-ODF

- **7 planetare ODF — Lauf teilweise gelandet.** `planetary-odf-cdn`
  (run 35076268649): magellan ✓, mgs ✓, messenger ✓; odyssey ✗ (exit 1), mro ✗
  (canceled); rosetta + mars_express liefen noch. (Schritt: Lauf-Ende prüfen,
  je Asset Byte-Existenz, odyssey/mro-Fehler messen und neu dispatchen —
  `.github/workflows/planetary-odf-cdn.yml`.)
- **160-Hz-Amplitudenzensus — der Lauf misst ihn nicht.** `pioneer-link-correction`
  (run 35076271979) lieferte die Link-Korrekturkette (Slipped-cycle-Maske,
  Ramp k=1.4318e-8, Dynamik a_P 4.0e-6, residual-RMS 3.191e3 Hz), aber keine
  Band-Amplitude bei 44–58 mHz. Der Census braucht eine eigene Probe auf dem
  korrigierten Residuum.
  (Schritt: Band-Amplituden-Probe benennen/bauen → §1
  `docs/paper/twenty-second-band-ground-chain.md`.)
- **NOCC-Reduktionsvorschrift** — Moyer 2000 + dsn_redr + 810-005-202E liegen;
  offen der Ketten-Vergleich im retrace gegen Moyer §10/§13.
  (Schritt: `docs/paper/twenty-second-band-ground-chain.md`; fremde
  retrace-Session.)

## Positionslinien / Ephemeriden

- **Die Weberin — Schritte 3–9 offen.** Schritt 1 (MPC gegen SPK) und Schritt 2
  (Stations-Konvergenz, ABK-Riss) sind gebaut und gemessen; §8/§9 in
  `docs/concepts/die-weberin.md` tragen beide. Offen: die topozentrische Kopplung
  (3), die Tafel-Abbildung (4), die Survey-Footprints (5), die GW-/Neutrino-/
  CR-Routen (6), der CDN-Weg des Vlies-Assets (7), der Riss-Knoten (8), der
  geliehene Sinn (9).
- **Zweite unabhängige Linie je Klasse — Re-Run 2026-09-16 gemessen.** Die
  leer gebliebenen Zweitlinien wurden erneut durchsucht (Nachtrag in
  `docs/surveys/survey-2026-09-14-weberin-quellen-rerun.md`): Messenger-ODF
  request-only bestätigt (Bundle 2007–2015, kein 2005-Erd-Encounter), jnogrv
  200, JUNO `not-published` bestätigt. Neu offen: Rosetta RSI-Unterbaum
  (Erd-Swingby-ODFs 2005/2007/2009 prüfen), NRS-Hydrophon-Bucket live aber ohne
  Compiler, TNO-Occultation-Kandidaten (Quaoar `10.5281/zenodo.21185812`,
  TNBFits `10.5281/zenodo.10620251`), Occultation-DB-URL pending
  (`J.Phys.Conf.Ser.` 1365, 012024). (Schritt: `sfetch` Rosetta-RSI-Baum;
  NRS-Compiler benennen; Zenodo-Records ins Register.)
- **CDN-Planetenbins ~116 km SSB-Offset** — neu aus vollem `de441.bsp`.
  (Schritt: `docs/surveys/survey-geometric-ground-truth.md`.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE 28./29.09.,
  Clipper 03.12. (Schritt: `docs/paper/flyby-path-2-preregistration.md`.)
- **JWST disequilibrium** — O2/O3-Absenz gemessen; der Re-Sweep 2026-09-16
  (red-edge + saisonale Kanäle) bestätigt die Absenz beider Branchen: kein
  JWST-Spektrum mit detektierter Vegetations-red-edge, keine gemessene saisonale
  O2/O3/CH4-Variabilität — nur Detectability/Retrieval/Prospects (§6
  `docs/paper/jwst-disequilibrium-survey.md`). Offen: die Kanäle bleiben
  `pending`, bis eine Detektion samt Spektrum ins Register tritt.

## Extern gebunden (kein Datum)

- NSE/Haug — Antwort von B. Keimer offen. Re-Sweep 2026-09-16: kein Deposit
  (DataCite absent, Zenodo kein Haug/YBCO, `--leads` 0 neue Heimatorte); neu
  gemessene Routen: DTU-Orbit-Volltext-PDF (200, pdf) und Zenodo-Präzedenz
  RESEDA/BaZrO₃ `10.5281/zenodo.18306252` (FRM-II-Spin-Echo-Deposits existieren).
  (Schritt: Wartelisten-Zeile
  `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`; Rohdaten
  bleiben pending.)
- Voyager Cruise / JPL-DSN — die Anfrage hält (request-only).
- **Fünf Sonden-Anfragen** (Voyager closed-loop, Mariner 10, Viking 1/2, Cassini
  closed-loop, Juno Earth-Flyby) — gemessen 2026-09-16: das in Handover 29 als
  Schritt genannte Auftrag-Dokument `docs/auftrag/auftrag-sonden-rohdaten-anfrage.md`
  existiert im Baum **nicht**; der Operator-Route fehlt ihr Text. (Schritt:
  Auftrag-Dokument schreiben — Entscheid-Linie.)
- Toth/Turyshev/Markwardt-Mails — die Prüfliste steht; senden in der
  Entscheid-Linie geführt.

## Benchmark

- **Re-Sweep-Atom 2026-09-16 an `research-max` dispatcht** (JWST/NSE + Weberin-
  Zweitlinien) — hartes Mehrstufen-Atom, kein flash-Gegenlauf; die flash/pro-Klasse
  bleibt offen für den doppelten Lauf, wenn das Atom wiederkehrt.
- **Rat dispatcht (pro/max) für die AIA-Abschlussentscheidung** — kein
  flash-Gegenlauf, der Rat ist die Architektur-Stimme (keine Routine-Klasse).
  (Schritt: `gh` in ein flash-Profil heben, dann die CI-Artefakt-Extraktion als
  Benchmark-Klasse messen.)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
