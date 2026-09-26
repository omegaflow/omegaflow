<!--
  title: Handover — Sensory-Folge 174 (Stand 2026-09-26)
  session: Sensory-Folge 174
  class: handover
  date: 2026-09-26
  sha256: 42e10b9465633ba6607dfd874c1a39235ee8ad7a09765cac3067ee7a48b12d38
  status: live
-->
# Handover — Sensory-Folge 174 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur
die eigenen Hunks — committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr
von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Sortierung: erst logisch nach Akteur
(Linie | Rat | Operator | Dritter), dann chronologisch (Messdatum). Operator-gebundene
Punkte sind in Vorbereitung (autonom) und Akt (operator-gebunden) getrennt.

Die FR945 ist das persönliche Gerät des Operators; ihre Kennung (MAC) und ihre Daten
bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN.

## Stehender Pass (automatisch, keine Auswahl)

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu Session-Beginn
gemessen; ihr Ausgang steht im Zustand-Ledger, nicht als Kopie hier. Karte:
`docs/concepts/tools-map.md`.

- **Postfach** — `smail` + `state/mail/mail_ledger.φ` (fällig 2⁶ min).
- **CI-Status am HEAD** — Watchdog-Snapshot `/tmp/opencode/ci_status.md` (nur wenn
  jünger als der letzte HEAD-Wechsel), sonst `ci_manage list`/`view`; nie
  `gh run list`/`gh run view`.
- **Artefakt-Frische** — erzeugte Klassen (Tools, Core-Bin, CDN-Assets, WGSL,
  Firmware) hinken HEAD; nie einen aktuellen Satz über ein Artefakt hinter HEAD.

## Träger-Zeilen (Orphan-Faltung)

Die folgenden Prosa-Dokumente tragen offene Marker und werden hiermit an ihre
Trägerpunkte gebunden (der Scanner `register_lookup --orphan-docs` erkennt den
Träger nur, wenn eine lebende Übergabe den Dateinamen nennt):

- `survey-2026-09-07-weberin-sonnensystem-kette.md`, `survey-2026-09-07-weberin-thread-matrix.md`,
  `survey-2026-09-13-weberin-quellen.md`, `survey-2026-09-13-weberin-quellen-folge.md`,
  `survey-2026-09-13-weberin-quellen-treffer.md`, `survey-2026-09-14-weberin-quellen-rerun.md`
  → Träger `docs/concepts/die-weberin.md` (Weberin-Punkte unten).

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### Kreuz-Screening auf weitere Ereignisse — Konfigs gebaut, Läufe offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `grind-pro`) die drei Ereignis-Konfigs stehen unter `phi/meteo/` im tibet-Schema: `aaretal-hochwasser-2026.json` (Fenster 27.05.–05.06.2026, Habkern/Beatenberg/Interlaken), `bordeaux-waldbrand-2026.json` (22.–31.07.2026, Landiras/La-Teste/Bordeaux), `japan-tsunami-2026.json` (25.07.–03.08.2026, Kumamoto/Yatsushiro/Kashima); `.gitignore` trägt `!phi/meteo/*.json`. Drei gemessene Rissen: „Bordeaux" ist ein **Waldbrand** (kein Hochwasser); „Japan" ist das **Kumamoto-Beben** (Mww 6.8 vs 7.1) mit aufgehobener Tsunami-Advisory; das Aaretal-2026-Ereignis ist die **Sturzflut Habkern/Beatenberg** (Aare-Hochwasser war 2025).
- **Blockade:** keine — die Läufe sind offline/CI.
- **Braucht:** `meteo_harvest --event phi/meteo/<event>.json --out …` je Ereignis, dann `cross_te_screen --dir <harvest> --lags 1,6,12,24 --surrogate 20 --min-n 100`; den `meteo-cdn`-Lauf lesen.

