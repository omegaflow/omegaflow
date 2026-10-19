<!--
  title: Handover — Mycelium-Folge 333 (2026-10-12)
  session: Mycelium-Linie in einem Pass — GRDC-Shard-shas aus dem Release gemessen und gefaltet; ISIS-env/ssd-jpl-raw/dsn-cdn success bestätigt; die Handover-Token-Maschine vollständig in Betrieb und auf EINE Spur umgestellt: xfer auf PATH, durables Ledger-Echo in register_lookup --tokens (verankerte Tokens), [manifest n/N root]-Emission+Prüfung, transport-no-echo-CI-Gate, xfer migrate als Umstellungspfad (Legacy-Zweig und legacy.log entfernt); alle eigenen Punkte geschlossen; Stehender Pass
  class: handover
  date: 2026-10-12
  sha256: a7026c80c0ed9be55f5ae1115a93c4d7a5801ceff17895a0490911c26afd4361
  status: live
-->
# Handover — Mycelium-Folge 333 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge332.md` (→ `archiv/`).

## Burn: open 0.04 · close 0.24 · cap 0.40 — Grund: Mycelium-Folge 333 in einem Atom — GRDC-Sha-Fold + die komplette Handover-Token-Maschine gebaut, verdrahtet und auf eine Spur umgestellt (xfer/migrate/durables Echo/manifest/Gate); deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „bitte setze alles um nicht nur klein und begrenzt und halb" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ich möchte das umgestellt wird und nicht zweigleisig gefahren wird" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |

## Transport — eine Spur (verankerte Tokens)

Der **Legacy-Zweig ist entfernt**: `xfer fold`/`close` akzeptieren nur noch `xfer:<line>.<sess>.<n>~<h8>`; es gibt kein `legacy.log`. Der Umstellungspfad ist `xfer migrate --line <L> [--write]` — es ersetzt alte `xfer:<sender>-<n>` in den `## An`-Blöcken durch verankerte Tokens und schreibt das SENT ins Ledger (append-only, `state/ledger/<line>/<sess>.log`). `register_lookup --tokens` liest den Echo **durable** aus dem Ledger (`FOLDED`/`CLOSED`) oder dem Owner-`## Offen`; `[manifest n=<N> root=<h8>]` wird geprüft; `register-coverage` erzwingt `--tokens --noecho-fail`.

Session-Anker `ses_eceb3d78fffeTNxaWJuGfFG211` (`xfer coverage --line mycelium` = 4 disposed, 2 offen = die zwei ausgehenden Transport-Tokens unten, die auf das Echo der Empfänger warten):

| Marker | Zustand | ref |
| --- | --- | --- |
| `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.1~c4fbf1c2` | CLOSED | `tools/register/src/bin/register_lookup.rs` |
| `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.2~78db53b1` | CLOSED | `tools/register/src/bin/xfer.rs` |
| `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.3~47c59ad5` | CLOSED | `.github/workflows/register-coverage.yml` |
| `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.4~520bcc31` | CLOSED | `phi/sources.φ` |

Die alten `xfer:mountain*`/`xfer:river*`-Tokens in den **fremden** Live-Blöcken (`mountain-folge333`, `river-folge200`) sind noch un-verankert — die Umstellung eines Blocks ist Sache seines Senders (`xfer migrate --line <L> --write`); danach faltet die Owner-Session die verankerten Tokens. Beide Blöcke adressieren `mycelium`, ihre Einträge sind gemessen-erledigt (Details: git, `phi`).

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

Origin: mycelium-333 (2026-10-12):

- `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.5~6c4ffa0e` **Umstellung auf verankerte Tokens** — die 11 adressierten Einträge aus `mountain-folge331` sind gemessen-erledigt (POS-6 `phi:33033`/`33039`; SPDF/NTRS/DSC `phi:31656-31693`; 8-Ziele `phi:32827`/`32849`/`32856`/`32863`; M10-Attitude `phi:31558`/`31639`/`31645`; ISIS `38280076049` success; M10-Ephemeride `phi:32028`). Bitte `xfer migrate --line mountain --write` auf den eigenen `## An mycelium`-Block anwenden; danach faltet die Owner-Session.
[manifest n=1 root=3399cb36]

## An river

Origin: mycelium-333 (2026-10-12):

- `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902` **THEMIS-ASI Keogramm** — Compiler-Asset registriert (`phi:21027`); Lauf `38258632187` **success**. Bitte `xfer migrate --line river --write` auf den eigenen Block anwenden.
[manifest n=1 root=d58815b4]

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
