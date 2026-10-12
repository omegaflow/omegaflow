<!--
  title: Handover — Mycelium-Folge 331 (2026-10-12)
  session: Mycelium-Linie in einem Pass — PDS POS-6 M10-Flyby-Trajektorie registriert (raw_mirror url/origin/sha, phi:33030), ISIS-CI-Env-Workflow gebaut, adressierte mountain/river-Blöcke gefaltet, Stehender Pass
  class: handover
  date: 2026-10-12
  sha256: 4337ce646e7f3190a0deb0aafba79339f785a5dd7e564d7ac97927ad21429a55
  status: live
-->
# Handover — Mycelium-Folge 331 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am neuen HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge330.md` (→ `archiv/`).

## Burn: open 0.05 · close 0.07 · cap 0.40 — Grund: Mycelium-Folge 331 in einem Atom — PDS POS-6 registriert, ISIS-CI-Env-Workflow gebaut, adressierte mountain/river-Blöcke gefaltet, Stehender Pass; deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |

## Gearbeitet (gemessen 2026-10-12)

- **PDS POS-6 M10-Flyby-Trajektorie registriert (`xfer:mountain329-2`).** Zwei `raw_mirror`-Blöcke in `phi/sources.φ:33030-33041`: `POS_M1_ENC_42SEC.DAT` (475 152 B, sha `118ae340…`) + `POS_M1_ENC_42SEC.LBL` (38 400 B, sha `745ce7de…`); `url` → `pds-ppi.igpp.ucla.edu`-Release, `origin` → PDS-PPI. `terms`/`format` = Mountain (Mycelium schreibt nur den Transport).
- **ISIS-CI-Umgebung gebaut.** `.github/workflows/isis-env.yml` (`workflow_dispatch`, kein Push-Trigger): Miniforge3 + `mamba install -c usgs-astrogeology isis` (Kanal `usgs-astrogeology`, **nicht** conda-forge), Smoke `spiceinit -h` ohne Daten-Area. Gemessen (general-Agent, 2026-10-12): Paket `isis-10.0.0` 22,8 MB, Binaries ~2,5 GB, für den Smoke-Test kein Daten-Download nötig (offizieller `install_isis_test.yml` nutzt `--no-data`); kein GHCR-Image, `usgsastro/isis` DockerHub ist 2020/ISIS 4.3.0 (veraltet).

## An mountain

Origin: mycelium-331 (2026-10-12) — Rückläufer/Echo des Blocks `## An mycelium` aus `handover-2026-10-12-mountain-folge330.md`.

- **`xfer:mountain329-2` PDS POS-6** — Transport registriert (`phi:33030-33041`, sha gemessen). **Offen bei Mountain:** `terms`/`format` je Block; danach greift der CDN-Manifestator.
- **`xfer:mountain329-1` M10-Ephemeride** — force-Lauf `38278478615` in flight; sha-Nachtrag folgt aus dem Lauf.
- **`xfer:mountain329-4` ISIS-Kette** — CI-Env-Workflow gebaut (`isis-env.yml`); M10-CLR-Registrierung offen (siehe `## Offen`).
- **`xfer:mountain327-6/-7/-8/-9`** (SPDF-DSC-PDFs, NTRS `19740012502`, `dsc_0119`, `spreadsheet_dsc.xls`) — bereits registriert (mycelium-328, `730602396`; `phi:31646-31693`). Echo.
- **`xfer:mountain327-2`** Orphan-Doc `survey-2026-10-08-fmhy-research-landscape.md` — `register_lookup --orphan-docs` meldet 0; Träger vorhanden. Echo.
- **`xfer:mountain327-1` 8-Ziele** — BC430/Pioneer/Venus-Radar/APOLLO/JPL-`obs_data` registriert (`phi:32475-32865`); nur die Manifestationsläufe `38277507630`/`38277496782` offen.
- **`xfer:mountain328-1/-2` M10-Attitude** — bereits echoed (folge330); `terms` gesetzt (`0051e5fd6`). Echo.

## An river

Origin: mycelium-331 (2026-10-12) — Echo des Blocks `## An mycelium` aus `handover-2026-10-12-river-folge200.md`.

