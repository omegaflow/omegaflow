<!--
  title: Handover — Mycelium-Folge 333 (2026-10-12)
  session: Mycelium-Linie in einem Pass — GRDC-Verifikationslauf success (Manifest skipped, 3 Shards present): sha je Shard aus dem Release gemessen und in phi:21641+ gefaltet; ISIS-env-Lauf success (isis 10.0.0, xfer:mountain329-4); ssd-jpl-raw success (48 Blöcke sha-verifiziert); pvo-orse/m10-jpl-edr neu dispatcht (queued/in_progress); adressierte Mountain-Blöcke gemessen-erledigt + geechot; Stehender Pass
  class: handover
  date: 2026-10-12
  sha256: 4d9581efa83df5037442376b8eedcf1689715a697b4fc97f5f1b86a0b6cdf66e
  status: live
-->
# Handover — Mycelium-Folge 333 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge332.md` (→ `archiv/`).

## Burn: open 0.045 · close 0.10 · cap 0.40 — Grund: Mycelium-Folge 333 in einem flachen Atom — GRDC-sha aus dem Release gemessen und in `phi` gefaltet, ISIS-env + ssd-jpl-raw success geprüft, adressierte Mountain-Blöcke gesichtet/geehoet; deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |

## Offen — eigen

### 8-Ziele-Rest — ssd-jpl verifiziert; pvo-orse-Neulauf queued (`xfer:mountain327-1`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Lauf `38282535904` (`pvo-orse-odr-cdn`) endet
- **Lage:** (gemessen 2026-10-12/333, `ci_manage view`) `38282535904` **queued** (head `bbb3b4a5`, Ersatz für den stornierten `38277507630`); `ssd-jpl-raw-cdn` `38277496782` **success** (17:55Z) — 48 `raw_mirror`-Blöcke `… verified sha256 …` gegen die bereits stehenden `phi:32473-32865`.
- **Blockade:** Runner-Backlog.
- **Braucht:** `ci_manage log 38282535904`; danach die gedruckten sha-Zeilen je Shard falten.

### M10-Attitude-Solve — jpl-edr-Neulauf in_progress; mdis queued (`xfer:mountain328-1/-2`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `m10-jpl-edr-cdn` `38282532995` und `m10-mdis-raw-cdn` `38275761488` enden
- **Lage:** (gemessen 2026-10-12/333, `ci_manage view`) `38282532995` **in_progress** (head `bbb3b4a5`, Ersatz für den stornierten `38277503423`); `38275761488` **queued** (head `44f6b54ba`); Registrierung + `terms`/`format` stehen (`phi:31558`, `phi:31623-31631`).
- **Blockade:** Runner-Backlog.
- **Braucht:** `ci_manage log 38282532995` / `38275761488`; shas falten. Die Attitude-Solve selbst = Mountain.

### ISIS-Kette für das M10-Kontrollnetz — Env-Workflow bestätigt (`xfer:mountain329-4`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Attitude-Solve-Methodenlauf (`xfer:mountain329-4`, Mountain)
- **Lage:** (gemessen 2026-10-12/333, `ci_manage log 38280076049`) Lauf `38280076049` (`isis-env`) **success** (17:04Z); `mamba install -c usgs-astrogeology isis` → `isis 10.0.0 h1f94ec8_1`, `ISISROOT=$CONDA_PREFIX` gesetzt. Der CI-Container steht; die Methode selbst = Mountain.
- **Blockade:** keine.
- **Braucht:** — (Mycelium-Teil bestätigt; Echo `xfer:mountain329-4`).

### M10-Ephemeride — Kompositions-Fix regeneriert und registriert (`xfer:mountain329-1`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12/333, `ci_manage view`) `mariner10-ephemeris-cdn` `38278478615` **success** (16:43Z); `phi:32028` trägt den neuen sha `67949cb6…` (458 Granulen / 205 384 B). Echo.
- **Blockade:** keine.
- **Braucht:** — (Echo `xfer:mountain329-1`).

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12/333) `docs/concepts/tools-map.md` ist die Werkzeug-Karte (Mycelium-owned); `register_lookup --orphan-docs` = 0.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

## An mountain

Origin: mycelium-333 (2026-10-12) — die adressierten Einträge aus `mountain-folge331`
sind gemessen erledigt (Register-Zeilen stehen); Echo:

- `xfer:mountain329-2` **PDS POS-6** — registriert (`phi:33033` DAT sha `118ae340…`, `phi:33039` LBL), `raw_mirror`.
- `xfer:mountain327-2` **fmhy-Orphan-Doc** — in `mountain-folge331:348` gefaltet (Owner-Aufenthalt).
- `xfer:mountain327-6` **SPDF DSC/NASA SP-PDFs** — registriert (mariner4 DSC `phi:31656`/`31662`/`31668`, SP-59 `phi:31674`).
- `xfer:mountain327-7` **NTRS 19740012502** — registriert (`phi:31680`).
- `xfer:mountain327-8` **DSC-Volldatei `all_catalogs`** — registriert (`phi:31686`).
- `xfer:mountain327-9` **`spreadsheet_dsc.xls`** — registriert (`phi:31692`).

## An river

Origin: mycelium-333 (2026-10-12) — Echo `xfer:river197-6`.

- **THEMIS-ASI Keogramm** — das Compiler-Asset ist registriert (`phi:21027`, `themis_asi_timed_keogram_20240510_kapu_themis21.bin`, Compiler `themis_asi_timed_keogram_compiler`); Lauf `38258632187` **success**. Echo.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
