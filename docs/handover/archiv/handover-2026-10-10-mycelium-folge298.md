<!--
  title: Handover — Mycelium-Folge 298 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. mountain-303-Faltung (Asservatenkammer-Träger); register-CI-Riss (Asservatenkammer-see-also) geheilt; Planetary-Radar-Origin gemessen; cmb-cdn-SPT-Re-Dispatch.
  class: handover
  date: 2026-10-10
  sha256: 67c969e4475dcedeb7d3d4fe4be20b60a63d626f28cf2b4dcf7185ab510028e5
  status: live
-->
# Handover — Mycelium-Folge 298 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge297.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-303): Träger für den Asservatenkammer-Zensus
gesetzt (s. u.). `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md:7`
verwies im `see-also` auf die nach `archiv/` bewegte Mountain-folge302-Übergabe —
`path_reference_scan` brach an dieser Zeile die register-CI-Kante; die Referenz
zeigt jetzt auf den archivierten Pfad (body-identisch, sha256 unverändert).

## Burn: open 0.0000 · close 0.0382 · cap 0.5 — Grund: Meta-Pass folge298 (mountain-303-Faltung + register-CI-Reparatur + Planetary-Radar-Origin + cmb-cdn-Re-Dispatch) · deepseek-flash, kein pro/max (gemessen `session_burn`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 298) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge297.md` §Operator-Wort-Register (via `git show <sha>:archiv/…`) | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — SPT-3G D1 `cmap` (cmb-cdn)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `cmb-cdn 38078371948` Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38058517809`) der Re-Lauf
  `38058517809` (head `f8f332884`) starb mit `##[error]The runner has received a
  shutdown signal` → `exit code 137` mitten im 7,87-GB-Tarball-Download
  (14:09→19:01Z, ~4h52m). Kein Transfer-Bound (kein `curl: (28)`), sondern
  **self-hosted-Runner-Shutdown**. Re-Dispatch `38078371948` (19:03Z).
- **Blockade:** der 7,87-GB-Download ist nicht resume-fähig; der self-hosted
  Runner wird nach ~5 h gestoppt.
- **Braucht:** `curl -C -`/Range im `cmb_planck_compiler` oder ein langlebiger
  Runner (Survey-Säule A); bei success den sha256 in den SPT-`cmap`-Block
  (`cmb_spt_d1_n64.json`).

### Planetary Radar — Ephemeris-Block (Origin gemessen)
- **Status:** eigen | **Bindung:** eigen (Manifestation)
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10 via `archive_search --verdict/--sniff`) die
  lebende Origin `https://ssd-api.jpl.nasa.gov/sb_radar.api` antwortet **HTTP 200**,
  **384112 B**, sha256 `0232b4c56b5cfec324da88d9d5fdadffc04697afce0082de13ad1d1c4ee8ac0b`;
  Doc `https://ssd-api.jpl.nasa.gov/doc/sb_radar.html` (SB Radar = Kleinkörper-
  Radar-Astrometrie, v1.1). mountain-301 hat den `at <zielplanet>`-Frame entschieden
  (`parse.rs:298`).
- **Blockade:** kein Compiler, keine Feld-Zuordnung definiert.
- **Braucht:** `sb_radar_compiler` (`tools/harvest`) + Feld (radar delay/doppler)
  + `at`-Anker festlegen, dann den Block (`origin`/`compiler`/`terms`/`ttl`)
  schreiben — kein Block ohne Compiler, kein Fabricat.

### Carriership — Asservatenkammer-Zensus (`research-api-mcp`)
- **Status:** eigen | **Bindung:** eigen (gefaltet aus mountain-303)
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) `docs/surveys/survey-2026-10-08-research-api-mcp.md`
  trägt 3 offene Marker (Consensus · Perplexity · Elicit/SciSpace). Der Survey-Stand:
  `--consensus` und `--perplexity` laufen live (letzterer `1569a26d8`); Elicit
  `descoped` (kommerziell); SciSpace kein Arm (`pending`, cookie-API, HTTP 403).
  Die Auth-Route/Keys sind Mycelium-Domäne.
