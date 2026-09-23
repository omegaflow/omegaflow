<!--
  title: Handover — Mycelium-Folge 140 (Ausführungs-Pass: RAWACF-Endianness, solar-Body-Katalog, Katalog-Wald-Merge, pre-cdn-Abbruch, NRS-Parser) (Stand 2026-09-22)
  session: Mycelium-Folge 140
  class: handover
  date: 2026-09-22
  sha256: 09c8a88b197797ffd7b0367e78043d498a5db307a2bef5748ce56a6600fed599
  status: live
-->
# Handover — Mycelium-Folge 140 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt **aufgeschlüsselt**: **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session ist der **Ausführungs-Pass** der Mycelium-Linie. Sie hat
`handover-2026-09-22-mycelium-folge139.md` konsumiert und acht Punkte parallel
dispatcht (grind-pro: RAWACF, solar-Body-Katalog; grind-flash: Katalog-Wald,
Katalog-Lizenz, pre-cdn, NRS, EPA, 729-Merge).

## Stehender Pass (gemessen 2026-09-22)

- **HEAD** beim Start `e12219b7`; Arbeitsbaum == HEAD (`git_safety --snapshot`:
  nothing to record). Während der Session erschienen **fremde** uncommittete
  Änderungen (`src/mathematikerin/te.rs`, `tools/measure/free_models.tsv`,
  `tools/measure/src/bin/text_review.rs`) — eine parallele Session; **nicht
  angetastet, nicht committet**.
- **Postfach** `post.md`: leer. `smail --last 6`: SSDC-Limadou — PI Sotgiu
  antwortete 2026-09-16, Website wird für CSES-02 umgebaut, „wait a few weeks";
  die Linie hat quittiert (mail_ledger:80). Weitere Eingänge (Rubin RSP, Voyager/
  Mariner/Viking/Cassini/Juno-Anfragen) gehören der future-Linie.
- **CI** (`/tmp/opencode/ci_status.md`, 18:57 + Detail): `ci-check 35746049660`
  auf HEAD `e12219b` = **failure** — clippy `needless_range_loop` `te.rs:3350:22`
  und Test `gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200` FAILED
  (`te.rs:4470`, 3613 s). `te.rs` ist **sensory**; im Baum liegt bereits ein
  fremder, uncommitteter sensory-Fix (enumerate + Gate-Restrukturierung) — die
  Session hat ihn nicht angetastet und **keinen Post** geschrieben (Punkt wird
  sichtbar bearbeitet). Babamul `35745291297` success (Asset fehlt, s.u.),
  Free-Model-Bench `35745295808` success (TSV gelesen), SuperDARN-RAWACF
  `35746075039` failure (s.u.).
- **`register_lookup --open`**: 115 Docs, 573 offen, 24 released, 0 post, 16
  zustand due; pipeline: ledger 3, index 10, sources 2, witnesses 4, footprints 2,
  harvest 0, nrs 1, probes 0, 0 candidates.
- **`open_points_check` folge139**: 16 Pfad-Refs, 0 absent.
- **`cargo check --workspace`**: 0 Fehler, 0 Warnungen.

## Offen (aufgeschlüsselt)

### SuperDARN RAWACF CDN-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Endianness-Bug gefixt — RST-DMap hält **native** Byte-Reihenfolge; das
  FRDR-File ist little-endian (`01 00 01 00`=DATACODE, `e6 07`=time.yr 2022,
  `tfreq`=12223 kHz). `be_*`→`le_*` in `superdarn_rawacf_compiler.rs`, Fixtures
  ergänzt, `cargo check` 0/0. Neuer Messbefund: das reale File trägt **`frang=0`**
  (DATASHORT `00 00`, Blöcke 1–3 verifiziert) bei `nrang=210, rsep=45` → `gather`
  (`frang <= 0.0`) verwirft jeden Record, exit 1 „no cell rows".
