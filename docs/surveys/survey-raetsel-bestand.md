<!--
  title: Rätsel-Bestand — was wir haben, was fehlt (ein Taucher je Rätsel)
  class: survey
  date: 2026-09-28
  sha256: f278343589c4aaa0002b36f508df318f4bbcaab2d558afc27ca36b4e1c0f9bab
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/concepts/ein-blatt-papier.md docs/paper/probe-front-dark-matter.md
-->
# Rätsel-Bestand — was wir haben, was fehlt

Stehende Messreihe, kein datierter Einmal-Zensus. Anlass: die Frage, was je Rätsel
wirklich im Baum steht. Methode: **ein read-only Taucher je Rätsel** (flash, 2026-09-28),
jede Zelle am Baum gemessen; der Hauptlauf hat die Register-Lücke und die Risse
gegengemessen. Vorläufer: `survey-2026-09-28-raetsel-zensus.md` (erste Messung,
`consumed`). Zellen sind `file:line`/`command@zeit` oder `pending`/`unread` — nichts
geglättet.

Register der Rätsel: zwölf Nadeln (`kybernetische-astrophysik.md:353-366`), drei
Blätter (`ein-blatt-papier.md:29-35`), der Kuprat (`kybernetische-astrophysik.md:396-398`).
**Kopf-Befund:** das Verdikt der 10 Rätsel lebt in seinem **Blatt/Sheet** (gemessene
Zahlen) und als Zeile dieser Survey (Index, Zeiger `paper:line`/`sheet:line`) — das
ist seine Register-Zeile. Ein `phi/*.φ`-Register trägt es nicht: jeder sources-/
Dispositions-Block verlangt `url`/`ttl` (`register_sort.rs:149-183/:211`), ein
Verdikt hat keine Adresse (Rats-Konsens 2026-09-29). Die Kanäle sind registriert;
`sgrep -ci dark_matter|dark_flow|corona_conditional|signal_cone|frb_blatt|kuprat|rixs|srd62|kugelblitz phi/` → je 0 ist der korrekte Zustand.

## Die zwölf Nadeln

