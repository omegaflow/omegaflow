<!--
  title: Handover: Asteroiden-Langbogen (de441/de442) + CI-Auftrag
  class: handover
  date: 2026-08-21
  sha256: b094de5fe5f248a73043d0120399ffae791f246fd31c96a93d7d8e50d6c86f13
  see-also: docs/SOURCE_PORT.md .github/workflows/kernel_flatten.yml
-->
# Handover: Asteroiden-Langbogen (de441/de442) + CI-Auftrag

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # fremde Arbeit benennen, nichts übernehmen
cargo check                      # muss 0/0 sein
grep -nE "ephemeris_(ceres|vesta|apophis)" phi/sources.φ
cargo run --release --bin ephemeris_compiler -- --summarize phi/sources_index.φ docs/reference/KERNEL_INDEX.md 2>&1 | head
```

Referenzen (stehend): `src/bin/ephemeris_compiler.rs` (der SPK-Kompilierer),
`src/bin/horizons_compiler.rs` (der 12-Monats-Kompilierer der Asteroiden —
die Lücke), `.github/workflows/kernel_flatten.yml` (der CI-Flatten),
`phi/sources_index.φ` (der Kernel-Index mit de441/de442), `phi/sources.φ`
(die `ephemeris_<body>.bin`-Blöcke).

## Auftrag

Den de441/de442-**Langbogen** der Asteroiden und Kometen als Chebyshev-Bodies
kompilieren — statt des heutigen Horizons-12-Monats-Fensters — und auf den
CDN heben. Das ist die Kraft **gravity** auf den Klein-Körpern.

## CI-Auftrag (konkret)

Der `bodies`-Job in `.github/workflows/kernel_flatten.yml` ruft heute:

```yaml
cargo run --release --bin ephemeris_compiler -- \
  --fetch-from phi/sources_index.φ \
  --systems planets,jupiter,saturn,mars,uranus,neptune,pluto \
  --dest kernels --ci-mode --omega-g … --index phi/sources_index.φ
```

… und danach `horizons_compiler --ci-mode` (dort entstehen die 12-Monats-Bins
der Asteroiden). Der CI-Auftrag: einen **`asteroids`-Systemmodus** im
`ephemeris_compiler` bauen (die `asteroids_de441/`- und `de441`/`de442`-SPK
aus `sources_index.φ` lesen, NAIF-IDs → `ephemeris_<name>.bin`), ihn in die
`--systems`-Liste aufnehmen (oder als eigenen Schritt) und `--ci-mode`
hochladen. Die Asteroiden-Bins überschreiben die bestehenden 12-Monats-Bins.
Der `horizons_compiler`-Schritt bleibt für die Sonden, nicht für die Asteroiden.

## Verifizierter Kontext (2026-08-21, selbst vermessen)

Grundwahrheit aus den kompilierten Bins (Magie `0xCF 0x86 0x02 0x00`,
Header 24 B, Granulen-Record = 16 + 3·(deg+1)·8, deg 17, Abstand 32 Tage,
t0 = JD der Granulenmitte):

- **Planeten + Mond** (de440-basiert): sun/earth 36.020 Granulen
  **1599,7–4755,4 ≈ 3156 J**; uranus ≈ 2601 J; saturn ≈ 1900 J; mars ≈ 1455 J;
  mercury/venus/moon/jupiter/neptune/pluto ≈ 1400 J (1599,7–3000,1).
- **Monde** (Satellitenkerne): phobos/deimos ≈ 1055 J; io/europa/ganymede/
  callisto/charon ≈ 600 J; titan/mimas/enceladus ≈ 500 J; triton ≈ 300 J.
- **Asteroiden + kleine Monde** (ceres, vesta, eris, haumea, makemake,
  apophis, bennu, encke, himalia, janus, epimetheus, atlas, prometheus,
  pandora): nur **13 Granulen, 2026,6–2027,6 ≈ 1 J** — Horizons (12 Monate
  voraus + 30 Tage zurück), NICHT der Langbogen.
- **Sonden** (iss, voyager1/2, new_horizons, parker, solar_orbiter, jwst,
  juno, atlas_3i): ~1 Monat (bewusst dynamisch).
- **Kernel-Inventur** (`sources_index.φ`): de440.bsp/de440s.bsp/de440t,
  de430/431/432/433/434 (ältere), die `*-merged-DE441.bsp`/`-DE431.bsp`
  PDC/TTX-Kerne (2015–2026), `asteroids_de441/` (laut
  `docs/concepts/kernel-curation-ci-automation-plan.md`), de442
  (`de442s.mrg` + tech-comments, als `misc` indexiert). DE441-Langbogen:
  13201 v. Chr.–17191 n. Chr.

## Session-Fragen / Arbeitsschritte

1. **Compiler-Systemmodus:** prüfen, wie `--systems` die Körper auswählt
   (`ephemeris_compiler.rs`) — gibt es schon ein `asteroids`-System oder
   muss es kommen? Die `asteroids_de441/`-SPK im Index sind Einzel-Asteroiden-
   SPKs; das Mapping NAIF-ID → `body_name_of`/`pck_id_of` muss für die
   Klein-Körper stehen (vgl. `docs/concepts/kernel-curation-ci-automation-plan.md`
   §4.2). Wenn Teile fehlen: benannt registrieren, nicht erfinden.
2. **Granulen-Maß:** der Langbogen ist riesig (13.000 v. Chr.–17.000 n. Chr.
   ≈ 30.000 J). Die 32-Tage-Granule ergäbe ~340.000 Granulen pro Körper —
   zu groß für einen Bin. Session-Entscheid: gröberes Granulen-Raster
   (z. B. 256-Tage), oder Fenster begrenzen (z. B. ±1000 J um J2000), oder
   degree erhöhen. Das Bin-Format (deg 17, 32 d) ist nicht in Stein.
3. **CI-Integration:** den neuen Schritt in `kernel_flatten.yml` `bodies`
   einhängen (oder eigener Job `asteroids`), `timeout-minutes` anpassen
   (die de441-Dateien sind groß), `--ci-mode` Upload gegen die bestehenden
   Assets (`gh release upload … --clobber`).
4. **Register:** `phi/sources.φ`-Blöcke der Asteroiden (URL `ephemeris_<body>.bin`)
   bleiben unverändert — nur das Asset auf dem CDN wächst. `TODO.md`-Zeile
   schließen, `docs/reference/KERNEL_INDEX.md` aktualisieren.

## Gates

- cargo check 0/0, cargo test. Ein Prototyp: `ceres` (oder `apophis`) aus dem
  de441-SPK kompilieren, roundtrip, Granulen-Fenster messen (soll 13.201
  v. Chr.–17.191 n. Chr. oder das gewählte Fenster treffen, nicht 2026–2027).
- CI-Lauf: der `asteroids`-Schritt lädt die Bins hoch, kein `void`-Report.
- Ein Commit je Einheit; das letzte schließt die TODO-Zeile.
- Diese Datei nach Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

Die Membran/`fs`-Rendering-Physik, `src/te.rs`, die NCEI-SSI-Ernte
(eigenes Atom: `handover-2026-08-21-ncei-ssi-hdf5.md`), die Sonden
(horizons dynamic), der `wm2_1au`-Pfad, die OMEGAFLOW_HIDDEN-Radiator-Stille.
Nur: Asteroiden-Langbogen + CI.
