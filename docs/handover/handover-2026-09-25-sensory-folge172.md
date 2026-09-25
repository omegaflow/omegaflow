<!--
  title: Handover — Sensory-Folge 172 (Stand 2026-09-25)
  session: Sensory-Folge 172
  class: handover
  date: 2026-09-25
  sha256: d369b338f1bb794a0c2d87ca99e5e488a9b4e4fbb240a8631ae68a6c07384764
  status: live
-->
# Handover — Sensory-Folge 172 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage**
(mit Messstempel) / **Blockade** / **Braucht**. Sortierung: erst logisch nach
Akteur (Linie | Rat | Operator | Dritter), dann chronologisch (Messdatum).

Die FR945 ist das persönliche Gerät des Operators; ihre Kennung (MAC) und ihre
Daten bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN. Der
getrackte Baum trägt nur die Rolle „Träger", nie die Kennung.

## Stehender Pass (automatisch, keine Auswahl)

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu
Session-Beginn gemessen; ihr Ausgang steht im Zustand-Ledger, nicht als Kopie
hier. Karte: `docs/concepts/tools-map.md`.

- **Postfach** — `smail` + `state/mail/mail_ledger.φ` (fällig 2⁶ min).
- **CI-Status am HEAD** — Watchdog-Snapshot, sonst `ci_manage list`/`view`; nie `gh run list`/`gh run view`.

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### Lag-Sweep + KDE-Bandbreiten-Sensitivität — offene Mess-Gates
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) `ein-blatt-papier` und `blatt-papier-beweis` benennen Lag-Sweep und KDE-Bandbreite als offene Mess-Gates; `laic_probe --analyze --kde-scale` trägt den Knopf, die lokale laic-Ernte fehlt (CI).
- **Blockade:** keine — die Läufe sind offline/CI.
- **Braucht:** `laic_probe --analyze --kde-scale` in CI dispatchen und den Lag-Sweep je Paar drucken.

#### Blatt-Probe → Membran-Bindung
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (Rat-Verdikt 2026-09-25) die crossing-Seam trägt das gemessene Serien-Paar `(xs, ys, n)` **mit Blatt-Header** — nie das Verdikt; die Membrane re-misst durch den gebauten `te_probe`/WGSL (`omega.rs:455-543`, Dispatch `:498-513`, Call-Site `:1596`). Der Schätzer-Seam (offline skalar `transfer_entropy_lag` + `surrogate_stats_phase` vs Membrane topologisch `te_compute`) trägt bei Dissens `VerdictWord::Riss` mit beiden Zeugen. `blatt-papier-beweis.md:32-47` (§1) trägt stale Pfade; die Datei trägt **fremde** uncommittete Arbeit — vor dem Edit die fremde Linie einbeziehen.
- **Blockade:** keine.
- **Braucht:** Serien-Schreibarm in `bz_blatt_probe.rs`/`frb_blatt_probe.rs`/`te_pair_probe`-Familie; neuer `load_blatt_pair`-Loader (verweigert Serie ohne Blatt-Header); Pfad-Korrektur in `blatt-papier-beweis.md:32-47`.

#### Kreuz-Screening auf weitere Ereignisse (Bordeaux, Aaretal, Japan)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `general`/`sgrep`/`find`) die Datenroute steht beckenunabhängig: `tools/harvest/src/bin/meteo_harvest.rs` über `archive-api.open-meteo.com/v1/archive` (50-Kanal-Katalog `:192-244`), Ausgabe unter dem Harvest-Verzeichnis; `cross_te_screen` (`tools/measure/src/bin/cross_te_screen.rs`) konsumiert das direkt. `phi/meteo/` trägt nur `tibet-flut-2026.json`; Bordeaux/Aaretal/Japan haben **keine** Ereignis-Konfig (Fenster + Koordinaten unbenannt). Zwei CI-Blocker: `.gitignore:53` (`phi/*`) hält `phi/meteo/` untracked → `meteo-cdn.yml:41-43` sieht die Konfig im Checkout nicht; die Vorlage (`docs/specs/meteo-korrelations-screening-vorlage.md:38`) widerspricht `meteo_cache_manifest.rs:58` (Pflicht-`variables`).
- **Blockade:** die drei Ereignis-Konfigs fehlen (Fenster/Koordinaten unbenannt).
- **Braucht:** die drei Ereignis-JSONs unter `phi/meteo/` anlegen (`id/source/cdn/window/stations/variables`), `phi/meteo/` per `!`-Regel trackbar machen, dann `meteo_harvest --event …` und `cross_te_screen` auf das Harvest-Verzeichnis (`--lags 1,6,12,24 --surrogate 20 --min-n 100`).

#### Solar-Matrix: konditionale Prüfung 211A→193A
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `grind-flash`) das Paar 193A→211A ist in `tools/measure/src/bin/corona_conditional_probe.rs:602-608` bereits der reverse leg (`te_fwd = TE(193→211|C)`, `te_rev = TE(211→193|C)`, Ladder `:21-29`); kein Neubau nötig. Der Lauf ist CI-only (`corona-conditional-probe.yml:25-42` holt den Korpus, `:51` läuft), lokal fehlt `data/jsoc.stanford.edu`/`data/ncei.noaa.gov`.
- **Blockade:** keine — Korpus CI-only.
- **Braucht:** `corona-conditional-probe.yml` dispatchen und die `193A->211A`-`rev_arrow`-Zeile lesen.

