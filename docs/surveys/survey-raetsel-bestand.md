<!--
  title: Rätsel-Bestand — was wir haben, was fehlt (ein Taucher je Rätsel)
  class: survey
  date: 2026-10-03
  sha256: 524d61dc130004f35b6642a97fe25f870918482a93288126273897fdf13245a5
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/concepts/ein-blatt-papier.md docs/paper/probe-front-dark-matter.md
-->
# Rätsel-Bestand — was wir haben, was fehlt

Stehende Messreihe, kein datierter Einmal-Zensus. Anlass: die Frage, was je Rätsel
wirklich im Baum steht. Methode: **ein read-only Taucher je Rätsel** (flash,
2026-09-28), jede Zelle am Baum gemessen; der Hauptlauf hat die Register-Lücke und
die Risse gegengemessen. Vorläufer: `survey-2026-09-28-raetsel-zensus.md` (erste
Messung, `consumed`).

**Messstempel (2026-10-03, river-folge84, zwei Taucher, HEAD `3be3ff972`+).** Die
`phi/sources.φ`-Zeilennummern der Erstmessung sind **systematisch gedriftet** (~+14 im
400er, ~+31 im 2400er, ~+146–172 im 10–11k-Bereich): die Kanäle existieren, die Ziffern
nicht. Die Zellen tragen darum **Kanal-Keys** (per `register_lookup <key>` auflösbar),
keine Zeilennummern; stabile `file:line` bleiben nur an Code-Pfaden. Native Prosa
Mountain (Träger seit folge227). Die echten Risse sind unten geheilt und als solche
vermerkt — nichts geglättet.

Register der Rätsel: zwölf Nadeln (`kybernetische-astrophysik.md:353-366`), drei
Blätter (`ein-blatt-papier.md:29-35`), der Kuprat (`kybernetische-astrophysik.md:396-398`).
**Kopf-Befund:** das Verdikt der 10 Rätsel lebt in seinem **Blatt/Sheet** (gemessene
Zahlen) und als Zeile dieser Survey (Index, Zeiger `paper:line`/`sheet:line`) — das
ist seine Register-Zeile. Ein `phi/*.φ`-Register trägt es nicht: jeder sources-/
Dispositions-Block verlangt `url`/`ttl` (`register_sort.rs:149-183/:211`), ein
Verdikt hat keine Adresse (Rats-Konsens 2026-09-29). Die Kanäle sind registriert;
`sgrep -ci dark_matter|dark_flow|corona_conditional|signal_cone|frb_blatt|kuprat|rixs|srd62|kugelblitz phi/` → je 0 ist der korrekte Zustand.

## Die zwölf Nadeln

