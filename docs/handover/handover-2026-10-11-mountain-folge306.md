<!--
  title: Handover — Mountain-Folge 306 (2026-10-11)
  session: Mountain-Linie in einem Pass — Ephemeriden Schritt 1 gefahren (Pioneer-10/11-ODF-Residuum gemessen), Asservatenkammer 9/9, FMHY-Klasse verortet
  class: handover
  date: 2026-10-11
  sha256: 804ad20e1dc1b7da770dcf3821b4bbe0efe9caa691892faa57ad76911d31fe43
  status: live
-->
# Handover — Mountain-Folge 306 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge305.md` (→ `archiv/`).
flash only, kein pro/max. Der Operator-Trigger dieser Session: „was ist mit unseren
ephemeriden und warum fängst du die grossen punkte immer wieder an anstatt sie fertig
zu machen" — **Schritt 1 der Ephemeriden-Kette ist jetzt gefahren, nicht neu angesetzt.**

## Burn: open 0.0 · close 0.128 · cap 0.2 — Grund: line 0.0531 + general 0.0615 + explore 0.0135 (gemessen `session_burn`) + 2 lokale Einzel-Bin-Läufe (pioneer10/11_odf_residuum); deepseek-flash, kein pro/max

## Offen (aufgeschlüsselt)

### Beobachtungsoperator + Fit — Schritt 1 gefahren (P10+P11-Residuum gemessen); Schritt 2 offen
- **Status:** eigen (Bau/Messung) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt — Schritt 2 (Residuum verhören: Kette falsch / Modell trägt Rest / Beobachtung trägt Artefakt)
- **Lage:** (gemessen 2026-10-11, zwei lokale Einzel-Bin-Läufe `OMEGAFLOW_HIDDEN=1 cargo run -p omegaflow-measure --bin pioneer{10,11}_odf_residuum`, Daten aus den registrierten CDN-Assets `pioneer{10,11}_odf.bin` + `ephemeris_{earth,pioneer{10,11}_daily}.bin`)
  - **P11:** 27 907 ODF-Samples (Stationen 11/12/14/42/43/44/51/61/62/63, 1974-04-19..1990-10-01) → **24 808 modelliert**; je Station `obs = A·ṙ₂w + B_Pass`, **A gemessen 7,65–9,16 Hz/(m/s)** (impliziert f = A·c ≈ 2,29–2,75 GHz, S-Band); Downlink-only A **exakt 2×** (St 11: 15,331 = 2,0000·7,6656) — struktur, weil die Rate Uplink+Downlink summiert. Residuum-RMS 2,6e2..6,6e4 Hz. `pioneer11_residuum.bin` **1 786 184 B, 24 808 Samples, Roundtrip parst**.
  - **P10:** 59 486 ODF-Samples (14/42/43/61/63, 1973-10-15..1998-07-21) → **59 363 modelliert**; A 3,996 (St 63) … 7,729 (St 43), Station 42 A 367 bei n=86 (Ausreißer); Station 61 A 7,636, RMS **0,169 Hz** bei n=6; Stationen 43/63 RMS 2,1e6/1,8e6 Hz. `pioneer10_residuum.bin` **4 274 144 B, 59 363 Samples, Roundtrip parst**.
  - **Verdikt (nachgeprüft, NICHT aus dem Tool-Label übernommen):** **Vorzeichen gemessen** (A > 0). **Der Maßstab ist NICHT verifiziert** — das Tool druckt `A ≈ +f/c` als hartkodiertes Label und rechnet f/c nirgends; A·c ergibt f ≈ 2,0–2,75 GHz, konsistent mit der ODF-Referenz (~2,3e9 Hz, `odf.rs:2432`/`:2458`). **Riss:** die gedruckte `ref_hz` ist **gemischt skaliert** — P10 zeigt `2.198e7..2.199e7..2.292e9 Hz`, d. h. die Pioneer/Turyshev-ODF-Variante trifft beide Parser-Zweige (`odf.rs:61–86`) mit ~100× unterschiedlicher Skala; ungelöst, welcher gilt. Das **erste Modell (`obs=A·ṙ+B_Pass`) trägt die Serie nicht bis Hz** (RMS 1e2–1e5 Hz); das Tool führt Uplink-Rate, Station und Lichtzeit **bereits** (`odp.rs`/`uplink_rate`) — es fehlen die **Medien** (Troposphäre/Ionosphäre), nicht die Beobachtungsgleichung. Gegen die Horizons-DE440-Tagesephemeride → **`fit-residuum`, kein Blindtest**.
  - **Träger** (neuer Bin, mech. Klon von `pioneer11_odf_residuum.rs`): `tools/measure/src/bin/pioneer10_odf_residuum.rs` — `cargo check -p omegaflow-measure --bin pioneer10_odf_residuum` 0/0. **Zwei Risse:** (a) Ausgabe serialisiert mit `odf::write_p11r_bin` (Formatsname, nicht Mission); (b) die Labels `(≈ +f/c)`/`(≈ 2×)` sind hartkodiert statt gerechnet.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** (1) den `ref_hz`-Skalen-Riss klären — welcher Parser-Zweig (`odf.rs:61–86`) die Pioneer-ODF trifft; (2) Schritt 2 — drei Lesarten (Kette/Modell/Beobachtung); (3) Medien (Troposphäre/Ionosphäre) in die Kette.

