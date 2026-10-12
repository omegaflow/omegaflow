<!--
  title: Handover — Mycelium-Folge 326 (2026-10-12)
  session: Mycelium-Linie in einem Pass (ENTSØ-E-WAF-Messung, NIST-Abschluss, adressierte Faltungen, Stehender Pass)
  class: handover
  date: 2026-10-12
  sha256: 6d96e7e8bcc617f3d5333183da2d6bf8ace51e75e22caa9f924c1aa2d710a02f
  status: live
-->
# Handover — Mycelium-Folge 326 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-12-mycelium-folge325.md`
(→ `archiv/`) und faltete die adressierten Blöcke `future-folge242` und
`mountain-folge327`.

**Gefaltet (gemessen 2026-10-12):** `future-242` führt die USGS-Kombi-Mosaik-Zeile mit
sha `1a90a8ff…` über 66 414 088 B — **ungültig** (mountain-326); der gemessene Wert ist
`324c0e49…` über 1 880 562 475 B (M10-Punkt unten). `mountain-327`s M10-Kamera/ISIS
(`ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt`, `phi:31570`), ASU-MVM (`phi:31519`),
SPDF-M4/M6/M7-`celestial_mechanics` (`phi:31526/31534`), DSC 0119 (`phi:31590`), JADES
(`phi:32391`), BK18 (`phi:31395`), FMI-NUR (`phi:20387`) und die NIST-Route (`phi:11817`)
sind **registriert**. Offen aus der Faltung bleibt die **Sonke/Robinson-Tabellen 1–8**
(keine Zeile im Register). **Geschlossen:** NIST-Arm (Lauf `38233919420` grün),
Prosa-Träger `tools-map.md` (`--orphan-docs` = 0).

## Burn: open 0.00 · close 0.13 · cap 0.15 — Grund: Mycelium-Folge 326 in einem Atom (deepseek-flash); kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt … entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auch das ist wieder problematisch mountain arbeitet an den ephemeriden er kann sich nicht darum kümmern" | 2026-10-12 | Operator (Session 325) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ist das wirklich alles Trigger oder was kannst du davon selbst bis zur Kante abarbeiten?" | 2026-10-12 | Operator (Session 326) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-12 | Operator (Start Mycelium-326) · session-weiter Delegations-Consent, nicht das Commit-Wort |

## Offen — eigen

### ENTSØ-E A78 — Fehlerkörper erweitert; Lauf rot (HTML-Fehlerseite)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `entsoe-cdn`-Lauf
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38237220726`) `38237220726` (head `e0510663b`) **rot**: `10YFI-1--------U: fetch void — HTTP 400 (5772 B): <!DOCTYPE html> … plain.error-page-wrapper …`, the domain stays unharvested. Der erweiterte Fehlerkörper greift (5772 B statt 200 B); der Body ist eine HTML-Fehlerseite ohne Parameter-Diagnose.
- **Blockade:** die Zone `10YFI-1--------U` liefert eine HTML-Fehlerseite (Gateway/WAF-Stil); kein Parameterhinweis im Body.
- **Braucht:** aus dem Body WAF vs. Parameter entscheiden; danach den `url`/`origin`/`compiler`/`sha256`/`format`/`ttl`-Block in `phi/sources.φ` eintragen.

### GRDC-Caravan — Resume-Fix gebaut; Verifikationslauf läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf `38234294097` endet
- **Lage:** (gemessen 2026-10-12 10:51Z via `ci_manage view`) Lauf `38234294097` (head `934c131f3`) **in_progress**; Resume-Fix (`src/archivar/fetch.rs`) steht.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38234294097`.

### M10↔MDIS-Raw-CDN — Mosaik + DSC 0119 registriert; Lauf queued
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `m10-mdis-raw-cdn`-Lauf `38237291082` endet
- **Lage:** (gemessen 2026-10-12) USGS-Mosaik sha `324c0e49…` über 1 880 562 475 B (der `future-242`-Wert `1a90a8ff…`/66 414 088 B ungültig); DSC 0119 `ec3c41eb…` (`phi:31590`); Workflow trägt jetzt 8 sha256-gated Assets. Lauf `38237291082` (head `b62fb87b9`) **queued**; `38236598081` cancelled.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38237291082`; der Mosaik-Registerblock `phi:31582-31586` braucht die `sha256`-Zeile (`324c0e49…` — Quellen-Eigenschaft, Mountain).

### INPOP/NOE4-Ephemeriden — CDN-Workflow läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ephemeris-inpop-noe4-cdn`-Lauf `38236601893` endet
- **Lage:** (gemessen 2026-10-12 10:51Z) Lauf `38236601893` (head `5985eb953`) **in_progress**; 12 Blöcke `phi:2185-2285` (`terms etalab-2.0`); Release `ftp.imcce.fr` fehlt noch.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38236601893`.

### CI-Messzeilen — Werkzeug-Register gebaut; Pilot-Lauf queued
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `ci-measurements-fold`-Lauf endet
- **Lage:** (gemessen 2026-10-12) `tools/measure/data/ci_measurements.φ` (Skelett) + `.github/workflows/ci-measurements-fold.yml` (`workflow_run` auf `field-te-query`, `block_commit` privat-Index + CAS). Pilot `field-te-query` `38238626309` (head `43f9c83d0`) **queued**.
- **Blockade:** der erste Fold-Push über `OMEGAFLOW_TOKEN`/`block_commit` ist ungeprüft.
- **Braucht:** nach dem Lauf `ci_manage log <fold-run-id>` + `sread tools/measure/data/ci_measurements.φ`.

### Sonke/Robinson-Tabellen 1–8 — nicht registriert (gefaltet mountain-327)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** — (kein externes Ereignis; hängt an der ungemessenen Quelle)
- **Lage:** (gemessen 2026-10-12 via `sgrep -i sonke/sonncke/robinson phi/sources.φ`) keine Zeile; `ser.im-ldi.com/M10/` (`phi:31517`) und `M10_ISIS/camera_angles.txt` (`phi:31570`) sind registriert.
- **Blockade:** die 8 Tabellen-URLs sind ungemessen (ohne Register-Eintrag, Quelle unbekannt).
- **Braucht:** die URLs über `archive_search` messen (`archive_search --verdict <url>`) und je als `url`/`origin`/`compiler` registrieren.

### de441/de442-sha — sha256 erst nach dem Merge-Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Merge-Lauf (`phi/sources.φ:32008`)
- **Lage:** (gemessen 2026-10-12) die NAIF-Blöcke stehen (`phi:32008`, recursive SPK/PCK harvest); der Merge-Lauf ist Mountain-Territorium.
- **Blockade:** Merge-Lauf.
- **Braucht:** Merge-Lauf (Mountain).

### dropped-gate-move-disposition — Punkt-ID/Hash-Baseline offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Baseline-Vergleich (`278ba2905`)
- **Lage:** (gemessen 2026-10-12) Baseline 924 keys; Disposition offen wie im Alt-Stand.
- **Blockade:** keiner.
- **Braucht:** `register_lookup --dropped-roster --baseline <datei>` gegen die Baseline.

### ui-seats — offene `pending`-Seats
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Stimmen-Round in `mycelium-ui`
- **Lage:** (gemessen 2026-10-12) `--orphan-docs` = 0; die `pending`-Seats Sakana-Tempo · SciSpace (Login-Wall) · Gemini Notebook (kein freier Composer) noch nicht gemessen.
- **Blockade:** kein Stimmen-Round in dieser Session.
- **Braucht:** im nächsten Stimmen-Round die `pending`-Seats messen und `docs/concepts/ui-seats.md` fortschreiben.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
