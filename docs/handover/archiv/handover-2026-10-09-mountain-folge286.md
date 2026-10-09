<!--
  title: Handover — Mountain-Folge 286 (2026-10-09)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-09
  sha256: 3540ceb8bae3bd4f55b00178a0a68d5643bf008a4ec71a8226339cd7c2c8a667
  status: live
-->
# Handover — Mountain-Folge 286 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium, gemessen 2026-10-09T15:33Z). Diese Session konsumierte
`handover-2026-10-09-mountain-folge285.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.0647 — `session_burn` 2026-10-09 (Mountain-Linie, flash only, kein pro/max: 1 `grind-flash` für den USGS-Compiler)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–286)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–286)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„ja bitte" — Index-Riss als Mountain-Verdikt `quantity` setzen + die `sources.φ`-Zeilen bauen | 2026-10-08 | Operator (Session, Mountain 276)
„ich glaube du musst nochmal breiter fragen" — Science-Layer + starke Frontier-Seats für die Route-Admission | 2026-10-08 | Operator (Session, Mountain 276)
„bitte umsetzen Offen (im Report benannt): 2 blocked_sources-Risse …" | 2026-10-09 | Operator (Session, Mountain 280)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) …" + „aber mach dann auch wirklich die Arbeit" | 2026-10-09 | Operator (Session, Mountain 281)
„1. natürlich Ja wir brauchen die lizenzen sind regeln der quellen nicht unsnere" | 2026-10-09 | Operator (Session, Mountain 283)
„2 bitte spreche dich mit river ab das ist teil seines plans" | 2026-10-09 | Operator (Session, Mountain 283)
„du sollst das prüfen, die lizenzen müssen korrekt sein" | 2026-10-09 | Operator (Session, Mountain 283)
„sind jetzt alle blöcke in allen asset files mit tes versehen …" | 2026-10-09 | Operator (Session, Mountain 283)
„wir sind immer noch nicht opensource" — omegaflow ist source-available (PolyForm NC/CC BY-NC-SA), NIE „open-source" nennen | 2026-10-09 | Operator (Session, Mountain 283)
„natürlich 1 wir sind nicht open source wir sind NC CC" | 2026-10-09 | Operator (Session, Mountain 283)
„das ist compliance theater" — keine Lizenz-Boilerplate in einer Anfrage-Mail; nur sagen, was die Frage braucht | 2026-10-09 | Operator (Session, Mountain 283)
„bitte fixen: ledger.φ JAXA-Zitat … ShadowCam admission ja, Format-Arm fehlt; Migration (river-148) gefaltet" | 2026-10-09 | Operator (Session, Mountain 284)
„bitte mit archive search all dem rat und den top tier frontier voices besprechen" — USGS-Extract-Arm-Architektur | 2026-10-09 | Operator (Session, Mountain 285)

## Offen (aufgeschlüsselt)

### PDS-PPI — Enumerator/Workflow stehen, `sources.φ`-Zeile offen
- **Status:** eigen (Register) | **Bindung:** eigen (Register)
- **Trigger:** entworfene `sources.φ`-Zeile für das dynamische Multi-Granule-Asset
- **Lage:** (gemessen 2026-10-09 via `sread`/`sgrep`) `pds_ppi_compiler.rs` (`115cf1657`) + `.github/workflows/pds-ppi-cdn.yml` (self-hosted, `gh release view pds-ppi.igpp.ucla.edu`, idempotent) + `harvest.φ`-Arm (`format pds_ppi` · `arm pds_ppi_compiler` · `workflow pds-ppi-cdn.yml` · `pattern ^pds_ppi_.+\.bin$`) stehen; NETLOC `pds-ppi.igpp.ucla.edu`, Asset-Name `pds_ppi_data_<bundle>_data_<stem>.bin`. `blocked_sources.φ`-Note auf diesen Stand gesetzt.
- **Blockade:** das Asset ist **multi-granule** (ein `.bin` je Tabelle, dynamischer Name) — eine einzelne `url`-Zeile greift nicht; die Manifestations-Form ist unentschieden.
- **Braucht:** keine statische Zeile möglich (Family unbounded: 31 Shards/1 Tabelle; kein Manifest; `pds4_fixed_width` verlangt explizite `field`-Zeilen). Zwei Code-Vorbedingungen: (P1) `pds_ppi_compiler.rs` emittiert ein `<name>.manifest` (modis-Modell), (P2) dynamische Feld-Deklaration aus dem P4FW-Spaltenblock. `terms` gemessen: TSPA = PD (`https://pds-ppi.igpp.ucla.edu/tspa.jsp`).

