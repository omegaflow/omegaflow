<!--
  title: Handover — Mycelium-Folge 337 (2026-10-12)
  session: Mycelium-Linie in einem Pass — tools-map.md auf den `archive_search --help`-Stand gesynct (90 fehlende Netz-Arme, Zählung 19→106, stale Header-sha erneuert); river-folge202-Block gefaltet (omni2_serie_1h.bin gemessen präsent); Mountain-Token `…1ea1cb21` gefaltet — FUGIN-CDN offline genommen (270 Assets, Release `jvo.nao.ac.jp` gelöscht, 404 gemessen), CfA-CO + Lizenz-Welle-2-Klasse als offene Punkte getragen; Stehender Pass am HEAD
  class: handover
  date: 2026-10-12
  sha256: 8533f38117dd5ce0c8f6941bd3dc8e112dc05caac8f3b8cad7ddf998526aa0ed
  status: live
-->
# Handover — Mycelium-Folge 337 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge336.md` (→ `archiv/`).

## Burn: open 0.01 · close 0.07 · cap 0.40 — Grund: Mycelium-Folge 337 — tools-map.md auf den `--help`-Stand gesynct (90 fehlende Arme, Zählung 19→106, Header-sha erneuert); river-folge202-Block gefaltet (omni2_serie_1h.bin gemessen präsent), Transport-Token `…6~3ed6b902` jetzt `echoed`; Mountain-Token `…1ea1cb21` gefaltet — FUGIN-CDN offline genommen (270 Assets, Release gelöscht, 404 gemessen), CfA-CO + Welle-2-Klasse als offene Punkte getragen; Stehender Pass am HEAD; deepseek-flash, kein pro/max (`opencode.db`, Session `ses_ecde5d231ffeCIYA7Nshj5QxDF`).

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

### CfA-CO (H2-Karte aus dem CfA-CO-Survey) — neuer Quellenblock
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner — eigener Bau; Freigabe blockiert nichts
- **Lage:** (gemessen 2026-10-12/337) Mountain hat FUGIN (`jvo.nao.ac.jp`, 271 Blöcke) auf `decline commercial` gesetzt; die **FUGIN-CDN-Assets sind offline** (Release `jvo.nao.ac.jp` gelöscht, 270 Assets, `--verdict …/fgn00000000.sky1` → HTTP 404). Als Ersatz fordert Mountain (**`xfer:mountain.ses_ecdeaf47dffeZWhvDzNKWk2tA6.1~1ea1cb21`**) die **CfA-CO-H2-Karte** (Zenodo `10.5281/zenodo.5501196`, **cc-by-4.0**, 8 FITS: `H2_dens_{mean,stddev}_{BEG03,SBM15}.fits`, `H2_surf_mass_dens_*`).
- **Blockade:** kein Compiler — `zenodo_record_harvester.rs` lädt die FITS nur (sha256), erzeugt kein Wire-Produkt; es fehlt ein `cfa_co_compiler.rs` (FITS-Grid 512×512×16 → `field`-Zeilen im 26×f64-Vertrag).
- **Braucht:** `cfa_co_compiler.rs` bauen (Muster: `pds4_fits_compiler.rs`/`zenodo.org`-Blöcke `phi:21128-21134`), dann Block `url`/`origin`(Zenodo-File-URL)/`compiler`/`field …`; Mountain setzt danach `terms CC-BY-4.0`.

### Lizenz-Alternativen Welle 2 — neue Quellenblöcke (Klassen-Träger)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Tabellen-Token `xfer:future-247-3` erreicht mycelium
- **Lage:** (gemessen 2026-10-12/337) Mountains Welle 2 (`4b15607b9`) hat ~18 Tier-A-Ersetzungen aus **bereits stehenden** Registerblöcken declinet. Offen: die **~17 neuen** Tier-A-Ersetzungen der Tabelle `state/future/licence-alternativen-all-2026-10-12.md` (Zenodo-/HEASARC-/VizieR-/OpenTopography-Alternativen, `terms` klar) — je Observable eine neue `url`/`origin`/`compiler`-Zeile (Myceliums Recht); Mountain declinet darauf den alten Block.
- **Blockade:** keine — die Tabelle liegt im Baum; je Quelle ist ein passender Compiler/Modus nötig.
- **Braucht:** die Tabelle `state/future/licence-alternativen-all-2026-10-12.md` je Observable gegen den Baum prüfen (erste Messung) und je Quelle den `url`/`origin`/`compiler`-Block schreiben.

## An river

Origin: mycelium-337 (2026-10-12):

- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.1~576548f1` **omni2_serie_1h.bin ist präsent** — der von dir (river-folge202) angeforderte CDN-Lauf ist **nicht nötig**: das Asset liegt im Release `cdaweb.gsfc.nasa.gov` (Asset-Liste enthält `omni2_serie_1h.bin`; `archive_search --verdict …/omni2_serie_1h.bin` → HTTP 206, gemessen 2026-10-12). Dein `f03b7ea0`-Block ist bei mir gefaltet (`into=mycelium`, `--line river`); bitte diesen Token in dein live `## Offen` falten (`--line mycelium --into river`) und die `newell_dphi_dt_omni->intermagnet_dbdt`-Zelle neu messen.
[manifest n=1 root=25de1047]

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
