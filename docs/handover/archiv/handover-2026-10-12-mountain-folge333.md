<!--
  title: Handover — Mountain-Folge 333 (2026-10-12)
  class: handover
  date: 2026-10-12
  sha256: 5852c9060ac26719f158efe29afb5e567362baaaeea28791d707c0df729fe1b4
  status: live
-->
# Handover — Mountain-Folge 333 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; zitiert: die Fassung
2026-10-12T18:31Z, HEAD `a32787a49` — älter als dieser HEAD). Diese Session konsumierte
`handover-2026-10-12-mountain-folge332.md` (→ `docs/handover/archiv/`). `--fired`/`--stale
--persist 3` = leer; `--addressed mountain` = 3 Blöcke (future-247, mycelium-333, river-200 —
unten gefaltet). **Jeder offene Punkt nennt seine Datei(en)/Pfade.**

## Burn: open 0.00 · close 0.00 · cap 0.50 — Grund: Ein-Pass-Atom (die `decline`-Gate-Fixture + `decline`-Scan gebaut, mariner10-Ephemeride geschlossen, adressierte Blöcke gefaltet), deepseek-flash, kein pro/max. Die close-Zahl setzt `session_burn` nach Session-Ende.

## Offen (aufgeschlüsselt)

### Auftrag „Eigene Ephemeriden + Kulturerbe" — Phase 0/1 (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Operator-Wort zum Start
- **Lage:** (gemessen 2026-10-12) `docs/auftrag/auftrag-eigene-ephemeriden-und-kulturerbe.md`
  (sha `4a9dac1d…`). Rat + UI konvergent: EIN Archivar, zwei Register
  (`obs`/`heritage`/`observable`/`asset`); `house_gate` → Konsistenz-Hülle, ehrliches Gate =
  **Hold-out**; minimaler Operator mit `pending` (nie 0.0); **Riss 1** M10-Bild = Erbe;
  **Riss 2** Anker reisen mit; `raw_level` 0/1/2; **Ordnung** Rahmenschicht → LLR (relativ) →
  ODF als Prüfling → Radar → ΔDOR/VLBI → Tiefraum; **Erbe→Brennstoff** beim Stand der inneren
  Region. Stimmen: `state/stimmen/2026-10-12_ui_eigene-ephemeriden-verdikt.md`.
- **Blockade:** (**gemessen future-244-2, gefaltet**) CDDIS/Earthdata ist **kein Key-Gap** —
  `EARTHDATA_EDL_TOKEN` gesetzt, `archive_search --verdict` auf `/archive/`,
  `/archive/slr/data/npt_crd/`, `/archive/vlbi/ivsdata/vgosdb/` = **HTTP 200**. Operativ
  servergebunden bleiben allein die **DSN-Delta-DOR**-Daten (autorisiert, kein Key). LLR-Beschaffung
  = Politik (Operator/Future).
- **Braucht:** Phase 1 = ITRF2020 (`https://itrf.ign.fr`) + IERS-EOP + LLR (ILRS/CDDIS,
  historisch POLAC/Z&N) + Operator `canObserve`.

### M10-Bildpfad — Riss „Erbe → Brennstoff": die Auflösung braucht die eigene M10-Bahn (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Fund einer M10-Radio-Tracking-Quelle (Zweiweg-Doppler/Range) oder das Operator-Wort
- **Lage:** (gemessen 2026-10-12/328) `camera_angles.txt` (`phi:31602`) trägt **zwei Rollen**: die Sondenposition `SX SY SZ` (**DE/SPICE-abgeleitet**) UND die Kameraorientierung `C1C2C3`. Solange **beide** aus der Datei kommen, ist jedes M10-Bild ein Re-Ingest des Hauses → **Erbe**. **Gemessen fehlt die eigene M10-Bahn:** SPDF `mariner/mariner10/celestial_mechanics_and_radio_science/` trägt nur die Venus-Okkultation (`red_tele_signal_data_venus_occlt`; als `mariner_occlt` registriert, `phi:20413`); `mariner_rst` ist **M6/7**, `mariner4_odg` ist **M4** — **M10-Zweiweg-Doppler/Range = 0 im Register** (**korrigiert future-244-3**: M6/7-Roh-Doppler und M4-ODG liegen geerntet vor; nur M10 fehlt als Track). **Auflösung (Entwurf; gehört durch die fünf Stimmen):** (1) Position ← **eigene M10-Bahn** aus Radio-Tracking; (2) Attitude ← aus den **Bildern selbst** (ISIS-cnet/Sternfeld, future-244); dann Bild → **Merkur-Richtung im ICRF** → O−C gegen DE/INPOP/EPM. Ohne (1)/(2) bleibt der Bildpfad **Erbe** — ehrlich benannt.
- **Blockade:** **kein M10-Roh-Tracking online** (**hart gemessen future-244-5, gefaltet**) — die 762 7-Spur-Tapes `MVM_1001–1007` (SDDPT `73-085A-0008`, NSSDC `PSCM-00009`) sind offline (kein Mirror: IA/Zenodo/OSF/GitHub/Wayback = absent); SPDF = nur Venus-Okkultation; SEDR `PSPA-00273/00055/00318` offline. Die NSSDC-Anfrage `PSCM-00009` ist seit 2026-09-16 offen (`state/mail/nssdc-mariner10-pscm-00009.md`).
- **Braucht:** (1) **PDS-PPI `MARINER_10`** `M10-H-POS-6-M1-FLYBY-TRAJ-V1.0` (SEDR-Pose, kein Roh-Tracking) — als `url` registrieren (Mycelium) und gegen die geheilte Ephemeride kreuzen; der **Roh-Track** bleibt die NSSDC-Anfrage (**Operator-Hand**); (2) der **unabhängige Attitude-Solve** aus den Bildern (ISIS `autoseed`/`pointreg`/`jigsaw` gegen MDIS-Basemap/DEM 166/665 m, `xfer:future-244-1`); (3) das Hold-out-Fenster **vor** dem Fit fixieren.

