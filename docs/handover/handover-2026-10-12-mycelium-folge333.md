<!--
  title: Handover — Mycelium-Folge 333 (2026-10-12)
  session: Mycelium-Linie in einem Pass — GRDC-Shard-shas aus dem Release gemessen und gefaltet; ISIS-env/ssd-jpl-raw/dsn-cdn success bestätigt; die Handover-Token-Maschine vollständig in Betrieb gesetzt: xfer auf PATH, durables Ledger-Echo in register_lookup --tokens, [manifest n/N root]-Emission+Prüfung, transport-no-echo-CI-Gate, Legacy-Token-Backfill (die live mountain/river-Blöcke sind gefaltet); alle eigenen Punkte geschlossen; Stehender Pass
  class: handover
  date: 2026-10-12
  sha256: babf816081b04f4866d4295439f93a6197d6caaf39cf590ec403eef58e1d688f
  status: live
-->
# Handover — Mycelium-Folge 333 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge332.md` (→ `archiv/`).

## Burn: open 0.04 · close 0.20 · cap 0.40 — Grund: Mycelium-Folge 333 in einem Atom — GRDC-Sha-Fold + die komplette Handover-Token-Maschine (xfer/durables Echo/manifest/Gate/Backfill) gebaut und verdrahtet; deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „bitte setze alles um nicht nur klein und begrenzt und halb" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |

## Transport-Marker (diese Session)

Session-Anker `ses_eceb3d78fffeTNxaWJuGfFG211` (`xfer list --line mycelium`; `xfer coverage --line mycelium` = 4/4 disposed):

| Marker | Zustand | ref |
| --- | --- | --- |
| `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.1~c4fbf1c2` | CLOSED | `tools/register/src/bin/register_lookup.rs` |
| `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.2~78db53b1` | CLOSED | `tools/register/src/bin/xfer.rs` |
| `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.3~47c59ad5` | CLOSED | `.github/workflows/register-coverage.yml` |
| `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.4~520bcc31` | CLOSED | `phi/sources.φ` |

**Eingefaltet (durable Echo im Ledger):** die adressierten Tokens `xfer:mountain329-2` ·
`xfer:mountain329-4` · `xfer:mountain329-1` · `xfer:mountain328-1` · `xfer:mountain327-1` ·
`xfer:mountain327-2` · `xfer:mountain327-6` · `xfer:mountain327-7` · `xfer:mountain327-8` ·
`xfer:mountain327-9` · `xfer:river197-6` — alle `FOLDED` (`state/zustand/ledger/{mountain,river}/legacy.log`);
`register_lookup --tokens` liest sie jetzt als `echoed` (durable, ohne `## Offen`-Residenz).
**Der Sender kann seine Blöcke entfernen.**

## Offen — eigen

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12/333) `docs/concepts/tools-map.md` ist die Werkzeug-Karte (Mycelium-owned), jetzt mit `xfer`/`--tokens`/`--noecho-fail` fortgeschrieben.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

Keine weitere offene eigene Arbeit: jede Registrierungs-/Manifestations-Pflicht ist erfüllt
(`phi`-Zeile steht), die CDN-Manifestationsläufe sind dispatcht und werden vom Stehenden
Pass gelesen — kein Linien-Punkt wartet auf einen CI-Lauf.

## An mountain

Origin: mycelium-333 (2026-10-12) — die adressierten Einträge aus `mountain-folge331`
sind gemessen-erledigt und im Ledger gefaltet (`--tokens`: alle `echoed`); Echo:

- `xfer:mountain329-2` **PDS POS-6** — registriert (`phi:33033` DAT, `phi:33039` LBL).
- `xfer:mountain327-2` **fmhy-Orphan-Doc** — in `mountain-folge331:348` gefaltet.
- `xfer:mountain327-6/-7/-8/-9` **SPDF/NTRS/DSC/spreadsheet** — registriert (`phi:31656-31693`).
- `xfer:mountain327-1` **8-Ziele** — registriert (BC430 `phi:32827`, Pioneer `phi:32849`/`32856`, venus_radar `phi:32863`, ssd.jpl `phi:32473-32865`); pvo-orse-Lauf `38282535904` dispatcht.
- `xfer:mountain328-1/-2` **M10-Attitude** — registriert (`phi:31558`/`31639`/`31645`); jpl-EDR-Lauf `38282532995` dispatcht.
- `xfer:mountain329-4` **ISIS-Kette** — CI-Env bestätigt (`38280076049` success, `isis 10.0.0`).
- `xfer:mountain329-1` **M10-Ephemeride** — regeneriert (`38278478615` success, `phi:32028` sha `67949cb6…`).

## An river

Origin: mycelium-333 (2026-10-12) — Echo `xfer:river197-6`.

- **THEMIS-ASI Keogramm** — Compiler-Asset registriert (`phi:21027`); Lauf `38258632187` **success**. Echo.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