### particle-cern — Teil B (Branches) gebaut; TStreamerElement-Liste + Baskets offen
- **Status:** eigen (Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt
- **Lage:** (gemessen 2026-10-11 via `explore`) `src/archivar/root.rs` trägt `inflate_zlib:391`, `decompress_object:573`, `parse_tree:687`, `visit_directory:734`, `walk_trees:774`, `parse_streamer_info_header:797` (endet nach `fLowerBound:835` **vor der Elementliste**). Baskets `fBasketBytes`/`fBasketEntry` ungebaut.
- **Blockade:** TStreamerElement-Elementliste nutzt ROOTs Klassen-Ref-Map (Vorbild `parse_tree:704–710`); verworfen statt unverifiziert ausgeliefert.
- **Braucht:** `parse_streamer_elements(obj, start, n_members) -> Result<Vec<StreamerElement>, _>` an Anker `root.rs:835` (n_members Objekte, je Klassen-Tag/Ref-Map, `kNewClassTag`→Name, sonst Ref-u32; TObject/TNamed-Präfix wie `:829–833`; `fType`+`fSize`); Header um `elements_offset` erweitern. **Baskets = eigener Schritt** (nicht mitbauen).

### FMHY-Quellen-Verdikt — research-data-Klasse verortet (nicht in den Surveys); 12 NEW gemessen
- **Status:** eigen (Quellen-Verdikt) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Verdikt-Schritt je Teilklasse
- **Lage:** (gemessen 2026-10-11) **Riss:** die „74/72-NEW"-Klasse lebt **nicht** in `survey-2026-10-08-fmhy-research-landscape.md`/`survey-2026-10-07-fmhy-forschungsschicht.md` (beide voll gelesen), sondern allein in `state/future/source-kandidaten-fmhy-2026-10-10.md:7` („Academic Papers", 422 Z.). 12 NEW gemessen (`archive_search --verdict`): `chinarxiv.org` 200 direkt · `alphaxiv.org` 200 (Arm `--alphaxiv` besteht) · `scholar.google.com` 200 direkt (ToS) · `citrus-search.com` 200 (Sniff) · `researchgate.net` 403/403/wayback-200 (bot) · `scilit.com` 403/403/wayback-200 (bot) · `zotero.org` 200 · `mendeley.com` 200 · `chunkr.ai` 200 (Sniff) · `sophon.at` 200 · `bulletpapers.ai` 429/429/wayback-200 (rate-limit) · `link.springer.com` 200 (Paywall).
- **Blockade:** die Klasse ist eine Kandidatenliste der Future-Linie, kein Mountain-Register; je Zeile fehlt der Arm/Compiler oder das Verdikt.
- **Braucht:** je Quelle `archive_search --verdict`/`--sniff` (Rest der Liste) + Verdikt-Zeile in `phi/{sources,declined_sources,blocked_sources}.φ` nach `docs/SOURCE_PORT.md`; Auth-Route statt `declined` (Operator-Wort 2026-10-08).

