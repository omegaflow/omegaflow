<!--
  title: Handover — Mycelium-Folge 297 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. Ephemeris-CDN-sha256 (llr/sinex/vmf3_site) + VMF3-GRID-Block und -Aufrufer (Origin gemessen) + cmb-cdn-SPT-Re-Dispatch; mountain-301-Block gefaltet.
  class: handover
  date: 2026-10-10
  sha256: 61ceced328ab10e0b1f9a1745c1c3ad47aab847b364ae6df9acf5ec21cb180b7
  status: live
-->
# Handover — Mycelium-Folge 297 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge296.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-301): die Ephemeriden-Register-Token sind
entschieden — ITRF2020 `terms unknown` **bleibt** (SPDX-NOASSERTION-Vokabel,
`license_census.rs:28`; mein `attribution`-Vorschlag verworfen); VMF3 `ttl 86400`
und LLR `no-cadence` bestätigt; planetary radar `at <zielplanet>`
(`at venus`/`at mercur`/`at mars`).

## Burn: open 0.0000 · close 0.0972 · cap 0.5 — Grund: Meta-Pass + 3 Ephemeris-CDN-sha256 + VMF3-GRID-Block und -Aufrufer (Origin via Playwright gemessen) + cmb-cdn/SPT-Re-Dispatch + mountain-301-Faltung · deepseek-flash, kein pro/max (gemessen `session_burn`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 297) |
| „ich möchte alle haben" (open-lidar-data · Bayern-DOM-Quelle · DOM-Reader-Bin) | 2026-10-10 | Operator (Session, Mycelium 296) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge295.md` §Operator-Wort-Register (via `git show <sha>:archiv/…`) | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — SPT-3G D1 `cmap` (cmb-cdn-Re-Lauf)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `cmb-cdn 38058517809` Abschluss
- **Lage:** (gemessen 2026-10-10) der Vorlauf `38032912687` wurde bei 4 h
  `cancelled`; die zwei Vorläufe (`38005262361`/`38007354300`) fielen am
  curl-Transfer-Bound: `curl: (28) … 964031862 out of 7873515864 bytes` (2048 s,
  `TRANSFER_BOUND_S`). Mountain-297 (`8a9699191`) hat den SPT-Bound auf
  `6 * 3600` (`SPT_TRANSFER_BOUND_S`, `cmb_planck_compiler.rs:7`) gehoben; der
  Workflow-`timeout-minutes` 360 ist die Kante (7,87-GB-Tarball ≈ 4,6 h @
  470 KB/s).
- **Blockade:** Laufdauer.
- **Braucht:** Abschluss → bei success den `sha256` in den SPT-`cmap`-Block
  (`cmb_spt_d1_n64.json`); bei erneutem Transfer-Void den Tarball-`--member`
  bzw. den Bound nachmessen.

### Planetary Radar — Ephemeris-Block (Origin offen)
- **Status:** eigen | **Bindung:** eigen (Manifestation)
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) mountain-301 hat den `at`-Frame entschieden
  (`at <zielplanet>`, Vokabel `parse.rs:298`); die **Origin-URL ist ungemessen**
  (iaaras.ru live unerreichbar via Wayback; JPL-Goldstone-Links tot; lebende
  JPL-DB = `sb_radar.api`).
- **Blockade:** keine gemessene Origin.
- **Braucht:** eine live Origin-URL messen (`archive_search --verdict`/`--sniff`
  der `sb_radar.api`-Endpunkte), dann den Block (`at venus`/`at mercur`/`at mars`,
  `compiler`, `terms`, `ttl`) schreiben — kein `at` ohne Origin, kein Fabricat.

### Weltweite LiDAR-Landschaft + `open-lidar-data` (COPC) + Manifestator
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Operator-Wort „alle" liegt vor
- **Lage:** (gemessen 2026-10-10) die **Survey**
  `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` trägt die weltweite
  Landschaft (4 Recherche-Taucher, jede URL per `--verdict` gemessen): global
  GEDI/ICESat-2/GLAD/OpenTopography/Copernicus; Europa bundesweit offen
  (NL/CH/DK/NO/SE/FI/EE/LV/LT/PL/CZ/SK/SI/PT/FR/BE/LU/IE/UK/AT), DE nur
  länderweise (Bayern/NRW/BW/NI/SN); Nordamerika (USGS 3DEP **registriert**
  `phi/sources.φ:20260`, NOAA NOS im Katalog, CanElevation/BC/ON/QC); LatAm
  (INPE, PMSP, IDE Chile, IGN AR …); Asien-Pazifik (JP-AWS, KR, TW, HK, IN, CN,
  SG, ID, PH-LiPAD, TH, MY, AU ELVIS/GA/QLD/NSW, NZ LINZ); Afrika/Nahost (kein
  nationales Massen-LiDAR, nur Kampagnen); **Unterwasser/Bathymetrie** (GEBCO,
  ETOPO, SRTM15+, Seabed 2030, NCEI/IHO-DCDB-Multibeam, EMODnet, IBCSO; IBCAO
  tot, AusSeabed 403, R2R 503). Die zuvor `pending` geführten Hosts wurden
  **inkl. Proton-Stufe** nachgemessen: PNOA/CNIG/RO/GR/SK/PT-IGEO/BG/
  Hessen/Thüringen/Hamburg/IBCAO/HCMC liefern auf direkt UND Proton keine
  Antwort (nur veralteter Wayback) → außer Betrieb, kein `pending`; BIG-Indonesien
  (206) und Alaska (200) sind erreichbar. Der **LAS/COPC-Reader steht**
  (`src/archivar/las/`, LASzip-Chunk-Dekoder gebaut); die frühere Zeile
  „omegaflow hat **kein** LiDAR registriert" war **falsch** (296→297 getragen,
  hier korrigiert).
