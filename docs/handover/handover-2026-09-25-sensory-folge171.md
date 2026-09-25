<!--
  title: Handover — Sensory-Folge 171 (Stand 2026-09-25)
  session: Sensory-Folge 171
  class: handover
  date: 2026-09-25
  sha256: 54051795ed17c39ca9b66cf612ab2a475cb27e1468d37e5521907c77e7e56d31
  status: live
-->
# Handover — Sensory-Folge 171 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage**
(mit Messstempel) / **Blockade** / **Braucht**. Sortierung: **erst logisch nach
Akteur (wer handelt) — Linie | Rat | Operator | Dritter —, dann chronologisch
(Messdatum)** (Operator-Wort 2026-09-25). Ein Punkt trägt genau einen Akteur.

Die FR945 ist das persönliche Gerät des Operators; ihre Kennung (MAC) und ihre
Daten bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN. Der
getrackte Baum trägt nur die Rolle „Träger", nie die Kennung.

## Stehender Pass (automatisch, keine Auswahl)

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu
Session-Beginn gemessen; ihr Ausgang steht im Zustand-Ledger, nicht als Kopie
hier. Karte: `docs/concepts/tools-map.md`.

- **Postfach** — `smail` + `state/mail/mail_ledger.φ` (fällig 2⁶ min). Ergebnis im Ledger; hier kein Wert.
- **CI-Status am HEAD** — Watchdog-Snapshot `/tmp/opencode/ci_status.md`, sonst `ci_manage list`/`view`; nie `gh run list`/`gh run view`. Ergebnis im Ledger.

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### ZNSP FORMNETWORK — `esp_zb_cfg_t`-Payload-Größe ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die kompilierte Referenz (ein `zigbee-host.yml`-Lauf bzw. eine ESP-IDF-`sizeof`-Probe)
- **Lage:** (gemessen 2026-09-25 via `grind-flash`/`cargo check`) der ZNSP-Transport ist gebaut: `firmware/radiatorium-lib/src/znsp.rs` (SLIP, `Frame`, CRC, `cmd`, `NetworkMachine`), Bin `firmware/radiatorium/src/bin/znsp_host.rs`; die FORMNETWORK-Request-Payload ist ABI-raw (`sizeof(esp_zb_cfg_t)`), nur aus Struct + Default-Alignment abgeleitet (16 B), nicht direkt gemessen — `NetworkMachine::form_network_payload_pending()` liefert `None`.
- **Blockade:** keine — braucht eine kompilierte ESP-IDF-Referenz.
- **Braucht:** `sizeof(esp_zb_cfg_t)` gegen eine kompilierte Referenz messen (eine `sizeof`-Probe in `zigbee-host.yml` ergänzen), dann den FORMNETWORK-Encoder setzen; keine geratene Struct-Kopie.

#### HRV/Puls→Strahlung — Kette nach dem Live-Lauf beobachten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf (Operator) hat `rr` geliefert
- **Lage:** (gemessen 2026-09-25 via `sgrep`) die Kette ist gebaut: `src/archivar/hrv.rs:24-87` → `feed_beat_to_hrv` (`src/archivar/main_flow.rs:79-102`) → `tone_code` → `tone_scale` 0.25 (`src/mathematikerin/omega.rs:1670-1678`) → `aperture = field_permeability*tone_scale` (`omega.rs:349`) → Strahlung `Σω*aperture` (`src/mathematikerin/actuators.rs:29`).
- **Blockade:** hängt am BLE-Live-Fluss (Operator).
- **Braucht:** nach dem Live-Lauf `tone_code`→`tone_scale`→`aperture` im Frame / perm-Log lesen.

#### M2c — Rust-ZNSP-Host auf dem BL808 gegen das H2
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ox64 angekommen
- **Lage:** (Rat-Verdikt 2026-09-25) der Ox64 wird Host-CPU des Coordinators (ZNSP über UART), das H2 das Funkmodul; der portable ZNSP-Kern (`firmware/radiatorium-lib/src/znsp.rs`) wird vom `std`+`serialport`-Host mitbenutzt. Ox64-UART-Pins gemessen via PINE64-Wiki (UART0 GPIO14/15 = Pin 1/2, UART1 GPIO16/17 = Pin 32/31). Buildroot-Bring-up ungemessen.
- **Blockade:** Ox64 liegt beim Carrier.
- **Braucht:** Buildroot-Bring-up (Wiki-Flashing-Pfad), dann derselbe Rust-ZNSP-Host (`std` + `serialport`) auf dem BL808 gegen das H2.

