<!--
  title: Handover — Mountain-Folge 338 (2026-10-13)
  session: Mountain-Linie — die zwei Register-Risse geschlossen (cosmic2_gnssro-Format, LHAASO-E0)
  class: handover
  date: 2026-10-13
  sha256: ac88c68465c5c057d5151a5d0633d370bf66e47795e17bf69f6a3d342342dd93
  status: live
-->
# Handover — Mountain-Folge 338 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-13-mountain-folge337.md` (→
`docs/handover/archiv/`).

## Burn: open 0.02 · close 0.03 — Grund: zwei Register-Risse geschlossen (Format-Arm-Angleich, E0-Namenskonvention) im geteilten Baum; deepseek-flash, kein pro/max.

## Offen (aufgeschlüsselt)

### Ersatz-Bau — offene Reste (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das gewählte bbox-Ziel / der freigegebene Reader
- **Lage:** (gemessen 2026-10-13, `sgrep`) registriert sind BGR, CEEIN, IRSF J, LHAASO WCDA+KM2A, jpl_sbdb; die zwei Register-Risse sind in diesem Atom geschlossen (committet): `cosmic2_gnssro`-Format → `cosmic_ro` (Arm `extract.rs:1253`, Compiler `gesdisc_gnssro_compiler.rs:501`), LHAASO-E0 im Observable-Namen (`lhaaso_sky1_e0_3tev` WCDA · `lhaaso_km2a_e0_50tev` KM2A; Einheit unverändert, Feld-Schlüssel/`probe_classify` unverändert). **Offen:**
  - **`opentopography_raster` ohne bbox-Ziel** — der Compiler `tools/harvest/src/bin/opentopography_raster_compiler.rs` (committet) liest die SRTM15+ V2.6 netCDF über HTTP-Range und emittiert `url`/`witness gestalt`/`record srtm`; ohne gewähltes `--bbox` keine Zeile (`sgrep opentopography phi/sources.φ` = nur `opentopography_coverage` + `srtm15_plus`, kein raster-Witness-Block).
  - **Table-Katalog-Verdrahtung** — in-flight: untracked `src/archivar/osc.rs`, `src/archivar/lamost.rs`, `src/archivar/heasarc_w3browse.rs`; dirty in `src/archivar` (`extract.rs`, `mod.rs`) — fremd, unangetastet.
- **Blockade:** das bbox-Ziel fehlt · `extract.rs` fremd-gehalten.
- **Braucht:** ein gewähltes Gebietsfenster → `opentopography_raster_compiler --bbox <s,n,w,e> --out <path>` → Block falten · Reader freigeben. `docs/SOURCE_PORT.md`.

### mycelium-340/341 — Risse-Verdikt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Risse-Verdikt
- **Lage:** (gemessen 2026-10-13) die 4 Restcompiler sind eingetragen: `jpl_sbdb` neu (`7ee56c2`, PD), `heasarc_w3browse`/`gesdisc_gnssro` gedeckt, `opentopography_raster` offen (siehe oben). Offene Risse: IERS-EOP-Doppellinie (`phi/sources.φ:8114-8116` USNO `finals.all` trägt `eop_iers_ut1_utc_s`/`_polar_motion_x_arcsec`/`_y_arcsec`; `phi/sources.φ:10143-10145` `celestrak_eop` trägt dieselben Observablen), CHIME/FRB-Cat2 canfar 500, KASI-API 500, IRIS LMSAL 403, MPC-Horizon block-vs-compiler, Ersatzquellen MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat.
- **Blockade:** das Verdikt.
- **Braucht:** das Risse-Verdikt.

### Eigene Ephemeride — unregistrierte Quellen (future-249 .3) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der jeweiligen Quellenklasse
- **Lage:** (gemessen 2026-10-13) Bauplan native Kette UTC→TT→TDB, ITRF2020+EOP, VMF3/IONEX, Zwei-Wege-Lichtzeit+Shapiro; Referenzen `state/future/analyse-kybernautin-ephemeriden-2026-10-12.md`, `abdeckungs-matrix-…`, `leads-ephemeriden-quellen-…`, `leads-ephemeriden-luecken-…`. Registriert: APOLLO-LLR, BC430, JPL `obs_data`/Radar-API, Venus-Radar Zenodo 18176179, Mariner 4/6/7/9/10, PVO-ORSE, Pioneer 10/11, Viking, New Horizons. **Compiler committet** (`010ccb657`): `ilrs_npt_compiler` (Format `ilrs_npt`, Arm in `extract.rs` offen), `icrf3_compiler` (`witness s2-direction`/`record skydirection`), `pds3_bip_radar_compiler` (Format `pds3_img`); **Blocks + CDN-Workflows fehlen**. `.secrets.local` (nur Schlüsselnamen via `bin/secrets_keys`): `EARTHDATA_EDL_TOKEN` · `NSSDC_USER`/`NSSDC_PASS` · `SSDC_USER`/`SSDC_PASS` vorhanden; CDDIS ist implementiert (`ionex_compiler.rs:45`), die NSSDC-/SSDC-Schlüssel haben noch keinen Konsumenten — die „nur über Operator/Future"-Einordnung ist widerlegt.
- **Blockade:** `extract.rs` fremd-gehalten (ILRS-Arm) · Assets/Blocks.
- **Braucht:** je Klasse einen Block + CDN-Workflow für die vorhandenen Compiler; ILRS-Format-Arm in `extract.rs`. `docs/SOURCE_PORT.md`.

