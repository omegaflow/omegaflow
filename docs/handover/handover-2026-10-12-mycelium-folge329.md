<!--
  title: Handover — Mycelium-Folge 329 (2026-10-12)
  session: Mycelium-Linie in einem Pass — die drei gefeuerten CDN-Läufe gemessen und gefixt (m10-jpl-edr URL-Join-Bug, pvo-orse INDEX-Parse an UNK-START_TIME, ssd-jpl 1000-Asset-Cap → Familien-Tag), adressierte mountain329-Entwürfe gefaltet, Stehender Pass
  class: handover
  date: 2026-10-12
  sha256: 71ec7b26db583b1b3308177a411c1112399b6d4a91ca0f62a885ddcdf8e4dd0c
  status: live
-->
# Handover — Mycelium-Folge 329 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am neuen HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge328.md` (→ `archiv/`) und faltete den
`--addressed mycelium`-Block aus `mountain-folge329` (`xfer:mountain329-1/-2/-4`).

## Burn: open 0.00 · close 0.13 · cap 0.40 — Grund: Mycelium-Folge 329 in einem Atom — drei gefeuerte CDN-Läufe gemessen + gefixt (m10-jpl-edr `enumerate` fehlender `/`; pvo-orse `index_rows` voidete an `START_TIME=UNK`; ssd-jpl-Release >1000-Asset-Cap → Familien-Tag `ssd.jpl.nasa.gov-planets` in Workflow + 48 phi-`url`-Zeilen), adressierte mountain329-Entwürfe gefaltet, Stehender Pass; deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt … entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auch das ist wieder problematisch mountain arbeitet an den ephemeriden er kann sich nicht darum kümmern" | 2026-10-12 | Operator (Session 325) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ist das wirklich alles Trigger oder was kannst du davon selbst bis zur Kante abarbeiten?" | 2026-10-12 | Operator (Session 326) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „mach" (8-Ziele-Rest + Debugger-Rechte am live Chrome) | 2026-10-12 | Operator (Session 329) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „das sind einzelne zeilen mach einfach" (Terms der Alt-Blöcke JPL-SSD/PDS auf `unknown`) | 2026-10-12 | Operator (Session 329) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-12 | Operator (Start Mycelium-329) · session-weiter Delegations-Consent, nicht das Commit-Wort |

## Gefaltet (gemessen 2026-10-12)

**`mountain-folge329` (`--addressed mycelium`):**
- `xfer:mountain329-1` M10-Ephemeriden-CD: der Compiler `ephemeris_mariner10_compiler.rs` ist geheilt (B1950-Summe einmal rotiert; `cargo check` 0/0 committet). Ein `--ci-mode`-Lauf regeneriert `ephemeris_mariner10.bin` (458 Granulen, 205 384 B; lokale sha `67949cb6718be222c62c47286f16edbfbab5e35975fbcf0dfb9257645bbe0ae8`) und setzt die `sha256`-Zeile `phi:31924` — das alte CDN-Asset ist im Sun-Fenster bis 941 000 km falsch. **Kein Workflow existiert** → offener Punkt unten.
- `xfer:mountain329-2` PDS POS-6 M10-Flyby-Traj: `.../M10-H-POS-6-M1-FLYBY-TRAJ-V1.0/DATA/POS_M1_ENC_42SEC.DAT` (HTTP 200, 475 152 B) + `.LBL` (38 400 B), DOI `10.17189/1519736`; zweite unabhängige M1-Ephemeriden-Quelle. `url`/`origin`-Zeile fehlt (Manifestation) → offener Punkt unten.
- `xfer:mountain329-4` ISIS-Kette für das M10-Kontrollnetz (B2/Phase 4): die S&R2026-Methode (SPICE-Pose + MESSENGER-DEM → ISIS `autoseed`/`pointreg` → Bundle Adjustment, Gate σ0 ≈ 0,570 px) ist der Türsteher, nicht der POS-6-Frame. Braucht ISIS als CI-Container (conda/apt), M10-CLR-Bilder (ASU, PD) als CDN-Assets, MESSENGER-Global-DEM v2 + MDIS-Basemaps als `url`-Zeilen → offener Punkt unten.
- Echo `xfer:mountain328-1/-2` (MVM-Merkur-Bilder + MDIS-Shape): bereits registriert + dispatcht (folge328); Mountain hat Terms `PD` gesetzt (`2b1b02d4c`).

## Offen — eigen

### GRDC-Caravan — Shard-Fix gebaut (>2 GiB-Upload-Limit); Verifikationslauf ausstehend
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `grdc-caravan-cdn`-Lauf (push-getriggert) endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38234294097`) Lauf `38234294097` **failure** — der Resume-Fix hielt (95 034 191 Records; bin + sha `f3d0600d…`), aber der Upload scheiterte an `HTTP 422 … size must be less than 2147483648`. **Gefixt** nach dem `voyager_odr`-Shard-Muster: `grdc_caravan_compiler.rs` shardet via `PODF_SHARD_BUDGET`/32 B = 1 GiB ⇒ 3 Shards; `grdc-caravan-cdn.yml`-Idempotenz `grep -q "^grdc_caravan_s"`; Register `phi:21633-21652` (ohne sha — die druckt erst der Lauf).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <neuer grdc-caravan-run-id>`; danach die gedruckten Register-Zeilen (sha je Shard) in `phi` falten.

### CI-Messzeilen — Fold rot (`Author identity unknown`) gefixt; `field-te-query` success
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-measurements-fold`-Lauf, gefeuert durch `field-te-query` `38248283974` (success, 13:32)
- **Lage:** (gemessen 2026-10-12 via `ci_manage view`) der Fold-Fix (`git config user.name/email`) steht; `field-te-query` `38248283974` ist **success** → der Fold-Trigger ist gefeuert.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <fold-run-id>` + `sread tools/measure/data/ci_measurements.φ`.