### Asservatenkammer — 9/9 Doks mit erstem Schritt gemessen
- **Status:** eigen (Register/Träger) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt je offenem Marker
- **Lage:** (gemessen 2026-10-11) gedeckt: `survey-2026-09-14-kapitulationen-pendings-inventur.md` 31/32 · `survey-2026-10-08-open-sources-delta.md` 11/13 · `survey-2026-09-03-orphan-verdicts.md` 12/15 · `survey-2026-10-09-redistribution-alternativen.md` 8/10 (Rest `globalfloods.eu`/GloFAS, `opensky-network.org` — kein Register-Träger) · `survey-2026-10-09-domaenen.md` 6/8 (Rest 44-Domänen-Liste `:86`, medizinisches Resultat `:93`) · `survey-2026-09-16-fremde-parser-sammlungen.md` 5/6 (Rest Airbyte-Totalzähler `:159`) · `survey-2026-10-08-fmhy-research-landscape.md` 4/5 (Rest MiMo Studio `:53`) · `survey-2026-10-07-fmhy-forschungsschicht.md` 2/2 · `survey-2026-09-17-omegaflow-legacy-konzepte.md` 2/2.
- **Blockade:** 5 Rest-Marker ohne Träger.
- **Braucht:** je Rest-Marker einen Träger (Register-Zeile/Code) **oder** ein Descope-Befund; `docs/surveys/survey-2026-10-08-research-api-mcp.md` vier Marker setzen (Mycelium-Bitte). Header-sha per `omega_sh sha` nachgerechnet.

### Mycelium-φ-Blöcke — Klima-Blöcke stehen; ecad-Riss + Serien-Manifest offen
- **Status:** eigen (Register/Verdikt) | **Bindung:** eigen (CI-Lauf: mycelium)
- **Trigger:** ecad-Riss entscheiden; je Block der erste CI-Lauf (sha256)
- **Lage:** (gemessen 2026-10-11) die Blöcke **stehen vollständig** in `phi/sources.φ` — SURFRAD `:11292–11310` (terms PD, format surfrad, origin, compiler, **sha256**, at earth, ttl 604800, field) · ECAD `:11172` · DWD-CDC `:21348–21398` · WorldClim `:31404` · AODN `:3` · KNMI `:1712`. Die in folge305 als „Verdikt-Zeilen fehlen" geführte Lage ist damit stale. Offen: der **`ecad`-Riss** (`ecad.eu` Portal vs. Quellhost `knmi-ecad-assets-prd.s3.amazonaws.com`); das **Serien-Manifest** (Compiler erzeugen je-Akt-Instanzen, die Blöcke sind Workflow-Default-Repräsentanten).
- **Blockade:** ecad-Quellhost nicht entschieden; ohne Serien-Manifest trägt jeder Block nur einen Default-Tag.
- **Braucht:** `archive_search --verdict` auf beide ECAD-Hosts + Verdikt-Zeile; Serien-Manifest-Format (Mountain) / CI-Lauf (Mycelium).

### PEP + Tudat — Schritt 0+1 gefahren; Schritt 3 (mehrere Bahnen) offen
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Schritt 3 — weitere Zeugen (Merkur/Venus Radar+VEX-Ranging) durch die Kette
- **Lage:** (gemessen 2026-10-11) Schritt 1 ist mit dem Beobachtungsoperator-Punkt gefahren (Pioneer-10 durch die Kette, `fit-residuum` gegen die Horizons-DE440-Ephemeride). Träger-Doks `docs/concepts/eigene-ephemeride.md`, `docs/surveys/survey-2026-10-10-ephemeris-quellen.md`. Vier Häuser (DE440/INPOP19a/EPM2021/PETREL19) als `ephemeris_*` registriert.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Schritt 3 — der Operator auf weiteren Zeugen gegen weitere Häuser; Kette CI-nah verankern.

### FMHY-Routing — Arm-Hälfte gegenstandslos; Riss 4 bot-gated
- **Status:** eigen (Architektur) | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss
- **Lage:** (gemessen 2026-10-10, unverändert) Grenze ist die Manifestations-Achse: Query → `archive_search`-Arm; Messwert → `phi/sources.φ` + Compiler; Operator-Werkzeug → `tools/`; Blick/Portal → Lead. Riss 4 (Overpass — HTTP 406 direkt+Proton → bot-gated `pending`). KNMI gebunden via EDR; Mindat wartend.
- **Blockade:** Riss 4 bot-gated.
- **Braucht:** Riss 4-Abschluss (bot-gated) oder Descope.