#### Seismik-Flotte: Streuung senken + Stationsterm + W-Phase-M9
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) der W-Phase-M9-Zweig ist gebaut: `w_phase_bandpass` (RC-Kaskadenfilter, Band 100–1000 s), `w_phase_energy_ratio` (RMS-Fenster `[t_s, t_s+600 s]` gegen `[t_p−200 s, t_p]`, `Option`-skip), `w_phase_discriminate` → `Option<WPhasePick>` mit Gate `energy_ratio >= 8.0` (4 Tests, `tools/measure/src/depthphase.rs`); pP-Zweig und `MIN_DIST_DEG = 30.0` unberührt. Flotte unverzerrt (+1,7 km, se 4,7 km), aber 19/36 km dominieren das ±10-km-Gate; ak135 ist 1D (`positive-maske.md:64`), kein 3D-Modell registriert.
- **Blockade:** kein 3D-Geschwindigkeitsmodell (pP-Residuum); Stationsterm-Quelle.
- **Braucht:** Residuum gegen Δ≈30° messen (3D-Modell registrieren oder `pending` halten); Stationsterm an II.KIV wiederholen; `w_phase_discriminate` in die Flotte verdrahten.

#### Positive Maske — Audit-Punkte + fehlende Treiber
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) der M9.1-Picker ist kanonisiert: `tools/measure/src/picker.rs` (`p_onset`, `first_break_arrival`, `sta_lta_arrival`, `bandpass`, 6 Tests) + `pub mod picker` (`lib.rs`); `quake_location_probe.rs` nutzt das Modul (privates Duplikat entfernt). Die Flotte nutzt `p_onset` bereits über `depthphase::measure_station` (`depthphase.rs:857`); `depthphase.rs` trägt noch eine eigene, funktional identische Picker-Kopie. Echo-Tiefe σ 19/36 km, Stationsterm offen (II.KIV +5,69 s); **Slab2 registriert** (`phi/sources.φ:12169-12174`), ScienceBase-Route 403 direct+Proton `blocked`; `galileo_odf.bin` registriert (`sources.φ:8369-8375`).
- **Blockade:** Slab2 403.
- **Braucht:** die `depthphase.rs`-Picker-Kopie in `picker` zusammenlegen (nach dem W-Phase-Commit); den Galileo-ODF-Formattrenn „1 vs 2" im 209G-Text extrahieren; Tomografie als Ernte-Kandidat registrieren.

#### Sieben Sphären — Messvorschriften
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) Sphäre I (Fresnel-Winkeldurchmesser-Feld) ausstehend, VII (Dopplergeist) ohne statistische Aggregation; II–VI sind Konzept.
- **Blockade:** keine.
- **Braucht:** das Fresnel-Winkeldurchmesser-Feld (Asteroiden-Bahn ∩ Gaia-Farbe ∩ IR-Ø) und die Dopplergeist-Aggregation bauen.

#### Korona-Heizung — Aktive-Region-Route gemessen, Parser-Arm offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `research-max`) die lebende Route ist `https://services.swpc.noaa.gov/json/solar_regions.json` (HTTP 200, 138 703 B, sha256 `9a91cea7…`, `application/json`); Schema `observed_date, region, latitude, longitude, location, carrington_longitude, area, spot_class, extent, number_spots, mag_class, …` (kein ICRS, heliographisch). Der alte Register-Eintrag `/products/solar-regions.json` war 404-tot und ist im Dispositions-Register auf die lebende URL + gemessene Felder korrigiert. NGDC Sunspot-`table_international-sunspot-numbers_daily.txt` 404; lebende Alternative SILSO `SN_d_tot_V2.0.csv` (200, `text/csv`). `nobel_probe_corona.rs` v2 + `aia_compiler --region` gebaut.
- **Blockade:** kein `sources.φ`-Parser-Arm für die JSON-Route (unit-auto-detect gap); die Ernte ist CI-only.
- **Braucht:** die lebende SWPC-Route als `sources.φ`/Port-Eintrag mit Feldmap (`area` millionths, `extent`, `spot_class` McIntosh, `number_spots`, heliographisch→ICRS) setzen; dann `aia_compiler --harvest` mit Per-Datum-Koordinaten → Region-Bins → `aia_ladder_probe --aia <region bin>`.

