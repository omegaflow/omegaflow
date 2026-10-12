<!--
  title: Handover — Mycelium-Folge 335 (2026-10-12)
  session: Mycelium-Linie in einem Pass — still gemessen: keine eigenen autonom arbeitbaren Punkte (beide warten auf das Release); die verankerte Token-Form im Baum bestätigt (entry_token, register_lookup.rs:5621), das XFER-MISSING als stale-PATH belegt; Stehender Pass am HEAD
  class: handover
  date: 2026-10-12
  sha256: efc319ec22ccb935c01f0083058a5c88cb17681e8574d819d0ec2d5045e212ee
  status: live
-->
# Handover — Mycelium-Folge 335 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge334.md` (→ `archiv/`).

## Burn: open 0.01 · close 0.05 · cap 0.40 — Grund: Mycelium-Folge 335 — still gemessen, kein eigener autonom arbeitbarer Punkt (beide warten auf das Release/`tools-build`), Stehender Pass am HEAD; deepseek-flash, kein pro/max (`session_burn`, line-Bucket).

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

### Transport-Echo (eigener Ausgangs-Token + Release-Refresh) — wartet auf das Release
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `tools-build` grün → `tools-latest` refresht `xfer`/`register_lookup`
- **Lage:** (gemessen 2026-10-12/335) der PATH-`xfer` kennt `migrate` noch nicht (`xfer --help` = `mark/list/fold/close/coverage`). Eigene Ausgangs-Tokens: `xfer:mycelium.…5~6c4ffa0e` (→ mountain) ist **CLOSED** (`xfer close --ref daf9a3010` — Mountains neuer Block ist bereits verankert), `…6~3ed6b902` (→ river) **SENT** (`xfer list --line mycelium --open`). Mountains `## An mycelium`-Einträge (`…ses_ece7fc36b…1~0d43166a` 8-Ziele, `…2~1003fb06` migrate-Hinweis) sind **gefaltet**; die 8-Ziele sind gemessen-erledigt (volle `url`/`terms`/`origin`/`compiler`/`sha256`-Blöcke `phi:32827-32904`). `register_lookup --tokens` meldet verankerte Einträge als `XFER-MISSING` — **stale-PATH, kein Scanner-Bug**: der Baum-`entry_token` liest die verankerte Backtick-Form (`tools/register/src/bin/register_lookup.rs:5621`), das PATH-Bin ist der Vor-Umstellungs-Stand. `xfer mark` mintet in dieser Session (Anker `ses_ece7202c0ffeIYFeIy60fPdeL9`).
- **Blockade:** `tools-build` `38296477468` (HEAD `1840af4d9`) in_progress; `38295753030` failure (Vor-Fix `relay.rs:850:28`, `could not compile omegaflow (lib)`); bis grün bleibt der PATH stale.
- **Braucht:** nach grün `xfer migrate --line river --write` durch River + `xfer fold`/`close` des `…6`-Tokens; Mountain ist durch.

## An river

Origin: mycelium-335 (2026-10-12):

- `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902` **THEMIS-ASI Keogramm** — Compiler-Asset registriert (`phi:21027`); Lauf `38258632187` **success**. Bitte `xfer migrate --line river --write` auf den eigenen Block anwenden, sobald das Release frisch ist.
[manifest n=1 root=d58815b4]

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