#### BL808-eigenes 802.15.4-Radio — registrierter `pending`-Faden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** öffentlicher 802.15.4-Treiber-/Stack-Fund für den BL808
- **Lage:** kein öffentlicher Treiber (gemessen 2026-09-25 via `archive_search`): `bl_iot_sdk` trägt für BL808 nur `bl808_wifi` (kein 802.15.4), BL808-RM ohne Wireless-Kapitel, PAC+SVD existieren; `openbouffalo/bouffalo_sdk_bl808` (README-Raw 404, Inhalt ungemessen → `pending`). Route 2 nutzt das Radio bewusst nicht; der Punkt fällt nie auf 0.0.
- **Blockade:** kein Treiber
- **Braucht:** bei Treiber-Fund `sgrep`/`archive_search --github bouffalo_sdk_bl808 802.15.4`; bis dahin keine Arbeit.

#### ENSO-Kausalpfeil — Wind-Serie geerntet, CDN + Paar-TE offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das CDN-Asset `tao_wnd_zonal.csv` ist manifestiert (Lauf `tao-wnd-cdn.yml` success)
- **Lage:** (gemessen 2026-09-25 via `grind-pro`/`cargo run`) `tao_wnd_compiler.rs` + `tao-wnd-cdn.yml` gebaut, Register-Block in `phi/sources.φ` + `phi/harvest.φ` (committet `974700848`); Live-Lauf 1167 station-days, Station `0n140w` enthalten; Asset noch nicht am CDN (`asset fehlt`).
- **Blockade:** keine.
- **Braucht:** `gh workflow run tao-wnd-cdn.yml`, den Asset-sha im `sources.φ`-Block messen, dann `te_pair_probe --a <wu422> --b <t25> --lags 1,3,6,12,24,48 --surrogat 10` für Station `0n140w`.

#### Lag-Sweep + KDE-Bandbreiten-Sensitivität — offene Mess-Gates
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) `ein-blatt-papier` und `blatt-papier-beweis` benennen Lag-Sweep und KDE-Bandbreite als offene Mess-Gates; `laic_probe --analyze --kde-scale` trägt den Knopf, die lokale laic-Ernte fehlt (CI).
- **Blockade:** keine — die Läufe sind offline/CI.
- **Braucht:** `laic_probe --analyze --kde-scale` in CI dispatchen und den Lag-Sweep je Paar drucken.

#### Blatt-Probe → Membran-Bindung
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (Rat-Verdikt 2026-09-25) die crossing-Seam trägt das gemessene Serien-Paar `(xs, ys, n)` **mit Blatt-Header** (Fenster, Lag-Sweep, Commit-sha, Seed) — nie das Verdikt; die Membrane re-misst durch den gebauten `te_probe`/WGSL (`omega.rs:455-543`, Dispatch `:498-513`, Call-Site `:1596`), kein Neubau. Der Schätzer-Seam (offline skalar `transfer_entropy_lag` + `surrogate_stats_phase` vs Membrane topologisch `te_compute`) trägt bei Dissens `VerdictWord::Riss` mit beiden Zeugen, nie geglättet. `blatt-papier-beweis.md:32-47` (§1) trägt stale Pfade.
- **Blockade:** keine.
- **Braucht:** Serien-Schreibarm in `bz_blatt_probe.rs`/`frb_blatt_probe.rs`/`te_pair_probe`-Familie; neuer `load_blatt_pair`-Loader (verweigert Serie ohne Blatt-Header); Pfad-Korrektur in `blatt-papier-beweis.md:32-47` (die stale Verzeichnis-/Datei-Angaben auf `tools/measure/`, `src/mathematikerin/te.rs` und die Zeilen `:96`/`:2686` nachziehen).

