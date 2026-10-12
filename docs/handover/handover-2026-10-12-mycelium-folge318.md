<!--
  title: Handover — Mycelium-Folge 318
  session: Mycelium-Linie in einem Pass (MCON-Registerdrift gelöst, bpa_gic↔MCON als gemessene Null, Risse benannt)
  class: handover
  date: 2026-10-12
  sha256: a743686547eb4c08cdc67bfb267ddced5f08deb33984f28bdbc7e368b40dcb0d
  status: live
-->
# Handover — Mycelium-Folge 318 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge317.md` (→ `archiv/`). Operator-Auftrag
(Session 318): den Mycelium- und Mountain-Teil der River-194-Tafel bearbeiten,
während Mountain beschäftigt ist. **Gebaut dieses Atom:** die MCON-Registerdrift
gelöst (`phi/sources.φ:21545-21551`); ein `general`-Diver maß die BPA-GIC-Frage.

## Burn: open 0.0 · close 0.09 · cap 0.20 — Grund: Meta-Pass in einem Atom (flash-first; ein `general`-Diver, kein pro/max). (close = gemessener `session_burn`-Linien-Posten ≈ $0.09; die Burn-DB wuchs in diesem Zeitraum durch parallele Linien-Sessions — kein sauberer Isolationswert.)

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „kannst du bitte deinen und mountains teil bearbeiten mountain ist beschäftigt" | 2026-10-12 | Operator (Session 318) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du dich bitte um deinen teil kümmern? Klare Trennung — und ich habe meinen Teil gleich gebaut." | 2026-10-12 | Operator (Session 317) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 315/316) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |

## Offen — eigen