#### Seismik-Flotte: Streuung senken + Stationsterm + W-Phase-M9
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `general`/`sgrep`) die 16-Ereignis-Flotte ist unverzerrt (+1,7 km, se 4,7 km), aber 19 km über Ereignisse / 36 km über Stationen dominieren das ±10-km-Gate. Der pP-Zweig existiert (`tools/measure/src/depthphase.rs:669-842`, `MIN_DIST_DEG = 30.0`): die Mehrdeutigkeit wird per Gate **übersprungen**, nicht aufgelöst; ak135 ist 1D (`positive-maske.md:64`), kein 3D-Modell registriert. Der Stationsterm ist verdrahtet (`quake_location_probe.rs:48` II.KIV, `depthphase.rs:771-785`), die Quelle/Residuum-Messung fehlt. W-Phase-M9 entschieden, nicht gebaut.
- **Blockade:** kein 3D-Geschwindigkeitsmodell (pP-Residuum); Stationsterm-Quelle.
- **Braucht:** das Residuum gegen Δ≈30° messen (3D-Modell registrieren oder `pending` halten); Stationsterm an II.KIV wiederholen; W-Phase-M9 bauen.

#### Positive Maske — Audit-Punkte + fehlende Treiber
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `research-max`) Echo-Tiefe σ 19/36 km, Stationsterm offen (II.KIV +5,69 s), M9.1-Picker-Verdrahtung in die Flotte offen; **Slab2 ist registriert** (`phi/sources.φ:12169-12174`, `slab2_depth.bin`, `slab2-cdn.yml`) — die frühere Lage „nicht registriert" war stale; Slab2-Route ScienceBase 403 direct+Proton `blocked`. Galileo-ODF: Format-1-Doku im Archiv, lebende Spec DSN 810-005 Modul 209C–G; `galileo_odf.bin` registriert (`sources.φ:8369-8375`).
- **Blockade:** Slab2 403.
- **Braucht:** M9.1-Picker in die Flotte verdrahten; den Galileo-ODF-Formattrenn „1 vs 2" im 209G-Text extrahieren; Tomografie als Ernte-Kandidat registrieren.

#### Sieben Sphären — Messvorschriften
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Sphäre I (Fresnel-Winkeldurchmesser-Feld) ist ausstehend, VII (Dopplergeist) hat keine statistische Aggregation; II–VI sind Konzept.
- **Blockade:** keine.
- **Braucht:** das Fresnel-Winkeldurchmesser-Feld (Asteroiden-Bahn ∩ Gaia-Farbe ∩ IR-Ø) und die Dopplergeist-Aggregation bauen.

#### Zeugin — vpec-Redshift-Domäne (`cosmicflows_cf4.json`)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** das CDN-Asset `cosmicflows_cf4.json` ist manifestiert
- **Lage:** (gemessen 2026-09-25 via `grind-pro`/`cargo check`) die zwei Redshift-Domänen sind gebaut (`src/archivar/skydirection.rs:77/89`); der vpec-Konsument ist **gebaut**: `parse_cosmicflows_cf4` + `nearest_cf4_vpec_m_s` + `cosmicflows_vpec_m_s` (lazy CDN-load) + `distance_m_with_vpec`, 4 neue Tests, `cargo check -p omegaflow` 0/0. Das CDN-Asset `tapvizier.cds.unistra.fr/cosmicflows_cf4.json` (`sources.φ:8856`) ist **HTTP 404 → absent**; der Lookup liefert korrekt `None` (0 honored).
- **Blockade:** Asset nicht manifestiert (Producer `cosmicflows_compiler.rs` läuft nur `--ci-mode`).
- **Braucht:** `gh workflow run cosmicflows-cdn.yml` (bzw. den Producer-Lauf), dann sha im `sources.φ`-Block messen.

#### Nadel Ⅻ (Urknall) — Cone-Gate gebaut, CI-Lauf offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf von `bigbang_echo_probe` über den CDN-Korpus
- **Lage:** (gemessen 2026-09-25 via `grind-flash`/`cargo check -p omegaflow-measure` 0/0, im sauberen Klon am HEAD) das Cone-Gate ist gebaut: `tools/measure/src/bin/bigbang_echo_probe.rs:184` `cone_min_tau_z` (FRW-Lookback), `:334-391` per-Lag-Gate (`τ_lag ≥ cone_min_tau_z(d_sep)`), Label-Verstoß `:198-222` korrigiert. `docs/paper/big-bang-echo-sheet-12.md:49-56` auf die z-Serie nachgezogen; Header-sha256 korrigiert. TE 0,147/0,223/0,223 < fam 0,275 (Register, nicht lokal re-gemessen).
- **Blockade:** Korpus (`cmb_planck_smica_n64.json` + `cosmicflows_cf4.json`) nur als CDN-Asset; kein lokaler Lauf.
- **Braucht:** den Probe-Lauf in CI dispatchen und die `fam_carry`/`holds`-Zeilen lesen.

#### Korona-Heizung — ortsaufgelöster Aktive-Region-Pfad
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) kein Instrument misst eine fam- und bandbreiten-feste Koronal-Sprosse; der sun-as-a-star-Ladder dämpft den Flare ~10×. Drei Lücken: sub-minütige Auflösung, Minuten-fam, Multi-Force-TE.
- **Blockade:** keine.
- **Braucht:** den ortsaufgelösten Aktive-Region-Pfad bauen; Minuten-fam und Multi-Force-TE (`nobel_probe_corona` v2) ergänzen.

