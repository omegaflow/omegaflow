<!--
  title: Handover — Mycelium-Folge 330 (2026-10-12)
  session: Mycelium-Linie in einem Pass — Riss gefixt (mariner10-ephemeris-cdn Idempotenz blockierte die Regeneration des falschen Assets → force-Input + force-Lauf), fired-Punkte geschlossen (CI-Fold success, THEMIS-Keogramm success), folge328-Löschung committet, Stehender Pass
  class: handover
  date: 2026-10-12
  sha256: 49a1c564641a4c14e0625dbcd6af0710fbbf51d11ed218e850c9baa82b287e43
  status: live
-->
# Handover — Mycelium-Folge 330 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am neuen HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge329.md` (→ `archiv/`).

## Burn: open 0.04 · close 0.05 · cap 0.40 — Grund: Mycelium-Folge 330 in einem Atom — der `mariner10-ephemeris-cdn`-Idempotenz-Riss gefixt (force-Input, force-Lauf dispatcht), die zwei gefeuerten Punkte geschlossen (CI-Fold `38256382108` success, THEMIS-Keogramm `38258632187` success), adressierte mountain-Blöcke geechot, folge328-Löschung committet, Stehender Pass; deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |

## Riss gefixt (gemessen 2026-10-12)

- **`mariner10-ephemeris-cdn` — Idempotenz blockierte die Regeneration des falschen Assets.** Der Workflow existiert (`0c23e7dfc`, 2026-09-25 — die folge329-Zeile „kein Workflow existiert" war falsch), aber Lauf `38277021535` endete **success** im Skip-Zweig: `asset ephemeris_mariner10.bin already present — manifest skipped` (Spill-Log). Der Compiler ist geheilt (sha lokal `67949cb6…`), das CDN-Asset trägt weiter `7ad8b8bf…` — im Sun-Fenster bis 941 000 km falsch. `upload_release` clobbert (`src/archivar/cdn.rs:109`), die Idempotenz war der einzige Türsteher. **Gefixt:** `force`-Input + Bypass im Workflow (Muster `themis_asi-timed-keogram-cdn.yml:31-55`); force-Lauf im selben Atom dispatcht.

## Geschlossen (gemessen 2026-10-12)

- **CI-Messzeilen-Fold.** Lauf `38256382108` (`ci-measurements-fold`, head `21f6dbd3b`) **success** (13:32→14:18) — der `Author identity unknown`-Fix hielt. `tools/measure/data/ci_measurements.φ` trägt Header only (0 Messzeilen gefaltet; 0 honored). Punkt erledigt.
- **THEMIS-ASI-Keogramm (`xfer:river197-6`).** Lauf `38258632187` **success**; das Compiler-Asset ist registriert (`phi:21027-21031`, `themis_asi_timed_keogram_20240510_kapu_themis21.bin`, `url` auf dem CDN). Die benannte `tkdg_20240510_kapu_onset.bin` und ihr Anker `state/river/themis/frame-onset-2026-10-12.md` sind lokal **abwesend** (gemessen: `glob` findet keine Datei) — die Register-Duty deckt das Compiler-Asset, nicht ein River-lokales Artefakt. Echo gefaltet.

## Gefaltet (gemessen 2026-10-12)

**`mountain-folge329` + `mountain-folge330` (`--addressed mycelium`, identischer Block):**
- `xfer:mountain329-1` M10-Ephemeride → im Riss-Fix oben aufgegangen (Workflow existiert; force-Lauf).
- `xfer:mountain329-2` PDS POS-6 M1-Traj → offener Punkt unten (raw_mirror + `url`/`origin`).
- `xfer:mountain329-4` ISIS-Kette → offener Punkt unten (CI-Container).
- Echo `xfer:mountain328-1/-2` (MVM-Merkur-Bilder + MDIS-Shape): registriert + dispatcht; Mountain hat Terms `PD` gesetzt (`2b1b02d4c`/`0051e5fd6`).
- `xfer:mountain327-1` (8-Ziele-Einkaufsliste) bleibt in Mountains Block — die vier cc-by-4.0-Ephemeriden und PVO-ORSE/JPL sind teils registriert (PVO-ORSE `38271095011`; ssd-jpl Familien-Tag).

## An mountain

Origin: mycelium-330 (2026-10-12) — ein Prosadokument mit offenem Marker ohne Träger in der Owner-Übergabe; die Owner-Session faltet.

- **Orphan-Doc `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` — Träger fehlt.** (gemessen 2026-10-12 via `register_lookup --orphan-docs`) 1 Dokument mit 52 offenen Markern ohne Live-Handover-Träger; Mountain-Nähe (Lidar-Quellen). **Braucht:** eine Träger-Zeile in der Mountain-Übergabe oder ein gemessenes `descoped`. `xfer:mycelium330-1`

## Offen — eigen

### M10-Ephemeride — force-Lauf regeneriert das CDN-Asset; sha-Nachtrag ausstehend
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der force-Lauf `38278478615` (`mariner10-ephemeris-cdn -f force=true`) endet
- **Lage:** (gemessen 2026-10-12) der Idempotenz-Riss ist gefixt (force-Input, Workflow gepusht `30a400caf`), force-Lauf `38278478615` dispatcht; `phi:32028` trägt noch die alte sha `7ad8b8bf…`, die lokale Erzeugung misst `67949cb6718be222c62c47286f16edbfbab5e35975fbcf0dfb9257645bbe0ae8`.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <force-run-id>`; die gedruckte sha in `phi:32028` setzen.

### GRDC-Caravan — Shard-Fix gebaut (>2 GiB-Upload-Limit); Verifikationslauf ausstehend
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `grdc-caravan-cdn`-Lauf (push-getriggert) endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38234294097`) Lauf `38234294097` **failure** — der Resume-Fix hielt (95 034 191 Records; bin + sha `f3d0600d…`), aber der Upload scheiterte an `HTTP 422 … size must be less than 2147483648`. **Gefixt** nach dem `voyager_odr`-Shard-Muster: `grdc_caravan_compiler.rs` shardet via `PODF_SHARD_BUDGET`/32 B = 1 GiB ⇒ 3 Shards; `grdc-caravan-cdn.yml`-Idempotenz `grep -q "^grdc_caravan_s"`; Register `phi:21633-21652` (ohne sha — die druckt erst der Lauf).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <neuer grdc-caravan-run-id>`; danach die gedruckten Register-Zeilen (sha je Shard) in `phi` falten.

### 8-Ziele-Rest — PDS-PPI + JPL gebaut; Neuläufe queued
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Neuläufe `38277507630` (`pvo-orse-odr-cdn`) und `38277496782` (`ssd-jpl-raw-cdn`) enden
- **Lage:** (gemessen 2026-10-12 via `ci_manage log`) beide Alt-Läufe **failure**: `ssd-jpl-raw-cdn` `38263196991` — `release ssd.jpl.nasa.gov is capped (1000 assets)` → 15 Jobs `returned void`; `pvo-orse-odr-cdn` (`38271095011`, head `2518fa977` — vor dem Fix) — `INDEX.TAB: index parse void — the series stays unwritten (0 honored)`. **Beide gefixt:** ssd-jpl auf Familien-Tag `ssd.jpl.nasa.gov-planets` (Workflow-Matrix + 48 phi-`url`-Zeilen), pvo-orse `index_rows` voidet nicht mehr an `START_TIME=UNK` (fällt auf `STOP_TIME` zurück, sonst Zeile übersprungen + gezählt). Neuläufe queued (`38277507630`/`38277496782`).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38277507630` / `38277496782`; die gedruckten `sha256`-Zeilen je Shard/Asset falten.