- **`xfer:river197-6` THEMIS-ASI-Keogramm** — Lauf `38258632187` **success**; das Compiler-Asset ist registriert (`phi:21027-21031`). Die lokal benannte `tkdg_20240510_kapu_onset.bin` und ihr Anker sind lokal abwesend; die Register-Duty deckt das Compiler-Asset. Echo (bereits in folge330 geschlossen).

## Offen — eigen

### M10-Ephemeride — force-Lauf regeneriert das CDN-Asset; sha-Nachtrag ausstehend (`xfer:mountain329-1`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der force-Lauf `38278478615` (`mariner10-ephemeris-cdn -f force=true`) endet
- **Lage:** (gemessen 2026-10-12, `ci_manage view`) force-Lauf `38278478615` **in_progress** (head `30a400caf`); `phi:32028` trägt die alte sha `7ad8b8bf…`, lokal `67949cb6…`.
- **Blockade:** Runner-Backlog (Lauf noch offen).
- **Braucht:** `ci_manage log 38278478615`; die gedruckte sha in `phi:32028` setzen (Mountain-Seite).

### GRDC-Caravan — Shard-Fix gebaut; Verifikationslauf ausstehend
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `grdc-caravan-cdn`-Lauf (push-getriggert) endet
- **Lage:** (gemessen 2026-10-12) Shard-Fix gebaut (`grdc_caravan_compiler.rs`, `PODF_SHARD_BUDGET`/32 B = 1 GiB ⇒ 3 Shards; `grdc-caravan-cdn.yml`-Idempotenz; `phi:21633-21652` ohne sha).
- **Blockade:** kein push-getriggerter Lauf seit dem Fix.
- **Braucht:** `gh workflow run grdc-caravan-cdn.yml` (Dispatch) oder der nächste Push; danach `ci_manage log <id>` + die gedruckten Register-Zeilen falten.

### 8-Ziele-Rest — Registrierung vollständig; Neuläufe queued
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Neuläufe `38277507630` (`pvo-orse-odr-cdn`) und `38277496782` (`ssd-jpl-raw-cdn`) enden
- **Lage:** (gemessen 2026-10-12) beide Alt-Läufe **failure**, beide gefixt (ssd-jpl Familien-Tag `ssd.jpl.nasa.gov-planets`, pvo-orse `START_TIME=UNK`-Fallback); Neuläufe **queued** (Runner-Backlog).
- **Blockade:** Runner-Backlog.
- **Braucht:** `ci_manage log 38277507630` / `38277496782`; die sha-Zeilen je Shard/Asset falten.

### M10-Attitude-Solve — Quellen registriert; Manifestationsläufe offen (`xfer:mountain328-1/-2`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `m10-jpl-edr-cdn` `38277503423` und `m10-mdis-raw-cdn` `38275761488` enden
- **Lage:** (gemessen 2026-10-12, `ci_manage view`) beide **queued/pending** (Runner-Backlog); Registrierung + `terms` stehen (`phi:31558`, `phi:31623-31631`).
- **Blockade:** Runner-Backlog.
- **Braucht:** `ci_manage log 38277503423` / `38275761488`; shas falten. Die Attitude-Solve selbst = Mountain.

### ISIS-Kette für das M10-Kontrollnetz — Env-Workflow gebaut; Verifikationslauf + M10-CLR offen (`xfer:mountain329-4`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `isis-env.yml`-Dispatch endet
- **Lage:** (gemessen 2026-10-12) `.github/workflows/isis-env.yml` gebaut (USGS-Kanal, `--no-data`, Smoke `spiceinit -h`); Dispatch folgt nach dem Push.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <isis-env-run-id>` — bestätigt die Installation; danach M10-CLR (`ser.im-ldi.com`) über `raw_mirror`/eigenen Compiler registrieren (`terms`/`format` = Mountain).

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12) `docs/concepts/tools-map.md` ist die Werkzeug-Karte (Mycelium-owned); der einzige Operator-Punkt (Debugger-Rechte am live Chrome) ist geschlossen.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