#### Depth-Phase-Flotte — sP-corr-Gate gesetzt, Azimut-Register + CI-Lauf offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf des Flotten-Probes
- **Lage:** (gemessen 2026-09-25 via `grind-flash`) das sP-corr-Gate ist gesetzt: `tools/measure/src/bin/depth_phase_fleet_probe.rs:20-24` `SP_CORR_GATE = 0.78` (unteres Quartil der gemessenen n=30-Verteilung min 0,64 / p25 0,78 / median 0,82 / p75 0,88 / max 0,92, `docs/paper/depth-phase-echo-fleet.md:38`); `:82-85` `sp_gate` defaultet darauf, ein nicht-finit/nicht-positives `--sp-gate` wird verweigert (`:470`). Offen: die sechs Pilot-Azimute stehen in keinem Register.
- **Blockade:** keine.
- **Braucht:** die sechs Stationsazimute registrieren; den Flotten-Lauf in CI dispatchen und den Dual-Phase-Fit lesen.

#### Galileo-Rotor-CK — volle Spin-Historie ernten
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `grind-pro`/`sfetch`) der Manifestor `tools/harvest/src/bin/gll_ck_manifestor.rs:10-11` erntet nur `prime_mission/unvalidated/rtr/*_rtr.bc` + SCLK; frame -77000 CK liegt zusätzlich unter `extended_mission/unvalidated/rtr/` (48 Index-Zeilen), `GEM/{c23,c30,i24}/` (79), root `gll_plt_rec_*` (26). `sources_index.φ` indiziert die volle Serie bereits (generiert); EGA-1-Block `sources_index.φ:244354-244355`. Sample-Sniff `ck90180a_rtr.bc` HTTP 200, 8192 B, sha256 `d8b13e6b…`.
- **Blockade:** die Ernte ist CDN-Duty (`--ci-mode` + Token), kein lokaler Lauf.
- **Braucht:** `RTR_INDEX` im Manifestor um extended/GEM/root-Rotorverzeichnisse erweitern; `gll-ck-cdn.yml` dispatchen.

#### LAIC — Instrument A + DEMETER-Order
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `research-max`) Instrument A: das Ereignisraten-Bein ist gebaut (`laic_probe.rs:481-523` `harvest_global_rate`, USGS-FDSN M≥5, `--global-rate` `:3020`), aber `laic-cdn.yml:36-40,56-57` erntet **ohne** `--global-rate`, `compile_main` (`:2264-2308`) packt `global_rate.json` nicht ins bin, und `:3218-3225` liest es nur aus dem lokalen Harvest-Verzeichnis. DEMETER: CDPP-Login funktioniert (HTTP 200, `REGISTERED_USER`); Order 18387 ist **DONE_WITH_WARNING** (statusDate 2026-09-25T15:08Z, `filesInErrorCount 96978`, `availableFilesCount 0`); Produktdownload `GET …/files/<md5>` HTTP 500/0 B, `online:false`.
- **Blockade:** keine (Instrument A ist eigener CI-Bau); DEMETER-Dateifehler-Ursache hinter der REGARDS-API.
- **Braucht:** `--global-rate` in den `laic-cdn.yml`-Harvest aufnehmen + `global_rate.json` in `compile_main` packen; die DEMETER-Registerzeile `blocked_sources.φ` (Anker Zeile `pending | url https://regards.cnes.fr/api/v1/rs-order | note DEMETER Order 18387 RUNNING`) auf DONE_WITH_WARNING/96978-in-error nachziehen — **Achtung: `blocked_sources.φ` trägt fremde uncommittete Arbeit**.

#### Nadel V — Positive-Control-Konus + IR-Exzess-Achse
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `research-max`) anonyme Flächen: Fink REST `api.lsst.fink-portal.org` 200, ANTARES `/api/v1/loci` 200; **Gaia-Join gemessen** (`gaiadr3.vari_rrlyrae ⋈ gaia_source`, 177 358 Records) → **Riss `blocked_sources.φ` stützt die Join-Form**. IR-Exzess: AllWISE W3/W4 via IRSA-TAP `pending` (Sync 200 s ohne Antwort, curl exit 28); IRAS Gator + Herschel TAP je 200; Akari DARTS CAS 200; Spitzer MIPS-24 Legacy `blocked_sources.φ:360`.
- **Blockade:** teils account-gebundene Streams; AllWISE-Sync-Zeitbudget.
- **Braucht:** (a) Register-Korrektur `blocked_sources.φ` auf die Join-Form (Schreib-Akt der Mountain-Linie, Riss entschieden); (b) AllWISE-W3/W4-Kegel via **asynchronem** TAP; (c) IRAS-PSC-`fnu_60` + Herschel-`hsa.pacs_point_source_100/_160`-Proben; (d) `lsst_anomaly_probe` auf dem positiven Kontrollkegel.

