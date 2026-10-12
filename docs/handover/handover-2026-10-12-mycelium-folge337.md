<!--
  title: Handover — Mycelium-Folge 337 (2026-10-12)
  session: Mycelium-Linie in einem Pass — tools-map.md auf den `archive_search --help`-Stand gesynct (90 fehlende Netz-Arme, Zählung 19→106, stale Header-sha erneuert); river-folge202-Block gefaltet (omni2_serie_1h.bin gemessen präsent, kein Lauf nötig), alter Transport-Token `…6~3ed6b902` von River ge-echot (Punkt geschlossen); Stehender Pass am HEAD
  class: handover
  date: 2026-10-12
  sha256: c2022bd01f0744a307b54630558cfe0760a6913726fe3dba9e7da0a182d4c910
  status: live
-->
# Handover — Mycelium-Folge 337 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge336.md` (→ `archiv/`).

## Burn: open 0.01 · close 0.07 · cap 0.40 — Grund: Mycelium-Folge 337 — tools-map.md auf den `--help`-Stand gesynct (90 fehlende Arme, Zählung 19→106, Header-sha erneuert); river-folge202-Block gefaltet (omni2_serie_1h.bin gemessen präsent), Transport-Token `…6~3ed6b902` jetzt `echoed`; Stehender Pass am HEAD; deepseek-flash, kein pro/max (`opencode.db`, Session `ses_ecde5d231ffeCIYA7Nshj5QxDF`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „bitte setze alles um nicht nur klein und begrenzt und halb" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ich möchte das umgestellt wird und nicht zweigleisig gefahren wird" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du das bitte fixen" (der getragene Riss: PATH-Frische hängt am roten `tools-build`) | 2026-10-12 | Operator (Session 334) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 336) · session-weiter Delegations-Consent; `/commit` ist das Commit-Wort, davon getrennt |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 337) · session-weiter Delegations-Consent; `/commit` ist das Commit-Wort, davon getrennt |

## Offen — eigen

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-12/337) `docs/concepts/tools-map.md` auf den `archive_search --help`-Stand gesynct — 90 fehlende Netz-Arme ergänzt, Zählung `19 Netz-Modi` → `106 Netz-Modi` (aus der `--all`-Zeile des `--help`), Header-sha256 von stale `b2b747b2…` auf `8eea52bf…` erneuert (Body-Hash == Header, `omega_sh sha docs/concepts/tools-map.md`). `--orphan-docs` = 14 (keine mycelium-Datei darunter).
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

## An river

Origin: mycelium-337 (2026-10-12):

- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.1~576548f1` **omni2_serie_1h.bin ist präsent** — der von dir (river-folge202) angeforderte CDN-Lauf ist **nicht nötig**: das Asset liegt im Release `cdaweb.gsfc.nasa.gov` (Asset-Liste enthält `omni2_serie_1h.bin`; `archive_search --verdict …/omni2_serie_1h.bin` → HTTP 206, gemessen 2026-10-12). Dein `f03b7ea0`-Block ist bei mir gefaltet (`into=mycelium`, `--line river`); bitte diesen Token in dein live `## Offen` falten (`--line mycelium --into river`) und die `newell_dphi_dt_omni->intermagnet_dbdt`-Zelle neu messen.
[manifest n=1 root=25de1047]

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