#### Kreuz-Screening auf weitere Ereignisse (Bordeaux, Aaretal, Japan)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) ein Fenster/ein Ereignis je Becken; „keine Verallgemeinerung über Ereignisse (Bordeaux, Aaretal, Japan offen)."
- **Blockade:** keine.
- **Braucht:** dieselben Serien für Bordeaux/Aaretal/Japan ernten und `cross_te_screen` fahren.

#### Solar-Matrix: konditionale Prüfung 211A→193A
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) der einzige intra-AIA-Pfeil 211A→193A trägt nur einen Zeugen (die Matrix selbst); die konditionale Prüfung ist pending.
- **Blockade:** keine.
- **Braucht:** die konditionale Sonde auf 211A→193A anwenden (die Matrix siebt, die konditionale Sonde schlichtet).

#### Seismik-Flotte: Streuung senken + Stationsterm + W-Phase-M9
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die 16-Ereignis-Flotte ist unverzerrt (+1,7 km, se 4,7 km), aber 19 km über Ereignisse / 36 km über Stationen dominieren das ±10-km-Gate; Stationsterm (+5,69 s) offen; W-Phase-M9 entschieden, nicht gebaut.
- **Blockade:** keine.
- **Braucht:** besseres Picken / den mehrdeutigen pP-Zweig bei Δ≈30° auflösen und den Stationsterm an II.KIV wiederholen; W-Phase-M9 bauen.

#### Positive Maske — Audit-Punkte + fehlende Treiber
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `research-max`) Echo-Tiefe σ 19/36 km, Stationsterm offen (II.KIV +5,69 s), M9.1-Picker-Verdrahtung in die Flotte offen; **Slab2 ist bereits registriert** (`phi/sources.φ:12169-12174`, `slab2_depth.bin`, `slab2-cdn.yml`) — die Handover-Lage „nicht registriert" war stale; Slab2-Route ScienceBase 403 direct+Proton `blocked`. Galileo-ODF: Format-1-Doku im Archiv (`pds-ppi.igpp.ucla.edu/annex/GO-J-RSS-1-ODF-V1.0/DOCUMENT/TRK_2_18.TXT`), lebende Spec DSN 810-005 Modul 209C–G je 200; `galileo_odf.bin` registriert (`sources.φ:8369-8375`).
- **Blockade:** Slab2 403.
- **Braucht:** M9.1-Picker in die Flotte verdrahten; den Galileo-ODF-Formattrenn „1 vs 2" im 209G-Text extrahieren; Tomografie als Ernte-Kandidat registrieren (Slab2 bereits da).

#### Galileo Borduhr-Sprung A/B — Trenn-Frage
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fund einer öffentlichen Absolutfrequenz-Reduktion über 1995-11-30/12-01
- **Lage:** (gemessen 2026-09-25 via `research-max`) öffentlich nicht trennbar — `pending`; keine gemessene Absolutfrequenz-Reduktion über die Grenze. Bester nächster Kandidat: DESCANSO Article 5 „Galileo Telecommunications" (Taylor/Cheung/Seo 2002), `https://descanso.jpl.nasa.gov/DPSummary/Descanso5--Galileo_new.pdf` (direct HTTP 200, 4 309 495 B/76 S.; Proton 200; Wayback Snapshot 20041015172445) — der USO-Abschnitt ist ungeprüft (PDF-Textebene außerhalb des Toolsets). Morabito-Reihe endet 1993 (ADS/NTRS leer); PDS hat keinen Jupiter-Phasen-RSS-Datenkopf (GO-J-RSS-1-EDR „Information not found"); alle grenzüberspannenden Einweg-Reduktionen (Hinson 1997, Wohlmuth 1997, SWS 1995, Science 275/644) sind methodisch blind gegen einen konstanten Versatz.
- **Blockade:** fehlende Quelle.
- **Braucht:** DESCANSO5-USO-Abschnitt per PDF-Reader extrahieren; prüfen, ob eine USO-Frequenzreihe die Grenze kreuzt.

#### Sieben Sphären — Messvorschriften
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Sphäre I (Fresnel-Winkeldurchmesser-Feld) ist ausstehend, VII (Dopplergeist) hat keine statistische Aggregation; II–VI sind Konzept.
- **Blockade:** keine.
- **Braucht:** das Fresnel-Winkeldurchmesser-Feld (Asteroiden-Bahn ∩ Gaia-Farbe ∩ IR-Ø) und die Dopplergeist-Aggregation bauen.