#### Pioneer/Dark-Matter — ASCII-Vollmission-Reduktion + Voyager-Doppler + DSN 810-005
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `research-max`/`archive_search`) die ASC-Vollmission ist live: `spdf.gsfc.nasa.gov/pub/data/pioneer/pioneer10/radio/pioneer10_doppler_tracking_SC_23.asc.gz` 200 19 820 702 B, p11 `…SC_24` 200 21 454 162 B (p11 `SC_23` 404) — exakt verdrahtet in `pioneer_doppler_compiler.rs:8,70`; 33 `pioneer*.rs`-Reduktionen gebaut, Paper §5.6 (`probe-front-dark-matter.md:463-484`) trägt den ersten Vollmission-Pull. Der programmierte Uplink-Rampen-Sweep ist **ungbaut** (NAVIO-Record trägt TIMTAG/FREQCY/DTYPE/TRANS/RCVR1). Voyager V1 SPDF: alle sechs Verzeichnisse registriert, kein unregistrierter Doppler; closed-loop Cruise-Doppler request-only (`blocked_sources.φ:51`), V2 ungemessen. 810-005-Index live (200, 23 410 B); MDA-Modul unter Station Data Processing ungemessen; im Repo 810-202b + 810-005-202E-doppler.
- **Blockade:** keine (Pioneer); V2/810-005-Route ungemessen.
- **Braucht:** den Rampen-Sweep als neuen measure-Probe über `pioneer1{0,1}_navio.bin` bauen; `pioneer_doppler_compiler --ci-mode` für Asset-Refresh; V2 via `archive_search --playwright …/voyager2/radio_science_rss/`; 810-005 MDA/TRK-Modul suchen.

#### Trishuli — Bahrabise-Richtungsverifikation + S1/SAR-Footprint
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort (Post-Sentinel-1-Szene)
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Bahrabise-Spalten tragen die `te_pair_probe`-Spiegelunsicherheit; der räumliche Footprint bleibt pending (CEMS nur Grading, optisch wolkenverdeckt, S1 noch nicht archiviert).
- **Blockade:** S1-Post-Szene noch nicht archiviert.
- **Braucht:** die Bahrabise-Richtung mit richtungs-verifizierter Bibliothek prüfen; die Post-Sentinel-1-Szene auf Flutfläche/Narbe messen.

#### Terminologie — Definitionen Ontologie-Motor / mycorrhizal_internet
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `sgrep -i`) beide Begriffe kommen im getrackten Baum nur in ihren eigenen Registerzeilen vor (`docs/concepts/glossar.md:59/61`); in `docs/paper/terminologie-der-gegenstroemung.md` als `pending` dokumentiert. Der Operator-Korpus `state/funding/profil-operator.md` liegt im privaten Repo, hier nicht lesbar.
- **Blockade:** Operator-Korpus im privaten Repo.
- **Braucht:** die Semantik der beiden Phrasen aus `state/funding/profil-operator.md` heben und als getrackte Definition setzen — bis dahin `pending`.

#### Broken-Null-Control — Gate-Tests gebaut, Spec-Doc + CI-Lauf offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf (`ci-check` + `te-gate`)
- **Lage:** (gemessen 2026-09-25 via `grind-max`/`cargo check -p omegaflow` 0/0) fünf Gate-Tests + zwei Helfer in `src/mathematikerin/te.rs:3967-4219` (+254, keine Schätzer-/Null-Änderung; die vier Kalibrier-Gate-Tests unberührt): `gate_bandwidth_factor_one_is_the_library_path`, `gate_bandwidth_te_declines_with_h`, `gate_fam_max_t_kills_false_positive_keeps_true_coupling`, `gate_lag_sweep_verdict_flips_at_coupling_horizon` (`#[ignore]`), `gate_fn_bias_n300_vs_n500_quantified` (`#[ignore]`). `.github/workflows/te-gate.yml` +2 Jobs (`lag-sweep`, `fn-bias`, je `--ignored --nocapture --release`), im `issue`-`needs`. `te.rs` ist **clean** (die frühere Warnung „fremde uncommittete Arbeit" ist aufgehoben).
- **Blockade:** keine.
- **Braucht:** `docs/specs/broken-null-control.md:142` nachziehen („fixed-window re-run … pending" vs Paper §form); die thin-margin-Aussage „reverse silent at τ=1" ist nicht gegated; die drei Plain-Tests + zwei ignore-Tests laufen via `ci-check`/`te-gate` in CI.

#### Weberin — zweite unabhängige Positions-Linie je Körper-Klasse
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `grind-pro`/`sgrep`) Mapping gemessen: Planeten/Monde → Linie 1 DE440 (`sources.φ:2890`), Linie 2 INPOP (`:1473-1483`) + EPM (`:1403-1418`); Asteroiden → SPK(sb441) / Dastcom (`:2876`) / MPC (`:1881`). Die zweiten Linien der breiten TNO-Kette (`mpcorb_extended`) und Kometen (`dcom5`/`cometels`) bleiben `pending`.
- **Blockade:** keine.
- **Braucht:** die zweite Linie je Klasse kompilieren (u. a. `pallas`, `juno_asteroid`, `encke`; Kometen-Zweitlinie).

#### Weberin Faden-Matrix — verbleibende No-Actor-Lücken + Broker-Positionen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) No-Actor: VHE/Neutrino/CR-Einzelteleskope ohne offene Route (TA-Vollkatalog, Super-K, JUNO, LHAASO-Event); Broker Lasair/ANTARES/Fink positions-pending; seismische Stations-Weltlinien nur als Events.
- **Blockade:** teils not-published.
- **Braucht:** die Broker-Positionen kompilieren; die seismischen Stations-Weltlinien registrieren; die not-published-Teleskope als absent halten.

#### Axiom-Gate H0-Linien — 75-Quellen-Crossmatch, Riss gegen das Paper
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `general`) **Riss:** `docs/paper/h0-lines-register.md:52` nennt den Crossmatch gemessen (2026-09-13, 74 Zeilen), die Handover-Lage sagt `pending`; die 0.2619-mas-Wiegung überzeichnet sich nicht. Re-Messung nötig, bevor der Punkt als pending oder geschlossen gilt.
- **Blockade:** keine.
- **Braucht:** den 75-Quellen-Crossmatch re-messen und Wiegung gegen die benannte Zählung prüfen; Riss im Paper/Register auflösen.