### M10-EDR Winkeldurchmesser — Riss: EDR-Korpus ↔ M10-Ephemeride disjunkt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine M10-Ephemeride, die den Venus-Encounter trifft
- **Lage:** (gemessen via `tools/measure/src/bin/m10_expected_diameter.rs` + future-241) EDR-Asset `mariner_m10_edr.bin` (`phi:31385-31390`, `mvm_0001.tgz`, `ser.im-ldi.com`) = 651 Frames, alle **Venus 1974-02-05/06**; `data/ssd.jpl.nasa.gov/ephemeris_mariner10.bin` deckt nur **Merkur-1 (JD 2442128–2442141)**. NAIF `…/naif/M10/kernels/spk/` nur `M10_archive_1.bsp`. Die M10-Pose je Frame liegt in `https://ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt` (194 640 B, sha `0a65df59…`; 811 Bilder 1974-03-29…1975-03-17).
- **Blockade:** keine breite M10-Ephemeride (falls doch nötig); der `camera_angles`-Parser-Arm fehlt nicht (gebaut).
- **Braucht:** JPL Horizons `-76` (`tools/harvest/src/bin/horizons_compiler.rs`) oder Merkur-Tapes `mvm_0002+` (`ser.im-ldi.com/M10/MVM/`).

### M10-Kontrollnetz (ISIS) + M10↔MDIS — Quelle gemessen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-/Parser-Pass
- **Lage:** (gemessen 2026-10-12) **ISIS-Quelle verifiziert:** `astrogeology.usgs.gov/software/ISIS` = **HTTP 404** (tot); live: `isis.astrogeology.usgs.gov`; Quell-Repo `github.com/DOI-USGS/ISIS3`, `LICENSE.md` = **CC0-1.0 / US-publikumsfrei** (1 761 B, sha `7162bb20…`). `camera_angles.txt` (194 640 B, sha `0a65df59…`) deklariert nur `JULIAN_DATE&FDS`/`SXSYSZ`/`C1C2C3` — **keine Einheit**; `C1C2C3` = die drei Kamera-Orientierungswinkel (Brennweiten A 1493,6 / B 1500,1 mm); `SX SY SZ` = Sondenvektor. Zukunft: `state/research/m10-mdis-sonke-robinson/MANIFEST.md` (Sonke & Robinson 2026, PSJ 7:35, `10.3847/PSJ/ae32f5`, CC BY 4.0).
- **Blockade:** `url`/`origin` (Mycelium) für die ASU-/ISIS-Quellen; `SXSY SZ`-Einheit/Frame nicht in der Datei selbst belegt.
- **Braucht:** Mycelium `url`-Zeile für `ser.im-ldi.com/M10/M10_ISIS/` (US-PD) → `terms`/`format` Mountain → die Sonke/Robinson-Tabellen 1–8 als `url`-Zeilen (Mycelium).

### M10 `camera_angles.txt` — Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der erste Extraktions-Lauf des Asset
- **Lage:** (gemessen 2026-10-12/327) 2 433 Zeilen = 811 Records × 3 Zeilen. **Gebaut:** `src/archivar/camera_angles.rs` (`parse_camera_angles`), `pub mod camera_angles;` in `src/archivar/mod.rs`; Dispatch `format camera_angles` in `src/archivar/extract.rs`; `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** die **FDS→Bild-Dateiname**-Zuordnung beim ersten Lauf prüfen (der FDS ist die Bilddatei, nicht die Zeit; JD/FDS nicht ko-monoton). `terms PD https://ser.im-ldi.com/M10/` ist gesetzt.

### `mariner_rst` — SP-Begleitbänder, eigener Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Parser-Pass
- **Lage:** (gemessen 2026-10-12/327) **Riss geklärt:** `DD005407`/`DD005409` sind die **Single-Precision-Begleitbänder** derselben M6/M7-Datensätze (DSC 0073, sha `02b97558…`). Der ODG-Data-File-Kopf ist **drei** Records — 10-Wort-Label + 201-Wort-Station-Summary + **97-Wort-Summary** = **308** Wörter. **Gebaut (Taucher):** SP/ODG-Reader in `tools/harvest/src/bin/mariner_rst_compiler.rs` auf variablen D-Record (`D = 6 + 2·N`) und die 201-Wort-Daten-Record-Grenze umgestellt; Tests grün; `cargo check -p omegaflow-harvest --bin mariner_rst_compiler` 0/0.
- **Blockade:** das **M6/M7-SP-spezifische** Feldlayout ist im Baum nicht transkribiert (nur DSC-0073-sha); die Frequenz-Skalierung (roh ~1.08e12, `/2⁹` ≈ 2114 MHz) ist nicht dokumentiert.
- **Braucht:** **Taucher-Messung (2026-10-12/331): `pending` — die Skalierung ist in keiner Baum-/Netzquelle belegt.** Der DSC-0073-Datenrecord typisiert die Frequenz als **DPFP, nicht SPFP**; der `/2^9`-Faktor ist ein numerischer Fit, keine dokumentierte Regel. Die DSN-Tape-Format-Spezifikation (`TM GTB900-1`) fehlt im Baum; NTRS `19730023976` ist `METADATA_ONLY`. SP-Band-`url` via Mycelium (`…/PSCM-00017_DD005407_…tar`, verdict HTTP 206) offen.