### blocked_sources — 1 offene Klasse (particle-cern)
- **Status:** eigen (Bau/Register) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt
- **Lage:** (gemessen 2026-10-11) `phi/blocked_sources.φ` trägt **1** `blocked parser-def`: `::gap:particle-cern ×1`. PDS-PPI-Arm und GWOSC-HDF5-Arm stehen.
- **Blockade:** TBranchElement-v9-Decode (siehe particle-cern).
- **Braucht:** (1) particle-cern-Decode; (2) PDS-PPI Manifest-CI-Lauf `pds-ppi-cdn.yml` + Register-Zeile (Mycelium); (3) GWOSC/LOSC Registrierung + sha256 (siehe GWOSC-Strain).

### GWOSC-Strain — Block steht ohne sha256; CI-Lauf queued
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** `gwosc-cdn`-Lauf `38096865460` Abschluss
- **Lage:** (gemessen 2026-10-11) Operator-Wort **A** (eigene `quantity`-Klasse, `relative`); Block `phi/sources.φ:22007` steht (`url`/`terms CC-BY-4.0`/`format losc`/`origin`/`compiler`/`on earth 0 0 0`/`ttl 31536000`/`quantity losc_strain …`); **sha256 fehlt** (kein Lauf). `ci_manage view 38096865460`: **queued**, Ergebnis nicht gelesen.
- **Blockade:** kein CI-Ergebnis.
- **Braucht:** `ci_manage view 38096865460` → sha256-Zeile in den Block `:22013` nachtragen.

### ci-gate-Rot — `pc.rs` von der Fremd-Linie committet; nächsten Lauf messen
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am HEAD
- **Lage:** (gemessen 2026-10-11) die Mountain-Heilung `mci.rs`/`parcorr.rs`/`receiver.rs` ist committet; `pc.rs` wurde von der parallelen River-Linie committet (`10e0d1323 river 177: fill the screened set in pc_stable_skeleton_screened`). Die `ci-gate`-Läufe am HEAD sind queued/in_progress — kein Ergebnis gelesen.
- **Blockade:** kein abgeschlossener Lauf.
- **Braucht:** `ci_manage log <ci-gate-id>` auf dem nächsten abgeschlossenen Lauf → grün/rot messen; bei rot die verbliebenen Clippy-Lints unter `-D warnings` tilgen.

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09, unverändert) 157 ODF-Referenzen; `doppler.rs` absent; Wahrheit `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### iEEG — registriertes Wort 2026-10-06 maßgeblich
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort, das den Riss über 2026-10-06 hebt
- **Lage:** (gemessen 2026-10-11, unverändert) iEEG = privates Experiment (`state/zustand/wartend.φ:40`), kein CDN.
- **Blockade:** keine.
- **Braucht:** kein Schritt — nur ein neues Operator-Wort öffnet es.

### GIC-Paper — Trigger: Mycelium-Artefakt
- **Status:** wartend | **Bindung:** mycelium (Träger folge295 `#te-ground-truth`)
- **Trigger:** `te-bias-n`-Lauf `38038722712` Abschluss → Mycelium meldet den Ground-Truth-Abschnitt
- **Lage:** (gemessen 2026-10-11, unverändert) `te_ground_truth` in `.github/workflows/te-bias-n.yml:48`.
- **Blockade:** kein CI-Ergebnis.
- **Braucht:** nach Mycelium-Meldung — Paper §3.5/Abstract/§7 nachziehen.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt ist Annahme-Akt
- **Status:** wartend | **Bindung:** eigen (Mountain-Feder)
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile gesetzt (Operator-Hand; Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-10) Kein offener Datensatz trägt eine im Datensatz gemessene Koordinate; x-Kern-Serien (OpenAQ/Open-Meteo/NASA POWER) stehen wie gemessen.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt — dann `phi/sources.φ`-Zeile (Mountain-Feder), dann Te-Paar-CI-Feed (Mycelium).

