<!--
  title: Handover — Mycelium-Folge 216 (2026-10-01)
  session: Mycelium-Folge 216
  class: handover
  date: 2026-10-01
  sha256: 78776412f39581390635f3a2e97078e922d118b7bcb17eea52ee06332ccc19ad
  status: live
-->
# Handover — Mycelium-Folge 216 (2026-10-01)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-01-mycelium-folge215.md` (→ `archiv/`).

## Burn: open 0.0362 · close 0.0000

## Haus (die vier Orte) — gemessen 2026-10-01

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02).
- `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all` über den gitignorierten Baum.
- `phi/pipeline/catalog/*` ist **gitignored**; `phi/pipeline/index.φ` + `ledger.φ` sind trackbar.

## Erledigt in diesem Atom (git trägt es)

- **Kaguya Re-Manifest (force):** `--sniff` der CDN-Zeile = HTTP 200, **8200300 B**, sha256 `772e51d1…` → `sha256`-Zeile in `phi/sources.φ` Kaguya-Block gesetzt.
- **DAS2 / `das2-iowa-cdn`:** `--sniff` = HTTP 200, **13814412 B**, sha256 `cd2e9d2b…` → `phi/harvest.φ` `asset fehlt` → `asset present`, `sha256`-Zeile in `phi/sources.φ:10227`.
- **Dispatches 2026-10-01:** `kernel-flatten 36836613716`, `cdse-stac-probe 36836617223`, `cosmicflows-cdn 36836706826`, `pioneer-telemetry-cdn 36836711251`.
- **Adressierte Blöcke gefaltet:** future-folge162 (`## An mycelium`), mountain-folge216 (`## An mycelium`), sensory-folge215 (`## An mycelium`).
- **Roter Baum am `1a340b191`** (3 fremde `ci-check`-Tests) von Mountain-216 geheilt (`946c7b232`); der Punkt ist geschlossen.
- **vco-rs PDS4** bereits an `0c32b4a87` umgestellt (mountain-folge216 bestätigt PDS4 kanonisch).
- **Gebaut 2026-10-01 (Operator-Wort „bearbeiten"):** `pioneer11_odf_compiler.rs` trägt `--ci-mode` + `upload_release("spdf.gsfc.nasa.gov")`; `pioneer-odf-cdn.yml` trägt den Pioneer-11-Arm; `pioneer-telemetry-cdn.yml`-Tag-Riss (`ssd.jpl.nasa.gov` → `spdf.gsfc.nasa.gov`) geheilt; `sources.φ` trägt `pioneer11_odf` + `pioneer10_telemetry` (url/format/origin/compiler; `ttl` bei Mountain); die drei generischen Ephemeriden-`url` auf `_long` umgestellt (`new_horizons_long` 78928 B/`cca3d4cd…`, `voyager1_long` 255888 B/`185f2c18…`, `voyager2_long` 211088 B/`dfaa08fc…`). `cargo check` grün.

## Offen (aufgeschlüsselt)

### CDSE-CCM STAC-Auth-Asset — Asset-Zeile messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `cdse-stac-probe 36836617223` (dispatched @`6c5490f79`)
- **Lage:** (gemessen 2026-10-01 via `ci_manage log`) der Lauf `36833184136` @`0c32b4a87` endet grün, die Asset-Stufe lieferte weiter `403` (`fetch_bytes_headers`), weil der Bin dort noch den Header beim Redirect mitsandte; der aktuelle Baum nutzt `fetch_raw_bytes_headers_redirect` (`stac_asset_fetch.rs:126`).
- **Blockade:** Lauf noch nicht durch
- **Braucht:** `ci_manage log 36836617223` — echte Asset-Zeile (`bytes sha url`); dann `sha256`/Größe in den Block in `phi/sources.φ`.

### Halley/Itokawa — Re-Manifest messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `kernel-flatten 36836613716`
- **Lage:** (gemessen 2026-10-01 via `--sniff`) `ephemeris_itokawa.bin` 404 (9 B); `itokawa` NAIF `2025143` steht in `horizons_compiler.rs:637` (`974e88030`).
- **Blockade:** keine
- **Braucht:** `archive_search --sniff https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov-ephemeris/ephemeris_itokawa.bin` nach dem Lauf.

### Registrierte Assets mit CDN-404 — Re-Manifest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `kernel-flatten 36836613716` (ephemeris pioneer10/11) + `cosmicflows-cdn 36836706826`
- **Lage:** (gemessen 2026-09-30 via `--verdict`) `cosmicflows_cf4.json` (`phi/sources.φ:10479`), `ephemeris_pioneer10_daily.bin`, `ephemeris_pioneer11_daily.bin` liefern CDN-404; lokal byte-gleich.
- **Blockade:** Re-Manifest-Läufe
- **Braucht:** `--verdict` der drei Zeilen nach den Läufen; die lokale Kopie bleibt Sicherung.

### Pioneer-Registrierung — gebaut, Manifest läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `pioneer-odf-cdn` (Pioneer-11-Arm) + `pioneer-telemetry-cdn`
- **Lage:** (gemessen 2026-10-01 via `sread`/`--sniff`) gebaut: `pioneer11_odf_compiler.rs` lädt nach `spdf.gsfc.nasa.gov`, `pioneer-odf-cdn.yml` trägt den Pioneer-11-Arm, `pioneer-telemetry-cdn.yml` prüft jetzt Tag `spdf.gsfc.nasa.gov` (vorher `ssd.jpl.nasa.gov`), `sources.φ` trägt beide Blöcke.
- **Blockade:** Manifestation noch nicht gelaufen (der frühere Dispatch nutzte die alte Workflow-Fassung)
- **Braucht:** nach dem Push `gh workflow run pioneer-odf-cdn.yml` + `gh workflow run pioneer-telemetry-cdn.yml`, dann `--sniff` der beiden Assets; `ttl` für beide Blöcke bei Mountain (`## An mountain`).

### `ci-gate` dropped-gate + queued Rot
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `ci-gate 36835769410` / `ci-check 36835769072` @`6c5490f79`
- **Lage:** (gemessen 2026-10-01 via `ci_manage jobs/log`) `ci-gate 36827564502` @`0c32b4a87` rot: clippy (`chebyshev_evaluate` E0425, needless_range_loop) + format + `dropped-gate baseline 1330 | current 1333 | delta 3`; clippy/format von Mountain-216 geheilt, der Delta-3-Drop offen.
- **Blockade:** CI-Zahl ist CI-only
- **Braucht:** `ci_manage jobs 36835769410`; hält der Delta, `docs/zustand/dropped-baseline.md` im annehmenden Commit bumpen (Wert = gemessene CI-Zahl).

### Ephemeriden-Re-Manifest nach dem Solver-Fix
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `kernel-flatten 36836613716`-Ende
- **Lage:** (gemessen 2026-10-01 via mountain-folge216 `## An mycelium`) `solve_normal_equations`-Fix healte den ~40-m-Fit; die alten Bins tragen den Fehler.
- **Blockade:** keine
- **Braucht:** nach dem Lauf `ephemeris_granule_census` gegen die neuen Bins.

### Kaguya-Idempotence-Audit
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `*-cdn`-Lauf mit geändertem Compiler
- **Lage:** (gemessen 2026-10-01 via `sread .github/workflows/pds3-binary-cdn.yml`) `:10/:29` trägt den `force`-Input.
- **Blockade:** keine
- **Braucht:** prüfen, ob weitere `*-cdn.yml` denselben `force`-Block für geänderte Compiler brauchen.

### pds3-img M3 — CI-Route 403
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pds3-img-cdn`-Lauf-Ende
- **Lage:** (gemessen 2026-09-30) `36737530030` failure: `M3G20081118T222604_V03_LOC.HDR` (`pds-imaging.jpl.nasa.gov`) HTTP 403; lokal direkt 206, Proton 403, kein Wayback-Snapshot.
- **Blockade:** CI-Runner-IP 403; kein Mirror
- **Braucht:** source-seitigen Mirror messen (`archive_search --playwright <url>`); sonst Descope-Befund.

### hips-png / ps1 — laufende Shards
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-10-01 via `ci_manage status`) `ps1-cdn 36723543966` success; `hips-png-cdn 36831989439` queued.
- **Blockade:** keine
- **Braucht:** `ci_manage jobs`/`log` beim Lauf-Ende.

### Legacy-CDN Re-Manifest (ssd family tag)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `nvss-cdn` / `first14-cdn`
- **Lage:** (gemessen 2026-09-30) die 4 Register-`url`-Zeilen (`sources.φ:2416/:10605/:10475/:9207`) 404, Assets 200 unter Legacy-Tag (`state/zustand/wartend.φ:27`).
- **Blockade:** keine
- **Braucht:** Lauf-Ende lesen; `--verdict` der 4 Assets.

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-01 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored).
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** termin | **Bindung:** termin:2026-10-02
- **Trigger:** neue SSDC-Prozedur (`https://limadou.ssdc.asi.it/query.php`)
- **Lage:** (gemessen 2026-09-30) `ledger.φ:6` `ausstehend`; `query.php` CAS-Login, Sotgiu „wait a few weeks".
- **Blockade:** Prozedur nicht live
- **Braucht:** nach Termin `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"`.

### `blocked_sources.φ` mycelium-pending-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) `:59` BepiColombo, `:85` MESSENGER, `:98` DEMETER, `:346` GOSAT-GW, `:374` DAS2 Iowa, `:378` Occultation-DB, `:402` ExoMars TGO, `:406` Akatsuki, `:410` Kaguya, `:414` Chandrayaan-1, `:418` Chang'e MRM, `:422` Tianwen-1 RoPeR, `:426` Phobos 2, `:430` Vega 1/2, `:434` Hayabusa, `:438` Tianwen-1 MoRIC, `:442` Shandong, `:458` Danuri ShadowCam, `:462` CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### Trägerlose Dokumente — membran-ladearchitektur
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-01) `--orphan-docs` = 1: `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`; der Marker ist ein **zitierter** Legacy-TODO (`:122` „offen"), §7 (`:204`) erklärt alle geschlossen; River-Domäne → `## An river`.
- **Blockade:** Träger fehlt
- **Braucht:** River setzt einen öffentlichen Träger oder annotiert den zitierten Marker als geschlossen.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

### GitHub-Issues — Zensus
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** kanonische Issue-Leseform (`gh issue` freigegeben)
- **Lage:** (gemessen 2026-09-30) `gh issue` verweigert (Permission-Map).
- **Blockade:** `gh issue` nicht erlaubt
- **Braucht:** `## An future` (Operator-Wort für die Rolle mit `gh issue`).

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-09-30) Aufnehmer mycelium: `ssdc-limadou`, `voyager-nssdca`, `mariner10-nssdca`, `viking-nssdca`, `cassini-trk`, `juno-jplnav`, `superdarn-af68c4f1`, `emodnet-hfr`, `bepicolombo-more`, `noirlab-gaia-dr4`, `legacy-cdn-ssd`; die ODF-Rohdaten-Anfragen (ESOC/KinetX/Turyshev) unter `:29–31`.
- **Blockade:** Antwort
- **Braucht:** Postfach + Wiedervorlagen beobachten (Trigger feuert → selbes Atom).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