### DSC 0119 / M4-Feldtabelle — Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der `mariner_rst`-Parser-Pass
- **Lage:** (gemessen 2026-10-12/327) `state/mountain/mariner4_dsc0119_format.md` (ODG/ODP-Record, 6-bit-Byte-Walk, Zeit dekodiert). **Der Vollscan** (`state/research/spdf-catalogs/all_catalogs/dsc_0119.pdf`, 65 S., 5 746 835 B, sha `fee7b06e…`) enthält **TM GTD 900-1** komplett; **Table 3** transkribiert (`D = 6 + 2·N`); Fig. 2 + Appendix A (DC1–DC7). Der `sp_odg_logical_records`-Walk ist auf variablen D umgestellt. **Registriert:** die *Auszug*-Datei `phi:31588-31590`; die **Volldatei** ist noch nicht registriert.
- **Blockade:** keine (Feldtabellen vollständig gelesen).
- **Braucht:** die Volldatei als `url` (Mycelium).

### `intermagnet_dbdt` — Resolver-Regel (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die Fenster-Disjunktheit der vier Arme
- **Lage:** (gemessen 2026-10-12/330) Resolver gebaut (Mycelium `3467ec4f9`); Verdikt `state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md`. **Die Zelle ist gemessen** (Lauf `38215224525`, head `3467ec4f9`, Job `matrix-newell-omni`): der Resolver greift (Riss `abk_dbdt_1m` vs `_1h`, corr 0,9388, feinster Zeuge 1m/n 527039), aber die Zelle liefert **keinen Rung**: `0 of 1 cells pass`. `align_many` verwirft mit `Err("bin window carries no overlap between the arms")` — die vier Arme (3 OMNI-Carrier + ABK-Band) teilen **kein gemeinsames Zeitfenster**.
- **Blockade:** der Grund der Fenster-Disjunktheit (live-OMNI-Fenster vs. ABK-Archiv-Fenster) ist ungemessen.
- **Braucht:** die Zeit-Fenster der vier registrierten Arme messen (`first`/`last` je Serie) und den Descriptor `src/mathematikerin/descriptors/newell_geospheric_omni.te` — **nie lokal**, Job `matrix-newell-omni` in `.github/workflows/field-te-query.yml` — auf ein gemeinsames Fenster stellen.

### Quellen-Terms — `decline`-Disposition (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12/333) **Gebaut (dieses Atom):** die Gate-Fixture `phi-sources-decline-token` — `prose_violation_for` (`src/gate/commit_gate.rs:2300-2319`) blockt jede `decline`-Zeile in `phi/sources.φ` ohne Token aus `{unphysical,model,registry,commercial,superseded}` **oder** ohne URL; Fixtures `phi_sources_decline_valid`/`phi_sources_decline_bad_token` (`src/gate/commit_gate_vocab.json:517-518`); Test `fp_sources_decline_token_closed_set` (`commit_gate.rs:4870`). **Der Scan:** `collect_disposed_urls` (`tools/register/src/bin/register_lookup.rs:1157-1161`) sammelt jetzt auch `decline`-URLs (der Katalog-Kandidaten-Abgleich liest sie); `cargo check` 0/0. WWLLN re-gehomt (`phi:33083`), Survey-Zeile `survey-2026-09-07-weberin-thread-matrix.md:168` korrigiert.
- **Blockade:** keine.
- **Braucht:** die übrigen getilgten Verdikte als `decline <klasse> <url>`-Zeilen re-homen — Schritt: die Drop-Historie der Registerblöcke via `git log` scannen, je Verdikt die Klasse setzen.

### Re-Admission der lizenz-blockierten Kandidaten (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Verdikt-Pass
- **Lage:** (gemessen 2026-10-11/12) ALMA→ESO CC BY 4.0, GRDC→GRDC-Caravan CC BY 4.0 / USGS NWIS CC0 1.0 / Copernicus GloFAS CEMS-free, RIPE RIS→RouteViews CC BY 4.0, **IMCCE INPOP/NOE-4 → erledigt: Open Licence 2.0**; TRISP→NIST NCNR NSE US-PD; NC-admissibel: INTERMAGNET (CC BY-NC 4.0), GIRO DIDBase (CC BY-NC-SA 4.0).
- **Blockade:** die NC-Admission ist eine Operator/Rat-Frage; Admission-Zeilen je Kandidat fehlen.
- **Braucht:** je Kandidat die Admission-/`terms`-Zeile in `phi/sources.φ` (Mountain) + `url`/`origin` (Mycelium); die NC-Kandidaten dem Operator vorlegen.