#### ZNSP FORMNETWORK — `esp_zb_cfg_t`-Payload-Größe ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die kompilierte Referenz (ein `zigbee-host.yml`-Lauf bzw. eine ESP-IDF-`sizeof`-Probe)
- **Lage:** (gemessen 2026-09-25 via `grind-flash`/`cargo check`) der ZNSP-Transport ist gebaut: `firmware/radiatorium-lib/src/znsp.rs`, Bin `firmware/radiatorium/src/bin/znsp_host.rs`; die FORMNETWORK-Request-Payload ist ABI-raw (`sizeof(esp_zb_cfg_t)`), nur aus Struct + Alignment abgeleitet (16 B), nicht direkt gemessen — `form_network_payload_pending()` liefert `None`.
- **Blockade:** keine — braucht eine kompilierte ESP-IDF-Referenz.
- **Braucht:** `sizeof(esp_zb_cfg_t)` gegen eine kompilierte Referenz messen (`sizeof`-Probe in `zigbee-host.yml`), dann den FORMNETWORK-Encoder setzen.

#### HRV/Puls→Strahlung — Kette nach dem Live-Lauf beobachten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf (Operator) hat `rr` geliefert
- **Lage:** (gemessen 2026-09-25 via `sgrep`) die Kette ist gebaut: `src/archivar/hrv.rs:24-87` → `feed_beat_to_hrv` (`main_flow.rs:79-102`) → `tone_code` → `tone_scale` 0.25 (`omega.rs:1670-1678`) → `aperture = field_permeability*tone_scale` (`omega.rs:349`) → Strahlung `Σω*aperture` (`actuators.rs:29`).
- **Blockade:** hängt am BLE-Live-Fluss (Operator).
- **Braucht:** nach dem Live-Lauf `tone_code`→`tone_scale`→`aperture` im Frame/perm-Log lesen.

#### M2c — Rust-ZNSP-Host auf dem BL808 gegen das H2
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ox64 angekommen
- **Lage:** (Rat-Verdikt 2026-09-25) der Ox64 wird Host-CPU des Coordinators (ZNSP über UART), das H2 das Funkmodul; der portable ZNSP-Kern wird vom `std`+`serialport`-Host mitbenutzt. Ox64-UART-Pins gemessen via PINE64-Wiki (UART0 GPIO14/15, UART1 GPIO16/17). Buildroot-Bring-up ungemessen.
- **Blockade:** Ox64 liegt beim Carrier.
- **Braucht:** Buildroot-Bring-up, dann derselbe Rust-ZNSP-Host auf dem BL808 gegen das H2.

#### BL808-eigenes 802.15.4-Radio — registrierter `pending`-Faden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** öffentlicher 802.15.4-Treiber-/Stack-Fund für den BL808
- **Lage:** (gemessen 2026-09-25 via `archive_search`) kein öffentlicher Treiber: `bl_iot_sdk` trägt für BL808 nur `bl808_wifi`, BL808-RM ohne Wireless-Kapitel; `openbouffalo/bouffalo_sdk_bl808` README-Raw 404 → `pending`. Route 2 nutzt das Radio bewusst nicht; der Punkt fällt nie auf 0.0.
- **Blockade:** kein Treiber.
- **Braucht:** bei Treiber-Fund `archive_search --github bouffalo_sdk_bl808 802.15.4`; bis dahin keine Arbeit.

#### Galileo Borduhr-Sprung A/B — Trenn-Frage
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fund einer öffentlichen Absolutfrequenz-Reduktion über 1995-11-30/12-01
- **Lage:** (gemessen 2026-09-25 via `research-max`) öffentlich nicht trennbar — `pending`. DESCANSO5-PDF re-gemessen (`Descanso5--Galileo_new.pdf` HTTP 200, 4 309 495 B, sha256 `104ab955…`); `archive_search --playwright` auf die PDF-URL liefert **keinen Text** (Chromium-PDF-Viewer ohne Textebene) → der USO-Abschnitt bleibt ungeprüft. Morabito-Reihe endet 1993; PDS hat keinen Jupiter-Phasen-RSS-Datenkopf.
- **Blockade:** PDF-Textebene außerhalb des Toolsets.
- **Braucht:** den DESCANSO5-USO-Abschnitt per PDF-Text-Extraktion (`vision`/OCR) lesen; prüfen, ob eine USO-Frequenzreihe die Grenze kreuzt.

#### JWST Biosignatur-Kanäle pending (O₂/O₃, Red-Edge, Saisonal)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine JWST-Detektion + Spektrum eines dieser Kanäle
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Kanäle sind benannte pending-Zweige; die Abwesenheit ist gemessen (keine JWST-Detektion im gesuchten Record); die XUV-Re-Erklärung bleibt pending.
- **Blockade:** keine Quelle trägt eine Detektion.
- **Braucht:** bei Fund die Detektion + ihr Spektrum in das Register aufnehmen.