### ShadowCam-Admission — admission ja, Format-Arm fehlt
- **Status:** blockiert | **Bindung:** mycelium (Format/Arm baut)
- **Trigger:** Format-Arm/TIFF-Compiler
- **Lage:** (gemessen 2026-10-09 via `archive_search --verdict` + curl) `pds.shadowcam.im-ldi.com/derived/` HTTP 200; DTM-Blatt `.cub` (ISIS) + `_cog.tif` (COG) + `.xml` (PDS4-Label); **kein `.fits`** — `pds4-fits` deckt Chang'e-MRM. Admission ja (PDS public, KPLO/LRO-NAC). Format: PDS4-Derivat-Raster.
- **Blockade:** Format-Arm fehlt.
- **Braucht:** Mycelium baut Format/Arm; Mountain schreibt die `sources.φ`-Zeile auf die Format-Entscheidung.

### Register-Physik-Migration (`force` → Quantity | Mechanism | Medium)
- **Status:** wartend | **Bindung:** river (Parser-Arm)
- **Trigger:** Rivers `channel`-Parser-Arm auf der genannten `pde_type`-Menge
- **Lage:** (gemessen 2026-10-09, river-150) `ChannelDescriptor` trägt `role: QuantityRole` + `pde_type: PdeType`, `hash()` faltet beide, `cargo check` grün, 2 Tests. Rivers Schema-Charge steht; `gravity`=elliptisch, `seismic`=hyperbolisch, `em`=`Mixed` (regime-abhängig). Antwort mit den Token an river gefaltet (`## An river`).
- **Blockade:** der `channel`-Parser-Arm (P0.3/P9.1) schreibt noch nicht.
- **Braucht:** Rivers Arm; danach schreibt Mountain die `quantity`/`pde_type`-Zeilen der ersten Charge (`gravity`/`seismic`).

### Flyby-Kette — Residual liegt in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen (Register) · river (`flyby_ephemeris_gate`)
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09) Der ESTRACK/DSN-Residual ist **nicht absent**: 157 ODF-Referenzen in `sources.φ`; `odf.rs` steht, `doppler.rs` absent. **Riss:** σ_recon ist die 1-σ-Kovarianz der ESOC-Post-Flyby-Recon-Ephemeride (`ephemeris_juice_recon.bin` 404), kein Doppler-Residual.
- **Blockade:** kein ESOC-Recon-Release; `doppler.rs` wird vom ODF-Residual nicht gebraucht.
- **Braucht:** ESOC-Release abwarten (river) oder Descope-Befund für `doppler.rs`.

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** wartend | **Bindung:** eigen (CI-Lauf)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-09 via `ci_manage view`) Lauf `37929072865` (`ci-check`, head `35fe7b5a`) **conclusion `cancelled`** — kein Testergebnis, kein grüner IGRF-Nachweis. `igrf.rs` steht (`src/archivar/igrf.rs`).
- **Blockade:** kein grüner Lauf (Queue-Abbruch).
- **Braucht:** nächster `ci-check`-Lauf am neuen HEAD; bei grün Punkt schließen.

### `blocked_sources.φ` — 15 Klassen-Träger (`gap`-Token)
- **Status:** eigen (Disposition) | **Bindung:** eigen (Bau/Disposition) · mycelium (Diver-Tabelle)
- **Trigger:** Bau je Klassen-Träger
- **Lage:** (gemessen 2026-10-09 via `archive_search --verdict` durch 5 Agenten; Notes in `blocked_sources.φ` aktualisiert) **14 `gap`-Träger**. **Buildable now** (Arm steht, Workflow/`sources.φ` fehlen): `kaguya-lrs` (`pds3_binary`) · `solar-vso` (neuer IRIS-FITS-Arm; SDO/AIA schon gebaut) · `cmb-lambda` (Rest WMAP/ACT/SPT) · `particle-cern` (CERN-Open-Data-API + GWOSC-Arm). **Compiler gebaut, Register blockiert:** `inpe-big-stac` (`inpe_stac_compiler.rs`, `samet_daily-1` CC-BY-4.0, NetCDF4 `tmax` ohne `units`-Attribut → Einheit ungemessen). **Quelle fehlt:** `hi-21cm` — `J/A+A/585/A136` existiert NICHT in VizieR (Falsch-Positiv, `+`-Dekodierung); EBHIS eigenständig suchen; HEASARC `hi4pi`/GASS 200. **Wartend** (NSSDC-/JPL-Antwort; echte `mail_ledger.φ:74/:76/:78`, Zitate `:207/:209/:211` waren stale): `tracking-doppler` (atdf-Arm steht) · `viking-tracking` (kein Tracking-Parser) · `juno-efb` (odf-Arm steht). **Blockiert/LOCK:** `bc-mpo-more` (release 2099) · `mariner-rst` (request-only) · `dmap-map-grid` (Globus EXPIRED, LOCK) · `blinkverse-frb` (Zugang pending). **Riss:** `laic-cssdc` — `cssdc.ac.cn` liefert jetzt eine Telegram-APK-Seite, keine Wissenschaft. Gap-Token-Header-Kanon **22**; Register-Gesamt **16** (14 `parser-def` + 2 `pending`: PDS-PPI, PETREL19).
- **Blockade:** buildable now an Workflow+`sources.φ`; `inpe` an der Einheiten-Messung; `hi-21cm` an der fehlenden Quelle; wartend an NSSDC/JPL-Antwort; `laic-cssdc` am Riss.
- **Braucht:** für die buildable je Workflow + `sources.φ`-Block; `inpe` `units` messen; `hi-21cm` EBHIS-Release lokalisieren; für wartend nur die Antwort (Trigger); für `laic-cssdc` ein Operator-Wort zur Disposition.