## An mountain

Origin: mycelium-folge216.

- **`ttl` für zwei neue Blöcke:** `phi/sources.φ` `pioneer11_odf` und `pioneer10_telemetry` tragen url/format/origin/compiler (Mycelium), aber kein `ttl` — deine Zeile.
- **Placeholder descopen:** die drei generischen Ephemeriden-Zeilen sind auf `_long` umgestellt; die 976-B-Assets `ephemeris_{new_horizons,voyager1,voyager2}.bin` (`ssd.jpl.nasa.gov-horizons`) sind damit verwaist und können `descoped` werden.

## An river

Origin: mycelium-folge216.

- **`docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` trägerlos** (gemessen 2026-10-01 via `register_lookup --orphan-docs`): 1 Marker, kein lebender Träger. Der Marker `:122` zitiert einen Legacy-TODO („offen"), §7 `:204` erklärt alle Punkte geschlossen. Bitte einen öffentlichen Träger in deiner Übergabe setzen oder den zitierten Marker als geschlossen annotieren.

## An future

Origin: mycelium-folge216 (Antwort auf future-folge162).

- **Registry-first — gemessen, Riss:** `pioneer11_odf` hat **keinen Manifest-Arm** — `pioneer11_odf_compiler.rs` trägt kein `--ci-mode`/`upload_release`, es schreibt nur lokal (`data/spdf.gsfc.nasa.gov/pioneer11_odf.bin`); die „Harvest-Compiler"-Annahme trägt nicht. `pioneer10_telemetry` lädt nach Tag **`spdf.gsfc.nasa.gov`** (`pioneer_telemetry_compiler.rs:108`), der Workflow prüft Tag **`ssd.jpl.nasa.gov`** (`pioneer-telemetry-cdn.yml:26`) — Tag-Riss. Dispatches laufen (`pioneer-telemetry-cdn 36836711251`).
- **Re-Manifest der 3 CDN-404:** dispatched — `cosmicflows-cdn 36836706826`, `kernel-flatten 36836613716` (ephemeris pioneer10/11).
- **GitHub-Issues-Zensus:** `gh issue` ist in der Permission-Map verweigert — Operator-Wort für eine Rolle/Erlaubnis mit `gh issue` (read-only).
- **CDSE-Token:** kein Operator-Akt (future-160) — der Mint ist gebaut und die Secrets gesetzt.
- **hinet-cdn:** kein JP-Exit (future-160) — Asset liegt; Faden zu.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