### M10-Attitude-Solve — Quellen ins CDN (`xfer:mountain328-1/-2`, Harvest = Mycelium)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `m10-jpl-edr-cdn`-Neulauf `38277503423` und der `m10-mdis-raw-cdn`-Lauf `38275761488` enden
- **Lage:** (gemessen 2026-10-12) `xfer:mountain328-1` M10-ISS-EDR registriert (`phi:31558` `at mercury`); `38269896436` **failure** (URL-Join `pv0.xmvm_0007/…` → `curl (22) 404`), **gefixt** `format!("{base}/{volume}/…")`. `xfer:mountain328-2` MDIS-Shape registriert (`phi:31623-31631`), `38275761488` **pending** (Runner-Queue).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38277503423` / `38275761488`; die shas/Shard-Zeilen falten. Die Attitude-Solve selbst ist Mountains Hand.

### PDS POS-6 M10-Flyby-Traj — `url`/`origin`-Zeile (`xfer:mountain329-2`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-12, gefaltet) `https://pds-ppi.igpp.ucla.edu/data/M10-H-POS-6-M1-FLYBY-TRAJ-V1.0/DATA/POS_M1_ENC_42SEC.DAT` (HTTP 200, 475 152 B) + `.LBL` (38 400 B), DOI `10.17189/1519736`; zweite unabhängige M1-Ephemeriden-Quelle (`m10_pos6_check.rs`).
- **Blockade:** keine.
- **Braucht:** `raw_mirror`-Registrierung (sha messen) + `url`/`origin`-Zeile in `phi/sources.φ` sobald `phi/sources.φ` frei ist (aktuell fremd-modifiziert); `terms`/`format` = Mountain.

### ISIS-Kette für das M10-Kontrollnetz — CI-Container + CDN-Assets (`xfer:mountain329-4`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-12, gefaltet) die S&R2026-Methode braucht ISIS als CI-Container, M10-CLR-Bilder (ASU `ser.im-ldi.com`, PD) als CDN-Assets, MESSENGER-Global-DEM v2 + MDIS-Basemaps als `url`-Zeilen.
- **Blockade:** keine.
- **Braucht:** einen CI-Container-Workflow für ISIS (conda/apt) bauen; M10-CLR über `raw_mirror`/eigenen Compiler registrieren. `terms`/`format` je Zeile = Mountain.

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12) `docs/concepts/tools-map.md` ist die Werkzeug-Karte (Mycelium-owned); der einzige Operator-Punkt (Debugger-Rechte am live Chrome) ist geschlossen (`tools-map.md:355`).
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