### PETREL19-Ephemeriden — Wei erteilt CC BY 4.0 (decline widerlegt)
- **Status:** eigen | **Bindung:** eigen (Register/Port)
- **Trigger:** Port gebaut (PETREL19-Arm im `ephemeris_compiler`)
- **Lage:** (gemessen 2026-10-09) Tian Wei erteilt **CC BY 4.0**; die frühere `decline redistribution` entfernt, PETREL19 steht als `pending`. **Riss korrigiert:** `ephemeris_compiler.rs` ist seit `0615424f6` (Mycelium 276) bereits auf `--release`/`--prefix` für den PETREL19-Namensraum parameterisiert — nicht der Compiler fehlt.
- **Blockade:** die Route (Kernel-Fetch + Workflow + `sources.φ`-Block) fehlt.
- **Braucht:** Kernel von `raw.githubusercontent.com/TIAN-we/petrel19/HEAD/fmt_spice/` (`PETREL19_translation.bsp` u.a.) → `de_compiler --label petrel19` (oder `ephemeris_compiler --prefix`) → `sources.φ`-Block `ephemeris_petrel19_<body>.bin` mit `terms CC-BY-4.0 https://github.com/TIAN-we/petrel19`.

## An mycelium

Origin: mountain-folge286.

- **GIC-Faden §A–G Rest:** SuperDARN-MAP-grid-Download (`wartend.φ:8`, Globus-Credentials) + Keogramm-Vision-Asset liegen bei dir.
- **`omegaflow/sources`-Manifestator:** Quell-Lizenz je Release-Body aus der `terms`-Zeile (aus 283) steht aus.
- **Gefaltet aus mycelium-279:** Substorm-Arm/Workflow + `ledger.φ:150` disponiert · ceic/Wolfx-Dublette in `declined_sources.φ` · DAS2-Iowa-`ledger.φ:100` korrigiert · `declined_sources.φ` register-sort canonical. Alle gelesen, nichts offen.

## An river

Origin: mountain-folge286.

- **Gefaltet aus river-150:** Schema-Charge gelesen (`ChannelDescriptor` `role: QuantityRole` + `pde_type: PdeType`, `hash()` faltet beide). **`pde_type`-Token der ersten Charge:** `gravity` → `Elliptic`, `seismic` → `Hyperbolic`, Rolle je `Primary`; `em` → `Mixed` (bestätigt). Sobald dein `channel`-Parser-Arm auf dieser Menge steht, schreibt Mountain die `quantity`/`pde_type`-Zeilen der `gravity`/`seismic`-Charge. `Domain/Rand`-Ausbau vor `em` bleibt Bedingung.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (dieser Atom: `/commit`).

Eigene Pfade: `tools/harvest/src/bin/usgs_geomag_compiler.rs` · `src/archivar/usgs_geomag.rs` · `src/archivar/extract.rs` · `src/archivar/main_flow.rs` · `src/archivar/mod.rs` · `.github/workflows/usgs-geomag-cdn.yml` · `phi/sources.φ` · `phi/harvest.φ` · `phi/blocked_sources.φ` · `docs/handover/handover-2026-10-09-mountain-folge286.md` · `docs/handover/archiv/handover-2026-10-09-mountain-folge285.md`.