### MCON — Registerdrift gelöst; field/record-Labeling offen (Mountain)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain-Verdikt der fünf `field`-Zeilen (P10)
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38176003927 --all`) die Registerdrift ist **behoben**: `phi/sources.φ:21545-21549` trägt jetzt `url …/zenodo.org/mcpherron_onsets.bin` · `format mcon_event` · sha256 `441df5f8c59ad57962d81cc82814a0d36c7770f827b4d1893b31425f035cc3f1` (Lauf `38176003927` success: 314555 Onsets, 10065773 B, format `mcon_event`). `register_sort` canonical. **Offen:** der Block trägt keine `field`/`record`-Zeilen; die fünf Komponenten (`mcon_event::component_name`: `mcon_sml_nt`/`mcon_amplitude_nt`/`mcon_bay_decrease_nt`/`mcon_bay_duration_min`/`mcon_bay_width_min`) brauchen die P10-Labeling (Quantity|Mechanism|Medium) — wissenschaftliche Prüfung, Mountain.
- **Blockade:** P10-Labeling (Mountain/Rat), nicht mechanisch erratbar.
- **Braucht:** die fünf `field`-Zeilen + `terms` (Zenodo-Record 22711679) — Mountain.

### bpa_gic ↔ MCON — keine Historie: gemessene Null, kein Pending
- **Status:** descoped (gemessen) | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12, `general`-Diver via `--verdict`/`--playwright`/`--all`) BPA publiziert **kein** historisches GIC-Archiv — nur ein rollierendes **4-Tage-5-Minuten-11-Station-Fenster** (`.../gic/gic.txt`, Header „last 4 days"; Landing `gic.aspx` verlinkt nur `gic.txt`; alle Archiv-Kandidaten 404; `--all` 1029 Zeilen ohne BPA-Archiv; Wayback 429/503). Die Überschneidung mit der MCON-Onset-Liste (endet 2024-05-29) ist **leer**. Die `bpa_gic`-Spanne ist **nicht ausdehnbar** (kein öffentlicher Weg; nur FOIA an BPA, Dritter-Akt). Divergenz zur Auftrags-Prämisse: das Fenster ist **5-min**, nicht 1-min (Register `ttl 300`/Feld-Skala 300.0 am Baum bestätigt).
- **Konsequenz:** die zweite GIC-Zeugenreihe bleibt leer (0 honored); die Relation läuft über die primäre FMI-Reihe (`gic_onset_relation.rs --gic`). `terms` bleibt `unknown` (BPA nennt keine Datenlizenz).

### `intermagnet_dbdt`-Riss — Rat: **Nein** zum Rename; Resolver-Regel statt Identität (Mountain-Bau)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rivers `field_te_query`-Atom schließt (dann ist die Datei frei für den Resolver-Bau)
- **Lage:** (gemessen 2026-10-12, Rat-Verdikt `state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md`) **Nein** — `intermagnet_dbdt` bleibt die Identität; die Kadenz ist ein Auflösungs-Attribut, keine zweite Quantität. Ein Rename `_1m`/`_1h` wäre Fabrikation (wählt still eine Aggregation und lässt die andere aus dem geteilten Namen fallen); der Register-Text ist unangetastet. Grund am Baum: die `field`-Zeile trägt **keine** Kadenz — das Token nach `<unit>` ist `tau` (`src/archivar/parse.rs:2361`), beide ABK-Zeilen tragen `86400`; die Kadenz wird live als `median_dt` gemessen. **Die Regel existiert im Kern bereits:** `tools/measure/src/bin/field_te_query.rs:1249` `AGGREGATION_IDENTITY_CORR = 0.99`; `:1263-1341` `load_field_across_sources` wählt die feinste `median_dt` als Zeugen und **verweigert** (Riss), wenn eine gröbere Registrierung unter dem Gate liegt. **Der benannte Rest:** die Regel gruppiert **nicht nach Station** (`abk_1m` wird auch gegen `sod_1h` geprüft → falscher Riss) und der `Err`-Pfad lässt den Zeugen fallen statt ihn zu tragen.
- **Blockade:** `tools/measure/src/bin/field_te_query.rs` wird von Rivers laufendem measure-Atom gehalten — ein zweiter Schreiber kollidiert.
- **Braucht:** der minimale Bau (Mountain-Schreibakt, Register+Wire unangetastet): pro `station_code` gruppieren, je Station die feinste native Kadenz als Zeugen, die gröbere als Riss **daneben tragen** (nie mitteln, nie fallenlassen); Korr-Gate 0.99 trennt ABK `0.9388` (Riss tragen) von f107 `0.0232` (verweigern). **Wer:** Mycelium (Mountain-Vertretung) als nächstes Atom, sobald Rivers Atom schließt — oder River (seine Datei).

### `solar_f107_flux_sfu` — erst messen (Join-/Zeitbasis-Verdacht, kein Quantitäts-Riss)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rivers f107-Join-Messung (`field_te_query`-Lauf `38211174118`)
- **Lage:** (gemessen 2026-10-12, Rat-Verdikt) `phi/sources.φ:520-524` (SWPC live `f107_cm_flux.json`, `ttl 600`, `first flux … 3600.0`) und `:29571-29579` (Penticton CDN `f107_penticton.bin`, `field f107_penticton solar_f107_flux_sfu … 86400`) tragen beide die Quantität `solar_f107_flux_sfu`; corr 0.0232. Der Rat: corr `0.0232` gegen `0.9388` ist ein **Join-/Zeitbasis-Verdacht**, kein Identitäts-Riss; das `0.99`-Gate verweigert hier bereits korrekt. Der Resolver hat keinen Binning-Floor außer `n ≥ 2`.
- **Blockade:** das Identitäts-Urteil braucht erst die Join-Messung.
- **Braucht:** Rivers measure-Schritt (Bin-Zahl + Roh-Zeitstempel der Überlappung); erst danach ein Identitäts-Urteil.

### 50 Vlies-`alignment pending`-Zellen — fehlende Quellenreihen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Reihe eine Quellenzeile im Register (`phi/sources.φ`)
- **Lage:** (gemessen 2026-10-12, River-194 via `ci_manage log 38172797345`) `alignment pending` = 50, alle zu den Reihen `qbo_30hpa_ms`/`d20_thermocline_depth_m`/`swarm_absolute_vertical_tec_tecu`/`eve_1032_line_irradiance`/`eve_131_line_irradiance` (keine Quellenzeile im Register). Die drei HAPI-Arme (`magnetosphere_imf_bt_nt`/`solar_wind_speed_km_s`/density) sind durch Rivers `field_te_query`-Fix adressiert (`solar_wind_density_cm3` echt absent).
- **Blockade:** je Reihe eine neue Quelle (Discovery+Registrierung).
- **Braucht:** `archive_search --all` je Reihe (QBO 30 hPa-Zonalwind · D20-Thermokline · Swarm absolute vertikale TEC · SDO EVE 1032/131 Å Irradianz) → `url`/`origin` (Mycelium) + `terms`/`field` (Mountain).

### THEMIS-ASI getimetes Keogramm — Registerform + Compiler-MIRACLE-Form offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Registerform (Per-Nacht vs. feste `url`) entschieden (Mountain/Rat)
- **Lage:** (gemessen 2026-10-12) `tools/harvest/src/bin/themis_asi_timed_keogram_compiler.rs:320-329` `emit()` druckt `at earth` (nicht die MIRACLE-Form `on earth <lat> <lon> <alt>` + `station <CODE>`); `format themis_asi_timed_keogram` = 0 Treffer. Stationstabelle + `h = H + N` in `state/mycelium/themis_asi_stations.md`. Der Compiler hat `--station`/`--date`, aber kein lat/lon im `emit()`-Pfad.
- **Blockade:** Registerform unentschieden; ohne sie kein `emit()`-Umbau.
- **Braucht:** Form-Entscheidung → `emit()` auf die MIRACLE-Form (Mycelium) + `terms` (UCalgary) + `field` (Mountain).

### GIC-Onset-Harvest — `lov` HTTP 400; zweiter Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `38164966250` (`intermagnet-xyz-cdn`) endet
- **Lage:** (gemessen 2026-10-12, River-194) Lauf `38164890620` trug 25 Stationen (2000–2024): 24 success, `lov` cancelled (HTTP 400 für 2023+); `38164966250` queued.
- **Blockade:** per-Station-`<code>_xyz_1m.bin`-Assets der fehlenden 154-Station-Familie.
- **Braucht:** `ci_manage log 38164966250`; dann `al_network_proxy`/`gic_onset_detector` je Familie.

### Mariner-CDN-Arme — Register + Workflows gebaut; Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `mariner4-odg-cdn`/`mariner-rst-cdn`/`mariner-m10-edr-cdn`-Läufe enden
- **Lage:** (gemessen 2026-10-12) `mariner4_odg`-Block (`phi:31390-31394`) + drei Workflows; dispatcht `38209368944`/`38209371259`/`38209373661`.
- **Blockade:** Lauf-Ergebnis + Asset-sha256; `terms` im M4-Block (Mountain).
- **Braucht:** `ci_manage log <id>`; sha256 nach grünem Lauf; `terms` (Mountain).

### ENTSO-E A78 — Lauf rot; Body-Diagnose gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `entsoe-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38169562211`) failure `curl status Some(22)`; geheilt (`-sSL` + `--write-out`).
- **Blockade:** ENTSO-E-Token (Ticket #9473923, `state/zustand/wartend.φ:28`).
- **Braucht:** `ci_manage log <run-id>`; bei grün Register-Block; `terms`/`ttl` (Mountain).

### GRDC-Caravan — 1h-curl-Cap geheilt; Reader-Arm + field/terms offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf (`38176538328`) endet
- **Lage:** (gemessen 2026-10-12) queued; Cap geheilt (Streaming).
- **Blockade:** `src/archivar/extract.rs`-Reader-Arm `grdc_caravan`; `field mm/d` + `terms` (Mountain).
- **Braucht:** `ci_manage log 38176538328`; Reader-Arm; `field mm/d` + `terms` (Mountain).

### MLA-Register · SPT-3G-D1 · GloFAS-EWDS · NIST-NCNR-NSE
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Lauf/Digest bzw. Mountain-`terms`/`field`
- **Lage:** (gemessen 2026-10-11/12) MLA repointet (`phi:12100`; NAME-Riss); SPT-3G Lauf `38150991206` success (sha offen); GloFAS Daten-Arm grün (`phi:11297`; grib2→φ offen); NIST NCNR NSE Lauf `38170359508` success (`phi:11711`).
- **Blockade:** Lauf-Ergebnisse/`terms`/`field` (Mountain).
- **Braucht:** `ci_manage log <id>`; sha256; `terms`/`field` (Mountain).

### Rätsel-/NSE-Routen — `url`/`origin` geschrieben; Produzenten/`terms` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ersten Manifest-Lauf je Quelle
- **Lage:** (gemessen 2026-10-12) UNCOVER/JADES/BK18/NAIF-Voyager-`url`/`origin` registriert; UNCOVER/JADES/BK18 ohne Compiler/Workflow.
- **Blockade:** Manifestations-Workflows fehlen; `terms`/`format` (Mountain).
- **Braucht:** Workflows für UNCOVER/JADES/BK18 (Mycelium) + `terms`/`format` (Mountain).

### Architektur — `on earth alt` — Registerform → `emit()`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain/Rat-Form-Entscheid bzw. Operator-Wort
- **Lage:** (gemessen 2026-10-12) `alt` = `h = H + N` definitiv (EGM2008, 24/24 in `state/mycelium/themis_asi_stations.md`); kein Default/Pad.
- **Blockade:** Registerform (Per-Nacht vs. feste `url`).
- **Braucht:** Form-Entscheidung → `emit()` mit der festen Tabelle; WHIT `pending`.

### wy-max-t-Artefakt — Werte im Bestand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`
- **Lage:** (gemessen 2026-10-12, River-194) α=0.05, B=9999, (1-α)-Quantil 2.4467/2.4433/2.4831 (Artefakt `38166787414`); **BCa-CI `pending`** (im Artefakt benannt).
- **Blockade:** BCa-CI (River) + Siegel (Operator).
- **Braucht:** die gemessenen Werte ins Register/Artefakt (Mycelium); BCa-Lauf (River); Siegel (Operator).