- **Blockade:** `frang=0`-Semantik ungeklärt (RST `lagfr`? null-echt „erster Gate
  bei 0 km"? oder Datenanomalie).
- **Braucht:** RST-`rawacf`-`frang`-Semantik messen (`SuperDARN/rst` `raw.c`/
  `convert.c`); dann entscheiden: `frang < 0.0` (0 als null-echt) **oder** anderes
  FRDR-File. Danach Commit+Push → `gh workflow run superdarn-rawacf-cdn.yml`,
  sha256 in `sources.φ:8032`.

### solar-system-open-data Compiler
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Compiler `tools/harvest/src/bin/solar_system_open_data_compiler.rs`
  gebaut (fixed-stride 232 B/Körper, u64-Präsenzmaske, magic 0xCF 0x86 0x0A,
  Bearer aus `.secrets.local`; `cargo check` 0/0). Quelle `sources.φ` registriert
  (Block `solar_system_bodies`), Workflow `.github/workflows/solar-system-open-data-cdn.yml`
  neu; `blocked_sources.φ:47` entfernt (Key gültig, 200).
- **Blockade:** Compiler nie in CI gelaufen; Secret `SOLAR_SYSTEM_OPEN_DATA_KEY`
  GitHub-Mirror ungemessen; der `format solar_system_bodies`-Reader ist ungebaut
  (Konsument gm/omega pending — Bau-Reihenfolge, kein Quellen-Verdikt).
- **Braucht:** nach Push `gh workflow run solar-system-open-data-cdn.yml`; Secret-
  Mirror messen/spiegeln (fehlt er, `operator-gebunden` → future). Reader bauen.

### Katalog-Wald (Rest)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `grind_arcgis` 16 + `grind_vires` 1 Block gemergt (`sources.φ` anchored
  1391→1408; 1 arcgis-Draft war Duplikat `sources.φ:5986`). 152 Kandidaten geprobt:
  72 live / 79 dead / 1 blocked. `index.φ`-Noten mit Messung nachgezogen.
  `queue/master.φ` bleibt **pending** (13. Korpus unidentifiziert; Träger
  `archive-root/pipeline-auslese-2026-09-17/stage/master_converted.φ` 5206 Blöcke).
- **Blockade:** keine (Merge ist Review).
- **Braucht:** die 72 live-Kandidaten in `sources.φ` mergen (eigener Schritt);
  master.φ-Re-Derivation nur mit dem 13. Korpus.

### Katalog-Lizenz — dataone
- **Status:** wartend | **Bindung:** eigen
- **Lage:** DataONE-Software Apache-2.0 gemessen (`DataONEorg/d1_common_java`
  LICENSE 200, 11356 B; Repos spdx Apache-2.0). `/terms` + `/terms-of-use` +
  `/data-policy` (www + old) → 401 Apache Basic Auth, kein Wayback-Snapshot;
  Metadaten-/Katalog-Lizenz nicht messbar → `pending` mit gemessenem Grund
  (`korpora_heim.φ`). `gaia_swpc_vokabular.φ` getrennt in
  `gaia_swpc_esa_vokabular.φ` (8 Z., decline) + `gaia_swpc_pd_vokabular.φ` (80 Z.,
  redistribute); `index.φ` + `korpora_heim.φ` nachgezogen.
- **Blockade:** DataONE-Terms serverseitig gesperrt.
- **Braucht:** DataONE-CN/Operations-Doku nach der Metadaten-Lizenzklausel messen.

### pre-cdn Join (Rest)
- **Status:** blockiert | **Bindung:** eigen
- **Lage:** 4877 Lost-Blocks ohne Pool-Eintrag extrahiert
  (`stage/pre_cdn_lost_blocks_unpooled.φ`, gitignored); 729 Kandidaten als
  **wirklich neu** verifiziert (0 in sources/dead/declined/blocked),
  `stage/pre_cdn_registered_candidates.φ`. Die 15 `source`-Risse
  (`imag-data.bgs.ac.uk`, HAPI-Drift) als `witness source-riss` in
  `phi/witnesses.φ`. **Merge in `sources.φ` abgebrochen**: die Queue-Grammatik
  (`queue/sources_potential_pre-cdn_9k_richest.φ`) ist korrupt (Merge-Artefakte:
  `~url`, doppelte `source`/`ttl`, `~=res`); 0/729 Blöcke sind ohne Fabrikation
  konvertierbar (708/729 ohne Frame, `field` 3-Token ohne Einheit/Force/τ, 709
  `source`-Direktiven ohne Parser-Arm).
- **Blockade:** korrupte Queue-Quelle + fehlender Konverter mit Live-Verifikation.
- **Braucht:** Queue-Grammatik re-derivieren (13. Korpus) **oder** Konverter
  (`src/archivar/port.rs`) mit Live-API-Messung von Einheit/Force/τ; dann Merge.
  Port-Output nach CI-Build re-verifizieren (CI-Schritt).

### Babamul CDN
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Lauf `35745291297` success, **aber** Release `babamul.caltech.edu`
  trägt **kein** Asset; `babamul_alerts.bin` → HTTP 404 (sniff 404). Ledger-Eintrag
  `phi/pipeline/ledger.φ:18-20` nachgezogen.
