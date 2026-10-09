<!--
  title: Handover — Mountain-Folge 284 (2026-10-09)
  session: Mountain-Folge 284
  class: handover
  date: 2026-10-09
  sha256: e7a524f493b09bc3b7854a6c6c4f3b30558be34fbfbc189c1dff5d1bf7876cf4
  status: live
-->
# Handover — Mountain-Folge 284 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium, gemessen 2026-10-09). Diese Session konsumierte
`handover-2026-10-09-mountain-folge283.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.0368 — flash only, kein pro/max (ein `general`-Taucher, read-only); session_burn (2026-10-09T14:53Z) top-line `Mountain-Linie in einem Pass abarbeiten` $0.0368

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–284)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–284)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„ja bitte" — Index-Riss als Mountain-Verdikt `quantity` setzen + die `sources.φ`-Zeilen bauen | 2026-10-08 | Operator (Session, Mountain 276)
„ich glaube du musst nochmal breiter fragen" — Science-Layer + starke Frontier-Seats für die Route-Admission | 2026-10-08 | Operator (Session, Mountain 276)
„bitte umsetzen Offen (im Report benannt): 2 blocked_sources-Risse (limadou/vco_rs Dubletten; cluster_ka-Zeile ohne gap), SuperDARN dritter Layout-Slot (kein pot.drop.err), ROTI-Gitter-Orientierung, Kellerman-CSV-Reader, themis_mag-CDN-Orphan → Mycelium." | 2026-10-09 | Operator (Session, Mountain 280)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher, höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." + „aber mach dann auch wirklich die Arbeit" | 2026-10-09 | Operator (Session, Mountain 281)
„1. natürlich Ja wir brauchen die lizenzen sind regeln der quellen nicht unsnere" | 2026-10-09 | Operator (Session, Mountain 283)
„2 bitte spreche dich mit river ab das ist teil seines plans" | 2026-10-09 | Operator (Session, Mountain 283)
„du sollst das prüfen, die lizenzen müssen korrekt sein" | 2026-10-09 | Operator (Session, Mountain 283)
„sind jetzt alle blöcke in allen asset files mit tes versehen (also auch die anderen weberinnen sources) und sollten wir eigentlich die beschreibungen von sources repo und assets noch anpassen?" | 2026-10-09 | Operator (Session, Mountain 283)
„wir sind immer noch nicht opensource" — omegaflow ist source-available (PolyForm NC/CC BY-NC-SA), NIE „open-source" nennen | 2026-10-09 | Operator (Session, Mountain 283)
„natürlich 1 wir sind nicht open source wir sind NC CC" | 2026-10-09 | Operator (Session, Mountain 283)
„das ist compliance theater" — keine Lizenz-Boilerplate in einer Anfrage-Mail; nur sagen, was die Frage braucht | 2026-10-09 | Operator (Session, Mountain 283)

## Offen (aufgeschlüsselt)

### GIC-Faden §A–G — Wind-Protonen-Arm gebaut; Rest liegt bei Mycelium
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** verbleibende `gap`-Träger (map-grid-Download / Keogramm-Vision-Asset)
- **Lage:** (gemessen 2026-10-09, diese Session) **§A-Wind-Route gebaut:** `WI_K0_SWE` (Protonen `Np` #/cc + 3× `V_GSE` km/s, 1994→2026-10-06) und `WI_H2_MFI` (`BGSM` nT) als CDAWeb-HAPI-Arme in `phi/sources.φ` registriert (Block bei `WI_H0_MFI@0`); das B-Feld-GSE ist bereits durch `WI_H0_MFI@0` (`sources.φ:799-806`) getragen, `WI_H2_MFI` ergänzt nur GSM. `register_sort` canon (ttl asc, url asc), `cargo check` 0/0. THEMIS-Tail + MMS-Magnetosheath stehen (283). Beide `blocked_sources.φ`-Einträge (`themis-tail`, `mms-magnetosheath`) bleiben entfernt.
- **Blockade:** SuperDARN-MAP-grid-Download liegt bei Mycelium (`wartend.φ:8`, Globus-Credentials stehen); Keogramm-Vision-Asset bei Mycelium.
- **Braucht:** Mycelium: MAP-grid-Download + Keogramm als Vision-Asset; sonst nichts Neues.

### Substorm-Onset / SuperMAG — Riss: ledger behauptet eine Registrierung, die fehlt
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium (Workflow/CDN)
- **Trigger:** `substorm`-`sources.φ`-Zeile + Workflow
- **Lage:** (gemessen 2026-10-09, `sread`/`sgrep`/`archive_search`) `tools/harvest/src/bin/substorm_compiler.rs` steht (BASE `supermag.jhuapl.edu/lib/services/`, `--list newell|forsyth|liou|frey|ohtani`, write_bin, `upload_release("supermag.jhuapl.edu", …)`); **kein** `substorm`/`supermag`-Block in `phi/sources.φ`. `ledger.φ:152` behauptet „SuperMAG registriert (sources.φ:18407-18431)" — diese Zeilen sind die Vega2/Mischa-PDS3-Felder (`sgrep` bestätigt, Riss). `supermag-cdn.yml` ruft `supermag_compiler` (Netzwerk), nicht den Onset-Compiler.
- **Blockade:** substorm-Workflow fehlt; Quelle nicht registriert.
- **Braucht:** 5 `sources.φ`-Blöcke (`format hapi_csv`, `compiler …/substorm_compiler.rs`, `origin supermag.jhuapl.edu/lib/services`) + eigener Workflow; `ledger.φ:152`-Zitat auf netloc-Key umstellen.

### USGS-geomag E-Feld — Riss: gleiche Basis-URL, zwei Produkte
- **Status:** eigen | **Bindung:** mycelium (`pending`-Eintrag schließen)
- **Trigger:** `sources.φ`-Zeile für das E-Feld-Produkt
- **Lage:** (gemessen 2026-10-09, `sgrep`) `blocked_sources.φ:100-102` `pending` `/ws/data/` (`elements=E-E,E-N` → mV/km, 200); `declined_sources.φ:2176-2177` dieselbe Basis-URL (`data.json`, XYZF) als Duplikat des registrierten InterMAGNET-Fanouts (BOU). **Riss = gleiche URL, zwei Produkte:** E-Feld (neu, physikalisch berechtigt) vs. Magnetik (Duplikat). Verdikt: E-Feld-Produkt admission, Magnetik bleibt `declined`; die URL ist per Query zu trennen.
- **Blockade:** keine (Verdikt steht).
- **Braucht:** `sources.φ`-Zeile `…/ws/data/?id=<IAGA>&elements=E-E,E-N&format=json` (field mV/km, `terms`), dann Mycelium `pending`-Eintrag schließen.

### PDS-PPI — Enumerator/Arm stehen, `sources.φ`-Zeile fehlt
- **Status:** eigen | **Bindung:** mycelium
- **Trigger:** `sources.φ`-Zeile + Workflow
- **Lage:** (gemessen 2026-10-09, `sgrep`) `blocked_sources.φ:61-63` `pending`; Enumerator `pds_ppi_compiler.rs` (`115cf1657`) + Arm `pds4_fixed_width_compiler` stehen; **kein** `pds-ppi`-Block in `phi/sources.φ` (nur `origin`-Zeilen der Voyager-ODR-Blöcke nennen den Host).
- **Blockade:** keine.
- **Braucht:** `sources.φ`-Zeile (`format pds4_fixed_width`, `field`/`terms`) + Workflow.

### ShadowCam-Admission — Verdikt: admission ja, Format-Arm fehlt
- **Status:** eigen | **Bindung:** mycelium (Format/Arm baut)
- **Trigger:** Format-Arm/Workflow
- **Lage:** (gemessen 2026-10-09 via `archive_search --verdict` + curl) `pds.shadowcam.im-ldi.com/derived/` HTTP 200 (PDS4 `Product_Collection`-CSV/XML + Subdirs `dtm/` `lronac_cmosaic/` `umosaic/` `vector/`); **kein `.fits`** — `pds4-fits` (`blocked_sources.φ:17`) deckt Chang'e-MRM, nicht diesen Baum. Admission: **ja** (PDS public, KPLO/LRO-NAC-Derivate). Format: PDS4-derived raster (ISIS `.cub` + `_cog.tif`) — neue Format-Klasse.
- **Blockade:** Format-Arm fehlt.
- **Braucht:** Mycelium baut Format/Arm; Mountain schreibt die `sources.φ`-Zeile auf die Format-Entscheidung.

### Register-Physik-Migration (`force` → Quantity | Mechanism | Medium)
- **Status:** eigen | **Bindung:** eigen (Register) · river (Schema)
- **Trigger:** Rivers erste Block-Charge (`gravity` 118 / `seismic` 41)
- **Lage:** (gemessen 2026-10-09, river-148) Schema + Achsen-Verdikt gefallen (`state/stimmen/2026-10-09-river-register-physik*.md`); gebauter Arm `parse.rs:990` (`FORCE_TYPE_QUANTITY = 255`). Erste Gruppe `gravity`/`seismic` + Wellenhöhen; `em` (6016) zuletzt, erst nach der `Domain/Rand`-Erweiterung (Quantisierung `k_j` trägt eigene Domain/Rand-Achse).
- **Blockade:** `Domain/Rand`-Erweiterung (vor `em`); konkrete Migrations-Charge.
- **Braucht:** Rivers Migrations-Charge; Mountain schreibt die Verdikt-Zeilen in `phi/sources.φ` auf die Schema-Entscheidung.

### Flyby-Kette — Residual liegt in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen (Register) · river (`flyby_ephemeris_gate`)
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09) Der ESTRACK/DSN-Residual ist **nicht absent**: 157 ODF-Referenzen in `sources.φ` (MEX/Rosetta/Juno/Magellan/Pioneer/Viking/VEX); `odf.rs` steht, `doppler.rs` absent. **Riss:** σ_recon ist NICHT ein Doppler-Residual, sondern die 1-σ-Kovarianz der ESOC-Post-Flyby-Recon-Ephemeride (`ephemeris_juice_recon.bin` 404; river-139 `:74-79`, gate `flyby_ephemeris_gate.rs:296-307`).
- **Blockade:** kein ESOC-Recon-Release; `doppler.rs` wird vom ODF-Residual nicht gebraucht.
- **Braucht:** ESOC-Release abwarten (river) oder Descope-Befund für `doppler.rs`.

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** wartend | **Bindung:** eigen (CI-Lauf)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-09) Grad-13-Synthese + `igrf.rs` stehen. Re-Dispatch `ci-check.yml` → run `37929072865` (head `ce47ce13c`); der vorige `37910517444` endete cancelled.
- **Blockade:** keiner.
- **Braucht:** `ci_manage view 37929072865` beim nächsten Pass (kein Polling); bei grün Punkt schließen.

### Lizenz-Disposition — `terms`-Feld (SPDX); jeder Block trägt `terms`
- **Status:** eigen | **Bindung:** eigen (Format/Datenkontrakt)
- **Trigger:** `terms`-Zeilen je Quelle geschrieben
- **Lage:** (gemessen 2026-10-09, diese Session: `license_census`) **blocks 2689 · terms 2689 · distinct 22 · no-terms 0 · pending 0 · 0 violation(s)**. `unbestimmt` bleibt der ehrliche Messzustand (~500 Hosts ohne messbaren Lizenzsatz). **Riss:** Hosts mit unterschiedlichem Service (`vo.astron.nl`, `data-argo.ifremer.fr`, `maxi.riken.jp`) — belassen, benannt. `ohne-lizenz` 27 lokal gehalten/nicht gespiegelt.
- **Blockade:** keine.
- **Braucht:** nichts; `unbestimmt` löst sich nur, wenn die Quelle selbst eine Lizenz publiziert.

### `blocked_sources.φ`-Aufräumen — 15 Klassen-Träger (`gap`-Token)
- **Status:** eigen | **Bindung:** eigen (Disposition) · mycelium (Diver-Tabelle)
- **Trigger:** Bau je Klassen-Träger
- **Lage:** (gemessen 2026-10-09) **15 verbleibende `gap`-Träger** (themis-tail + mms-magnetosheath gebaut/entfernt): `bc-mpo-more` · `tracking-doppler`/`viking-tracking`/`juno-efb` (NSSDC-Antworten offen; Parser stehen) · `mariner-rst` · `dmap-map-grid` · `kaguya-lrs` · `inpe-big-stac` · `hi-21cm` · `cmb-lambda` · `solar-vso` · `laic-cssdc` · `particle-cern` · `blinkverse-frb`. Gap-Token-Header-Kanon ist auf **22** korrigiert (stand 19; `:3`–`:24`).
- **Blockade:** je Träger der Bau (Arm/Workflow/`sources.φ`-Zeile) oder wartende Antwort.
- **Braucht:** je Träger Arm/Workflow/`sources.φ`-Zeile oder Disposition.

### PETREL19-Ephemeriden — Wei erteilt CC BY 4.0 (decline widerlegt)
- **Status:** eigen | **Bindung:** eigen (Register/Port)
- **Trigger:** Port gebaut (PETREL19-Arm im `ephemeris_compiler`)
- **Lage:** (gemessen 2026-10-09) Tian Wei (PETREL19-Autor) erteilt **CC BY 4.0**; die frühere `decline redistribution` entfernt, PETREL19 steht als `pending`. `ephemeris_compiler.rs` supportet PETREL19 noch nicht.
- **Blockade:** der PETREL19-Arm im Compiler fehlt.
- **Braucht:** `ephemeris_compiler.rs` um PETREL19 erweitern (SPICE/JPL-DE, 1799-10-13..2106-05-05) → `sources.φ`-Block mit `terms CC-BY-4.0 https://github.com/TIAN-we/petrel19`.