### Rätsel-Datenlücken (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Zeilen (Mycelium)
- **Lage:** (gemessen 2026-10-12) JADES `7460116` · UNCOVER `14281664` (`phi:32408`, `format fits` + CC-BY-4.0) · BK18 (erledigt) · GIC-Mäntsälä (FMI nur `space.fmi.fi`) · FMI-NUR `?month=YYYYMM&stations=NUR` (200, CC BY 4.0, Format undetermined) · NIST NCNR NSE (`https://ncnr.nist.gov/pub/ncnrdata/nse/`, US-PD) · Teil C A5 `GO-SS-RSS-1-TDF-V1.0`.
- **Blockade:** `url`/`origin` (Mycelium); FMI-NUR-`format` (GET-Body opaque).
- **Braucht:** je Lücke die `url`-Zeile → `terms`/`format` → Parser.

### NIST NCNR NSE · Teil C A5 · opentopography · SWPC/RTSW · Sechs Klima-Quellen · eigene Ephemeride · Frame/Epoche-Sammel (eigen/wartend/blockiert)
- **Status:** eigen/wartend/blockiert | **Bindung:** eigen/termin:2026-11-01
- **Trigger:** je der benannte (CDN-/Manifest-Lauf, `url`-Zeile, Freigabe `src/mathematikerin`, Termin 2026-11-01)
- **Lage:** (gemessen 2026-10-11/12) NIST NCNR NSE braucht `url` (Mycelium `4b5fe3fdb` baut den HDF5-NeXus-Arm); Teil C A5 `GO-SS-RSS-1-TDF-V1.0` braucht `url`; opentopography_coverage braucht Manifest; SWPC/RTSW braucht Archivreihe-Verdikt +`ttl`; Sechs Klima-Quellen (`series-manifest`) brauchen den ersten Lauf; eigene Ephemeride blockiert an `src/mathematikerin`; Frame/Epoche CRS-VLR ungemessen.
- **Blockade:** `url`/Manifest (Mycelium); die Freigabe von `src/mathematikerin`.
- **Braucht:** je `url`-Zeile (Mycelium) → `terms`/`format` (Mountain); Läufe lesen (`ci_manage log <id>`); iEEG **nicht erneut vorlegen** (Operator-Wort 2026-10-06).

### Plasma-`channel`-Direktive für die §D-Zeugen (eigen; `xfer:river200-2`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Registrierung der §D-Zeugen-Quellen (`state/river/gic-wunschliste-erweiterung-2026-10-12/`)
- **Lage:** (gemessen 2026-10-12/333; adressiert river-200, gefaltet) River/Rat haben `Plasma` als vierte Medium-Klasse entschieden (`src/mathematikerin/channel.rs`, `cargo check` 0/0). Die §D-Zeugen (Riometer-CNA · Ionosonde-foF2 · ISR-Ne/Te/Ti · GNSS-TEC · ELF/VLF · Airglow/FPI · BARREL) tragen künftig `…:plasma:…` (Region als Datum, `pending` bis IRI-2020/NRLMSISE-00/IGRF; Kanon `docs/concepts/kanal-ontologie-komplettbau.md:205-247`). **Gemessen:** **kein** Zeuge trägt eine `witness`/`channel`-Zeile in `phi/sources.φ` (`sgrep -c` Riometer/Ionosonde/BARREL = 0).
- **Blockade:** die §D-Zeugen sind noch nicht als Quellen registriert; die Rivern-Wunschliste liegt unter `state/river/gic-wunschliste-erweiterung-2026-10-12/` (`riometer-all.txt` · `ionosonde-all.txt` · `isr-all.txt` · `elfvlf-all.txt` · `fpi-all.txt`).
- **Braucht:** je §D-Zeuge die Quelle (`url`/`origin` Mycelium) → `witness`-Block + `channel …:plasma:…`-Direktive (Mountain).

## Taucher-Messungen 2026-10-12 (offen)

### Auftrag Phase 0 — Landschaft je Beobachtungsklasse (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** Operator-Wort Phase 1
- **Lage:** (gemessen 2026-10-12/327, `general`-Taucher) fünf Konsolidate unter `state/research/auftrag-ephemeriden-phase0/`: `a-llr.txt` (15 Kandidaten) · `b-odf.txt` (24) · `c-planetary-radar.txt` (15) · `d-delta-dor.txt` (17) · `e-ephemerides.txt` (15). Terms durchweg `unknown`.
- **Blockade:** keine.
- **Braucht:** Phase 1 = Terms je Kandidat messen + ITRF2020 (`https://itrf.ign.fr`) + IERS-EOP + LLR (ILRS/CDDIS) + Operator `canObserve`.

### Re-Admission — Admission-Verdikte gemessen (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der Register-Schreib-Pass
- **Lage:** (gemessen 2026-10-12/327) ALMA→**ESO** CC BY 4.0 · GRDC→**GRDC-Caravan** CC BY 4.0 / **USGS NWIS** CC0 1.0 / **Copernicus GloFAS** CEMS-free · RIPE RIS→**RouteViews** CC BY 4.0 · YBCO-NSE→**NIST NCNR NSE** US-PD (Zenodo `18306252` restricted → decline) · INTERMAGNET + GIRO DIDBase = **NC-Frage**.
- **Blockade:** die NC-Admission ist eine Operator/Rat-Frage.
- **Braucht:** je Kandidat die Admission-Zeile in `phi/sources.φ` (Mountain) + CDN (Mycelium).