- **Blockade:** keine.
- **Braucht:** den ersten offenen Arm konkretisieren — die zwei lebenden Arme am
  gebauten Binary bestätigen (`archive_search --consensus`/`--perplexity`), SciSpace
  als `pending` benannt lassen; Keys liegen in `.secrets.local` (Operator, per-Akt).

### Weltweite LiDAR-Landschaft + `open-lidar-data` (COPC) + Manifestator
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Operator-Wort „alle" liegt vor
- **Lage:** (gemessen 2026-10-10) die Survey
  `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` trägt die weltweite
  Landschaft (jede URL per `--verdict` gemessen, inkl. Bathymetrie/Unterwasser);
  `portal_harvest` (`tools/harvest`) steht (CKAN `package_search` → Lizenzklassifikation
  → `--package-show`); erster open-lidar-data-Asset gemessen (BE-COPC, HTTP 206).
- **Blockade:** keine.
- **Braucht:** je Quelle ein Manifestator (`tools/harvest`, Vorlage
  `las_compiler.rs`/`copernicus_dem_compiler.rs`) + die Mycelium-Registrierung.

### Bayern-DOM-Rasterquelle (`geodaten.bayern.de`) + DOM-/GeoTIFF-Reader
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Operator-Wort „alle" liegt vor
- **Lage:** (gemessen 2026-10-10) Bayern laser-DOM „DOM20" (nativ 0,2 m → 1 m),
  CC BY 4.0; Float32, 1 m, NHN, COG. TIFF-Predictor-Refactor geheilt (`3c4786a3b`);
  `copernicus_dem_compiler.rs` ist die Vorlage.
- **Blockade:** keine.
- **Braucht:** (a) einen COG-Kachel-URL messen; (b) ein `dom_compiler`-Bin auf
  `tiff::apply_predictor`; (c) `sources.φ`-Block (format dom, at earth, CC-BY-4.0, ttl).

### CI — VNP46A3-CDN LAADS-Auth
- **Status:** eigen | **Bindung:** eigen (Manifestation)
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38045886149`) der Lauf
  `38045886149` starb an `vnp46a3: fetch … returned (exit status: 22)` →
  `granule stays pending` (curl 22 = HTTP-Fehler, kein Parse-Fehler); der
  LAADS-Granule-URL fehlt die Earthdata-Auth (EDL-Token).
- **Blockade:** kein Auth-Header im `vnp46a3_compiler`.
- **Braucht:** `EARTHDATA_EDL_TOKEN`-Header in den LAADS-Fetch des Compilers
  (Auth-Route, Token in `.secrets.local`), dann Re-Dispatch.

### Architektur — GitHub/CI/CDN-Optimierung (Survey + Rat)
- **Status:** eigen | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey
  `docs/surveys/survey-2026-10-10-github-ci-cdn-optimierung.md`; Maßnahme 1–5
  gebaut; Maßnahme 3 (ci-check 4-fach-nextest-Shard) wartet auf die Messung der
  Shard-Wall-Clock.
- **Blockade:** keine.
- **Braucht:** (a) ci-check-Shard am Log bestätigen; (b) **Rat Runde 2**:
  Test-Suite-Dedup als eigener begrenzter Dispatch.

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
  „The runner has received a shutdown signal" → `rotor slice ended rc=137`.
  Gleiche Signatur wie der SPT-Shutdown oben — self-hosted Runner-Stop.
- **Blockade:** der ~5 h `rotor slice` wird durch den Runner-Stop präemptiert.
- **Braucht:** gecheckpointete kürzere Slices (State alle 120 s liegt vor) oder
  dauerhafter Runner; Survey-Säule A.

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