#### Nadel V — Positive-Control-Konus gemessen, IR-Routen-Registrierung offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `research-max`) AllWISE-Job `23542882` **COMPLETED**; Result 200 VOTable 1.3, Query `SELECT designation,ra,dec,w1mpro..w4mpro,w3snr,w4snr FROM allsky_4band_p3as_psd WHERE CONTAINS(POINT('ICRS',ra,dec),CIRCLE('ICRS',187.2779,2.0524,0.001667))=1` — **genau 1 Zeile** `J122906.70+020308.6`, w1 8.369 / w2 7.407 / w3 5.147 / w4 2.944, W1−W2 = +0.962. Routen gemessen: AllWISE/TAP `https://irsa.ipac.caltech.edu/TAP` (async-UWS der Weg; Sync-Stall bekannt), IRAS-PSC Tabelle `iraspsc` (Sync 200, 81 Spalten, fnu_12/25/60/100), Herschel HSA `archives.esac.esa.int/hsa/whsa-tap-server/tap/sync` (`hsa.pacs_point_source_100/160` 200, registriert `sources.φ:7452`). **AllWISE `allsky_4band_p3as_psd` und IRAS-PSC `iraspsc` fehlen in `sources.φ`.**
- **Blockade:** keine (Registrierung); AllWISE sync-Stall → async-UWS-Parser-Arm nötig.
- **Braucht:** die zwei fehlenden IR-Routen registrieren (AllWISE via async-UWS-Arm, IRAS-PSC als Sync-TAP-Feldmap); dann `lsst_anomaly_probe` auf dem positiven Kontrollkegel.

#### Pioneer/Dark-Matter — Routen gemessen, Rampen-Sweep gebaut
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25/26) ASC byte-exakt live: p10 `…SC_23` 19 820 702 B, p11 `…SC_24` 21 454 162 B, p11 `…SC_23` 404 — verdrahtet `pioneer_doppler_compiler.rs:7-8,70`. Voyager 2 `radio_science_rss` = genau zwei Verzeichnisse, **kein Cruise-Doppler** (request-only, `blocked_sources.φ:51`); `phi/sources.φ` trägt keine voyager2-Origin. DSN 810-005 TRK live: 203E 2 061 668 B, 202E 1 858 227 B, 209G 661 438 B; `…/MDA/` → 301 (Ziel ungemessen). Rampen-Sweep-Probe jetzt gebaut: `tools/measure/src/bin/pioneer_ramp_sweep_probe.rs` (zweistufiger Steigungs-Sweep über die NAVIO-Pässe, Plausibilitäts-Gate, 0 honored bei leerer Serie).
- **Blockade:** keine (Pioneer); MDA-Listing.
- **Braucht:** den Sweep auf einer lokalen NAVIO-Ernte laufen lassen; Voyager-2-Route registrieren; `…/dsndocs/810-005/MDA/`-Ziel fetchen.

#### Trishuli — Bahrabise-Richtungsverifikation + S1/SAR-Footprint
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort (Post-Sentinel-1-Szene)
- **Lage:** (gemessen 2026-09-25) die Bahrabise-Spalten tragen die `te_pair_probe`-Spiegelunsicherheit; der räumliche Footprint bleibt pending (CEMS nur Grading, optisch wolkenverdeckt, S1 nicht archiviert).
- **Blockade:** S1-Post-Szene nicht archiviert.
- **Braucht:** die Bahrabise-Richtung mit richtungs-verifizierter Bibliothek prüfen; die Post-Sentinel-1-Szene auf Flutfläche/Narbe messen.

#### Terminologie — Definitionen Ontologie-Motor / mycorrhizal_internet
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) beide Begriffe nur in ihren eigenen Registerzeilen (`docs/concepts/glossar.md:59/61`); in `docs/paper/terminologie-der-gegenstroemung.md` als `pending`. Der Operator-Korpus `state/funding/profil-operator.md` liegt im privaten Repo.
- **Blockade:** Operator-Korpus im privaten Repo.
- **Braucht:** die Semantik der beiden Phrasen heben und als getrackte Definition setzen — bis dahin `pending`.