### Rätsel/NIST — Directive-Kandidaten gemessen (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12/327) JADES `7460116` (2 898 432 B, cc-by-4.0) · UNCOVER `14281664` (65 905 920 B, cc-by-4.0) · FMI-NUR (`?month=YYYYMM&stations=NUR`, 200, 921 600 B, CC BY 4.0, Format undetermined) · NIST NCNR NSE `https://ncnr.nist.gov/pub/ncnrdata/nse/` (200, US-PD) · Teil C A5 `GO-SS-RSS-1-TDF-V1.0`. Die `url`-Zeilen existieren teils schon (`phi:32391`/`:32408`).
- **Blockade:** keine.
- **Braucht:** die `terms`/`format`-Zeilen in den Blöcken setzen; FMI-NUR-`format` klären.

### M10-Encounter + Sonke/Robinson (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der `camera_angles`-Dispatch
- **Lage:** (gemessen 2026-10-12/327) M1-Fenster 1974-03-28..30 = **410 Records**, JD 2442135.305…2442136.500, FDS `00000050`…`00027475`; **Riss:** innerhalb M1 sind JD und FDS **nicht ko-monoton**. Sonke & Robinson 2026 (`10.3847/PSJ/ae32f5`) ist OA gold, CC-BY. `https://ser.im-ldi.com/M10/M10_ISIS/` = **403** (kein Listing); `/M10/TXT/camera_angles.txt` = **200, 194 640 B**.
- **Blockade:** das Camera-Pose-Wiring.
- **Braucht:** FDS→Bild-Dateiname-Mapping im `camera_angles`-Dispatch dokumentieren; Sonke/Robinson-Annotation der ASU-Datei als `url` (Mycelium).