| # | Artefakt (existiert?) | Kanäle vorhanden (Quelle:Zeile) | Kanäle fehlend | Daten-Holding | Register-Verdikt | Nächster Schritt |
|---|---|---|---|---|---|---|
| Ⅰ Dunkle Materie | `dark_matter_probe.rs` ja; Bin unread | Gaia `sources.φ:10309/10311` · HI `:8870/8879` · pastel `:10616/10617` · rave `:10683/10684` · Binaries `:8902` | per-Voxel-Jeans-Engine (`sgrep jeans tools` → 0); Gaia DR4 (termin 2026-12-02) | kein `data/`-Treffer (gitignored) | **keins** — Paper `probe-front-dark-matter.md` (0/5040 Flags) | Register-Zeile für das Paper-Verdikt; `archive_search --verdict <HI-url>` |
| Ⅱ Flyby | `flyby_probe.rs` · `flyby_ephemeris_gate.rs` (gebaut `d310d5888`) | Sonden-ICRS `:15524` · DE440-442 `:3419-3484` · Solarwind `:750/:164` · IMF-Bz `:746` · Kp `:1160` · Swarm `:7240` | DSN-Live-Tracking (`eyes`/`dsn` absent; `dead_sources.φ:355`) | `data/ssd.jpl.nasa.gov/ephemeris_juice.bin`; `data/flyby2/gate-juice-2026-09-28.json` | **pending** — Gate-JSON `delta_km`/`threshold_km`/`verdict` pending; `edition_rift_km=0.16847` | nach Perigäum `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate` (CI) |
| Ⅲ Koronale Heizung | `corona_conditional_probe.rs` · `solar_causal_graph_probe.rs` | GOES X `:152/:812` · EUV `:417` · F10.7 `:412` · IMF `:743-745` · OMNI `:736-750` · AIA `:2320/2334/2348` · EVE `:2362-2378` | keiner der Kanäle; Riss: Probe lädt `ssd.jpl.nasa.gov`-Assets, Register führt dieselben unter `ncei`/`cdaweb` | keins gemessen | **keins** — Verdikt nur Paper `corona-heating-ladder.md:18` (family-bound) | Netloc-Riss klären; Register-Zeile |
| Ⅳ LAIC | `laic_probe.rs` · `nobel_probe_laic.rs` · Paper `laic-arrow-direction.md` v4 | USGS `:86/96/103` · INTERMAGNET `:1787/5386-5390` · Swarm `:7240` · CHAMP-TEC `:31` · Safecast `:228/235` · Kp `:1160` | Swarm-TEC (nur CHAMP); CSES (`ledger.φ:14-16` ausstehend; `wartend.φ:12` termin 2026-10-02) | `laic.bin`/`laic_champ.bin` CDN 206; `phi/pipeline/laic_harvest/` 4.1 GB | **keins** — Paper `:14/:25` Stille | 2026-10-02 CSES; Swarm-TEC messen |
| Ⅴ Technosignaturen | `lsst_anomaly_probe.rs` · `ztf_anomaly_probe.rs` · `negativ_fuzzy_probe.rs` (kein dip/FAP → Zweck-Riss) | ZTF `:9409/9416` · IRAS `:13896` · AKARI `:11148` · VSX `:10750` · GCVS `:10511` · witnesses Lasair `:70`/Fink `:10` | Gaia `bp_rp` descoped (`blocked_sources.φ:203`) | keins | **keins** — Paper `docs/paper/nadel-v-fresh-area-dip-scan.md:39/:47` (0 unexcluded, <3σ) | Zweck `negativ_fuzzy_probe` prüfen; Paper-Verdikt registrieren |
| Ⅵ Planet 9 | `kbo_compiler.rs` · `kbo_residue_probe.rs` | KBO `:10274` · MPC-Distant `:2401` · Sonden-Arcs `:15889/15896/15679/15735` · Planeten-Eph `:3419-3484` | keine | keins | **keins** — Paper `planet-nine-kbo-residue.md:18` (kein fam-Pfeil); descoped in `sensory-folge201:193-194` | descope-Träger/Register |
| Ⅶ Wurmloch | `signal_cone_audit_probe.rs`; Doc nennt `signalkegel_audit_probe` (Name-Riss) | `te.rs:96` · `membrane.rs:336` · `spatial.rs:549-554` · Probe `:34-44` | `max(0,\|Δt\|−d/c)`-Fold nicht in `src/`; Sonden-Bahnen-Kanal unverifiziert | `data/kegel_audit_voll.log` pending | **keins** — Paper `signal-cone-audit-sheet.md:14/:63` (0 honored) | Fold lokalisieren; Name-Riss klären |
| Ⅷ Dunkler Fluss | `dark_flow_probe.rs` | CMB `:9374/9383` · Abell `:10351` · MCXC `:10561` · PSZ2 `:10673` · CF4 `:10432/10442` | z≳10-Sample | kein `cosmicflows_cf4.json` im Baum | **keins** — Paper `dark-flow-sheet-8.md:14/:33` Stille | Release-Namespace-Riss (Register `tapvizier` vs Probe `ssd.jpl`) |
| Ⅸ FRB | `frb_blatt_probe.rs` · `frb_compiler.rs`; `write_blatt_pair` steht (`:295`) | `:9086-9088` · Streuung `:10497` · peak-flux `:10509` (Zensus: 10508 off-by-one) · Magnetar `:9174/9185` | kein Paar-Artefakt (glob 0); Herkunftsadresse | keins | kein frb-Verdikt; FRBCAT `dead:879`; CHIME `declined:1254` | `archive_search --verdict <frb_chime_cat1.json>`; Probe `--write` Blatt |
| Ⅹ Kugelblitz | keiner (glob 0) | keine | alle vier co-lokalisierten (em×electric×thermal×acoustic) | keins | **descoped (gemessen: → `kybernetische-astrophysik.md:294-300`; kein Probe-Artefakt, glob 0)** | — (Befund steht) |
| Ⅺ Placebo | `placebo_pair_eeg_probe.rs` · `openneuro_eeg.rs` | `harvest.φ:192` (2 Assets); `sources.φ:2447/2454` sham/verum | Gabe×HRV×Blutmarker | CDN `openneuro.org` Assets | **keins** — Verdikt nur Archiv-Handover `folge10:24-25`; kein lebendes Blatt | `gh workflow run placebo-ave-cdn.yml`; Verdikt ins Register/Blatt |
| Ⅻ Urknall | `bigbang_echo_probe.rs`; CI `bigbang-echo.yml` | CMB `:9374/9383` · CF4 `:10432/10442` · z-Code `:16-17` | PTA `declined:1402-1403`; B-Moden (0) | keins | **keins** — Sheet `big-bang-echo-sheet-12.md:14/:35/:59-60` | `gh workflow run bigbang-echo.yml` |

