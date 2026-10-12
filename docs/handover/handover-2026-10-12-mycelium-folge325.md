<!--
  title: Handover — Mycelium-Folge 325 (2026-10-12)
  session: Mycelium-Linie in einem Pass (ENTSØ-E-Fehlerkörper, USGS-Mosaik-sha + DSC-0119-Zeile, INPOP/NOE4-Workflow, Spill-Wurzel im Wrapper)
  class: handover
  date: 2026-10-12
  sha256: 64c22626688b17a332bf9718e49535a1b35c93ea6254767c03d536c77d838088
  status: live
-->
# Handover — Mycelium-Folge 325 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge324.md` (→ `archiv/`) und faltete die
adressierten Blöcke `future-folge241` (bereits getragen) und `mountain-folge326`.
**Fremd-Punkte aus 324 wurden korrigiert:** wy-max-t · Receiver-Apertur ·
MCP/MCON · GIC-Onset sind **River-eigen** (river-196) und hier entfernt;
--all-Spill · Fetch-Provenienz · Prosa-Träger sind in diesem Atom geschlossen
(s. git).

## Burn: open 0.00 · close 0.11 · cap 0.15 — Grund: Mycelium-325 in einem Atom (deepseek-flash; 3 grind-flash/general-Dispatches: USGS-Mosaik-sha + INPOP/NOE4-Workflow, Spill/Provenienz-Audit); kein pro/max. Gemessen via `session_burn`.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt … entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auch das ist wieder problematisch mountain arbeitet an den ephemeriden er kann sich nicht darum kümmern" | 2026-10-12 | Operator (Session 325) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ist das wirklich alles Trigger oder was kannst du davon selbst bis zur Kante abarbeiten?" | 2026-10-12 | Operator (Session 326) · `state/operator-gespraeche/2026-10-12-mycelium.md` |

## Offen — eigen

### ENTSØ-E A78 — Fehlerkörper erweitert; Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `entsoe-cdn`-Lauf `38236593646` endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38233635426 --all`) `38233635426` **rot**: `10YFI-1--------U: fetch void — HTTP 400: <!DOCTYPE html> …`; der Body war auf 200 B gekürzt (`entsoe_compiler.rs:129`). **Fix:** `fetch` zeigt jetzt bis 1500 B + Länge (`HTTP {code} ({len} B): {head}`); `cargo check` grün; neu dispatcht.
- **Blockade:** kehrt der neue Lauf denselben HTML-400, nennt der erweiterte Body Gateway/WAF oder Parameter.
- **Braucht:** `ci_manage log 38236593646` lesen; bei Grün den `url`/`origin`/`compiler`/`sha256`/`format entsoe_unavailability`/`ttl`-Block in `phi/sources.φ` eintragen.

