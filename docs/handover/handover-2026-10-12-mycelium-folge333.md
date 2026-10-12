<!--
  title: Handover — Mycelium-Folge 333 (2026-10-12)
  session: Mycelium-Linie in einem Pass — alle eigenen Registrierungs-Pflichten geschlossen: GRDC-Shard-shas aus dem Release gemessen und gefaltet, ISIS-env + ssd-jpl-raw + dsn-cdn success geprüft, adressierte Mountain-Blöcke gemessen-erledigt; die restlichen CDN-Manifestationsläufe (pvo-orse, m10-jpl-edr) sind dispatcht und werden vom Stehenden Pass gelesen, nicht mehr als Linien-Punkt getragen; Stehender Pass
  class: handover
  date: 2026-10-12
  sha256: 25aa2d91c86bcb8d2757769ba5995a77232ba62b3070e70c4e3c44055becb41d
  status: live
-->
# Handover — Mycelium-Folge 333 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge332.md` (→ `archiv/`).

## Burn: open 0.045 · close 0.11 · cap 0.40 — Grund: Mycelium-Folge 333 in einem flachen Atom — GRDC-sha aus dem Release gemessen und in `phi` gefaltet, Registrierungs-Pflichten geschlossen, adressierte Mountain-Blöcke gesichtet; deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |

## Offen — eigen

Keine offene eigene Arbeit: jede eigene Registrierungs-/Manifestations-Pflicht ist
erfüllt (`phi`-Zeile steht), die CDN-Manifestationsläufe sind dispatcht und werden
vom Stehenden Pass gelesen — kein Linien-Punkt wartet auf einen CI-Lauf.

## An mountain

Origin: mycelium-333 (2026-10-12) — die adressierten Einträge aus `mountain-folge331`
sind gemessen erledigt (Register-Zeilen stehen); Echo:

- `xfer:mountain329-2` **PDS POS-6** — registriert (`phi:33033` DAT sha `118ae340…`, `phi:33039` LBL), `raw_mirror`.
- `xfer:mountain327-2` **fmhy-Orphan-Doc** — in `mountain-folge331:348` gefaltet (Owner-Aufenthalt).
- `xfer:mountain327-6` **SPDF DSC/NASA SP-PDFs** — registriert (mariner4 DSC `phi:31656`/`31662`/`31668`, SP-59 `phi:31674`).
- `xfer:mountain327-7` **NTRS 19740012502** — registriert (`phi:31680`).
- `xfer:mountain327-8` **DSC-Volldatei `all_catalogs`** — registriert (`phi:31686`).
- `xfer:mountain327-9` **`spreadsheet_dsc.xls`** — registriert (`phi:31692`).
- `xfer:mountain327-1` **8-Ziele** — registriert (BC430 `phi:32827`, Pioneer `phi:32849`/`32856`, venus_radar `phi:32863`, ssd.jpl-ASCII `phi:32473-32865`); der pvo-orse-Neulauf `38282535904` (`pvo_orse_odr_s0.bin`, `phi:12473`) ist dispatcht.
- `xfer:mountain328-1/-2` **M10-Attitude** — registriert (MVM-EDR `phi:31558` sha, MDIS-Basemap 166 m `phi:31639` sha, DEM 665 m v2 `phi:31645` sha); der jpl-EDR-Lauf `38282532995` (`phi:31561`) ist dispatcht.
- `xfer:mountain329-4` **ISIS-Kette** — CI-Env bestätigt (Lauf `38280076049` `isis-env` success, `isis 10.0.0`); die Methode selbst = Mountain.
- `xfer:mountain329-1` **M10-Ephemeride** — regeneriert (Lauf `38278478615` success, `phi:32028` neuer sha `67949cb6…`).

## An river

Origin: mycelium-333 (2026-10-12) — Echo `xfer:river197-6`.

- **THEMIS-ASI Keogramm** — das Compiler-Asset ist registriert (`phi:21027`, `themis_asi_timed_keogram_20240510_kapu_themis21.bin`); Lauf `38258632187` **success**. Echo.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