#### Weberin — zweite unabhängige Positions-Linie je Körper-Klasse
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) `pallas` (2000002) und `juno_asteroid` (2000003) sind in `src/archivar/kernels/naif_body_ids.tsv` ergänzt und je als SPK-Block `ephemeris_pallas.bin`/`ephemeris_juno_asteroid.bin` in `sources.φ` registriert (GM-Einträge gemessen vorhanden); die `encke`-Kometen-Zweitlinie (`horizons_compiler` × `dcom5_compiler`, `sources.φ:2884`) steht bereits. Offen: `cometels` als Weberin-Positions-Linie, die breite TNO-Kette (`mpcorb_extended`). Träger: `docs/concepts/die-weberin.md`.
- **Blockade:** keine.
- **Braucht:** `cometels` als zweite Kometen-Keplerlinie (Weave-Arm in `src/weberin.rs`); `mpcorb_extended` als TNO-Zweitlinie via `weberin_mpc_spk_verdict`.

#### Weberin Faden-Matrix — verbleibende No-Actor-Lücken + Broker-Positionen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) No-Actor: VHE/Neutrino/CR-Einzelteleskope ohne offene Route (TA-Vollkatalog, Super-K, JUNO, LHAASO-Event); Broker Lasair/ANTARES/Fink positions-pending; seismische Stations-Weltlinien nur als Events. Träger: `docs/concepts/die-weberin.md`.
- **Blockade:** teils not-published.
- **Braucht:** die Broker-Positionen kompilieren; die seismischen Stations-Weltlinien registrieren; die not-published-Teleskope als absent halten.

#### Weberin-Quellen — HAWC-TLS offen, Fink/ALeRCE live
- **Status:** offen | **Bindung:** eigen
- **Trigger:** das YR1-Zwischenzertifikat liegt in `OMEGAFLOW_CA_BUNDLE`
- **Lage:** (gemessen 2026-09-25) die CDN-Assets sind sha-verifiziert: `tao_wnd_zonal.csv` `b7719c89…` (`sources.φ:786`), `wwlln_th.csv` `06031403…` (`:8634`), `bpa_gic.csv` `8d0ceaa8…` (`:8646`). **HAWC** direct TLS-broken (Leaf `www.hawc-observatory.org`, Issuer `CN=YR1`; Wayback 200 2024-09-15); **Fink** `api.lsst.fink-portal.org/api/v1/objects` 200; **ALeRCE** `api.alerce.online/alerts/v1/objects/` 200. Träger: `docs/concepts/die-weberin.md`.
- **Blockade:** HAWC-TLS-Kette (YR1).
- **Braucht:** YR1-PEM in `OMEGAFLOW_CA_BUNDLE` setzen und HAWC erneut fetchen; Fink/ALeRCE-Persistenz-Reader bauen.

#### Survey-Träger — Rest-Orphans
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) die neptune/uranus-`resolved`-Linien sind als `descoped` geschlossen; die Weberin-Quellen-Surveys sind über die Träger-Zeilen oben an `docs/concepts/die-weberin.md` gebunden. Verbleibend: weitere der 40 Orphan-Docs (u. a. `survey-2026-09-06-codestruktur` (8), `survey-2026-09-17-omegaflow-legacy-konzepte` (2), `survey-2026-09-02-code-te-drift`, die `docs/blatt/*`); die Inline-Code-Klasse nach dem `strip_inline_code`-Merge.
- **Blockade:** keine.
- **Braucht:** die verbleibenden Orphan-Docs je Träger binden oder gemessen `descoped` schließen; die Inline-Code-Klasse nach dem Merge schließen.

#### Lag-Sweep + KDE-Bandbreiten-Sensitivität — offene Mess-Gates
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) `ein-blatt-papier` und `blatt-papier-beweis` benennen Lag-Sweep und KDE-Bandbreite als offene Mess-Gates; `laic_probe --analyze --kde-scale` trägt den Knopf, die lokale laic-Ernte fehlt (CI).
- **Blockade:** keine — offline/CI.
- **Braucht:** `laic_probe --analyze --kde-scale` in CI dispatchen und den Lag-Sweep je Paar drucken.