#### Planet-Nine/KBO — h-Sweep + Real-Kernel-Gegenprobe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Typ-1-`.bsp` liegt im Baum (z. B. Ceres/Vesta)
- **Lage:** (gemessen 2026-09-25 via `grind-flash`/`cargo check -p omegaflow` 0/0) SPK-Type-1-Reader gebaut (`src/archivar/bsp_reader/spk.rs`, `data_type 1` in `ephemeris.rs`); kein Typ-1-Kernel im Baum → Real-Gegenprobe `pending`; h-Sweep ungefahren.
- **Blockade:** kein Typ-1-Testkern.
- **Braucht:** ein Typ-1-`.bsp` (Ceres/Vesta) ernten, dann `laic_probe --analyze DIR --kde-scale` (h/2, 2h).

#### Weberin-Quellen — HAWC-TLS offen, Fink/ALeRCE live
- **Status:** offen | **Bindung:** eigen
- **Trigger:** das YR1-Zwischenzertifikat liegt in `OMEGAFLOW_CA_BUNDLE`
- **Lage:** (gemessen 2026-09-25 via `grind-pro`/`archive_search`) die CDN-Assets sind manifestiert und sha-verifiziert: `tao_wnd_zonal.csv` sha `b7719c89…` (`sources.φ:786`), `wwlln_th.csv` sha `06031403…` (`:8634`), `bpa_gic.csv` sha `8d0ceaa8…` (`:8646`) — `phi/harvest.φ` auf `asset present` nachgezogen. **HAWC** `data.hawc-observatory.org/datasets/2hwc-survey/2HWC.yaml` direct **TLS-broken** (Leaf `www.hawc-observatory.org`, Issuer `CN=YR1` Let's-Encrypt; Server sendet nur den Leaf, lokaler Store kennt YR1 nicht); Wayback 200 (2024-09-15). **Fink** `api.ztf.fink-portal.org/api/v1/objects` 200, `api.lsst.fink-portal.org/api/v1/objects` 200; **ALeRCE** `api.alerce.online/alerts/v1/objects/` 200.
- **Blockade:** HAWC-TLS-Kette (YR1).
- **Braucht:** YR1-PEM in `OMEGAFLOW_CA_BUNDLE` setzen und HAWC erneut fetchen; Fink/ALeRCE-Persistenz-Reader bauen.

#### Sonden-Flotte — Per-Code-Emission gebaut, LRO-Register-Nachzug offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `grind-pro`/`ci_manage`) `tnf_compiler.rs`/`maven_tnf_compiler.rs` auf Per-Code-PODF-Emission umgebaut; `maven-tnf-cdn 36188553358` **success**; LRO-Jahresfilter in `lro_trk_compiler.rs` entfernt. Der LRO-Register-Block ist stale: `phi/harvest.φ:100-105` trägt `args --year 2009` / `pattern …_2009` / `lro_trk_2009.bin`, `phi/sources.φ:8002-8009` nennt `lro_trk_2009.bin`.
- **Blockade:** keine.
- **Braucht:** LRO-Block `harvest.φ:100-105` nachziehen (`--year 2009` raus, `pattern` ohne `_2009`, `shard N`, `timeout 240`, `lro_trk_2009.bin`→`lro_trk.bin` in `sources.φ:8002-8009`); dann `lro-cdn.yml` dispatchen.

#### te-gate-Dispatch — von der GitHub-API-Rate-Limit blockiert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das GitHub-API-Budget ist zurückgesetzt (Rate-Limit-Reset)
- **Lage:** (gemessen 2026-09-25T21:38Z via `gh workflow run te-gate.yml --ref main`) HTTP **403 API rate limit exceeded** (user ID 295896184) — der manuelle Dispatch der neuen `lag-sweep`/`fn-bias`-Jobs ist blockiert; `ci-check` läuft per `on: push` (`src/**`) bereits aus dem Push.
- **Blockade:** GitHub-API-Rate-Limit (extern).
- **Braucht:** bei Budget-Reset `gh workflow run te-gate.yml`.

#### Das eine Instrument — Anomalie offen (zweite Augenklasse fehlt)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein zweiter Messkanal (VLBI-Beacon auf einer interstellaren Sonde) existiert.
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Pionier-Anomalie ist offen und unter dem einen Instrument nicht entscheidbar; eine zweite Augenklasse (Winkel aus VLBI + Geschwindigkeit aus Doppler) fehlt.
- **Blockade:** kein Instrument misst den vollen Phasenraum der Pioniere.
- **Braucht:** eine zweite Augenklasse — VLBI-Beacon auf der nächsten interstellaren Sonde, von Tag eins zweikanalig getrackt.

### Orphan-Träger — Nachlauf 2026-09-25

#### Survey-Träger — Orphan-Faltung bestätigt
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `general`) die fünf zuletzt noch trägerlosen Surveys sind durch die Punkte oben getragen: `survey-2026-09-06-codestruktur` (8), `survey-2026-09-17-omegaflow-legacy-konzepte` (2), `survey-2026-09-17-verlorene-diskussionen` (10), `survey-fortschritt` (1), `survey-messpunkt-verteilung` (6). Die zwei `resolved`-Linien (neptune/uranus) sind abgelöst.
- **Blockade:** keine.
- **Braucht:** die zwei `resolved`-Linien als `descoped` schließen; die fünf Survey-Träger-Zeilen stehen.