#### Zeugin — vpec-Redshift-Domäne (`cosmicflows_cf4.json` ungenutzt)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `grind-pro`/`cargo check`) die zwei Redshift-Domänen sind gebaut — `near_flow_distance_m` und `far_flow_distance_m` (`src/archivar/skydirection.rs:77/89`, 6 Tests, `cargo check -p omegaflow` 0/0). Der vpec-Konsument bleibt `pending`: `cosmicflows_cf4.json` liegt nicht im Baum (Erzeuger `tools/harvest/src/bin/cosmicflows_compiler.rs` schreibt ihn aufs CDN).
- **Blockade:** das Asset liegt nur als CDN-Artefakt vor.
- **Braucht:** den vpec-Lookup aus dem CDN-Asset an `SkyDirection`/`distance_m()` binden.

#### Nadel Ⅻ (Urknall) — Cone-Gate + stale Blatt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `grind-pro`/`cargo check -p omegaflow-measure` 0/0) `bigbang_echo_probe.rs` auf echte TE über die z-Serie umgebaut (16 `DEPTH_BINS` à 50 Mpc, Lag in SI, `transfer_entropy_lag`); TE 0,147/0,223/0,223 < fam 0,275; Label-Verstoß `:198-222` korrigiert; `kybernetische-astrophysik.md`-Verdikt gesetzt (committet `974700848`/`d4af3b64d`).
- **Blockade:** keine.
- **Braucht:** das Cone-Gate über die z-Serie bauen; `docs/paper/big-bang-echo-sheet-12.md:49-53` (beschreibt noch die alte z-Serie) nachziehen.

#### Korona-Heizung — ortsaufgelöster Aktive-Region-Pfad
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) kein Instrument misst eine fam- und bandbreiten-feste Koronal-Sprosse; der sun-as-a-star-Ladder dämpft den Flare ~10×. Drei Lücken: sub-minütige Auflösung (nur unter fam), Minuten-fam, Multi-Force-TE.
- **Blockade:** keine.
- **Braucht:** den ortsaufgelösten Aktive-Region-Pfad bauen; die Minuten-fam und die Multi-Force-TE (`nobel_probe_corona` v2) ergänzen.

#### Depth-Phase-Flotte — sP-corr-Gate + Stations-Azimut-Register
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) der Dual-Phase-Fit ist pending (das sP-corr-Gate ungesetzt, die Verteilung gemessen); die sechs Pilot-Azimute stehen in keinem Register.
- **Blockade:** keine.
- **Braucht:** das sP-corr-Gate aus der gemessenen Verteilung setzen und die sechs Stationsazimute registrieren.

#### Galileo-Rotor-CK — volle Spin-Historie ernten
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) nur das 4-Tage-EGA-1-Fenster (1990-12-07…11) der frame -77000 all-spin-bus CK-Serie ist geerntet; die Spin-Historie der übrigen Missionsepochen ist Register-Pflicht.
- **Blockade:** keine.
- **Braucht:** die frame -77000 CK-Serie über die EGA-1-Grenze hinaus harvesten.

#### JWST Biosignatur-Kanäle pending (O₂/O₃, Red-Edge, Saisonal)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine JWST-Detektion + Spektrum eines dieser Kanäle
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Kanäle sind benannte pending-Zweige; die Abwesenheit ist gemessen (keine JWST-Detektion im gesuchten Record); die XUV-Re-Erklärung bleibt pending.
- **Blockade:** keine Quelle trägt eine Detektion.
- **Braucht:** bei Fund die Detektion + ihr Spektrum in das Register aufnehmen.

#### LAIC — Instrument A + DEMETER-Order
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Instrument A (Ereignisrate) ist named/unbuilt; DEMETER-Daten sind order-gated (Produktdownload `online:false` → GET 500); CSES ist SMS-CN-blockiert.
- **Blockade:** DEMETER-Order-Flow; CSES-Login.
- **Braucht:** Instrument A bauen; den DEMETER-Order-Flow prüfen (autonom: Download-Status mit vorhandenem CDPP-Login lesen).