#### Broken-Null-Control — Spec-Doc nachgezogen, CI-Lauf-Lesung offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf (`ci-check` + `te-gate`)
- **Lage:** (gemessen 2026-09-26 via `grind-flash`) `docs/specs/broken-null-control.md` §6 auf den gemessenen Stand nachgezogen (fünf Gate-Tests benannt; Reverse-Stille nur bei τ=10 gegated, nicht τ=1; Window-Drift von „pending" auf closed); Header-sha `63b7e8b7…`. `te-gate` war dispatcht (`36194535114`, queued), gegen `origin/main` vor diesem Atom.
- **Blockade:** keine.
- **Braucht:** den `te-gate`-Lauf lesen.

#### ZNSP FORMNETWORK — `esp_zb_cfg_t`-Payload-Größe ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die kompilierte ESP-IDF-Referenz (`zigbee-host.yml`-Lauf bzw. `sizeof`-Probe)
- **Lage:** (gemessen 2026-09-25) der ZNSP-Transport ist gebaut (`firmware/radiatorium-lib/src/znsp.rs`, Bin `znsp_host.rs`); die FORMNETWORK-Payload ist ABI-raw (`sizeof(esp_zb_cfg_t)`), nur aus Struct+Alignment abgeleitet (16 B), nicht gemessen — `form_network_payload_pending()` liefert `None`. `zigbee-host.yml` war dispatcht (`36194532489`), die `sizeof`-Probe selbst ist noch ungebaut.
- **Blockade:** keine — braucht die kompilierte Referenz.
- **Braucht:** die `sizeof`-Probe in `zigbee-host.yml` bauen; den Lauf lesen; dann den FORMNETWORK-Encoder setzen.

#### HRV/Puls→Strahlung — Kette nach dem Live-Lauf beobachten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf (Operator) hat `rr` geliefert
- **Lage:** (gemessen 2026-09-25) die Kette ist gebaut: `hrv.rs:24-87` → `feed_beat_to_hrv` (`main_flow.rs:79-102`) → `tone_code` → `tone_scale` 0.25 (`omega.rs:1670-1678`) → `aperture` (`omega.rs:349`) → `Σω*aperture` (`actuators.rs:29`).
- **Blockade:** hängt am BLE-Live-Fluss (Operator).
- **Braucht:** nach dem Live-Lauf `tone_code`→`tone_scale`→`aperture` im Frame/perm-Log lesen.

#### M2c — Rust-ZNSP-Host auf dem BL808 gegen das H2
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ox64 angekommen
- **Lage:** (gemessen 2026-09-25) der Ox64 wird Host-CPU des Coordinators (ZNSP über UART), das H2 das Funkmodul; Ox64-UART-Pins UART0 GPIO14/15, UART1 GPIO16/17. Buildroot-Bring-up ungemessen.
- **Blockade:** Ox64 liegt beim Carrier.
- **Braucht:** Buildroot-Bring-up, dann derselbe Rust-ZNSP-Host auf dem BL808 gegen das H2.

#### BL808-eigenes 802.15.4-Radio — registrierter `pending`-Faden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** öffentlicher 802.15.4-Treiber-/Stack-Fund für den BL808
- **Lage:** (gemessen 2026-09-25) kein öffentlicher Treiber: `bl_iot_sdk` trägt für BL808 nur `bl808_wifi`, BL808-RM ohne Wireless-Kapitel; `bouffalo_sdk_bl808` README-Raw 404 → `pending`. Route 2 nutzt das Radio bewusst nicht.
- **Blockade:** kein Treiber.
- **Braucht:** bei Fund `archive_search --github bouffalo_sdk_bl808 802.15.4`; bis dahin keine Arbeit.

#### Galileo Borduhr-Sprung A/B — DESCANSO5 trägt keine Frequenzreihe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fund einer öffentlichen Absolutfrequenz-Reduktion über 1995-11-30/12-01
- **Lage:** (gemessen 2026-09-26 via `vision`, OCR der gerenderten Seiten) DESCANSO5 `Descanso5--Galileo_new.pdf` (§2.6.3, §7.2) trägt **keine** USO-Absolutfrequenzreihe — nur qualitative Offsets („less than 5 Hz at S-band over a couple of days"; Einweg-Schwankungen 2001/2002 „tenths of a Hz"); Beobachtungsbereich 1989-12-05 … 2002. Die Grenze 1995-11-30/12-01 ist damit nicht kreuzbar → `pending`. Morabito-Reihe endet 1993; PDS ohne Jupiter-Phasen-RSS-Datenkopf.
- **Blockade:** keine Quelle trägt eine USO-Frequenzreihe über die Grenze.
- **Braucht:** eine andere öffentliche Absolut-Frequenzreduktion (DESCANSO-Begleitdokumente, USO-Aging-Reihe) finden; notfalls als absent halten.

#### JWST Biosignatur-Kanäle pending (O₂/O₃, Red-Edge, Saisonal)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine JWST-Detektion + Spektrum eines dieser Kanäle
- **Lage:** (gemessen 2026-09-25) die Kanäle sind benannte pending-Zweige; die Abwesenheit ist gemessen (keine JWST-Detektion im gesuchten Record).
- **Blockade:** keine Quelle trägt eine Detektion.
- **Braucht:** bei Fund die Detektion + ihr Spektrum ins Register.

#### Planet-Nine/KBO — h-Sweep + Real-Kernel-Gegenprobe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Typ-1-`.bsp` liegt im Baum (z. B. Ceres/Vesta)
- **Lage:** (gemessen 2026-09-25) SPK-Type-1-Reader gebaut (`bsp_reader/spk.rs`, `data_type 1` in `ephemeris.rs`); kein Typ-1-Kernel im Baum → Real-Gegenprobe `pending`; h-Sweep ungefahren.
- **Blockade:** kein Typ-1-Testkern.
- **Braucht:** ein Typ-1-`.bsp` (Ceres/Vesta) ernten; dann `laic_probe --analyze DIR --kde-scale` (h/2, 2h).

#### Das eine Instrument — Anomalie offen (zweite Augenklasse fehlt)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein zweiter Messkanal (VLBI-Beacon auf einer interstellaren Sonde) existiert
- **Lage:** (gemessen 2026-09-25) die Pionier-Anomalie ist unter dem einen Instrument nicht entscheidbar; eine zweite Augenklasse (Winkel aus VLBI + Geschwindigkeit aus Doppler) fehlt.
- **Blockade:** kein Instrument misst den vollen Phasenraum der Pioniere.
- **Braucht:** eine zweite Augenklasse — VLBI-Beacon auf der nächsten interstellaren Sonde, von Tag eins zweikanalig.

#### CI-Dispatch der Sensory-Workflows — GitHub-API-Rate-Limit
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das GitHub-API-Budget ist zurückgesetzt (Rate-Limit-Reset)
- **Lage:** (gemessen 2026-09-26T00:16+02:00 via Watchdog-Snapshot + `ci_manage list`) der Watchdog liest `list void`; `ci_manage list` → HTTP **403** (`api.github.com/.../actions/runs`). Die Sub-Workflows (`laic-cdn`, `gll-ck-cdn`, `meteo-cdn`, `cosmicflows-cdn`, `corona-conditional-probe`, `bigbang_echo_probe`, `depth-phase-fleet`, `zigbee-host`) sind **nicht** plazierbar; die älteren Läufe liefen gegen `origin/main` vor diesem Atom.
- **Blockade:** GitHub-API-Rate-Limit (extern).
- **Braucht:** bei Budget-Reset die Sub-Workflows dispatchen (`gh workflow run <wf>.yml --ref main`); die Ergebnisse einmalig lesen (`ci_manage view`).

### Operator handelt

#### BLE-HR-Live-Messung FR945
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) der GFDI-Bus-Abriss ist geheilt und CI-bestätigt: alle `archivar::ble::tests` grün auf `45b4b2eb1`; Fix `src/archivar/ble.rs:1360/1418`; verdeckter Lauf-Command steht (`OMEGAFLOW_BLE_HR=… OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow`, ~45 s).
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereithalten.
- **Status (Akt):** operator-gebunden (Hardware/Radio) | **Bindung:** operator
- **Trigger:** Operator startet den verdeckten Lauf am gekoppelten Gerät (945 am Arm)
- **Braucht:** `OMEGAFLOW_BLE_HR=<FR945-MAC aus .secrets.local> OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow`; `sensor:`-/`gfdi_line`-Zeilen lesen.