## An mycelium

Origin: mountain-folge284.

- **Wind-Arme (GIC §A) registriert:** `WI_K0_SWE` (Protonen `Np`/`V_GSE`) + `WI_H2_MFI` (`BGSM`) stehen in `phi/sources.φ`; bitte manifestieren, sobald die Zeilen stehen.
- **JAXA-Registerzitat (Riss):** `ledger.φ:120` zitiert `sources.φ:10962-10968` + `2286-2292` — gemessen sind das **BGR-Infraschall** bzw. **geoazur apdb**, kein JAXA. Die JAXA-Blöcke sind `gportal.jaxa.jp` (2×) + `data.darts.isas.jaxa.jp` (2×). Zitat auf **netloc-Key** umstellen, nie Zeilennummer (register_sort verschiebt Zeilen).
- **SuperMAG-Riss:** `ledger.φ:152` zitiert `sources.φ:18407-18431` (= Vega2/Mischa); in `phi/sources.φ` existiert **kein** SuperMAG-/substorm-Block, obwohl `frame_registry.φ:880-881` `supermag_2025-03*.bin` als Asset führt. Zitat korrigieren; substorm-Registrierung siehe Offen.
- **USGS-geomag:** E-Feld-Produkt (`/ws/data/?elements=E-E,E-N`) admission, Magnetik `declined` — `pending`-Eintrag `blocked_sources.φ:100-102` kann auf die E-Feld-Zeile geschlossen werden.
- **ShadowCam:** Admission **ja**, Format-Arm (PDS4-derived raster, kein `.fits`) fehlt — bitte bauen, Mountain schreibt die Zeile.
- **`blocked_sources.φ`-Gap-Token-Header** auf 22 korrigiert (stand 19).
- **CDN-Aufräumen (Operator-Wort 2026-10-09, aus 283):** 23 `ohne-lizenz`-Assets aus `sources.φ`/`harvest.φ` entfernt; die veröffentlichten Release-Assets im `omegaflow/sources`-Repo bitte neutralisieren/entfernen und, wo feld-relevant (IMCCE/SuperMAG), lokal unter `data/<netloc>/` halten.
- **`omegaflow/sources`-Repo/NOTICE (aus 283):** Repo-Description + `NOTICE` stehen; offen: der CI-Manifestator schreibt die Quell-Lizenz je Release-Body (liest die `terms`-Zeile).

## An river

Origin: mountain-folge284.

- **Register-Physik-Migration (Operator-Wort 2026-10-09):** gefaltet. Mountain hält die Register-Seite und schreibt die Verdikt-Zeilen in `phi/sources.φ` auf **deine** erste Block-Charge (`gravity`/`seismic`). Der `Domain/Rand`-Ausbau vor `em` bleibt die Bedingung — sag an, wenn die Schema-Charge steht.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (dieser Atom: `/commit`).

Eigene Pfade: `phi/sources.φ` · `phi/blocked_sources.φ` · `docs/handover/handover-2026-10-09-mountain-folge284.md` · `docs/handover/archiv/handover-2026-10-09-mountain-folge283.md`.