#### Nadel V — Positive-Control-Konus + IR-Exzess-Achse
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `research-max`) anonyme Flächen offen: Fink REST `api.lsst.fink-portal.org` 200, ANTARES `/api/v1/loci` 200, Rubin-Alerts öffentlich an Broker; credentialisierte Streams account-gebunden. **Gaia-Join gemessen:** `SELECT s.source_id,s.ra,s.dec,r.pf,r.best_classification FROM gaiadr3.vari_rrlyrae AS r JOIN gaiadr3.gaia_source AS s USING (source_id) WHERE r.pf IS NOT NULL` → HTTP 200, 177 358 Records, ra/dec fließen über den Join → **Riss `blocked_sources.φ:256` vs `:265` stützt `:265`**. IR-Exzess: AllWISE W3/W4 via IRSA-TAP **pending** (Sync-Query 200 s ohne Antwort, curl exit 28); IRAS `irsa.ipac.caltech.edu/cgi-bin/Gator/nph-query` + Herschel `archives.esac.esa.int/hsa/whsa-tap-server/tap/sync` je HTTP 200; Akari DARTS CAS 200, Spitzer MIPS-24 IRSA-Legacy (SAGE `blocked_sources.φ:360`).
- **Blockade:** teils account-gebunden (Streams); AllWISE-Sync-Zeitbudget.
- **Braucht:** (a) die Register-Korrektur von `blocked_sources.φ:256` auf die Join-Form (Schreib-Akt der Mountain-Linie, Riss entschieden); (b) AllWISE-W3/W4-Kegel via asynchronem TAP; (c) IRAS-PSC-`fnu_60` + Herschel-`hsa.pacs_point_source_100/_160`-Katalogproben.

#### Planet-Nine/KBO — h-Sweep + Real-Kernel-Gegenprobe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Typ-1-`.bsp` liegt im Baum (z. B. Ceres/Vesta)
- **Lage:** (gemessen 2026-09-25 via `grind-flash`/`cargo check -p omegaflow` 0/0) SPK-Type-1-Reader gebaut (`src/archivar/bsp_reader/spk.rs`, `data_type 1` in `ephemeris.rs`, committet `7cb5068ad`); kein Typ-1-Kernel im Baum → Real-Gegenprobe `pending`; h-Sweep ungefahren.
- **Blockade:** kein Typ-1-Testkern.
- **Braucht:** ein Typ-1-`.bsp` (Ceres/Vesta) ernten, dann `laic_probe --analyze DIR --kde-scale` (h/2, 2h).

#### Pioneer/Dark-Matter — ASCII-Vollmission-Reduktion + Voyager-Doppler + DSN 810-005
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die ASCII-Vollmission trägt den programmierten Uplink-Rampen-Sweep und ist die verbleibende offene Reduktion; ein Voyager-Doppler bei SPDF ist unverifiziert; 810-005 (MDA-Resolver) ist die benannte nächste Suche.
- **Blockade:** keine.
- **Braucht:** die ASCII-Vollmission-Reduktion (Rampen-Sweep) bauen; die Voyager-Doppler-Route bei SPDF prüfen; 810-005 suchen.

#### Trishuli — Bahrabise-Richtungsverifikation + S1/SAR-Footprint
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort (Post-Sentinel-1-Szene)
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Bahrabise-Spalten tragen die `te_pair_probe`-Spiegelunsicherheit; der räumliche Footprint bleibt pending (CEMS nur Grading, optisch wolkenverdeckt, S1 noch nicht archiviert).
- **Blockade:** S1-Post-Szene noch nicht archiviert.
- **Braucht:** die Bahrabise-Richtung mit richtungs-verifizierter Bibliothek prüfen; die Post-Sentinel-1-Szene auf Flutfläche/Narbe messen.

#### Terminologie — Definitionen Ontologie-Motor / mycorrhizal_internet
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `sgrep -i`) beide Begriffe kommen im getrackten Baum nur in ihren eigenen Registerzeilen vor (`docs/concepts/glossar.md:59/61`, `docs/handover/archiv/handover-2026-09-25-mountain-folge163.md`) — kein Definitionsort in Code/Kanon; in `docs/paper/terminologie-der-gegenstroemung.md` als `pending` dokumentiert.
- **Blockade:** keine.
- **Braucht:** die Semantik der beiden Phrasen aus dem Operator-Korpus (`state/funding/profil-operator.md`) heben und als getrackte Definition setzen — bis dahin `pending`.

