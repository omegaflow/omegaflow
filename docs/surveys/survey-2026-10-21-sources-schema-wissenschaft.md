<!--
  title: Wissenschaftliche Fundierung des sources-Schemas (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 857aa6b37bb4a38d3793ec35669f3801857845855ac2307320504f54a739a137
  status: draft
  see-also: docs/surveys/survey-2026-10-21-sources-schema-archaeologie.md docs/specs/livefeed-gate.md docs/specs/iau-2000-eop.md
-->
# Wissenschaftliche Fundierung des sources-Schemas

**Frage:** Was muss ein Quellen-Schema tragen, damit jede Messung eine
Raumzeit-Adresse und ein Bezugssystem hat — und welche dieser Dinge fehlen
heute in `phi/sources.φ`?

## 0. Die Adresse einer Messung

Eine Messung ist erst eine Messung, wenn sie sagen kann: **wer/was** wurde
**wo** **wann** in **welchem Bezugssystem** mit **welcher Einheit** gemessen.
Das ist die operationale Form von „A = A“ und „ICRS & TDB“:

| Bestandteil | Frage | Standard |
|---|---|---|
| Origin | Wo liegt der Ursprung? | Baryzentrum (BCRS) / Geozentrum / Körperzentrum |
| Achsen | Wie sind die Achsen orientiert? | ICRS-Achsen (IAU 2000 B1.3) |
| Frame | Körperfest oder inertial? | ICRS/BCRS, ITRF/WGS84, WGCCRE, SPICE-Frames |
| Epoche | Zu welcher Zeit? | TDB; UTC/TT über Leap Seconds |
| Einheit | In welcher Skala? | SI über UDUNITS; `alt` in Metern |
| Coverage | Punkt, Bahn, Fläche, global? | STC/ObsCore/CF `bounds`/`grid_mapping` |
| Provenienz | Woher, wann geholt? | PROV/FORCE11, SHA-256, `origin`/`fetched_at` |

Ohne die ersten fünf ist die Zahl ein Datum ohne Ort; ohne Coverage ist sie
ein Punkt, der eine Fläche behauptet; ohne Provenienz ist sie nicht
nachprüfbar.

## 1. Die sechs Messklassen

Die 2.861 Blöcke sind nicht eine Klasse. Wissenschaftlich sind es sechs:

1. **Punkt/Station** — fester Ort, `lat`/`lon`/`height`, Datum (WGS84/ITRF),
   UTC-Epoche → body-fixed → ICRS(TDB). Beispiele: INTERMAGNET, NDBC, METAR.
2. **Körper/Body** — Baryzentrum oder Körperzentrum, Zustand aus Ephemeride
   (Position/Geschwindigkeit) zur TDB-Epoche. Beispiele: `at sun`, `at earth`,
   SPICE-Kernel.
3. **Katalog/Celestial** — ICRS `ra`/`dec`, Parallaxe, Eigenbewegung,
   `catalog_epoch`; die volle Adresse ist bereits im `cmap`-Pfad.
4. **Track/Profile** — Position pro Datensatz (`lat`/`lon`/`alt`/`epoch`),
   bewegte Plattform (Drifter, Argo, Flugzeug, Satellit).
5. **Coverage/Global** — Fläche, BBox, Polygon, Swath, Gitterzelle, globales
   Mittel; kein einzelner Punkt.
6. **Query** — die Position ist ein Aufrufparameter (`{lat}`, `{lon}`,
   `{bbox}`, `{station}`); die Antwort trägt die Zeilenposition.

Die aktuelle Spec kennt (1) über `on`, (2) über `at`, (3) über `cmap`,
(4) über `map`/`rows` + `lat`/`lon`/`epoch`; (5) und (6) haben **keine**
Anker-Art. Genau dort liegen die 290 `on earth 0 0 0`.

## 2. Standards, die die Spec nicht oder nur halb nutzt

- **ICRS/BCRS** — IAU 2000 Resolution B1.3; IERS Conventions 2010; SOFA.
  `at <body>` ist ein Körper-Frame, keine Position; die Position des Körpers
  kommt aus einer Ephemeride zur Epoche.
- **TDB/TT/UTC** — TDB ist die baryzentrische Zeit; UTC→TT→TDB über
  Leap Seconds. Die Register-Spec hat `epoch` pro Zeile (ISO/unix), aber
  keinen Time-Scale-Tag; das `livefeed-gate` nutzt `ISO-UTC` und TDB
  (`livefeed-gate.md:97-110`), das Register nicht.
- **Erd-Frame** — WGS84/ITRF und GMST/ERA für die Erdrotation; das
  `livefeed-gate` kann geo→ICRS geozentrisch (WGS84 + GMST; Präzession/
  Nutation pending, `:107-110`). `on earth` nutzt WGCCRE — für den Erdball
  ist das nicht dasselbe wie WGS84/ITRF.
- **Planeten-Frame** — IAU WGCCRE für Körperfest-Frames; SPICE `PCK`/`FK`/
  `CK`/`SPK` für Rotation und Zustand (`docs/reference/naif-daf-required-reading.md`).
- **FITS/WCS** — `RADESYS` (ICRS/FK5), `EQUINOX`, `MJD-OBS`, `CTYPE`;
  die Sprache für Katalogpositionen und Epochen.
- **CF/COARDS** — `coordinates`, `grid_mapping`, `cell_methods`, `bounds`,
  `time`; die Sprache für Gitter/Coverage und Zellmittel.
- **IVOA ObsCore** — `s_ra`, `s_dec`, `s_fov`, `t_min`, `t_max`; die
  Sprache für Himmels-Coverage.
- **HAPI** — `parameters`, `units`, `fill`, `bins`; die Sprache für
  Zeitreihen, aber ohne Frame.
- **PROV/FORCE11** — Entity/Activity/Agent, `source`, `fetched_at`,
  SHA-256; die Sprache für Provenienz.
- **UDUNITS-2** — Einheiten-Grammatik und Konversion; die Sprache für
  `field`-Einheiten und `alt`.

## 3. Mapping auf `sources.φ`

| Schema-Teil | Was es kann | Was fehlt |
|---|---|---|
| `at <body>` | Körper-Frame, ICRS-Achsen | Körperzustand/Epoche; `at` ist kein Ort |
| `on <body> lat lon alt` | body-fixed Punkt, WGCCRE, `alt` Meter | Datum (WGS84/ITRF), UTC-Epoche, ICRS-Transformation |
| `map`/`rows` + `lat`/`lon`/`alt`/`epoch` | Zeilenposition | Pflicht? Der Validator prüft sie nicht |
| `cmap` + `ra`/`dec`/`plx`/`pm`/`dist`/`catalog_epoch` | volle ICRS-Adresse | Vollständigkeit/Unsicherheit |
| `range` | 1-D-Serienachse | keine Coverage (BBox/Polygon/Swath) |
| `field` | Kraft/Einheit/τ/Absorption/Advection | Unsicherheit; Einheiten-Check beim Laden |
| `terms`/`origin`/`compiler`/`sha256` | Lizenz/Provenienz | nicht in der v2-Tabelle; `origin`-Prosa verboten |
| `zustand`/`lebenszyklus`/`zugang` | v3-Achsen (separat) | nicht im `sources`-Block |

**Der harte wissenschaftliche Befund:** `on earth 0 0 0` ist als
**Frame-Anker** (Presence-Gate, `{lat}`-Template) zulässig, aber als
**Messposition** falsch. Für globale Mittel ist es ein erfundener Punkt;
für Queries ist die Position im Aufruf; für Tracks ist sie pro Zeile. Die
Spec sagt das selbst (`sources-v2-spec.md:371-380`), aber der Validator
prüft den Anker, nicht die Zeile (`elev_zero`, soft).

**Der zweite Befund:** `on` ist WGCCRE, nicht WGS84/ITRF; für Erdstationen
fehlt das Datum und die UTC-Epoche. Das `livefeed-gate` hat den genaueren
Weg (WGS84 + GMST, `:107-110`), aber das Register nutzt ihn nicht.

**Der dritte Befund:** `at <body>` ist ein Frame, kein Ort. Die Position
des Körpers ist eine Ephemeride zur TDB-Epoche; `at` allein sagt nicht,
welcher Zustand gemeint ist.

## 4. Anforderungen an ein v3-Schema (Vorschlag, keine Entscheidung)

1. **Anker-Art Pflicht:** `body` | `point` | `catalog` | `track`/`profile` |
   `coverage` | `query`. Jede Art hat ihre eigenen Pflichtfelder.
2. **Position pro manifestierender Zeile:** `lat`/`lon`/`alt`/`epoch` oder
   `ra`/`dec`/`plx`/`dist`/`catalog_epoch`; globale Mittel tragen
   `coverage global`, Queries `query`, Tracks `track`.
3. **Frame + Zeit:** explizit, wo mehrdeutig: `frame icrs|body-fixed|wgs84|
   itrf`, `epoch`, `time_scale tdb|utc`; Defaults aus der Anker-Art.
4. **Einheiten:** SI am Anker; `alt`/`elev` in Metern; Tiefe negativ;
   Konversion beim Laden, nicht im Text.
5. **Coverage:** `bbox`, `polygon`, `swath`, `global`, `grid`/`bounds`;
   `range` bleibt die 1-D-Achse, nicht die Fläche.
6. **Provenienz:** `origin` als URL (keine Prosa), `sha256`, `fetched_at`.
7. **Achsen:** `terms`/`zugang`/`zustand`/`lebenszyklus` entweder im Block
   oder in einem klar referenzierten Sidecar; eine Autorität.
8. **Validator:** hard für Anker-Art, Zeilenposition/Coverage, Einheit,
   `terms`; soft für Präzision/Plausibilität.

## 5. Was die Wissenschaft nicht entscheidet

- Die **Namen** der Anker-Arten und Felder (Rat).
- Ob das Schema **eine Datei** oder **Block + Sidecar** ist (Rat).
- Die **Migrationsreihenfolge** und die 290 Klassen (Operator/Rat).
- Die **Messzustands-Achsen** — die sind mit v3 bereits entschieden; sie
  werden hier nur referenziert, nicht neu geöffnet.

## 6. Prüfstand der Primärquellen (2026-10-21)

Die erste Fassung war eine Synthese aus dem Baum und Trainingswissen. Dieser
Stand ist gemessen:

| Quelle | geöffnet | Befund |
|---|---|---|
| CF Conventions 1.11 | **ja** | §7.3 `cell_methods`; §9.5 `coordinates`; §7.1 `bounds`; §5.6 `grid_mapping` |
| IVOA ObsCore 1.1 (2017-05-09) | **ja** | `s_ra`, `s_dec`, `s_fov`, `t_min`, `t_max` |
| IERS Conventions 2010 | nein | nur Navigation; Chapter 1/TN36 nicht extrahiert |
| IAU 2000 B1.3 | nein | IAU-URL 404; nicht geöffnet |
| FITS WCS Paper II | nein | nur Landing-Page; `RADESYS`/`EQUINOX` nicht primär geprüft |
| HAPI 3.0 | nein | nicht geöffnet |
| W3C PROV / UDUNITS-2 | nein | nicht geöffnet |

**Folge:** Die Aussagen zu ICRS/BCRS, TDB, WGS84/ITRF, WGCCRE und FITS/WCS
sind in dieser Datei noch **nicht** primär belegt; sie stehen als
Prüfauftrag. Der Rat wird erst nach dem vollständigen Prüfstand gefragt.

## 7. Referenzen

- IAU 2000 Resolution B1.3 (ICRS/BCRS); IERS Conventions 2010; SOFA.
- IAU 2006 (TDB/TT); IERS Leap Seconds; SPICE LSK.
- IAU WGCCRE (planetary cartographic frames); SPICE PCK/FK/CK/SPK.
- FITS WCS Paper II (Greisen et al. 2006); `RADESYS`/`EQUINOX`/`MJD-OBS`.
- CF Conventions 1.11 (`coordinates`, `grid_mapping`, `cell_methods`,
  `bounds`); IVOA ObsCore 1.1 (`s_ra`, `s_dec`, `s_fov`, `t_min`, `t_max`).
- HAPI 3.0 (`parameters`, `units`, `fill`, `bins`); UDUNITS-2.
- W3C PROV / FORCE11 (Provenienz).
- Im Haus: `docs/specs/livefeed-gate.md:97-110`,
  `docs/specs/iau-2000-eop.md`, `docs/specs/binary-protocol.md` (26×f64),
  `docs/reference/naif-daf-required-reading.md`.