### phi/pipeline nach Verbraucher trennen (Future-Paket `483709e`) — 3 Naturen getrennt; Register + Vorrat offen
- **Status:** eigen (Bau/Register) | **Bindung:** eigen
- **Trigger:** je Klasse der Konsumenten-Umbau (`cargo check` 0/0) → dann verschieben/entfernen
- **Lage:** (gemessen 2026-10-11) `phi/` = **308 Dateien, 10 Ordner** (110 getrackt, ~198 lokal/ignored) → nach dem Descope: **`phi/pipeline/` 74 Dateien** (von 280). **Getrennt (dieses Atom, 3 Naturen):** (1) **Kanal-Deskriptoren** `descriptors/*.te` (20) → `src/mathematikerin/descriptors/`, Konsument `field_te_query.rs` (`d039f7618`); (2) **Compiler-Eingaben** `asteroid_gm_{inpop25c,sb441}.φ` (getrackt) + `asteroid_diameters_{neowise,akari}.φ` → `tools/harvest/data/asteroid/`, Konsumenten `dastcom_compiler.rs:10–11`, `ephemeris_compiler.rs:1537`, `kernel-flatten.yml:89/113/114`, `canon.φ`, `MANIFEST.φ` (`dc9a2e792`); (3) **Test-Fixtures** 8 Dir `catalog/{twomass,ncei_ssi,ncei_goes_xrs,noaa_goes16,noaa_gk2a,noaa_wod,cosmic_wetprf,celestrak_eop}` → `src/archivar/testdata/`, 5 Konsumenten `src/archivar/{hdf5,netcdf,twomass}.rs` + `goes_abi_compiler`/`cosmic_ro_compiler` (`01449065f`). **Descoped (dieses Atom):** lokale Vorratsmasse `research/` (98, kein Code liest sie) · `stage/` (33) · `queue/` (2) · `leads.φ`; getrackte **Katalog-Indizes** `catalog/tap_index_*.φ` (72, aus canon + `.gitignore`) + der tote Konsument `tap_index_merge.rs` (`1a885775f`). **Offen:** (a) **Verdikt-Register** `ledger.φ`/`index.φ`/`decline_lens.φ` — `descoped`; die `ledger.φ`-`pending`-Zeilen (THEMIS GMAG, Wind SWE/MFI, VLF AWESOME) müssen ins Warte-Register; Konsumenten `register_lookup.rs:14/53/54/71/72`, `source_physics_lint.rs:39`, `silence_map_probe.rs:431`, `regtap_census.rs:128`; (b) **Kandidaten-Vorrat** Rest `probe_*.φ`, `library.φ`, `frame_registry.φ`, `master_urls.txt` — Konsumenten `discovery.rs:5–8`, `register_lookup.rs:60–65`, `probe_sweep.rs`, `frame_registry.rs:3`, `port.rs`.
- **Blockade:** (a)/(b) brauchen den Konsumenten-Umbau voraus; (b) ist ein Live-Arm-Umbau (`discovery.rs`), kein Verschieben.
- **Braucht:** je Klasse **ein Atom**: Konsument umbauen (`cargo check` 0/0 wie bei den Deskriptoren), dann verschieben/entfernen — nicht Datei-für-Datei löschen. Danach trägt `phi/pipeline/` eine einzige Natur (oder entfällt).
- **Protokoll-Riss:** das Paket kam über den Operator; **Futures privater `## An mountain`-Block wird von `register_lookup --addressed mountain` nicht erfasst** (nur öffentliche Handover) — die Faltung lief über den Wortlaut des Operators, nicht über `register_lookup`.

