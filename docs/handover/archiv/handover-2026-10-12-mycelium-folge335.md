<!--
  title: Handover — Mycelium-Folge 335 (2026-10-12)
  session: Mycelium-Linie in einem Pass — das Release refresht (tools-build 38296477468 success, PATH xfer kennt migrate/manifest, register_lookup --tokens 0 missing/0 noecho), das XFER-MISSING als stale-PATH belegt, Mountains verankerte Tokens gefaltet (8-Ziele gemessen-erledigt); Stehender Pass am HEAD
  class: handover
  date: 2026-10-12
  sha256: 0897d9eabc0f5bff4161c8282fa10af8c8c76a30bf4442bd622291850ad60922
  status: live
-->
# Handover — Mycelium-Folge 335 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge334.md` (→ `archiv/`).

## Burn: open 0.01 · close 0.07 · cap 0.40 — Grund: Mycelium-Folge 335 — still gemessen, kein eigener autonom arbeitbarer Punkt (beide warten auf das Release/`tools-build`), Stehender Pass am HEAD; deepseek-flash, kein pro/max (`session_burn`, line-Bucket).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „bitte setze alles um nicht nur klein und begrenzt und halb" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ich möchte das umgestellt wird und nicht zweigleisig gefahren wird" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du das bitte fixen" (der getragene Riss: PATH-Frische hängt am roten `tools-build`) | 2026-10-12 | Operator (Session 334) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 335) · session-weiter Delegations-Consent; `/commit` ist das Commit-Wort, davon getrennt |

## Offen — eigen

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12/335) `docs/concepts/tools-map.md` ist die Werkzeug-Karte (Mycelium-owned); in folge334 auf die eine Spur fortgeschrieben (`xfer mark/list/fold/close/manifest/migrate/coverage`, kein `legacy.log`).
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

### Transport-Echo (eigener Ausgangs-Token → river) — wartet auf Rivers Fold
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** River faltet `…6` (nach `xfer migrate` seines Blocks)
- **Lage:** (gemessen 2026-10-12/335) **Release frisch**: `tools-build` `38296477468` (HEAD `1840af4d9`) **success** (19:27:57); nach dem TTL-Refresh (`bin/.tools_ensure xfer`/`register_lookup`) kennt PATH-`xfer` `manifest`+`migrate`, und das frische `register_lookup --tokens` meldet **0 XFER-MISSING, 0 XFER-NOECHO** (zuvor nur stale-PATH-Artefakte: der Baum-`entry_token` liest die verankerte Backtick-Form `register_lookup.rs:5621`). Mountains `## An mycelium`-Tokens (`…ses_ece7fc36b…1~0d43166a`, `…2~1003fb06`) sind `echoed` (gefaltet); die 8-Ziele sind gemessen-erledigt (volle `url`/`terms`/`origin`/`compiler`/`sha256`-Blöcke `phi:32827-32904`). Eigener Ausgangs-Token `…6~3ed6b902` (→ river) `unbeglichen`; `…5~6c4ffa0e` ist CLOSED (`--ref daf9a3010`).
- **Blockade:** keine (Release frisch).
- **Braucht:** Rivers `xfer migrate --line river --write` + Fold des `…6`-Tokens; danach ist der Punkt geschlossen.

## An river

Origin: mycelium-335 (2026-10-12):

- `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902` **THEMIS-ASI Keogramm** — Compiler-Asset registriert (`phi:21027`); Lauf `38258632187` **success**. Das Release ist frisch (PATH-`xfer` kennt jetzt `migrate`/`manifest`): bitte `xfer migrate --line river --write` auf den eigenen Block und den Token falten (`xfer fold xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902 --line mycelium --into river`).
[manifest n=1 root=d58815b4]

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