## Die Blätter und der Kuprat

| Blatt | Artefakt | Kanäle vorhanden | Fehlend | Register-Verdikt | Nächster Schritt |
|---|---|---|---|---|---|
| ENSO | **keiner** (kein enso-Bin; nur Reader `esacci_sst_read.rs`) | SST `:1428/1434` · Drifter `:637/643` · OOI `:1347/1352` · TAO `:774/786` | ENSO-TE-Probe; Becken-Windfeld (scatterometer absent) | MEI `declined:2865`; SOI `declined:4419/4426`; ERA5 `declined:1231`; Paar-Zuschnitt operator-gebunden (`wartend.φ:23`) | Operator-Zuschnitt; dann Probe nach `nobel_probe_corona.rs`-Muster |
| GIC/Bz | `bz_blatt_probe.rs` · `bz_retro_probe.rs` | OMNI `:575/584` · RTSW `:158/164` · BPA `:890` · FMI `:10217` · INTERMAGNET `:1787/:5386` | co-lokales Mäntsälä-dB/dt | Riss Paper `gic-causal-driver.md:405-423/:492-495`; Paper-Ref `:431` stale (`:8590`=PRES) | Mäntsälä-Kanal; Paper-Ref heilen |

| Kuprat | Artefakt | Kanäle (4, alle CDN 206) | Fehlend | Register | Nächster Schritt |
|---|---|---|---|---|---|
| Hoch-Tc | `rixs_cuprate_probe.rs` · `suprastrom_cuprate_probe.rs` · `suprastrom_form_probe.rs` · `crystal_compiler.rs` · `srd62_compiler.rs` · `cuprate-cdn.yml` · `srd62-cdn.yml` | RIXS spin (Zenodo 7286412) · RIXS charge (15179114) · EELS · SRD62 | 5. Ader NSE (privat `data/lab_a.data/SAMPLE_NSE_[redacted]/`, 13 Läufe); Kuprat-Zeugenklasse | **0 Register-Zeilen** (`sgrep kuprat\|rixs\|srd62 phi/` → 0) | Mycelium: `url`/`origin`/`compiler` + Tag-Drift `ssd.jpl.nasa.gov`; Mountain: `witness kuprat` + `format`/`field`; Operator-Wort ausstehend |

## Querschnitt

1. **Verdikt-Träger (Rats-Konsens 2026-09-29).** Für 10 der 15 Rätsel lebt das
   Verdikt im Blatt/Sheet + als Survey-Zeile (Index) — nicht in einem `phi/*.φ`-
   Register (`register_sort.rs` verlangt `url`/`ttl`; ein Verdikt hat keine Adresse).
   Der Kuprat braucht `witness kuprat` + `format`/`field` (Mountain) + `url`/
   `origin`/`compiler` (Mycelium).
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
3. **Doc-vs-Baum-Risse.** **In diesem Atom geheilt:** Ⅳ MiniSEED-Decoder-Zeile
   (`laic-arrow-direction.md:45` → „decoder built"); Ⅶ Bin-Name
   (`signal-cone-audit-sheet.md:27` → `signal_cone_audit_probe`); Ⅸ peak-flux off-by-one
   (im Vorläufer-Zensus, `consumed`; hier `:10509` korrekt). **Offen:** Ⅳ
   `laic_champ.bin` Paper-Größe weicht von der Messung ab; Ⅹ descope ohne Befund;
   Ⅴ `negativ_fuzzy_probe` trägt keinen Dip/FAP; ENSO Doku-Zeilen
   (`ein-blatt-papier.md:76`) divergieren von `sources.φ`.
4. **Operator-gebunden.** ENSO-Zuschnitt (`wartend.φ:23`), Kuprat-Zeugenart (Operator-Wort),
   NSE-Redistribution (`state/mail/`).
5. **Fremde Feder (Mountain).** `format`/`field`/`ttl` für Kuprat; `witness kuprat`.