### phi/ Ordnung — 308 → 14 Dateien, flach in phi/ (keine Unterordner); tracked == declared 14/14
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) **`phi/` = 14 Dateien** (von **308**). **Geblieben (lebend, alle getrackt/deklariert):** **8 Kern-Register** (`sources`/`dead_sources`/`declined_sources`/`blocked_sources`/`witnesses`/`footprints`/`harvest`/`canon`) · **`nrs_stations.φ`** (Harvester-Eingabe + `register_lookup`) · **2 `bindings`** (`bands`/`dust-maske`) · **`pipeline/library.φ`** (gelernte Such-Linse → `discovery.rs`/`source_scanner`) · **`pipeline/catalog/korpora_heim.φ`** (Lizenz-/Redistributions-Verdikt → `korpora-cdn.yml`) · **`pipeline/catalog/solar_omega_g.φ`** (→ `kernel-flatten`/`ephemeris_compiler --omega-g`) · **`reports/scan_coverage.φ`** (→ `disappearance_probe`/`mycelium_fan_navigator`). **Katalog-Dispositionen descoped (`3bc61e8af`):** `copernicus_disposition.φ`/`noaa_nodd_disposition.φ` — die Bau-Warteschlange war **stale** (13 der 17 NOAA-`compiler-lease`-Compiler existierten längst, die Quellen stehen in `phi/sources.φ:1121/1085/11640/11672/11700/3142` + `harvest.φ:227`); ihre `~310` `descoped`-Verdikte duplizierten eine live neu trierbare Ablehnung. Die **12 echten `pending`** (8 Copernicus-In-situ-Netze + NOAA `coastal-lidar`/`eri`/`global-hourly`/`oar-hourly-gdp`) sind nach `state/zustand/wartend.φ` getragen. Frühere Descopes: `MANIFEST`/`bayestar19`/`bathymetrie-gebco`/`gate_state`/`sources_index` · Vorratsmasse `research`(98)·`stage`(33)·`queue`(2)·`leads.φ` · ~40 MB Katalog-Dumps + `tap_index_*`(72) · `meteo/*.json`(4) + probe-sweep(2) · statische Corpora · `supermag_stations`/`frame_registry`/`prompt` · Pipeline-Register `ledger`/`index`/`decline_lens`/`probe_*`.
- **Blockade:** keine.
- **Braucht:** nichts Offenes. **Fabrikations-Befund (`a52d9f176`):** `phi/pipeline/catalog/solar_omega_g.φ` (`sun 1277 10`) hatte **keine Quelle** — 1 Zeile, Commit leer, kein Doc/Journal nennt die Herkunft des solar-ΩG-Werts → entfernt; der gravity-Kanal der Sonne bleibt **abwesend** (0 honored), beide `--omega-g`-Workflow-Haken raus. Die Register-Pflicht steht in `state/zustand/wartend.φ` (`solar-omega-g`). Doc-Träger (ΩG-Zeilen auf „entfernt" korrigiert; ihre `pending`/`offen`-Wörter sind topisch, kein Register-Zustand): `docs/concepts/zeugnis.md`, `docs/concepts/die-weberin.md`. `korpora_heim.φ` bleibt (Lizenz-Verdikt; `korpora-cdn.yml` dormierend). Gate-Regel `canon_format_violations` auf jeden `phi/`-Pfad erweitert (`1eadd4a32`).

### api.sensor.community — zulassen (der neue Faden), Zielzeile unbenannt
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** das wörtliche Future-Wort (welcher Arm/welche Zeile)
- **Lage:** (gemessen 2026-10-11) `https://api.sensor.community/` → HTTP 200, sha256 `ad345805c333430eeb3fd3e6e40fdf1f29b3429046df4c180ee3f17e7ebc3ae3`, Body `{"push-sensor-data":"…/v1/push-sensor-data/","now":"…/v1/now/"}`. `sensor.community` ist bereits registriert (`phi/sources.φ:338/339/1218/1219` data/maps; `dead_sources.φ:324`; `declined_sources.φ:1515/1519/1520` SPS30-Parser-Gap).
- **Blockade:** „zulassen" nennt keinen Arm und keine Zeile; `api.sensor.community/v1/now` (live) wäre ein neuer Arm — ungemessen.
- **Braucht:** das wörtliche Future-Wort, sonst wird ein Doppel-Block geschrieben.

### 26 queue/stage + `ledger.φ` + Katalog-Indizes — `descoped`
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** das Descope-Verdikt je Klasse
- **Lage:** (gemessen 2026-10-11) `phi/pipeline/queue/` **2**, `stage/` **33**, `tap_index_*` **72** (in `canon.φ:55–123` deklariert), `phi/pipeline/ledger.φ` (6 offene Zeilen per `register_lookup --open`).
- **Blockade:** ohne Konsumenten-Umbau hinterlässt das Löschen schreibende Pfade (`port.rs`, `leads_merge`, `probe_sweep`).
- **Braucht:** je Klasse Messung + Descope-Zeile; `phi/canon.φ` von den `tap_index_*`-Zeilen lösen.

### Keine Leads-Datei — live suchen, on demand zulassen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** `discovery.rs` live + `leads_merge` entfernt
- **Lage:** (gemessen 2026-10-11) `phi/pipeline/leads.φ` + `tools/utils/src/bin/leads_merge.rs:70/90/91` (schreibt `leads.φ` aus `queue/`+`stage/`) + `tools/utils/src/discovery.rs`.
- **Blockade:** `leads_merge` liest gerade den Kandidaten-Vorrat.
- **Braucht:** `discovery.rs` auf live (`archive_search`), `leads_merge` + `leads.φ` entfernen.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf lokal/silent,
  nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen:
  der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)
