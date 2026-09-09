<!--
  title: Handover — de441-mars-Rekompiilat: Struktur gemessen, Rat (a), CI-Lücken geschlossen; Dispatch + Re-Verifikation stehen aus
  class: handover
  date: 2026-09-09
  sha256: bb434c0e13698aa1c97ead27ab3d5218069b752382cb314c6c01cec293d87689
  status: archived
  see-also: docs/handover/archiv/handover-2026-09-09-membran-sonde.md
-->
# Handover — de441-mars-Rekompiilat

Übergabe für die nächste Sitzung. Die Membran-Sonde-Linie wurde weitergemessen:
der Struktur-Unterschied ist gemessen, der Rat hat entschieden (Verdikt a), die
CI-Lücken sind geschlossen. Der Dispatch und die Re-Verifikation stehen aus.

## 1. Struktur-Befund (gemessen, ephemeris_structure_probe)

Die ssd-Bins tragen zwei Gestalten:

- **Alt** (183 150 744 B — CDN: sun, mars, mercury, venus, moon, jupiter,
  saturn, uranus, pluto, neptune): 346 876 Granulen + ebenso viele Matrizen,
  Spanne jd [-3100015.5 .. 8000016.5] = 30 390 Jahre, Kadenz 32,0 d; per-Körper
  PCK-Props (α0/δ0/w0/Radius/GM), aber j2/j4 absent, ω_g absent. Vor-Fix-Matrizen.
- **Neu** (19 018 776 B — CDN: earth allein): 36 020 Granulen + Matrizen,
  Spanne jd [2305328.5 .. 2816848.5] = 1400 Jahre (AD 1600–3000), Kadenz-Median
  6,0 d; volle PCK-Props mit j2/j4, ω_g absent. Fix-Matrizen.

Der kanonische Produzent (kernel-flatten-Körperjob, `ephemeris_compiler
--systems planets,…`, GRANULE_DAYS 32,0, Flatten-Policy K01 wählt das volle
de441.bsp, kein Spannen-Deckel) erzeugt die 32-d/30-390-Jahre-Gestalt. Die
19-MB-earth-Gestalt stammt nicht aus diesem Produzenten.

## 2. Rat (Verdikt a)

ssd-de441-Bins = volle 32-d-CI-Flatten-Gestalt (30 390 Jahre, metergenau,
~183 MB je Körper) mit Matrix-Fix (d41a945) + j2/j4 + ω_g (solar_omega_g.φ
sun 1277 10). Mars-Rekompilat CI-only zu ~183 MB; die ganze Linie wird
regeneriert; der de441-cdn-watch (≥183 MB, S14) läuft grün.

## 3. Befund: die 19-MB-earth, Produzent unverified (vor dem Overwrite benannt)

`ephemeris_earth.bin` (19 MB, 1400 Jahre, 6 d) — Produzent `unverified`
(Kandidat: partieller de441.bsp-Download unter der Size-Gate `landed > 0`,
oder fremder lokaler Compile; kein Kandidat fabriziert). Die Matrix trägt den
Fix, j2/j4 liegen vor — aber die Form ist nicht die CI-Flatten-Form.
Transient: die nächste CI-Flatten überschreibt sie mit der 183-MB-Form. Die
Producer-Frage ist `pending` (Register-Pflicht), kein Datenwert.

## 4. CI-Lücken geschlossen

`59bf7c4` — `.gitignore`-Whitelist + Commit der zwei Katalog-Eingaben
(`phi/pipeline/catalog/asteroid_gm_sb441.φ`, `phi/pipeline/catalog/
solar_omega_g.φ`); die kernel-flatten-Körperjob-Kette hat sie nun auf dem
CI-Runner. `--omega-g` liest nicht mehr void; der TNO-Split findet sein
GM-Katalog.

## 5. Ausstehend (die nächste Sitzung trägt es ab)

1. **Dispatch** (Operator-Wort): `gh workflow run kernel-flatten` auf
   origin/main nach Push von 59bf7c4 (und der Befund-Benennung §3). Der
   bodies-Job regeneriert die ganze ssd-Linie (earth, mars, sun, …) in der
   183-MB-Form mit Fix + j2/j4 + ω_g.
2. **Re-Verifikation** (nach grünem Lauf): local auffrischen
   (`data/ssd.jpl.nasa.gov/`, Membran-Cache löschen), dann
   `OMEGAFLOW_HIDDEN=1 cargo run -p omegaflow-measure --bin orientation_probe`
   — de441-mars-Anker-Δ soll in die ~0-km-Klasse fallen (vorher 6 045,3 km);
   `cargo test -p omegaflow-measure --bin orientation_probe` (Lehrbuch-Gate,
   silent, kein Fenster). Die structure-probe bestätigt die 183-MB-Gestalt für
   earth/mars.
3. **Orientierungs-Sonde Funktions-Check**: der hidden run ist der Check; kein
   visible run.

## 6. Register-Reste (übernommen, unverändert offen)

witness presence bleibt reserviert (Consent-Wurzel Art (c) recorded, nicht
gebaut; ein maschinell gemessener Bio-Ton ist eine akustische Serie, nie
presence). feature-gate `gpu` = eigenes Atom, `pending`. Membran-Reste:
M02 ESP32-Firmware no_std; M03 Audio-Gain ohne tanh; M04 Navigation
(Nebra-Kalibrierung); M07 ⌘K-Palette; M05/M06 Station-Sensoren als SI-4-Token;
Kamera ~19k Pixel-Quellen als WS-Traffic-Hotspot; OPeNDAP-Integration;
advective per-Quelle.

## 7. Zustand des Arbeitsbaums

Fremde parallele Sitzung aktiv: `baab781` (committet meine
`ephemeris_structure_probe.rs` als „ownerless stray"), `4e2df4a`; uncommittet
`src/archivar/ak135.rs` + `tools/measure/src/depthphase.rs` — unangetastet.
`59bf7c4` ist mein Commit dieser Sitzung. Kein Stray von mir; die Probe ist
committet.