### MCP no-leak · de441-de442-sha · dropped-gate · SWPC/RTSW · CI-Optimierung · Domain→UCD
- **Status:** wartend/eigen/pending | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart · Merge-Lauf (`phi/sources.φ:4321-4369`) · Q2/Q3-Fold (`dropped_gate.rs`) · `matrix-vlies`-Lauf · `odf.rs`-clippy · messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11/12) MCP: Allowlist konsistent, Live-Seat-Restart offen; de441/de442: sha256 erst nach Merge-Lauf; dropped-gate: Match-Fix `offer_word` + Test (312), Q2/Q3-Fold + 924-vs-78-Riss (future-240 `278ba2905` vs Baseline 78) offen; SWPC/RTSW: `matrix-vlies` am ACE-Arme offen; CI-Opt: `odf.rs` clippy (`:179`/`:213`/`:2222`, Träger Mountain); Domain→UCD: keine messbare Domain-Quelle.
- **Braucht:** je der genannte Schritt (MCP-Restart · Merge-Lauf · Q2/Q3-Fold · `matrix-vlies` · `odf.rs` · Domain-Quelle).

### matrix-rotor — descoped (gemessen)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-11/12) letzter rot = transienter Runner-Shutdown; kein eigener Schritt (`Braucht: —`).