„bitte die grossen punkte parallel mit agenten abarbeiten und ABSCHLIESSEN, NICHT VERSCHLEPPEN" | 2026-10-11 | Operator (Session, Mountain 305)
„was ist mit unseren ephemeriden und warum fängst du die grossen punkte immer wieder an anstatt sie fertig zu machen" | 2026-10-11 | Operator (Session, Mountain 306)
„bypass-mirror 23 (UrhG-§95a- was bedeutet das die möchte ich bitte raus haben keine fragwürdigen links" | 2026-10-10 | Operator (Session, Mountain 303)
„können wir nun eine untersuchung machen was davon als arme in archive search sollte, was in tools und was in phi dateien?" | 2026-10-10 | Operator (Session, Mountain 303)

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. **Dieses Atom (Mountain 306):** Ephemeriden-Schritt 1 gefahren (P10+P11-Residuum),
Asservatenkammer 9/9, FMHY-Klasse verortet, drei begrenzte Dispatches (general/grind-flash/explore).
- **Ephemeriden Schritt 1 gefahren:** `pioneer{10,11}_odf_residuum` lokal (Einzel-Bin, `OMEGAFLOW_HIDDEN=1`);
  A ≈ +f/c je Station, Downlink ≈2×; Vorzeichen gemessen, Maßstab ein hartkodiertes Tool-Label (`ref_hz`-Skalen-Riss offen);
  erstes Modell trägt die Serie nicht bis Hz (Medien fehlen); `fit-residuum`, kein Blindtest. Neuer Bin `tools/measure/src/bin/pioneer10_odf_residuum.rs` (cargo check 0/0).
- **phi/ Ordnung:** `phi/` 308 → **17 Dateien**; `tracked == declared` **17/17**. Jede Restdatei von einem Agenten angesehen; descoped: Katalog-Dumps ~40 MB + `tap_index_*`(72) + `research`(98)·`stage`(33)·`queue`(2) + Pipeline-Register `ledger`/`index`/`decline_lens`/`probe_*`; **Einzel-Experimente** `meteo/*.json`(4) + probe-sweep(2); statische Corpora gegen Live-Arme; `supermag_stations` (selbst-descoped) · `frame_registry` (Schreib-Snapshot) · `prompt` (stale) · `MANIFEST` (kein Format-Konsument) · `bayestar19`/`bathymetrie-gebco` (kein Code-Leser) · `gate_state`/`sources_index` (stale Ableitung). Gate auf jeden `phi/`-Pfad erweitert (`1eadd4a32`). 3 Naturen getrennt: Deskriptoren `d039f7618` · Compiler-Eingaben `dc9a2e792` · Test-Fixtures `01449065f`.
- **Asservatenkammer 9/9 Doks** mit erstem Schritt gemessen; 5 Rest-Marker ohne Träger benannt.
- **FMHY:** research-data-Klasse liegt in `state/future/source-kandidaten-fmhy-2026-10-10.md:7`, nicht in den Surveys (Riss); 12 NEW gemessen.
- **Mycelium-φ-Blöcke:** als stale belegt — die Klima-Blöcke stehen vollständig (SURFRAD sha256 verifiziert).
- **particle-cern:** nächster begrenzter Schritt exakt verortet (`root.rs:835`, `parse_streamer_elements`).
Geteilter Baum: `tools/utils/src/bin/archive_search.rs` und die drei `tools/harvest/src/bin/*_coverage.rs`
gehören fremden Linien — nicht angefasst/committet.
Eigene Pfade dieses Atoms (committet, `git show --stat`): die 6 Commits `d039f7618` · `dc9a2e792` · `01449065f` · `b5b46739c` · `1eadd4a32` + die früheren (Ephemeriden/Folding) · diese Übergabe · der Archiv-Move folge305.