| # | Artefakt (existiert?) | Kanäle vorhanden (Key) | Kanäle fehlend | Daten-Holding | Register-Verdikt | Nächster Schritt |
|---|---|---|---|---|---|---|
| Ⅰ Dunkle Materie | `dark_matter_probe.rs` ja; `jeans_residuum_probe.rs` **ja** (per-Voxel-Kinematik, `--census`); Bin unread | Gaia DR3 (`gaia_dr3`) · pastel/rave (Key re-measure) · Binaries (`galah_dr3_radial_velocity`) | **HI-Kanal fehlt** (`hi4pi`/`21cm`/`hi_gas`/`neutral` je 0); Gaia DR4 (termin 2026-12-02) | kein `data/`-Treffer (gitignored) | **keins** — Paper `probe-front-dark-matter.md` (0/5040 Flags) | Register-Zeile für das Paper-Verdikt; HI-Kanal suchen |
| Ⅱ Flyby | `flyby_probe.rs` · `flyby_ephemeris_gate.rs` (gebaut `d310d5888`) | Sonden-ICRS · DE440-442 · Solarwind · IMF-Bz · Kp · Swarm | **DSN-Live-Tracking steht** (`dsn_snapshot.bin`, eyes.nasa.gov, `sources.φ:109`) — das Erst-„absent" war der Riss; `dead_sources.φ` trug Euclid | `data/ssd.jpl.nasa.gov/ephemeris_juice.bin`; `data/flyby2/gate-juice-2026-09-28.json` | **pending** — Gate-JSON `delta_km`/`threshold_km`/`verdict` pending; `edition_rift_km=0.16847` | nach Perigäum `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate` (CI) |
| Ⅲ Koronale Heizung | `corona_conditional_probe.rs` · `solar_causal_graph_probe.rs` | GOES X · EUV · F10.7 · IMF · OMNI (`omni_hro_*`) · AIA · EVE | keiner der Kanäle; Riss: Probe lädt `ssd.jpl.nasa.gov`-Assets, Register führt dieselben unter `ncei`/`cdaweb` | keins gemessen | **keins** — Verdikt nur Paper `corona-heating-ladder.md:18` (family-bound) | Netloc-Riss klären; Register-Zeile |
| Ⅳ LAIC | `laic_probe.rs` · `nobel_probe_laic.rs` · Paper `laic-arrow-direction.md` v4 | USGS · INTERMAGNET (`intermagnet_xyz_*`) · Swarm · CHAMP-TEC · Safecast · Kp | **Swarm-TEC steht** (`swarm_absolute_vertical_tec_tecu`, `sources.φ:7276`); CSES: `ledger.φ:15` trägt **AFAD** (nicht CSES), Termin 2026-10-02 **gefeuert, Operator „Nein"**; **kein `laic*.bin` / `phi/pipeline/laic_harvest/` im Baum** (Erst-Holding überholt) | keins | **keins** — Paper `:14/:25` Stille | Swarm-TEC messen; LAIC-Kanal neu bestimmen |
| Ⅴ Technosignaturen | `lsst_anomaly_probe.rs` · `ztf_anomaly_probe.rs` · `negativ_fuzzy_probe.rs` (TE-Negativ-Fuzzy; Dip/FAP in `lsst`/`ztf_anomaly_probe`) | ZTF · IRAS (`iras_psc_*`) · AKARI (`akari_mir_flux_jy`) · VSX (`vsx_*`) · GCVS (`gcvs_*`) · witnesses Lasair/Fink | Gaia `bp_rp` descoped (`blocked_sources.φ:203`) | keins | **keins** — Paper `docs/paper/nadel-v-fresh-area-dip-scan.md:39/:47` (0 unexcluded, <3σ) | Paper-Verdikt registrieren |
| Ⅵ Planet 9 | `kbo_compiler.rs` · `kbo_residue_probe.rs` | KBO (`kbo_h_mag`) · MPC-Distant (`mpcorb_distant`) · Sonden-Arcs · Planeten-Eph | keine | keins | **keins** — Paper `planet-nine-kbo-residue.md:18` (kein fam-Pfeil); descoped in `sensory-folge201:193-194` | descope-Träger/Register |
| Ⅶ Wurmloch | `signal_cone_audit_probe.rs` (Bin) | `te.rs` · `membrane.rs` · Retardierungs-Fold `max(0,\|Δt\|−d/c)` (`spatial.rs:610-611`) · Probe | Sonden-Bahnen-Kanal unverifiziert | `data/kegel_audit_voll.log` pending | **keins** — Paper `signal-cone-audit-sheet.md:14/:63` (0 honored) | Name-Riss geschlossen (Bin = `signal_cone_audit_probe`) |
| Ⅷ Dunkler Fluss | `dark_flow_probe.rs` | CMB · Abell · MCXC · PSZ2 · CF4 (`cf4`) | z≳10-Sample | kein `cosmicflows_cf4.json` im Baum | **keins** — Paper `dark-flow-sheet-8.md:14/:33` Stille | Release-Namespace-Riss (Register `tapvizier` vs Probe `ssd.jpl`) |
| Ⅸ FRB | `frb_blatt_probe.rs` · `frb_compiler.rs`; `write_blatt_pair` steht (`:295`) | FRB-Kanäle · Streuung · peak-flux · Magnetar | kein Paar-Artefakt (glob 0); Herkunftsadresse | keins | kein frb-Verdikt; FRBCAT `dead`; CHIME `declined` | `archive_search --verdict <frb_chime_cat1.json>`; Probe `--write` Blatt |
| Ⅹ Kugelblitz | keiner (glob 0) | keine | alle vier co-lokalisierten (em×electric×thermal×acoustic) | keins | **descoped (gemessen: → `kybernetische-astrophysik.md:294-300`; kein Probe-Artefakt, glob 0)** | — (Befund steht) |
| Ⅺ Placebo | `placebo_pair_eeg_probe.rs` · **`openneuro_compiler.rs`** (+ Reader-Modul `openneuro_eeg.rs`) | `harvest.φ` 2 Assets; `sources.φ` sham/verum | Gabe×HRV×Blutmarker | CDN `openneuro.org` Assets | **keins** — Verdikt nur Archiv-Handover `folge10:24-25`; kein lebendes Blatt | `gh workflow run placebo-ave-cdn.yml`; Verdikt ins Register/Blatt |
| Ⅻ Urknall | `bigbang_echo_probe.rs`; CI `bigbang-echo.yml` | CMB · CF4 · z-Code | PTA `declined`; **B-Moden (0 — nicht nachgemessen, open)** | keins | **keins** — Sheet `big-bang-echo-sheet-12.md:14/:35/:59-60` | `gh workflow run bigbang-echo.yml`; B-Moden messen |