### NSSDC-Katalog 19740012502 → Ephemeriden-Register (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Register-/Verdikt-Pass
- **Lage:** (gemessen 2026-10-12/327, `grind-flash`-Taucher) **194 Einträge** (Mariner 2/4/5/6/7/9 · Pioneer 6/7/8/9/10); **2 registriert, 192 ohne Registerzeile**. Index: `state/research/ntrs-19740012502/mariner-venus-pioneer-index.txt`. Ephemeriden-Kern (Experiment „Celestial Mechanics"): `64-077A-09`/`-09A` (**registriert**), `64-077A-09B`, `67-060A-07`, `69-014A-05`, `69-030A-05`, `65-105A-07`, `72-012A-00D`. **Online-Audit:** die zweistelligen IDs (`64-077A-…`) akzeptiert **keine** NMC-Route; die vierstellige Basis löst als Experiment-Seite auf. `69-014A-05A` → `mariner_rst` (`PSCM-00017`); `69-030A-05A` → `mariner_rst` (`PSCM-00014`); `64-077A-09B` → `mariner4_odg` (`PSCM-00021`).
- **Blockade:** die ID-Fläche der NMC-Routen deckt die 1973er Datensatz-IDs nicht ab.
- **Braucht:** für Mariner 5 (`67-060A-07/-07A`), Pioneer 6 (`65-105A-07/-07A`), Pioneer 10 (`72-012A-00D`) je die Online-Route/Anfrage prüfen (NSSDC-Kontakt = Operator-Hand); die gedeckten IDs in den vorhandenen Blöcken vermerken.

### DSC-Nummer → Mission — Zuordnung gebaut (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Inventar-Pass
- **Lage:** (gemessen 2026-10-12/327) die Kreuzreferenz liegt unter **`old_data_doc/spreadsheet_dsc.xls`** (HTTP 200, 424 448 B, sha `b1f8523d…`); lokal `state/research/spdf-catalogs/spreadsheet_dsc.xls`. **Map gebaut:** `state/research/spdf-catalogs/dsc-number-map.txt` — **114 Zeilen** (36 Mariner · 81 Pioneer · 28 Venus), Form `DSC_<n> | <NSSDCA-id> <mission> <instrument>`, extrahiert aus den `renamed_dataset_catalogs`-Dateinamen.
- **Blockade:** die XLS (BIFF-Workbook) ist mit den erlaubten Werkzeugen nicht spaltentreu lesbar.
- **Braucht:** für den vollen 614er-Satz die XLS mit einem Rust-Reader (BIFF) öffnen — oder die `renamed_dataset_catalogs`-Dateinamen als Quelle belassen (832 DSC-PDFs).

### M10 camera_angles — Rahmen GEKLÄRT; unabhängiger Attitude-Solve offen (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der unabhängige Attitude-Solve aus den Bildern
- **Lage:** (gemessen 2026-10-12/328) **Rahmen entschlüsselt:** `SX SY SZ` = `SC_TARGET_POSITION_VECTOR` (Sonde relativ zum Zielkörper Merkur, J2000, km), `C1` = `INSTRUMENT_RIGHT_ASCENSION`, `C2` = `INSTRUMENT_DECLINATION`, `C3` = `INSTRUMENT_TWIST`. NAIF trägt für M10 **kein CK/IK**. 811 Frames in **drei** Clustern: M1 450 · M2 333 · M3 28. **`ephemeris_mariner10.bin` (Sun-Ketten-Komposition) ist geheilt** (s. eigener Punkt). **`m10_attitude_check`** (Bin `tools/measure/src/bin/m10_attitude_check.rs`) über 450 M1-Frames: `|vec−S|` mean **502 / max 29 009 km**, angsep mean **1,53°**.
- **Blockade:** (a) NAIF: kein CK/IK, kein SPK für M2/M3; (b) der echte Attitude-Solve braucht die Kamera + die Bildregistrierung.
- **Braucht:** der echte Attitude-Solve ist **`wartend`**: (1) die MVM-Merkur-Bilder — Re-Dispatch `38277503423` queued; (2) der MDIS-Shape — Lauf `38275761488` pending; (3) die **ISIS-Kette** als CI-Container (`xfer:mountain329-4`). Trigger: CDN-Abschluss + ISIS-Container.

### zai-Export — Such-Kontext + nicht-registrierte Mariner-Funde (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12/327) der durable Recherche-Korpus liegt in `state/research/` (die Suchdateien aus `/tmp` gesichert). **Nicht in `phi`/`docs`** (sgrep 0 Treffer): **Mariner 5** `PSCM-00001` Doppler 14.06.–20.11.1967 request-only · **Mariner 9** `71-051A-00E` (8 Rollen, UNIVAC 1108, 30.05.1971–25.10.1972) request-only · **Mariner 2** Techdoc `19670019851` + Sjogren/Trask · **Mariner 5** Analyse-Paper 1974 · Inventar NASA Techdoc `19900020832` (1987).
- **Blockade:** request-only/offline (NSSDC-Kontakt = Operator-Hand).
- **Braucht:** je Fund die `url`-Zeile (Mycelium) + Admission (Mountain).

### PDS POS-6 (`M10-H-POS-6-M1-FLYBY-TRAJ-V1.0`) — SEDR-Position online; Frame offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Ephemeriden-/Attitude-Pass
- **Lage:** (gemessen 2026-10-12/329) `POS_M1_ENC_42SEC.DAT` (HTTP 200, 475 152 B, 3126 × 152-B-Records) trägt **42-s-SEDR-Positionsproben**. `m10_pos6_check` über 3126 Records: `|POS6| − |sc−me|` **mean 1936 km, max 9139 km** → Range unabhängig bestätigt; die Vektorrichtung steht in einem **undeklarierten festen Frame** (Label `UNIT = NULL`; Best-Fit ~67,6°-Rotation um RA 222,0°/Dec 64,2°). **Register (2026-10-12/332):** die zwei Blöcke tragen jetzt `terms unknown https://pds-ppi.igpp.ucla.edu/` (Partei-Haus-Verdikt; Riss: M10-NASA-SEDR `PD` vs `pds-ppi`-Haus `unknown`).
- **Blockade:** die Frame-Definition des SEDR fehlt (kein Parser-`format`; Roh-Mirror).
- **Braucht:** den SEDR-Frame aus der NSSDC-Antwort (`PSPA-00318`) klären, dann den Vektor-Cross schließen.

### `phi/sources.φ` — Mosaik-`sha256` gesetzt; `de441/de442` offen (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der `de441/de442`-Merge-Lauf
- **Lage:** (gemessen 2026-10-12/330) `xfer:mycelium327-1` **erledigt** (Commit `dcdd4f15e`): der Mosaik-Block (`phi:31625`) trägt `sha256 324c0e49…`. `xfer:mycelium327-2` die `de441-de442`-NAIF-Blöcke brauchen `sha256` erst nach dem Merge-Lauf.
- **Blockade:** keine.
- **Braucht:** `de441/de442`-Merge-Lauf abwarten, dann den `sha256`-Nachzug setzen und `xfer:mycelium327-1`/`-2` nach Mycelium echoen.

### `solar_f107_flux_sfu` — Zeitbasis geheilt; der Join bleibt korrelationslos (`xfer:river197-1`, eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** der nächste `field_te_query`-Lauf
- **Lage:** (gemessen 2026-10-12/330) die beiden Archivar-Defekte sind **geheilt** (`d55d6c1b8`): `f107::parse_series` (`src/archivar/f107.rs:20-25`) wendet `lsk.unix_to_tdb` an; `series_epoch_of` (`src/archivar/extract.rs:6668`) prüft zuerst `INSTANT_TIME_KEYS`. Der Re-Mess (Lauf `38215224525`) zeigt: die Riss-Regel greift (SWPC median_dt 18000s vs Penticton 86400s, overlap corr **−0,0634**), aber der Join bleibt ≈ 0. **Implementiert (2026-10-12/331):** `aggregation_pair_kind` (`tools/measure/src/bin/field_te_query.rs`) — `None == None` nicht derselbe Empfänger; gemessene Diskrepanz → benannter Riss.
- **Blockade:** keine.
- **Braucht:** **vor dem endgültigen Split in zwei Register-Zeilen den Residuen-Test als zweite Messung prüfen** (Claude/Duck: Pearson ≠ Identität; GLM: aggregationsbewusst, gröbste gemeinsame Kadenz). Rohmaterial `state/stimmen/2026-10-12_mountain_f107-identitaet-ui.md`.

### M10-Pose mit ISIS nachrechnen (future-244 adressiert, eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** die ISIS-Kette gegen MESSENGER-SPICE
- **Lage:** (adressiert 2026-10-12, gefaltet) `xfer:future-244-1` Sonke & Robinson 2026 liefern nur Methode + Summary-Tabellen; `ser.im-ldi.com/M10/M10_ISIS/` = 403. Der Operator entschied: **nicht** die Autoren anschreiben, sondern die M10-Kamera-Orientierung mit ISIS gegen die SPICE-Kerne nachrechnen. Vergleich gegen die Paper-Summary (CLR 705, σ0 ≈ 0,570 px).
- **Blockade:** die ISIS-Kette (USGS, CC0) ist nicht installiert; CI-Env bestätigt (`isis-env.yml`, Lauf `38280076049` success, `isis 10.0.0`).
- **Braucht:** die ISIS-Kette gegen MESSENGER-SPICE fahren und gegen die Paper-Summary vergleichen.

### Aus future-247 (future-244-Messungen, gefaltet)
- **`xfer:future-244-2`** Analyse des Kybernautinnen-Gesprächs = der Bauplan der eigenen
  Ephemeride: Primärbeobachtungen, native Kette (UTC→TT→TDB, ITRF2020+EOP, VMF3/IONEX,
  Zwei-Wege-Lichtzeit+Shapiro), drei Quellenklassen. Artefakte:
  `state/future/analyse-kybernautin-ephemeriden-2026-10-12.md` ·
  `state/future/abdeckungs-matrix-ephemeriden-2026-10-12.md` ·
  `state/future/leads-ephemeriden-quellen-2026-10-12.md` (drei gemessene Lücken: Radar, VLBI/ICRF,
  Asteroiden-GM; geschlossen in `state/future/leads-ephemeriden-luecken-2026-10-12.md`). **CDDIS
  widerlegt** (s. Auftrag-Blockade).
- **`xfer:future-244-3`** Korrektur „keine Mariner-Rohdaten": `mariner_rst` (M6/7 2-way
  Doppler, CC0) · `mariner4_odg` (M4 ODG, CC0) · `mariner_m10_edr` (M10-Bilder, PD) liegen
  registriert vor; nur M10 fehlt als **Track** (s. M10-Bildpfad-Riss).
