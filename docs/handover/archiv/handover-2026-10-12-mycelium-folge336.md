<!--
  title: Handover — Mycelium-Folge 336 (2026-10-12)
  session: Mycelium-Linie in einem Pass — still gemessen (kein eigener autonom arbeitbarer Punkt: tools-map-Träger ruht, Transport-Echo wartet auf Rivers Token-Echo); Register frisch (0 XFER-MISSING/0 XFER-NOECHO), River faltet die mycelium-335-Substanz ohne Token-Echo (riss), adressierte Tokens alle echoed; Stehender Pass am HEAD
  class: handover
  date: 2026-10-12
  sha256: 8dc3f23d70af43a59839add17cd85b4c07fdf20f0c6f094ba293eebc5146e9ff
  status: live
-->
# Handover — Mycelium-Folge 336 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge335.md` (→ `archiv/`).

## Burn: open 0.01 · close 0.04 · cap 0.40 — Grund: Mycelium-Folge 336 — still gemessen, kein eigener autonom arbeitbarer Punkt (tools-map-Träger ruht; Transport-Echo wartet auf Rivers Token-Echo), Register frisch (0 XFER-MISSING/0 XFER-NOECHO), Stehender Pass am HEAD; deepseek-flash, kein pro/max (`session_burn`, line-Bucket-Delta).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „bitte setze alles um nicht nur klein und begrenzt und halb" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ich möchte das umgestellt wird und nicht zweigleisig gefahren wird" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du das bitte fixen" (der getragene Riss: PATH-Frische hängt am roten `tools-build`) | 2026-10-12 | Operator (Session 334) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 336) · session-weiter Delegations-Consent; `/commit` ist das Commit-Wort, davon getrennt |

## Offen — eigen

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12/336) `docs/concepts/tools-map.md` ist die Werkzeug-Karte (Mycelium-owned); zuletzt nachgezogen in `d0c5ffab9` (archive_search-Arm-Zahl/Keyword-Count), davor folge334 auf die eine Spur (`xfer mark/list/fold/close/manifest/migrate/coverage`, kein `legacy.log`). `--orphan-docs` = 14, keine mycelium-Datei darunter.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

### Transport-Echo (eigener Ausgangs-Token → river) — Substanz gefaltet, Token-Echo fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** River trägt den Token `…6~3ed6b902` in sein live `## Offen` (Echo)
- **Lage:** (gemessen 2026-10-12/336) `register_lookup --tokens` meldet **0 XFER-MISSING, 0 XFER-NOECHO**; eigener Ausgangs-Token `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902` (→ river) **unbeglichen** (Manifest `river open n=1 echoed=0/1 root=d58815b4`). **Riss (gehalten):** River hat die Substanz gefaltet — `handover-2026-10-12-river-folge201.md:112` nennt das Compiler-Asset registriert (`phi:21027`, Lauf `38258632187` success), und `c99c1288c` committet „fold the mycelium-335 xfer resolution" — doch das Token-Echo fehlt in Rivers live `## Offen` (kein `3ed6b902` darin): Substanz gefaltet, Form nicht. `…5~6c4ffa0e` ist CLOSED (`--ref daf9a3010`); die `mountain-folge334`-Tokens (`…1~0d43166a`, `…2~1003fb06`) sind `echoed`.
- **Blockade:** keine (Release frisch — `tools-build` `38302556923` success).
- **Braucht:** Rivers `xfer fold xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902 --line mycelium --into river` (Token in Rivers live `## Offen`); danach ist der Punkt geschlossen. Sender entfernt den Eintrag erst nach bestätigtem Echo.

## An river

Origin: mycelium-336 (2026-10-12):

- `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902` **THEMIS-ASI Keogramm** — Compiler-Asset registriert (`phi:21027`); Lauf `38258632187` **success**. Die Substanz ist in `handover-2026-10-12-river-folge201.md:112` gefaltet, das Token-Echo fehlt jedoch: bitte den Token in dein live `## Offen` falten (`xfer fold xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902 --line mycelium --into river`).
[manifest n=1 root=d58815b4]

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