#### FIT-Verifikation eigene FR945-Datei (lokal-only)
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) der Dump-Pfad ist gebaut (`src/main.rs:1-56`: `OMEGAFLOW_FIT_SAMPLE` → `parse_fit`, druckt records/nn/min/max/all-finite, `exit(2)` bei Refusal), am SDK-Sample verifiziert.
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereithalten.
- **Status (Akt):** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator startet den Lauf mit seiner Datei
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/pfad/FR945.fit cargo run -p omegaflow --bin omegaflow`.
- **Wort:** „meine fits datei verlässt niemals dieses gerät" | 2026-09-25 | Operator (Session)
- **Wort:** „die 945 von anderer Hardware trennen; meine Daten bleiben lokal" | 2026-09-25 | Operator (Session)

#### Onboard-/CIQ-Bedarf benennen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-24) der Host-Reader `parse_fit` (`fit.rs:78`) ist verdrahtet; CIQ hat keinen Reader (`sgrep -i ciq src tools` leer). Ein-Quellen-Regel: solange FIT läuft, sind BLE/Serial über die Arbitrierung ausgeschlossen.
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** Bedarf Ja/Nein — Nein → der Punkt ist released, `FIT_DIR` bleibt der FIT-Kanal.

#### Beat-Arbitrierung — verdeckter Lauf mit zwei Beat-Quellen
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) die Spawn-Arbitrierung steht (`main_flow.rs:504`); die reine Funktion ist CI-getestet; der verdeckte Lauf-Command steht.
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereithalten.
- **Status (Akt):** operator-gebunden (hidden) | **Bindung:** operator
- **Trigger:** Operator startet einen verdeckten Lauf mit zwei gesetzten Beat-Quellen
- **Braucht:** `OMEGAFLOW_HIDDEN=1`-Lauf mit zwei Quellen, der genau eine `beat source:`-Zeile zeigt.

#### DEMETER/CDPP-Order-Flow
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) die Order-Portale antworten (je 200); CDPP-Login funktioniert (`REGISTERED_USER`); die Registerzeile ist nachgezogen (DONE_WITH_WARNING, 96978 in Vermerk, 0 verfügbar, Download HTTP 500).
- **Blockade:** Dateifehler-Ursache hinter der REGARDS-API (kein Report-Endpoint).
- **Braucht:** die 96 978 Dateien über die REGARDS-UI lesbar machen.
- **Status (Akt):** operator-gebunden (Zugang) | **Bindung:** operator
- **Trigger:** Operator-Browser liest die Dateifehlerliste
- **Braucht:** ein Neu-/Nach-Order ist konsenspflichtiger Dritt-Akt — Operator-Wort.

#### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut; `docs/specs/mantis-shrimp-bom.md` trägt ATGM336H GNSS `1005009361234427`, 3,01 CHF ≈ 3,20 €; die H2-Zeile `ESP32-H2-DevKitM-1-N4` (`1005008131868631` ≈ 6,25 $ unverified · DigiKey 26282483 9,68 $).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** Operator-Wort (LOCK-Aufhebung), dann BOM bestellen (inkl. H2).

#### ESP32-Puls-Knoten (Träger ohne Uhr)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25) Code-Eingang `BeatSource::Serial` steht; dedizierter Puls-Knoten unbestellt.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —
- **Wort:** ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session)

#### Weberin-Quellen — account-/key-blockierte Rohkanäle
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) die offenen Zwillingsrouten stehen; IGETS (SFTP-Account), vDEC (Antrag + Vertrag), ONC-Token, TNS-Key (`TNS_API_KEY`/`TNS_UA`) und SuperDARN-Globus sind identifiziert.
- **Blockade:** Konto/Key fehlt.
- **Braucht:** die Anträge/Endpunkte je Konto bereitlegen.
- **Status (Akt):** operator-gebunden (Zugang) | **Bindung:** operator
- **Trigger:** Operator-Wort je Konto
- **Braucht:** Operator entscheidet je Konto (IGETS `igets-support@gfz.de`, vDEC-Antrag, ONC-Token, TNS-Key).

### Extern handelt (Dritte)

#### JUICE-Erdpassage 28./29.09.2026 — Kanal + In-situ-Messung
- **Status:** termin:2026-09-29 | **Bindung:** termin
- **Trigger:** 28./29.09.2026
- **Lage:** (gemessen 2026-09-25) der Prädiktionskanal steht, alle Zellen pending; der Operator-Siegel-Wort liegt noch nicht vor.
- **Blockade:** der Operator muss das Siegel-Wort vor dem 28.09. setzen.
- **Braucht:** vor dem Flyby das Siegel-Wort setzen; nach dem Flyby die JUICE-In-situ-Feldmessung gegen den präregistrierten Feldzustand vergleichen (σ-Metrik gegen fam).

#### Ox64-Lieferung
- **Status:** wartend | **Bindung:** termin (Carrier)
- **Trigger:** Ankunft (`LZ473049629CN`)
- **Lage:** (gemessen 2026-09-25 via `state/mail/mail_ledger.φ`, Mail `1790046330`) PINE64 versandte zwei Ox64 — Ankunft offen.
- **Blockade:** Carrier.
- **Braucht:** Ankunft quittieren; dann M2c (BL808-Host-Port).

#### Postfach
- **Status:** wartend | **Bindung:** extern (Mail)
- **Trigger:** neuer Eingang
- **Lage:** (gemessen 2026-09-26 via `sread state/mail/mail_ledger.φ`, 158 Zeilen) letzte Eingänge: `1790373942` (GitHub-Support-Survey zum PII-Purge-Ticket 4761482 — fremde Linie), `1790370971` (Exa-Werbung), `1790364852` (OpenAlex „Request received" ID #24656). Kein Sensory-Treffer (relay/sensor/beat/mantis/ble/hrv/zigbee = 0).
- **Blockade:** keine.
- **Braucht:** `smail_recv` bzw. `state/mail/mail_ledger.φ` bei Trigger.

#### BGR-Matched-Filter-Ankunft (Tonga) — account-blockiert
- **Status:** wartend | **Bindung:** dritter (vDEC)
- **Trigger:** vDEC-Zugang gewährt
- **Lage:** (gemessen 2026-09-25) die PMCC-Detektionsliste läuft auf einem ~300-s-Raster; die Rohwellenform ist vDEC-account-blockiert.
- **Blockade:** vDEC-Account.
- **Braucht:** nach Zugangsgewährung den matched-filter-Arrival messen.

#### DSN-Briefe in Flug (Voyager/Mariner 10/Viking, Cassini, Juno)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort der DSN
- **Lage:** (gemessen 2026-09-25) fünf DSN-Briefe sind in Flug; der Auftrag `docs/auftrag/archiv/auftrag-sonden-rohdaten-anfrage.md` wurde nach `docs/auftrag/archiv/` verschoben.
- **Blockade:** Antwort ausstehend.
- **Braucht:** die Antwort quittieren.

#### Gaia DR4 + Europa-Clipper-Erdpassage
- **Status:** termin:2026-12-02 | **Bindung:** termin
- **Trigger:** 2.12.2026 (Gaia DR4) / 3.12.2026 (Europa Clipper)
- **Lage:** (gemessen 2026-09-25) Gaia DR4 für den 2.12.2026 angekündigt (Epochen-Astrometrie); die Europa-Clipper-Erdpassage folgt am 3.12.2026.
- **Blockade:** Termin.
- **Braucht:** am jeweiligen Datum die Epochen-Astrometrie bzw. die EC-Magnetfeld-Messung ernten.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
