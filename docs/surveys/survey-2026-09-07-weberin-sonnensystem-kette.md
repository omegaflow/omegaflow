<!--
  title: Survey — Weberin: Bestands-Inventar der Körper-Kette im Sonnensystem (die zweite Linie je Klasse)
  class: survey
  date: 2026-09-07
  sha256: 0fa1fbdbde50f834b0e2c8587d1642421ac140be7e5e755b822b416cdc7b051e
  status: live
  see-also: docs/concepts/die-weberin.md docs/surveys/survey-2026-09-07-weberin-thread-matrix.md
-->

# Weberin — Bestands-Inventar der Körper-Kette im Sonnensystem

Antwort auf die Übergabe `handover-2026-09-07-weberin-sonnensystem-kette.md`,
Schritt 1: die Ephemeriden-Weltlinien listen und benennen, welcher Körper schon
eine zweite unabhängige Positions-Linie trägt (nicht nur Masse). Gemessen gegen
`phi/sources.φ` (die `format ephemeris_binary`- und `format orbit_bin`-Blöcke),
`src/weberin.rs` (`BODY_NUMBER`, der DASTCOM-Zwirn) und
`tools/harvest/src/bin/horizons_compiler.rs` (die Horizons-Ziel-ID je Körper).

## 1. Der Bestand — 73 Körper-Weltlinien

72 × `format ephemeris_binary` + 1 × `format orbit_bin` (Wind), alle `at <body>`,
ttl 86400. CDN `ssd.jpl.nasa.gov`; Wind auf `spdf.gsfc.nasa.gov`.

| Klasse | Anzahl | zweite unabhängige Positions-Linie |
|---|---|---|
| Sonne | 1 (sun) | keine — `Absent` |
| Planeten | 8 (mercury, venus, earth, mars, jupiter, saturn, uranus, neptune) | keine — `Absent` |
| Monde | 45 | keine — `Absent` |
| Kleinkörper | 9 | **8 ja** (DASTCOM-Kepler) — encke nicht |
| Raumsonden | 9 (ephemeris_binary) + 1 (wind, orbit_bin) | keine — `Absent` (Doppler `pending`) |

Die einzigen Körper mit **zwei** Linien heute: `ceres, vesta, apophis, bennu,
pluto, eris, makemake, haumea` (SPK × DASTCOM-Kepler).

### 1.1 Die volle Liste (die 72 + Wind)

- **Sonne:** sun.
- **Planeten (8):** mercury, venus, earth, mars, jupiter, saturn, uranus, neptune.
- **Monde (45):** moon (Erde); phobos, deimos (Mars); io, europa, ganymede,
  callisto, amalthea, thebe, metis, adrastea, himalia (Jupiter); mimas,
  enceladus, tethys, dione, rhea, titan, iapetus, hyperion, phoebe, janus,
  epimetheus, atlas, prometheus, pandora, helene (Saturn); ariel, umbriel,
  titania, oberon, miranda (Uranus); triton, nereid, proteus, larissa, despina,
  galatea, thalassa, naiad (Neptun); charon, hydra, nix, kerberos, styx (Pluto).
- **Kleinkörper (9):** ceres, vesta, apophis, bennu, encke, pluto, eris,
  makemake, haumea.
- **Raumsonden (9 ephemeris_binary + 1 orbit_bin):** iss, jwst, juno,
  new_horizons, parker_solar_probe, solar_orbiter, voyager1, voyager2, atlas_3i;
  wind (`orbit_bin`).

## 2. Die zweite Linie je Klasse

Die Weberin flicht erst bei zwei unabhängigen Linien derselben Weltlinie
(die-weberin §7). Der Befund je Klasse:

- **Kleinkörper:** die zweite Linie ist die DASTCOM-Kepler-Bahn (`AsteroidRec`,
  `state_at`). Sie trägt für die 8 genannten; für `encke` (Komet 2P/Encke,
  Horizons `90000031`) liegt sie im Kometen-Zweig (`CometRec`/`dcom5`), der noch
  nicht als `AsteroidRec` gelesen wird → `Absent{Dastcom}`, die Kometen-Linie
  `pending`. `pallas` (2) und `juno_asteroid` (3) tragen die DASTCOM-Linie, aber
  keinen SPK-Block → `Absent{Spk}`.
- **Planeten/Monde:** eine Linie (SPK). Die zweite (INPOP vs DE oder gemessene
  Astrometrie) ist `pending`.
- **Raumsonden:** eine Linie (SPK, Horizons-dynamisch; Wind `orbit_bin`). Die
  Doppler-/NAVIO-abgeleitete Bahn ist `pending`.
- **Breite TNO-Kette:** `mpcorb_extended`
  (`docs/handover/handover-2026-09-09-mechanische-reste.md`) ist die
  erste Kepler-Linie für Tausende; die zweite `pending`.

## 3. Gemessene Anomalien + Korrekturen

1. **Namens-Kollision `juno`.** `horizons_compiler.rs:19` bindet `-61` (die
   Raumsonde Juno) → `ephemeris_juno.bin`. `weberin.rs` band `("juno", 3)` (den
   Asteroiden Juno). Die Weberin zwirnte Sonde gegen Asteroid → ein Riss aus der
   Kollision, kein gemessener Widerspruch. Korrigiert: der Asteroid heißt
   `juno_asteroid` (`("juno_asteroid", 3)`), `body_number("juno")` → `None`.
   Test `spacecraft_juno_does_not_pair_with_asteroid_three`.
2. **`atlas_3i`** ist ein Horizons-dynamischer Körper (`2020-047A`,
   Raumsonden-Kategorie in `bodies_dynamic`), nicht der Saturnmond `atlas`. Die
   thread-matrix führte ihn in der Kleinkörper-Klasse; hier als Raumsonden-
   Körper geführt.
3. **`encke`** ist Komet 2P/Encke, kein nummerierter Asteroid → kein
   `AsteroidRec`; die Kometen-Zweitlinie `pending`.

## 4. Die Probe — register-driven, voller Körper-Satz

`weberin_body_verdict` liest den Körper-Satz jetzt aus `phi/sources.φ` (kein
hartcodiertes 10er-`BODY_NUMBER`-Register): alle `ephemeris_binary`- und
`orbit_bin`-Blöcke, je `at <body>`, via `cdn_parts` aus der `url` aufgelöst,
Wind über den `orbit_bin`-Pfad (`wind_orbit::parse_bin`). Die Weberin webt
union(eph, BODY_NUMBER) = 75 Körper-Linien (73 registrierte + pallas +
juno_asteroid) und beurteilt jede. Der Tally zählt `opened/registered` statt
`opened/BODY_NUMBER`.

## 5. Register

Die Lücke „zweite Linie je Körper-Klasse" bleibt benannt: Planeten/Monde
(INPOP-vs-DE/Astrometrie), Raumsonden (Doppler), breite TNO-Kette
(`mpcorb_extended`), Kometen (`dcom5`/`cometels`) — alle `pending`, nie
erfunden. Die Namens-Schuld der Raumsonde `juno` (sie hält den Namen des
Naturkörpers; `atlas_3i` ging den anderen Weg) ist eine Register-Schuld, nicht
glattgezogen.