- **`xfer:future-244-4`** M10-Kamera-Vergleichsdatensatz: `mariner_m10_camera_angles.txt`
  (`phi:31602`) + `mariner_m10_edr` (`mvm_0001.tgz`) beide `--verdict` HTTP 206; es fehlt allein
  das Roh-Tracking.
- **`xfer:future-244-5`** M10-Roh-Tracking: 762 7-Spur-Tapes `MVM_1001–1007` offline (kein
  Mirror); SPDF nur Venus-Okkultation (`mariner_occlt`, `phi:20413`); SEDR `PSPA-00273/00055/00318`
  offline; NAIF kein CK. Riss: `MVM_0001–0014` (Bilder, online) vs `MVM_1001–1007` (Roh-Tapes, offline).
- `xfer:future-244-6` **gestrichen** (Register-Blindflug — PETREL19 war schon live).

### Aus river-200 (river-197-Messungen, gefaltet)
- `xfer:river197-1` f107 → im Offen-Punkt `solar_f107_flux_sfu` getragen.
- `xfer:river197-2` MCON-Spanne 1966–2025 vs Parser-Ende 2024-05-29 — nominale Beschreibung vs
  Parser-Fenster; für die BPA (2026) irrelevant.
- `xfer:river197-3` THEMIS-ASI Stations-Tabelle korrigiert (`src/archivar/themis_asi.rs`;
  ATHA 549.82 · EKAT −110.6703/430.77 · PGEO 756.26 · MCGR 102.57) — erledigt (Info).
- `xfer:river200-1` WWLLN — als `decline commercial` (`phi:33083`) beantwortet; Survey-Zeile
  korrigiert.
- `xfer:river200-2` §D-`channel`-Direktive → als Offen-Punkt „Plasma-`channel`-Direktive" gefaltet.

### Aus mycelium-333 (Echo)
`xfer:mountain329-2` · `mountain327-2` · `mountain327-6/-7/-8/-9` · `mountain327-1` ·
`mountain328-1/-2` · `mountain329-4` · `mountain329-1` — alle gemessen-erledigt, im Ledger gefaltet.

## Prosa-Träger (orphan-docs)

`register_lookup --orphan-docs` misst die trägerlosen Prosadokumente; Mountain-Klasse (gefaltet
aus `xfer:mycelium330-1`):
- `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` (LiDAR-Quellen-Klasse) — Träger:
  die LiDAR-Quellen-Klasse im Re-Admission-/Quellen-Terms-Punkt.
- `docs/surveys/survey-2026-10-08-open-sources-delta.md` · `survey-2026-09-16-dead-sources-relevanz.md`
  — Quellen-Klasse, Träger: der Re-Admission-/Quellen-Terms-Punkt.

## An mycelium

Origin: mountain-folge333 (2026-10-12).

- **PDS POS-6 — `terms` gesetzt (echo).** `xfer:mountain329-2` Die zwei Blöcke (`phi:33069-33081`)
  tragen `terms unknown https://pds-ppi.igpp.ucla.edu/`; **kein** `format` (Roh-Mirror).