## An mountain

Origin: mycelium-318 (2026-10-12).

- **MCON-Registerdrift gelöst** (`phi:21545-21549`, `format mcon_event`, sha `441df5f8…`); offen bleiben die fünf `field`-Zeilen (P10) + `terms` (Zenodo 22711679).
- **`intermagnet_dbdt`** — Rat-Verdikt **Nein** zum Rename (`state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md`): Identität bleibt, Kadenz ist Auflösungs-Attribut; Register+Wire unangetastet. Offen ist nur die Resolver-Regel (per-Station-Gruppierung + Riss-daneben-tragen, `field_te_query.rs:1263-1341`) — Mountain-Schreibakt.
- **MCON-Spanne** — Zenodo nominiert `1966–2025`, Rivers Fenster `2024-05-29`; nominal vs Inhalt. Eine Inhaltsmessung des manifestierten `mcpherron_onsets.bin` entscheidet es (nicht die BPA-Überschneidung berührend).
- **`odf.rs` clippy (`:179`/`:213`/`:2222`)** blockt `ci-check`.
- **bpa_gic**: kein historisches Archiv (gemessene Null); `terms unknown` bleibt.

## An river

Origin: mycelium-318 (2026-10-12).

- **`bpa_gic`-Spanne** — gemessene Null: BPA führt nur ein 4-Tage/5-min-Fenster, kein Archiv; die zweite GIC-Zeugenreihe bleibt leer; die Relation läuft über die FMI-Reihe.
- **`solar_f107_flux_sfu`** corr 0.0232 — Join-/Zeitbasis-Verdacht, eine Wiederholungsmessung lohnt (measure).
- **MCON-Registerzeile steht** (`format mcon_event`); die Zelle/Relation kann neu gelesen werden.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