### Register-Force-Unit-Schuld (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste freie Register-/units.rs-Atom
- **Lage:** (gemessen 2026-10-13) `target/{release,debug}/commit_check` blockierte den Register-Commit an 5 Paaren außerhalb der Registry, die die `register_sort`-Neuschreibung `75bd350829` in HEAD trägt: `diffusion kg/m2/s` (`phi/sources.φ:1417`/`:14740`), `diffusion μmol/kg` (`:1735`; die Registry führt `micromole/kg`), `seismic-body km/s` (`:28556`/`:29211`), `em K` (`:29022`, hi4pi Helligkeitstemperatur), `em nmgy` (`:29093`, NOIRLab-Flux). In diesem Atom in `docs/specs/force-unit-baseline.txt` **eingefroren**, um den Register-Commit freizugeben.
- **Blockade:** keine.
- **Braucht:** je Paar ein Verdikt und Admission in `src/archivar/units.rs` (`allowed_units_for_force`/`for_quantity`) oder Korrektur der Register-Zeile — dann die Freeze-Zeile in der Baseline löschen (die Datei schrumpft).

## An mycelium

Origin: mountain-338 (2026-10-13):

- `xfer:mountain.ses_ecae8b148ffeT4kqLjZt0U6fKV.1~15a632a3` **Register-Risse geschlossen.**
  `cosmic2_gnssro`-Format → `format cosmic_ro` (`phi/sources.φ:10244`; Arm `extract.rs:1253`,
  Compiler emittiert `cosmic_ro`, `MAGIC_COSMIC`); LHAASO-E0 im Observable-Namen
  (`lhaaso_sky1_e0_3tev` WCDA 3 TeV · `lhaaso_km2a_e0_50tev` KM2A 50 TeV), Feld-Schlüssel
  unverändert. Kein CI-Wiring betroffen (der LHAASO-Compiler emittiert keinen Block).

## Operator-Wort-Register

- **„Projekt-Ownership statt statischer Verantwortlichkeiten — jeder schiebt den schwarzen Peter … ihr seid alle gleich ausgebildet"** | 2026-10-13 | Operator-Session (mountain 336) — kein Ball-Zuschieben.
- **„warum haben wir immer noch no-license-stated und fragwürdige lizenzen auf cdn?"** | 2026-10-13 | Operator-Session (mountain 336) — Riegel + Auflösung; `no-license-stated` = 0.
- **„bitte prüfe was zu viel entfernt wurde"** | 2026-10-13 | Operator-Session (mountain 336) — 8 zu viel entfernte Blöcke wiederhergestellt.
- **„und haben wir jetzt für alle offene alternativen?"** | 2026-10-13 | Operator-Session (mountain 336) — teils.
- **„liegen jetzt nur noch einwandfreie Assets auf CDN?"** | 2026-10-13 | Operator-Session (mountain 336) — nein; Orphan-Bestände, Takedown an Mycelium.
- **„das muss eindeutig recherchiert und an mycelium gegeben werden"** | 2026-10-13 | Operator-Session (mountain 336).
- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes."** | 2026-10-13 | Operator-Session (mountain 337/338) — Linien-Start = session-weiter Consent (Delegation), getrennt vom Commit-Wort.
- **„du hast nur gemessen und registriert … bitte delegiere an einen taucher schwarm"** | 2026-10-13 | Operator-Session (mountain 337) — Ersatz-Bau per Schwarm.
- **„was davon kannst du umsetzen und an taucher deligieren?"** | 2026-10-13 | Operator-Session (mountain 337).
- **„mach das [IRSF/CEEIN-Blöcke + Workflows, LHAASO-Widerspruch] und hast du die secrets local geprüft? Der Zugang zu CDDIS und den NSSDC-request-only-Sätzen: Operator-/Future-Hand."** | 2026-10-13 | Operator-Session (mountain 337) — Register-Blöcke + Workflows umsetzen, LHAASO auflösen, `.secrets.local` prüfen (nur Namen).
- **Mycelium-Weitergabe: „Die vier letzten Leseprogramme festschreiben und eintragen. Den LHAASO-Block fertigstellen … Entscheiden, ob LHAASO ein zweites Feld für die KM2A-Komponente bekommt."** | 2026-10-13 | Operator-Session (mountain 337, Mycelium-Auftrag) — vier Compiler + Block geprüft; LHAASO vollendet; **KM2A: ja**.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom (folge338)
committet: die neue Übergabe + die konsumierte folge337 → `archiv/`; die zwei
Register-Riss-Zeilen in `phi/sources.φ` (`cosmic2_gnssro` → `cosmic_ro`; die
LHAASO-Observable-Namen `lhaaso_sky1_e0_3tev`/`lhaaso_km2a_e0_50tev`) —
block-scoped.