## Die Blätter und der Kuprat

| Blatt | Artefakt | Kanäle vorhanden (Key) | Fehlend | Register-Verdikt | Nächster Schritt |
|---|---|---|---|---|---|
| ENSO | **`enso_blatt_probe.rs` existiert** (`tools/measure`); Reader `esacci_sst_read.rs` | SST (`esacci_sst`/`ersstv5_nino34`) · Drifter (`hydrosphere_drifter_sst_k`) · OOI (`hydrosphere_sst_ooi_c`) · TAO | ENSO-TE-Zuschnitt; Becken-Windfeld (scatterometer absent) | MEI/SOI/ERA5 `declined`; Paar-Zuschnitt operator-gebunden (`wartend.φ` — **resolved**) | Probe nach `nobel_probe_corona.rs`-Muster |
| GIC/Bz | `bz_blatt_probe.rs` · `bz_retro_probe.rs` | OMNI · RTSW · BPA · FMI · INTERMAGNET | co-lokales Mäntsälä-dB/dt | Riss Paper `gic-causal-driver.md:405-423/:492-495`; Paper-Ref stale | Mäntsälä-Kanal; Paper-Ref heilen |

| Kuprat | Artefakt | Kanäle (4, alle CDN 206) | Fehlend | Register | Nächster Schritt |
|---|---|---|---|---|---|
| Hoch-Tc | `rixs_cuprate_probe.rs` · `suprastrom_cuprate_probe.rs` · `suprastrom_form_probe.rs` · `crystal_compiler.rs` · `srd62_compiler.rs` · `cuprate-cdn.yml` · `srd62-cdn.yml` | RIXS spin (Zenodo 7286412) · RIXS charge (15179114) · EELS · SRD62 | 5. Ader (privates Labor-Holding, 13 Läufe); Kuprat-Zeugenklasse | 4 `witness substance`-Zeilen (`witnesses.φ:120-142`); kein `sources.φ`-Block (Verdikt ohne Adresse) | Mycelium: `url`/`origin`/`compiler` + Tag-Drift `ssd.jpl.nasa.gov`; kein `witness kuprat` (Vertrag: 4 Klassen, `witnesses.φ:1`); 5. Ader: **LOCK `privat`** (Operator 2026-10-01 — nichts verlässt das Haus; Rat 2026-09-29: Substance, kein Wire-Arm, `archiv/handover-2026-09-29-mountain-folge204.md:80-86`); **Treiber-Lauf** über die öffentlichen Kanäle **nicht wohlgestellt** (gemessen 2026-10-01, grind-flash: Spin/Charge/Lattice/Supercurrent tragen keine gemeinsame geordnete Achse; TE braucht eine gepaarte Reihe; die RIXS/EELS-Bins sind PSD ohne Phase) — der einzige wohlgestellte Pfad wäre das private Holding (LOCK) |