### GRDC-Caravan — Resume-Fix gebaut; Verifikationslauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf `38234294097` endet
- **Lage:** (gemessen 2026-10-12) Resume-Fix steht (`src/archivar/fetch.rs`: `-C -` auf jedem Versuch, `--retry-max-time` = `transfer_bound_s`; `cargo check` grün; `934c131f3`). Lauf `38234294097` **pending**; `38217882014` war `curl: (18) transfer closed with 6822877025 bytes remaining`.
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log 38234294097`.

### M10↔MDIS-Raw-CDN — Mosaik + DSC 0119 registriert; Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `m10-mdis-raw-cdn`-Lauf nach dem Push endet
- **Lage:** (gemessen 2026-10-12) **Mosaik-Riss geklärt:** 302 → S3, `Content-Length 1880562475`; voller Download + `sha256sum` = `324c0e49…` (`complete=yes`); der `mountain-folge326`-Wert `1a90a8ff…` über 66 414 088 B ist **ungültig**. **DSC 0119 gemessen:** `ec3c41ebbb082287437fe6ff9b9ba91a7fd9474311b274d8cbd975ddd5ce55b0`, 1 380 249 B → `url`/`origin`/`compiler`-Zeile in `phi/sources.φ:31582ff` + Matrix. `.github/workflows/m10-mdis-raw-cdn.yml` trägt jetzt **8** sha256-gated Assets.
- **Blockade:** keine.
- **Braucht:** `ci_manage log` des neuen Laufs `38236598081`; der Register-Mosaikblock `phi:31582-31586` trägt noch **keine** `sha256`-Zeile — Mountain setzt `324c0e49…` (Quellen-Eigenschaft).

### NIST-NCNR-NSE Daten-Arm — Caller gebaut; Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nist-ncnr-nse-data-cdn`-Lauf `38233919420` endet
- **Lage:** (gemessen 2026-10-12) Workflow gebaut; Lauf `38233919420` **in_progress**; Coverage-Block `phi/sources.φ:11721`.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38233919420`; die Compiler-`url`-Blöcke in `phi/sources.φ` eintragen.

### INPOP/NOE4-Ephemeriden — CDN-Workflow gebaut; Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ephemeris-inpop-noe4-cdn`-Lauf `38236601893` endet
- **Lage:** (gemessen 2026-10-12) Origins gegen `phi/sources.φ:2185-2285` verifiziert; Release `ftp.imcce.fr` fehlt noch. `.github/workflows/ephemeris-inpop-noe4-cdn.yml` gebaut (2 Jobs: `inpop_compiler` / `noe4_compiler`, `--ci-mode`).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38236601893`.

### CI-Messzeilen durabel in den Baum — Ort korrigiert: **Tool-Register**, nicht `phi/`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `fold`-Job eines messenden Workflows (`.github/workflows/field-te-query.yml`) läuft
- **Lage:** (gemessen 2026-10-12) **Operator-Wort (Session 326):** „ich möchte [den `phi/`-Müll] nicht wieder anfangen — gehört das nicht zu den jobs oder tools?" — **Der Baum bestätigt es:** sechs getrackte Werkzeug-Register existieren bereits: `tools/harvest/data/{asteroid_gm_inpop25c,asteroid_gm_sb441,dust-maske,nrs_stations}.φ` · `tools/measure/data/scan_coverage.φ` · `tools/register/data/bands.φ` · `tools/utils/data/library.φ`. Der Ort ist **`tools/measure/data/ci_measurements.φ`** — beim produzierenden Werkzeug, neben `scan_coverage.φ`. **Kein `REGISTER_CLASSES`-Eingriff** (jener Kanon-Riegel gilt nur für `phi/*.φ`); `phi/` bleibt die eine Datei. Die Tool-Register erlauben `note` (anders als `phi/sources.φ`). **Riss benannt:** der Rat sprach `phi/ci_measurements.φ` — seine Prämisse „`phi/` ist der einzige Register-Container" ist gemessen falsch; der Baum (und der Operator) trägt.
- **Blockade:** keine.
- **Braucht:** `tools/measure/data/ci_measurements.φ`-Skelett (Kopf + Blockform: `run`/`head`/`descriptor` + rohe Beobachtung; 0-Kanon) + `fold`-Job im Pilot-Workflow `field-te-query.yml` (Commit via `block_commit` privat-Index + CAS).

### de441/de442-sha · SWPC/RTSW · Domain→UCD (getrennt, je ein Gegenüber)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Merge-Lauf (`phi/sources.φ:32008`) · River-`matrix-vlies` · messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-12) **de441/de442:** sha256 erst nach dem Merge-Lauf; die NAIF-Blöcke stehen (`phi/sources.φ:32008`, recursive SPK/PCK harvest) — der Merge-Lauf ist Mountain-Territorium. **SWPC/RTSW:** `matrix-vlies`-Wiederholung (River). **Domain→UCD:** keine messbare Quelle (`pending`).
- **Blockade:** je der genannte Lauf/Quelle.
- **Braucht:** Merge-Lauf (de441) · River-`matrix-vlies` · messbare Domain-Quelle.

### ui-seats-Träger (`docs/concepts/ui-seats.md`) — offene `pending`-Seats
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Stimmen-Round in `mycelium-ui` (`open-weight-ui`-Lock)
- **Lage:** (gemessen 2026-10-12) `--orphan-docs` = 5, keines ist `ui-seats`; die `pending`-Seats Sakana-Tempo · SciSpace (Login-Wall) · Gemini Notebook (kein freier Composer) noch nicht gemessen.
- **Blockade:** kein Stimmen-Round in dieser Session.
- **Braucht:** im nächsten Stimmen-Round die `pending`-Seats messen und den Stand fortschreiben.

### Prosa-Träger — `docs/concepts/tools-map.md` (Mycelium-eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Zensus
- **Lage:** (gemessen 2026-10-12) `--orphan-docs` = **9**; Mycelium-eigen: `docs/concepts/tools-map.md` (1 offener Marker) — der Träger ist diese Zeile. Fremd (Mountain/River): `docs/auftrag/auftrag-2026-10-11-wartend-aufloesung.md` (3) · `docs/blatt/blatt-gic-breitenband-familien.md` (4) · `docs/plans/ref-mariner-faden-index.md` (3) · `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (1) · `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1) · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4) · `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` (2) · `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2).
- **Blockade:** keine.
- **Braucht:** die 8 fremden Dokumente je von ihrer Owner-Linie getragen (nicht Mycelium); `tools-map.md` ist hier getragen — der nächste Lauf belegt es.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