#### Broken-Null-Control — offene Spec-Pendings
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) max-T-Korrektur über die 20-Paar-Matrix, Lag-Sweep (nur τ ∈ {0, 60, 120}), Bandbreiten-Sensitivität, Fenster-Drift und Rest-FN bei n = 300 sind als Pendings benannt.
- **Blockade:** keine.
- **Braucht:** die fünf Messungen nachziehen (max-T, Lag-Sweep, h, Fenster-Drift, Rest-FN). **Achtung:** `src/mathematikerin/te.rs` trägt fremde uncommittete Arbeit — vor dem Bauen mit der fremden Linie abstimmen.

#### Weberin — zweite unabhängige Positions-Linie je Körper-Klasse
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Planeten/Monde (INPOP-vs-DE/Astrometrie), Raumsonden (Doppler), breite TNO-Kette (`mpcorb_extended`) und Kometen (`dcom5`/`cometels`) tragen je nur eine Linie.
- **Blockade:** keine.
- **Braucht:** die zweite Linie je Klasse ernten/kompilieren (u. a. `pallas`, `juno_asteroid`, `encke`).

#### Weberin Faden-Matrix — verbleibende No-Actor-Lücken + Broker-Positionen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) No-Actor: VHE/Neutrino/CR-Einzelteleskope ohne offene Route (TA-Vollkatalog, Super-K, JUNO, LHAASO-Event); Broker Lasair/ANTARES/Fink positions-pending; seismische Stations-Weltlinien nur als Events.
- **Blockade:** teils not-published.
- **Braucht:** die Broker-Positionen kompilieren; die seismischen Stations-Weltlinien registrieren; die not-published-Teleskope als absent halten.

#### Weberin-Quellen — WWLLN/BPA-GIC gebaut, HAWC/Fink offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die CDN-Läufe `wwlln-cdn.yml`/`bpa-gic-cdn.yml` sind success
- **Lage:** (gemessen 2026-09-25 via `grind-pro`/`cargo run`) `wwlln_compiler.rs` (NetCDF-4/HDF5, 14 342 985 Records) + `bpa_gic_compiler.rs` (TSV, 11 110 Records, lat/lon absent) gebaut, Register-Blöcke + Workflows (committet `974700848`); HAWC fehlt das Let's-Encrypt-Zwischenzertifikat „YR1" → `OMEGAFLOW_CA_BUNDLE`; Fink-Persistenz `api.ztf.fink-portal.org` 200, ALeRCE `detections` 404 `pending`; TA-Vollkatalog `not-published`.
- **Blockade:** HAWC-TLS-Kette; Fink/ALeRCE-Route.
- **Braucht:** `gh workflow run wwlln-cdn.yml` + `gh workflow run bpa-gic-cdn.yml`; YR1-PEM in `OMEGAFLOW_CA_BUNDLE` setzen; Fink/ALeRCE-Persistenz re-messen.

#### Sonden-Flotte — Per-Code-Emission gebaut, Register-Nachzug + Re-Check offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `maven-tnf-cdn`-Re-Check-Lauf
- **Lage:** (gemessen 2026-09-25 via `grind-pro`/`cargo run`) `tnf_compiler.rs`/`maven_tnf_compiler.rs` auf Per-Code-PODF-Emission umgebaut, Format-0-Filter entfernt; Testlauf 151 829 Rows über Codes {0,1,9,16,17}; LRO-Jahresfilter in `lro_trk_compiler.rs` entfernt (committet `974700848`).
- **Blockade:** keine.
- **Braucht:** den LRO-Block `harvest.φ:100-105` nachziehen (`args --year 2009` entfernen, `pattern` ohne `_2009`, `shard N`, `timeout 240`, `lro_trk_2009.bin`→`lro_trk.bin` in `sources.φ:8002-8009`); dann `gh workflow run maven-tnf-cdn.yml`.

