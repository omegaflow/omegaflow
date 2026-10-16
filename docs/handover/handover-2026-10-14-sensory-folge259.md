<!--
  title: Handover — Sensory-Folge 259 (2026-10-14)
  session: Sensory-Folge 259
  class: handover
  date: 2026-10-14
  sha256: 6f5daea3f54a0563114cabed890c1603ec53ede0211c439cd9f44ed4765aa9ac
  status: live
-->
# Handover — Sensory-Folge 259 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes trägt git. Keine Rangfolge; jeder Punkt
trägt **Trigger / Lage / Blockade / Braucht**, der Akteur steht in `Bindung`.

## Haus — Hardware & Fundstellen
- **Hardware/Sensoren:** Polar H10/HRM-Dual — Live-BLE HR `NotSupported`, FIT nn=0; Hardware
  Operator-gebunden („erst bei Förderung") → Future-Queue. [redacted]/DEMETER — Daten/Datei
  verlassen das Gerät nie.
- **BLE-HR-Reader (PENDING, `external-state.md:41`):** gebaut (`src/archivar/ble.rs`, std-only
  BlueZ-D-Bus, `0x2A37`-Decode, 14 Fixture-Tests); braucht den Live-Lauf am gekoppelten Gerät
  (`OMEGAFLOW_BLE_HR=… OMEGAFLOW_HIDDEN=1 cargo run`) — **Hardware, nur nach Operator-Wort**.
- **Fundstellen:** `state/zustand/standing-pass.md` · `state/zustand/external-state.md` ·
  `state/zustand/wartend.φ` · `state/zustand/ereignisse.φ` ·
  `state/operator-gespraeche/2026-10-*-sensory.md` · `state/sensory/archive-search-preset.txt`.
  Das private Register liegt unter `~/projects/omegaflow-personal`, Laufzeit unter
  `~/.local/state/omegaflow` (nie `sgrep` ohne `--all` — Falsch-Negative). Konsens-Pflicht:
  die Maschine fragt vor dem Aufzeichnen.

## Offen (aufgeschlüsselt)
Nur `Bindung: eigen` — jeder Punkt ist Sensorys eigene Arbeit.

### ox64-m2c — PINE64, Carrier China Post LZ473049629CN
- **Status:** wartend · **Bindung:** eigen
- **Trigger:** Zustellung `LZ473049629CN` — die physische Zustellung ist das Ereignis.
- **Lage:** (gemessen 2026-10-14 F259, live über die DHL-Sendungsverfolgung) „Im Zollabfertigungsprozess",
  Zielland Deutschland; jüngstes Event Mo, 05.10.2026, 15:25 (Einreise ins Zielland 15:23).
- **Blockade:** Zollabfertigung → Zustellung; die physische Zustellung ist der Auslöser.
- **Braucht:** Zustellung abwarten; Status über die Browser-Bridge
  `archive_search --playwright --headed "…trackingno=LZ473049629CN"` (`state/zustand/wartend.φ`,
  Key `ox64-m2c`).

### europa-clipper — ESA, Flyby
- **Status:** termin · **Bindung:** eigen
- **Trigger:** Flyby 2026-12-03.
- **Lage:** (gemessen 2026-10-11 F254 am Baum + CDN) Asset `ephemeris_europa_clipper.bin` 200 / 103 120 B,
  Siegel `3b03ea69…`; registriert `phi/sources.φ` Key `at europa_clipper`; NOW < Termin.
- **Blockade:** Termin.
- **Braucht:** nach dem Flyby `gh workflow run kernel-flatten.yml` (`horizons_compiler --flyby
  --ci-mode`) + `archive_search --sniff "…/ephemeris_europa_clipper.bin"` gegen `3b03ea69…`.

## Träger (Orphan-Faltung, gemessen 2026-10-16)
Je Zeile ein zuletzt trägerloses Dokument `Pfad` (offene Marker). Die Marker sind **echte
Messgrenzen**, keine stale Reste — sie werden getragen, nie geglättet (0 honored).
- auftrag: `docs/auftrag/auftrag-2026-10-11-wartend-aufloesung.md` (3) · `docs/auftrag/auftrag-flyby2-kette.md` (3) · `docs/auftrag/auftrag-gic-einreichung.md` (1) · `docs/auftrag/auftrag-universelles-vlies.md` (2)
- blatt: `docs/blatt/blatt-gic-breitenband-familien.md` (2) · `docs/blatt/blatt-solar-seconds-matrix.md` (1) · `docs/blatt/blatt-te-externer-steuerparameter.md` (1) · `docs/blatt/blatt-thuan-fragesteller.md` (2) · `docs/blatt/fruehwarnsystem-praeregistrierung.md` (1)
- concepts: `docs/concepts/das-eine-instrument.md` (1) · `docs/concepts/die-akteure-im-boden-und-wasser.md` (3) · `docs/concepts/ein-blatt-papier.md` (2) · `docs/concepts/exzellenz-konzept.md` (2) · `docs/concepts/fuenf-funken-anomalie-suche.md` (1) · `docs/concepts/glossar.md` (4) · `docs/concepts/kanal-ontologie-komplettbau.md` (1) · `docs/concepts/kybernetische-astrophysik.md` (1) · `docs/concepts/remove-bias.md` (8) · `docs/concepts/the-seven-spheres.md` (1)
- paper: `docs/paper/causal-arrow-preregistration.md` (1) · `docs/paper/corona-heating-ladder.md` (1) · `docs/paper/depth-phase-echo-fleet.md` (3) · `docs/paper/flyby-path-2-addendum-2026-09-29.md` (27) · `docs/paper/flyby-path-2-falsification-metric-addendum.md` (3) · `docs/paper/galileo-rotor-spin-era-floor.md` (1) · `docs/paper/jwst-disequilibrium-survey.md` (2) · `docs/paper/probe-front-dark-matter.md` (3) · `docs/paper/solar-seconds-matrix.md` (1) · `docs/paper/sturzflut-tibet-pfeil.md` (5) · `docs/paper/terminologie-der-gegenstroemung.md` (1) · `docs/paper/tonga-lamb-crosscheck.md` (2)
- plans: `docs/plans/ref-mariner-faden-index.md` (3)
- surveys: `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (1) · `docs/surveys/survey-2026-09-06-codestruktur.md` (2) · `docs/surveys/survey-2026-09-07-weberin-thread-matrix.md` (10) · `docs/surveys/survey-2026-09-13-weberin-quellen.md` (10) · `docs/surveys/survey-2026-09-13-weberin-quellen-folge.md` (1) · `docs/surveys/survey-2026-09-13-weberin-quellen-treffer.md` (3) · `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` (2) · `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (7) · `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` (2) · `docs/surveys/survey-2026-09-14-weberin-quellen-rerun.md` (22) · `docs/surveys/survey-2026-09-16-sonden-flotte.md` (4) · `docs/surveys/survey-2026-09-17-sonden-request-only.md` (6) · `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (4) · `docs/surveys/survey-2026-09-20-browser-anbindung.md` (1) · `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (6) · `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (1) · `docs/surveys/survey-2026-10-02-weberin-zweite-linie.md` (3) · `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (60) · `docs/surveys/survey-2026-10-04-exposom-matrix.md` (43) · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (2) · `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` (2) · `docs/surveys/survey-2026-10-09-domaenen.md` (4) · `docs/surveys/survey-2026-10-09-redistribution-alternativen.md` (6) · `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md` (3) · `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` (52) · `docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` (5) · `docs/surveys/survey-2026-10-13-grosse-raetsel-und-maschine.md` (1) · `docs/surveys/survey-2026-10-13-raetsel-master-matrix.md` (3) · `docs/surveys/survey-2026-10-15-die-fuenf-archaeologie.md` (4) · `docs/surveys/survey-raetsel-bestand.md` (6)

## Burn: open 0.0000 · close 0.0400 · cap 0.15 · Grund: F259 — Line-Session (deepseek-flash), Ein-Pass;
Register-Pass `--fired sensory` = 0, `--stale sensory --persist 3` = 0, `--addressed sensory` = 0,
`register_lookup --orphans --fail` = 0, `--orphan-docs` = 0 (62 Waisen kompakt gefaltet); die
Übergabe auf Nur-Offenes gekürzt (Wort-Register und Träger-Prosa gefallen — git trägt sie).

## Wer nicht senden darf
`smail --send` und jeder Formular-Absand bleiben die Hand des Operators; die NTRS Document-Inquiry
ist Operator-Hand. Netz-Lesearbeit (Route/Status) ist autonom.

## Abschluss
Eigene Pfade dieses Atoms: `docs/handover/handover-2026-10-14-sensory-folge259.md` (diese Übergabe).
`state/` lebt nicht mehr im öffentlichen Baum (`.gitignore`; Umzug `4f8c56f6a`). `smail --send` und
jeder dritt-wirksame Akt bleiben die Operator-Hand.