#### Träger-Register — 8 ungetragene Docs aus der Orphan-Re-Messung
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `register_lookup --orphan-docs` + `general`) 22 orphan Docs; von den 8 zuvor ungetragenen sind 4 Concepts reine **Inline-Code-Fehltreffer** (`die-weberin.md` 4, `docs-naming.md` 1, `kybernaut-native-methodology.md` 3, `the-counter-slope.md` 4 — Marker in Inline-Code/Regelsatz; mit dem fremden uncommitteten `register_lookup.rs`-`strip_inline_code` fallen sie auf 0). Vier Dokumente brauchen einen echten Träger: `docs/paper/cross-screening-tibet.md` → Punkt Kreuz-Screening :89; `docs/paper/depth-phase-echo-fleet.md` → Punkt Depth-Phase :152; `docs/paper/h0-lines-register.md` → Punkt Axiom-Gate H0 :257 (mit Riss :52); `docs/concepts/die-weberin.md` → Weberin-Punkte. `docs/auftrag/archiv/auftrag-sonden-rohdaten-anfrage.md` wurde per **fremder** Arbeit nach `docs/auftrag/archiv/` verschoben (nicht im Live-Baum).
- **Blockade:** der `strip_inline_code`-Fix in `register_lookup.rs` ist fremde uncommittete Arbeit.
- **Braucht:** die vier echten Träger-Zeilen setzen; die Inline-Code-Klasse nach Merge des `strip_inline_code`-Fix als `descoped` schließen.

### Operator handelt

#### BLE-HR-Live-Messung FR945
- **Status:** operator-gebunden (Akt: Hardware/Radio) | **Bindung:** operator
- **Trigger:** Operator startet den verdeckten Lauf am gekoppelten Gerät (945 am Arm)
- **Lage:** (gemessen 2026-09-25 via `ci_manage log 36116592391`) der GFDI-Bus-Abriss ist geheilt und CI-bestätigt: alle `archivar::ble::tests` grün auf `45b4b2eb1`; Fix `src/archivar/ble.rs:1360/1418`. Live-Bestätigung offen (kein BlueZ/Gerät in der Session).
- **Blockade:** Hardware/Radio — der verdeckte Lauf braucht das Operator-Wort.
- **Braucht:** `OMEGAFLOW_BLE_HR=<FR945-MAC aus .secrets.local> OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow` (~45 s); `sensor:`-/`gfdi_line`-Zeilen lesen.