### 8-Ziele-Rest — PDS-PPI + JPL gebaut; beide Läufe gemessen+gefixt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Neuläufe von `pvo-orse-odr-cdn` und `ssd-jpl-raw-cdn` enden
- **Lage:** (gemessen 2026-10-12 via `ci_manage log`) beide Läufe **failure**: (1) `ssd-jpl-raw-cdn` `38263196991` — `release ssd.jpl.nasa.gov is capped (1000 assets)` → 15 Jobs `returned void`; (2) `pvo-orse-odr-cdn` `38263550677` — `INDEX.TAB: index parse void — the series stays unwritten (0 honored)`. **Beide gefixt (dieses Atom):** ssd-jpl auf Familien-Tag `ssd.jpl.nasa.gov-planets` (Workflow-Matrix + 48 phi-`url`-Zeilen; `viking_lander_tracking.bin` + 5 Alt-Assets bleiben am gecappten Tag), pvo-orse `index_rows` voidet nicht mehr an `START_TIME=UNK` (fällt auf `STOP_TIME` zurück, sonst Zeile übersprungen + gezählt). Neuläufe dispatcht.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <pvo-orse-neu>` / `<ssd-jpl-neu>`; die gedruckten `sha256`-Zeilen je Shard/Asset falten.

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12) `docs/concepts/tools-map.md` ist die Werkzeug-Karte (Mycelium-owned); der einzige Operator-Punkt (Debugger-Rechte am live Chrome) ist geschlossen (`tools-map.md:355`). Prosa-Marker bleiben → Träger-Zeile für `--orphan-docs`.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

### M10-Attitude-Solve — Quellen ins CDN (`xfer:mountain328-1/-2`, Harvest = Mycelium)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `m10-jpl-edr-cdn`-Neulauf und der `m10-mdis-raw-cdn`-Lauf enden
- **Lage:** (gemessen 2026-10-12) `xfer:mountain328-1` M10-ISS-EDR (2758 Dateien à 702 768 B ≈ 1,94 GB) registriert (`phi:31558` `at mercury`); der Lauf `38269896436` **failure** — der `enumerate`-Pfad baute die URL ohne Trenner (`pv0.xmvm_0007/…` → `curl (22) 404`). **Gefixt:** `format!("{base}/{volume}/…")`. `xfer:mountain328-2` MDIS-Shape (Basemap 166 m `d4c0880c…`, DEM 665 m v2 `defce776…`) registriert (`phi:31623-31631`), `m10-mdis-raw-cdn` `38275761488` **pending** (Runner-Queue).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <m10-jpl-edr-neu>` / `38275761488`; die shas/Shard-Zeilen falten. Die Attitude-Solve selbst ist Mountains Hand.

### M10-Ephemeriden-CDN-Regeneration (`xfer:mountain329-1`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** — (ein `--ci-mode`-Lauf muss erst gebaut werden)
- **Lage:** (gemessen 2026-10-12, gefaltet aus `mountain-folge329`) der Compiler ist geheilt; **kein Workflow** ruft `ephemeris_mariner10_compiler --ci-mode` (nur der Bin + `m10-jpl-edr-cdn.yml` existieren). Das alte CDN-Asset ist im Sun-Fenster bis 941 000 km falsch.
- **Blockade:** keine.
- **Braucht:** einen Workflow `ephemeris-mariner10-cdn.yml` (Muster `m10-jpl-edr-cdn.yml`) anlegen und dispatchten; danach `sha256` `phi:31924` auf die gedruckte sha setzen.

### PDS POS-6 M10-Flyby-Traj — `url`/`origin`-Zeile (`xfer:mountain329-2`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-12, gefaltet) `https://pds-ppi.igpp.ucla.edu/data/M10-H-POS-6-M1-FLYBY-TRAJ-V1.0/DATA/POS_M1_ENC_42SEC.DAT` (HTTP 200, 475 152 B) + `.LBL` (38 400 B), DOI `10.17189/1519736`; zweite unabhängige M1-Ephemeriden-Quelle (`m10_pos6_check.rs`). `M10-H-POS-6-M3-…` ist die M3-Schwester.
- **Blockade:** keine.
- **Braucht:** `raw_mirror`-Registrierung (sha messen) + `url`/`origin`/`compiler`-Zeile in `phi/sources.φ`; `terms`/`format` = Mountain.

### ISIS-Kette für das M10-Kontrollnetz — CI-Container + CDN-Assets (`xfer:mountain329-4`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-12, gefaltet) die S&R2026-Methode braucht ISIS als CI-Container, M10-CLR-Bilder (ASU `ser.im-ldi.com`, PD) als CDN-Assets, MESSENGER-Global-DEM v2 + MDIS-Basemaps als `url`-Zeilen. Kein ISIS-Workflow im Baum.
- **Blockade:** keine.
- **Braucht:** einen CI-Container-Workflow für ISIS (conda/apt) bauen; M10-CLR über `raw_mirror`/eigenen Compiler registrieren. `terms`/`format` je Zeile = Mountain.

### Orphan-Doc `survey-2026-10-10-worldwide-lidar-quellen.md` — Träger fehlt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12 via `register_lookup --orphan-docs`) 1 Dokument mit 52 offenen Markern ohne Live-Handover-Träger; Mountain-Nähe (Lidar-Quellen).
- **Blockade:** keine.
- **Braucht:** Träger-Zeile in der Owner-Übergabe (Mountain) oder gemessenes `descoped`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