## Querschnitt

1. **Verdikt-Träger (Rats-Konsens 2026-09-29).** Für 10 der 15 Rätsel lebt das
   Verdikt im Blatt/Sheet + als Survey-Zeile (Index) — nicht in einem `phi/*.φ`-
   Register (`register_sort.rs` verlangt `url`/`ttl`; ein Verdikt hat keine Adresse).
   Die Kuprat-Kanäle sind als `witness substance` admitiert (`witnesses.φ:120-142`);
   eine `witness kuprat`-Klasse gibt es nicht (Vertrag: 4 Zeugenklassen, `witnesses.φ:1`).
2. **Release-Namespace-Risse (systemisch).** Gemessen 2026-09-28: die Compiler laden
   bereits unter den **Produzenten-Tag** (`cosmicflows_compiler.rs:204` →
   `tapvizier.cds.unistra.fr`; `goes_xrs_compiler.rs:430` → `ncei.noaa.gov`), die
   Register-URLs sind korrekt — aber das **physische Asset** liegt unter dem Legacy-Tag
   `ssd.jpl.nasa.gov` (`--verdict`: `ssd.jpl` 206 vs. `ncei`/`tapvizier` 404), und
   Probes/Workflows **hardkodieren den Legacy-Tag** (`dark_flow_probe.rs:64`,
   `bigbang_echo_probe.rs:65`, `solar_causal_graph_probe.rs:9-15`,
   `wso_cycle_probe.rs:8-10`, `goes-xrs-cdn.yml:26`). Kein Spot-Fix: die Migration ist
   corpus-weit (u. a. `jwst_spectra`/`spectra.bin`/`dr3_stars`/`dastcom`; `cdn_reconcile.rs`
   kennt die `ssd.jpl.nasa.gov-*`-Präfixe).
3. **Doc-vs-Baum-Risse.** **Geheilt (gemessen 2026-09-30):** Ⅳ `laic_champ.bin` — die
   CDN-Release-Metadaten des Tags `ssd.jpl.nasa.gov-laic` messen 676 408 222 B
   (676,4 MB / 645,1 MiB); die Paper-Angabe „676 MB" (`laic-arrow-direction.md:214`)
   stimmt dezimal. Ⅴ `negativ_fuzzy_probe` ist der TE-Negativ-Fuzzy-Ausschluss,
   kein Dip/FAP-Träger; der Dip/FAP lebt im FAP-Gate von
   `lsst_anomaly_probe`/`ztf_anomaly_probe` (`nadel-v-fresh-area-dip-scan.md`).
   **Geheilt (2026-10-03, river-folge84):** Ⅰ per-Voxel-Jeans-Engine **steht**
   (`jeans_residuum_probe.rs`) — der „fehlt"-Befund war falsch; Ⅱ DSN-Live-Tracking
   **steht** (`dsn_snapshot.bin`) — „absent" war falsch; Ⅳ Swarm-TEC **steht**
   (`swarm_absolute_vertical_tec_tecu`), CSES-Termin gefeuert (Operator „Nein");
   Ⅺ Artefakt ist `openneuro_compiler.rs` (das Reader-Modul `openneuro_eeg.rs` steht);
   ENSO `enso_blatt_probe.rs` **steht**, der `wartend.φ`-Zuschnitt ist **resolved**;
   Ⅶ Fold auf `spatial.rs:610-611`. Ⅹ (Kugelblitz) bleibt **descoped mit Befund**
   (`kybernetische-astrophysik.md:299-300`). Ⅻ B-Moden **open** (nicht nachgemessen).
4. **Operator-gebunden.** ENSO-Zuschnitt (`wartend.φ` — resolved), Kuprat 5. Ader
   (privat, NSE), NSE-Redistribution (`state/mail/`).
5. **Fremde Feder (Mountain).** Kuprat-Zeugenklasse: `witness kuprat` existiert nicht
   (0 Treffer in `phi/`); die vier Kanäle sind als `witness substance` admitiert
   (`witnesses.φ:120-142`) — kein `format`/`field`/`ttl` nötig (Verdikt 2026-09-30).