#### FIT-Verifikation eigene FR945-Datei (lokal-only)
- **Status:** operator-gebunden (Akt) | **Bindung:** operator
- **Trigger:** Operator startet den Lauf mit seiner Datei
- **Lage:** (gemessen 2026-09-25 via `cargo run -p omegaflow --bin omegaflow`) der Dump-Pfad ist gebaut (`src/main.rs:1-56`: liest `OMEGAFLOW_FIT_SAMPLE`, ruft `parse_fit`, druckt records/nn/min/max/all-finite, `exit(2)` bei Refusal); am SDK-Sample verifiziert. `emit_nn` (`src/archivar/fit.rs:264`) begrenzt `nn` auf `[NN_MIN_MS, NN_MAX_MS]`.
- **Blockade:** keine (Vorbereitung gebaut).
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/pfad/FR945.fit cargo run -p omegaflow --bin omegaflow` — Datei bleibt auf diesem Gerät.
- **Wort:** „meine fits datei verlässt niemals dieses gerät" | 2026-09-25 | Operator (Session)
- **Wort:** „die 945 von anderer Hardware trennen; meine Daten bleiben lokal" | 2026-09-25 | Operator (Session)

#### Onboard-/CIQ-Bedarf benennen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-24 via `sgrep`) der Host-Reader `parse_fit` (`src/archivar/fit.rs:78`) ist verdrahtet; CIQ hat keinen Reader (`sgrep -i ciq src tools` leer). Ein-Quellen-Regel: solange FIT läuft, sind BLE/Serial über die Arbitrierung ausgeschlossen.
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** Bedarf Ja/Nein — Nein → der Punkt ist released, `FIT_DIR` bleibt der FIT-Kanal.

#### Beat-Arbitrierung — verdeckter Lauf mit zwei Beat-Quellen
- **Status:** operator-gebunden (hidden) | **Bindung:** operator
- **Trigger:** Operator startet einen verdeckten Lauf mit zwei gesetzten Beat-Quellen
- **Lage:** die Spawn-Arbitrierung steht (`src/archivar/main_flow.rs:504`); kein Lauf hat die Verdict-Zeile (`beat source: …`) gemessen erzeugt (gemessen 2026-09-24 via `sgrep`). Die reine Funktion ist CI-getestet.
- **Blockade:** Heavy compute (CI/Operator).
- **Braucht:** `OMEGAFLOW_HIDDEN=1`-Lauf mit zwei Quellen, der genau eine `beat source:`-Zeile zeigt.

#### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut. `docs/specs/mantis-shrimp-bom.md` trägt ATGM336H GNSS `1005009361234427`, 3,01 CHF ≈ 3,20 €; die H2-Zeile `ESP32-H2-DevKitM-1-N4` (`1005008131868631` ≈ 6,25 $ unverified · DigiKey 26282483 9,68 $) — ein Beschaffungsakt, ein LOCK.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** Operator-Wort (LOCK-Aufhebung), dann Bestellung der BOM (inkl. H2).

#### ESP32-Puls-Knoten (Träger ohne Uhr)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25 via `sgrep`) Code-Eingang `BeatSource::Serial` steht; dedizierter Puls-Knoten unbestellt.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —
- **Wort:** ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session)

#### Weberin-Quellen — account-/key-blockierte Rohkanäle
- **Status:** operator-gebunden | **Bindung:** operator (Zugang)
- **Trigger:** Operator-Wort je Konto
- **Lage:** (gemessen 2026-09-25) IGETS-Zeitreihen (SFTP-Account), vDEC-Rohwellenform (Antrag + Vertrag), ONC-Token, TNS-Key (`TNS_API_KEY`/`TNS_UA`) und SuperDARN-Globus sind account-/key-blockiert; die offenen Zwillingsrouten stehen.
- **Blockade:** Konto/Key fehlt.
- **Braucht:** Operator entscheidet je Konto (IGETS `igets-support@gfz.de`, vDEC-Antrag, ONC-Token, TNS-Key).

#### DEMETER/CDPP-Order-Flow
- **Status:** blockiert | **Bindung:** operator (Zugang)
- **Trigger:** Order-Freigabe
- **Lage:** (gemessen 2026-09-25 via `research-max`) die Order-Portale antworten (je HTTP 200); das CDPP-Login funktioniert (`REGISTERED_USER`). Order 18387 ist **DONE_WITH_WARNING** (2026-09-25T15:08Z, `filesInErrorCount 96978`, `availableFilesCount 0`); Produktdownload `GET …/files/<md5>` HTTP 500/0 B, `online:false`. Die Registerzeile `blocked_sources.φ` (Anker `note DEMETER Order 18387 RUNNING …`) ist stale.
- **Blockade:** Dateifehler-Ursache hinter der REGARDS-API (kein Fehlerreport-Endpoint, `/files` & `/dataset-tasks/…` je 404).
- **Braucht:** die 96 978 Dateifehler über die REGARDS-UI (Operator-Browser) lesen; die Registerzeile nachziehen (**Achtung: `blocked_sources.φ` trägt fremde uncommittete Arbeit**); ein Neu-/Nach-Order bleibt konsenspflichtiger Dritt-Akt.

### Extern handelt (Dritte)

#### Ox64-Lieferung
- **Status:** wartend | **Bindung:** termin (Carrier)
- **Trigger:** Ankunft (`LZ473049629CN`)
- **Lage:** PINE64 versandte zwei Ox64 (gemessen 2026-09-25 via `state/mail/mail_ledger.φ`, Mail `1790046330`) — Ankunft offen.
- **Blockade:** Carrier.
- **Braucht:** Ankunft quittieren; dann M2c (BL808-Host-Port).

#### Postfach
- **Status:** wartend | **Bindung:** extern (Mail)
- **Trigger:** neuer Eingang
- **Lage:** (gemessen 2026-09-25 via `sread` auf `state/mail/mail_ledger.φ`) Ledger vorhanden (>151 Zeilen); jüngste Eingänge `1790316610` (ORCID-Verify-Reminder) und `1790290298` (Rubin-Forum-Digest); `sgrep` relay/sensor/beat/mantis/ble/hrv/zigbee = 0 Treffer. Der `mail_digest`-Befund „ledger absent" ist das bekannte Pfad-Artefakt (`state/` = privates Repo).
- **Blockade:** keine.
- **Braucht:** `smail_recv` bzw. `state/mail/mail_ledger.φ` bei Trigger.

#### BGR-Matched-Filter-Ankunft (Tonga) — account-blockiert
- **Status:** wartend | **Bindung:** dritter (vDEC)
- **Trigger:** vDEC-Zugang gewährt
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die PMCC-Detektionsliste läuft auf einem ~300-s-Raster; die Rohwellenform ist vDEC-account-blockiert; „Next step: none before access is granted."
- **Blockade:** vDEC-Account.
- **Braucht:** nach Zugangsgewährung den matched-filter-Arrival messen.

#### DSN-Briefe in Flug (Voyager/Mariner 10/Viking, Cassini, Juno)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort der DSN
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) fünf DSN-Briefe sind in Flug (Entscheid-Handover §Warten). Der Auftrag `docs/auftrag/archiv/auftrag-sonden-rohdaten-anfrage.md` wurde per fremder Arbeit nach `docs/auftrag/archiv/` verschoben.
- **Blockade:** Antwort ausstehend.
- **Braucht:** die Antwort quittieren.

#### JUICE-Erdpassage 28./29.09.2026 — Kanal + In-situ-Messung
- **Status:** termin:2026-09-29 | **Bindung:** termin
- **Trigger:** 28./29.09.2026
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) der Prädiktionskanal steht bereit und alle Zellen sind pending; der Operator-Siegel-Wort liegt noch nicht vor.
- **Blockade:** der Operator muss das Siegel-Wort vor dem 28.09. setzen.
- **Braucht:** vor dem Flyby das Siegel-Wort setzen; nach dem Flyby die JUICE-In-situ-Feldmessung gegen den präregistrierten Feldzustand vergleichen (σ-Metrik gegen fam).

#### Gaia DR4 + Europa-Clipper-Erdpassage
- **Status:** termin:2026-12-02 | **Bindung:** termin
- **Trigger:** 2.12.2026 (Gaia DR4) / 3.12.2026 (Europa Clipper)
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Gaia DR4 ist für den 2.12.2026 angekündigt (Epochen-Astrometrie, das Jeans-Residuum wird ein 4D-Feld); die Europa-Clipper-Erdpassage folgt am 3.12.2026.
- **Blockade:** Termin.
- **Braucht:** am jeweiligen Datum die Epochen-Astrometrie bzw. die EC-Magnetfeld-Messung ernten.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