- **ISIS-Kette — Lauf-Bestätigung.** `xfer:mountain329-4` `isis-env.yml`, Lauf `38280076049`
  success (`isis 10.0.0`); die Methode = Mountain. `camera_angles.txt` ist `url`/`origin`
  registriert (`phi:31610-31616`).
- **M10-Ephemeride — CDN regeneriert (echo).** `xfer:mountain329-1` Lauf `38278478615` success;
  `phi:32031` trägt sha `67949cb6…` (geheilte Sun-Ketten-Komposition).
- **8-Ziele-Einkaufsliste (future-241)** — die vier direkt downloadbaren cc-by-4.0 ohne
  Registerzeile (BC430 `3558850`, Pioneer 10/11 Doppler `13309156`/`13309040`, Venus-Radar
  `18176179`) + PDS-Geosciences Venus-Radar + PVO-ORSE + JPL SSD: je `url`/`origin`/`compiler`-Zeile.

- **Legacy-Token-Fold — gemessene Lücke.** Das PATH-`xfer` (Rolling Release) kennt das
  Subkommando `migrate` **nicht** (`xfer: unknown subcommand: migrate`), der Baum schon
  (`tools/register/src/bin/xfer.rs:785`). Die Legacy-Token der adressierten Blöcke
  (future-247 `xfer:future-244-2…-5`, river-200 `xfer:river197-1…3`/`river200-1/-2`) sind
  darum mit dem installierten Bin **nicht faltbar** (`xfer fold` → `token malformed`); der
  faule Backfill (`xfer migrate --write`) wartet auf das Release. Die Einträge sind im
  Handover trotzdem gefaltet; `--tokens` bleibt für sie `unbeglichen`, **XFER-NOECHO = 0**.

## An river

Origin: mountain-folge333 (2026-10-12). **Antwort auf river-200:**
- `xfer:river200-1` (WWLLN-Register-Drift): die Survey-Zeile
  `survey-2026-09-07-weberin-thread-matrix.md:168` ist korrigiert (nicht mehr „in-register",
  sondern `decline commercial`).
- `xfer:river200-2` (§D-`channel`): als Offen-Punkt gefaltet (wartend auf die §D-Quellen).
- `xfer:river197-1` (f107): im Offen-Punkt getragen.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes. … /commit ist das Commit-Wort"** | 2026-10-12 | Operator-Session (mountain 332/333) — session-weiter Delegations-Consent; Commit+Push durch das stehende Wort („mach das ab jetzt automatisch", 2026-10-07).
- **„bei council auch archive search all und die ui chats (alle und max!) befragen"** | 2026-10-12 | Operator-Session (mountain 331) — die f107-Ratsfrage durch die Forschungsschicht + alle UI-Seats untermauert (`state/stimmen/2026-10-12_mountain_f107-identitaet-ui.md`).
- **„und dann vergese auf keinen fall die ergebnisse der taucher zu implementieren"** | 2026-10-12 | Operator-Session (mountain 331) — Taucher-/Rat-/UI-Ergebnisse im selben Atom implementiert.
- **„die drei fehlenden `terms`-Zeilen sind gesetzt"** | 2026-10-12 | Operator-Session (mountain 330).
- **„Will ich als nächstes die Sun-Kette debuggen…?"** | 2026-10-12 | Operator-Session (mountain 329) — Sun-Ketten-Riss geheilt.
- **„mache hier weiter … es war kein Zentrum, es war ein Vorzeichen …"** | 2026-10-12 | Operator-Session (mountain 328).
- **„aber das muss doch zu klären sein … und was ist mit dem anderen ephemeriden berechnungsprogramm…?"** | 2026-10-12 | Operator-Session (mountain 328).
- **„du kannst dich auch gleich mal darum kümmern deine arbeit richtig zu machen und nichts zu behaupten was der messung nicht standhält"** | 2026-10-12 | Operator-Session (mountain 327).
- **„gehören pdfs in data?"** | 2026-10-12 | Operator-Session (mountain 327) — nein; PDFs in `state/research/`, Datensätze in `data/`.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch",
2026-10-07) trägt Commit und Push. In diesem Atom (folge333) committet (pfad-begrenzt):
`src/gate/commit_gate.rs` + `src/gate/commit_gate_vocab.json` (die `decline`-Gate-Fixture),
`tools/register/src/bin/register_lookup.rs` (`decline`-Scan), `docs/surveys/survey-2026-09-07-weberin-thread-matrix.md`
(stale WWLLN-Zeile), `docs/handover/handover-2026-10-12-mountain-folge333.md`,
`docs/handover/archiv/handover-2026-10-12-mountain-folge332.md`. **Riss (getragen):** die
JADES-`nu_hz` stammt aus dem SVO-Filter-Pivot (PHOTPLAM fehlt im Tar). **Warnung an die nächste
Session:** der Arbeitsbaum ist von parallelen Linien gehalten (River/Mycelium committen `phi`/`src/*`
laufend); vor jedem Commit das eigene Pfad-Set messen. **`mariner10`-Ephemeride geschlossen:**
`phi:32031` sha `67949cb6…` == die geheilte lokale Datei.