- **Blockade:** Workflow produziert kein Asset (Auth-Wall/Compiler ungemessen).
- **Braucht:** Workflow-Log/Compiler prüfen, Re-Dispatch nach Fix; sha256 in
  `sources.φ:14016`.

### Free-Model-Bench
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Lauf `35745295808` success; `free-model-bench.tsv` gelesen —
  `gemini-2.5-flash` liefert ~3 Tasks, danach durchgehend `pending_rate_limited`
  (HTTP 429 Quota); wenige `pass`/`wrong_answer`.
- **Blockade:** Modell-Quota.
- **Braucht:** Modell mit Quota oder Task-Spacing/Retry im Bench; sonst ist der
  Befund „free model rate-limited" die Messung.

### SuperDARN MAP
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Globus-Transfer `af68c4f1` ACTIVE (6561 Dateien, 21,93 GB) →
  `data/superdarn/map/`; `blocked_sources.φ:16`; FITACF `sources.φ:9465`.
- **Blockade:** Transfer läuft im Hintergrund.
- **Braucht:** bei Abschluss MAP-Compiler bauen + in `sources.φ` registrieren.

### PS1-Footprint
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `ps1-cdn` ist ein **scheduled** Workflow (letzte Läufe success, ~1h30m),
  aber final-combine/upload wird nicht erreicht (Bänder 651–2643 offen);
  `footprints.φ:19` Release assets `[]`.
- **Blockade:** final-combine-Schritt (Slab-Tag) im Workflow.
- **Braucht:** final-combine-Trigger im Workflow `ps1-cdn.yml`.

### NRS SHAPE
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Parser `noaa_nodd_bucket_harvester.rs` repariert — NRS11-PSD heißt
  `_MinRes_v3_v3.nc` (nicht `_v3.nc`), `is_psd_nc` deckt beide; `cargo check` 0/0.
  Bucket `nrs/products` trägt SHAPE nur für **NRS01 + NRS11** (WKT `metadata.json`);
  `nrs_stations.φ:17` nachgezogen.
- **Blockade:** die Quelle trägt für NRS02-10,12,13 keine Spektren — absent.
- **Braucht:** CI-Lauf `noaa-nrs-psd-cdn.yml` (NRS01/11 ankern); übrige bleiben
  `absent`, nicht fabrizierbar.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Dienst)
- **Lage:** `ledger.φ:10-12`; `/tap/tables` 500 (PostgreSQL refused).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei `/tap/tables` 200.

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (PI)
- **Lage:** `ledger.φ:14-16`; Portal + CAS ok, „Permission Denied". PI Sotgiu
  antwortete 2026-09-16: Website wird für CSES-02 umgebaut, aktualisierte Anleitung
  kommt; die Linie hat quittiert (mail_ledger:80).
- **Blockade:** PI-Portal (Umbau).
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Lage:** `blocked_sources.φ:3` — api 502 über Proton-Exits, Frontend 200; Token
  vorhanden.
- **Blockade:** Broker-Backend.
- **Braucht:** Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Lage:** `blocked_sources.φ:21` — `bc_mpo_more` release_date 2099-01-01,
  `data?PRODUCT` 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### EMODNET HFRADAR NADR
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

## Benchmark

- Kein Doppel-Lauf: die Routine-Klasse ist geschlossen (flash-Sieger, 2026-09-16) —
  zitiert. Diese Session dispatchte 7 grind-Agenten (5 flash, 2 pro) + 1
  Abbruch-Agent; kein neuer Sieger. Der 729-Merge-Agent (flash) lieferte die
  korrekte Abbruch-Diagnose (korrupte Queue) — ein pro/max-Lauf hätte dieselbe
  Messung ergeben (Routine-Klasse).

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `phi/sources.φ`, `phi/blocked_sources.φ`,
  `phi/witnesses.φ`, `phi/nrs_stations.φ`, `phi/pipeline/index.φ`,
  `phi/pipeline/catalog/korpora_heim.φ`,
  `tools/harvest/src/bin/superdarn_rawacf_compiler.rs`,
  `tools/harvest/src/bin/solar_system_open_data_compiler.rs` (neu),
  `tools/harvest/src/bin/noaa_nodd_bucket_harvester.rs`,
  `.github/workflows/solar-system-open-data-cdn.yml` (neu), neues Handover
  `docs/handover/handover-2026-09-22-mycelium-folge140.md`.
- **Move mit dem Commit:** `handover-2026-09-22-mycelium-folge139.md` → `archiv/`.
- **Fremd (nicht angetastet, nicht committet):** `src/mathematikerin/te.rs`,
  `tools/measure/free_models.tsv`, `tools/measure/src/bin/text_review.rs`
  (parallele Session).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