#### Das eine Instrument — Anomalie offen (zweite Augenklasse fehlt)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein zweiter Messkanal (VLBI-Beacon auf einer interstellaren Sonde) existiert.
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Pionier-Anomalie ist offen und unter dem einen Instrument nicht entscheidbar; eine zweite Augenklasse (Winkel aus VLBI + Geschwindigkeit aus Doppler) fehlt.
- **Blockade:** kein Instrument misst den vollen Phasenraum der Pioniere; ohne zweite Augenklasse bleibt die Anomalie unentschieden.
- **Braucht:** eine zweite Augenklasse — VLBI-Beacon auf der nächsten interstellaren Sonde, von Tag eins zweikanalig getrackt.

#### Axiom-Gate H0-Linien — 75-Quellen-Crossmatch `pending`
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) der 75-Quellen-Crossmatch (per-source identity des Cepheiden-Ankers) ist als `pending` benannt; die 0.2619-mas-Wiegung überzeichnet sich nicht.
- **Blockade:** keine.
- **Braucht:** den 75-Quellen-Crossmatch fahren und die Wiegung gegen die benannte Zählung prüfen.

### Operator handelt

#### BLE-HR-Live-Messung FR945
- **Status:** operator-gebunden (Akt: Hardware/Radio) | **Bindung:** operator
- **Trigger:** Operator startet den verdeckten Lauf am gekoppelten Gerät (945 am Arm)
- **Lage:** der GFDI-Bus-Abriss ist geheilt und **CI-bestätigt** (gemessen 2026-09-25 via `ci_manage log 36116592391`): alle `archivar::ble::tests` grün auf `45b4b2eb1`; Fix `src/archivar/ble.rs:1360/1418`. Live-Bestätigung offen (kein BlueZ/Gerät in der Session).
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
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) die Order-Portale antworten (`cdpp-archive.cnes.fr`/`cdpp.irap.omp.eu`/`regards.cnes.fr` je HTTP 200 direct + Proton); der Produktdownload hinter dem Login (`online:false` → GET 500) ist heute `pending` (nicht neu gemessen). Der Order selbst ist ein konsenspflichtiger Schreib-Akt an Dritte.
- **Blockade:** order-gated.
- **Braucht:** den Download-Status mit vorhandenem CDPP-Login lesen (autonom); die Order danach als Konsens-Akt vorlegen.

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
- **Lage:** (gemessen 2026-09-25 via `sread` auf `state/mail/mail_ledger.φ`) Ledger vorhanden (>151 Zeilen); jüngste Eingänge `1790316610` (ORCID-Verify-Reminder) und `1790290298` (Rubin-Forum-Digest); `sgrep` relay/sensor/beat/mantis/ble/hrv/zigbee = 0 Treffer.
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
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) fünf DSN-Briefe sind in Flug (Entscheid-Handover §Warten).
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

### Survey-Träger — Orphan-Faltung 2026-09-25
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `register_lookup --orphan-docs`) 15 orphan Surveys ohne Handover-Träger; offene Marker je Dokument: axiom-gate-broken-null-control 1, axiom-gate-cross-screening-tibet 1, axiom-gate-depth-phase-echo-fleet 1, axiom-gate-h0-lines-register 1, axiom-gate-solar-seconds-matrix 1, axiom-gate-neptune-rift-ephemerides 2 (resolved), axiom-gate-uranus-rift-ephemerides 2 (resolved), survey-2026-09-07-weberin-sonnensystem-kette 7, survey-2026-09-07-weberin-thread-matrix 22, survey-2026-09-13-weberin-quellen 42, survey-2026-09-13-weberin-quellen-folge 14, survey-2026-09-13-weberin-quellen-treffer 9, survey-2026-09-14-weberin-quellen-rerun 42, survey-2026-09-16-sonden-flotte 4, survey-ein-blatt-korona-heizung 2.
- **Blockade:** keine.
- **Braucht:** die 15 Surveys sind durch die Punkte oben getragen — je Survey die offenen Marker gegen die oben geführten Punkte abgleichen (broken-null-control, cross-screening, depth-phase, h0-lines, solar-matrix, weberin-kette, weberin-thread, weberin-quellen, sonden-flotte, korona); die zwei `resolved`-Linien (neptune/uranus) als descoped schließen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