- **Blockade:** keine.
- **Braucht:** je Quelle ein Manifestator (`tools/harvest`, Vorlage
  `las_compiler.rs`/`copernicus_dem_compiler.rs`) + die Mycelium-Registrierung.
  Erster Schritt: einen LAZ/COPC-Key aus dem open-lidar-data-Bucket (`curl` der
  S3-Liste, Key extrahieren), Reader gegen die echte Datei, dann Block +
  `*-cdn.yml`.

### Bayern-DOM-Rasterquelle (`geodaten.bayern.de`) + DOM-/GeoTIFF-Reader
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Operator-Wort „alle" liegt vor
- **Lage:** (gemessen 2026-10-10) Bayern laser-DOM „DOM20" (nativ 0,2 m → 1 m),
  Bayerische Vermessungsverwaltung, Lizenz **CC BY 4.0**; Float32, 1 m, NHN, COG.
  Der TIFF-Predictor-Refactor ist zentral (`src/archivar/tiff.rs`
  `apply_predictor` + `undo_predictor2/3`, geheilt `3c4786a3b`);
  `copernicus_dem_compiler.rs` ist die Vorlage.
- **Blockade:** keine.
- **Braucht:** (a) einen COG-Kachel-URL messen; (b) ein `dom_compiler`-Bin
  (`tools/harvest`) auf `tiff::apply_predictor` gegen die echte Kachel;
  (c) `sources.φ`-Block (format dom/GeoTIFF, at earth, terms CC-BY-4.0, ttl).

### Architektur — GitHub/CI/CDN-Optimierung (Survey + Rat)
- **Status:** eigen | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey
  `docs/surveys/survey-2026-10-10-github-ci-cdn-optimierung.md`; Maßnahme 1–5
  gebaut; **Maßnahme 3** (ci-check 4-fach-nextest-Shard) wartet auf die
  Messung der Shard-Wall-Clock.
- **Blockade:** keine.
- **Braucht:** (a) die 4 roten Kern-Tests sind geheilt (`30eaa7bca`), den
  ci-check-Shard am Log bestätigen; (b) **Rat Runde 2**: Test-Suite-Dedup als
  eigener begrenzter Dispatch.

### Speicher — 1,76 TB Bulk vs. R2-10-GB
- **Status:** eigen (Architektur) | **Bindung:** eigen
- **Trigger:** Entlastung/Rebalancing nötig
- **Lage:** (gemessen 2026-10-10 via GitHub-API) `omegaflow/sources` = 344
  Releases / 1763 GB → $0; R2 10 GB frei trägt Manifeste/Indizes.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort (Survey-Säule B), ob R2 als Hot-Tier kommt.

### MCP — lokale no-leak-Server
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey Säule F: `local` stdio no-leak-Fit;
  GitHub-MCP readonly.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort (Rat) für den MCP-`block` in `opencode.json`,
  dann begrenzter Dispatch.

### Zweite CI-Lane — self-hosted t420
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Bedarf (regelt sich selbst)
- **Lage:** (gemessen 2026-10-10) 20 Workflows `runs-on: [self-hosted, Linux]`;
  Throttle aktiv; Konzept `docs/concepts/self-hosted-runner.md`.
- **Blockade:** keine.
- **Braucht:** tunen via `/etc/default/runner-throttle`; `pending`: per-Gerät-QoS.

### CI-Hygiene — `matrix-rotor` GitHub-Präemption
- **Status:** wartend | **Bindung:** eigen (Workflow, ggf. linie:river)
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38054215238`) erneut rot:
  „The runner has received a shutdown signal" → `rotor slice ended rc=137`
  (hosted-Runner-Präemption). Watchdog-Re-Run verbraucht.
- **Blockade:** der ~5 h `rotor slice` auf gehosteten Runnern wird präemptiert.
- **Braucht:** gecheckpointete kürzere Slices (State alle 120 s liegt vor) oder
  dauerhafter self-hosted Runner; Survey-Säule A.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** `hips-png-cdn`-Lauf Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) `37932098229` in_progress
  (seit 2026-10-09T12:45Z) und `38043712533` in_progress;
  `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer.
- **Braucht:** Abschluss → bei success `phi/pipeline/ledger.φ:110` → `disponiert`
  + CDN-Asset prüfen.

### CI — `te_ground_truth` Artefakt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Artefakt `te-bias-n` gelesen
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) Lauf `38038722712` success
  (head `ea4025b28`, trägt `POINT te-ground-truth`).
- **Blockade:** keine.
- **Braucht:** Artefakt `te-bias-n` prüfen; scalar-KDE-Arm bleibt benannter Riss
  (mountain-299).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.
