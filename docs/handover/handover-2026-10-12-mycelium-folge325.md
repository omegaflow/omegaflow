<!--
  title: Handover — Mycelium-Folge 325 (2026-10-12)
  session: Mycelium-Linie in einem Pass (ENTSØ-E-Fehlerkörper erweitert, USGS-Mosaik-sha gemessen, INPOP/NOE4-CDN-Workflow gebaut)
  class: handover
  date: 2026-10-12
  sha256: 1863a320ebdfe10dd30caa23944be185c43babe1b501334a637aeff645123a9a
  status: live
-->
# Handover — Mycelium-Folge 325 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge324.md` (→ `archiv/`) und faltete die
adressierten Blöcke `future-folge241` (bereits getragen) und `mountain-folge326`
(INPOP/NOE4, M10-Kamera, DSC 0119). Jeder offene Punkt nennt Status/Bindung,
Trigger, Lage (gemessen), Blockade, Braucht.

## Burn: open 0.00 · close 0.09 · cap 0.15 — Grund: Mycelium-325 in einem Atom (deepseek-flash; 2 grind-flash-Dispatches: USGS-Mosaik-sha256 + INPOP/NOE4-CDN-Workflow); kein pro/max. Gemessen via `session_burn` (Linien-Agent `$0.6421`→`$0.6990`, grind-flash `$0.4586`→`$0.4921`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt, unsere arbeitsweise muss sich ändern ihr dürft euch nicht mehr länger blockieren entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du bitte deinen und mountains teil bearbeiten mountain ist beschäftigt" | 2026-10-12 | Operator (Session 318) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du dich bitte um deinen teil kümmern? Klare Trennung — und ich habe meinen Teil gleich gebaut." | 2026-10-12 | Operator (Session 317) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 325) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auch das ist wieder problematisch mountain arbeitet an den ephemeriden er kann sich nicht darum kümmern" | 2026-10-12 | Operator (Session 325) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |

## Offen — eigen

### ENTSØ-E A78 — Fehlerkörper erweitert; Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `entsoe-cdn`-Lauf nach dem Push endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38233635426 --all`) `38233635426` **rot**: `10YFI-1--------U: fetch void — HTTP 400: <!DOCTYPE html> …` — der Body war auf 200 B gekürzt (`entsoe_compiler.rs:129`), Grund nicht lesbar; 537-Zeilen-Job-Log trug nur den Anfang. Repo-Secret `ENTSOE_API_TOKEN` steht (materialisiert, `ENTSOE_API_TOKEN: ***`). **Fix in diesem Atom:** `fetch` zeigt jetzt bis 1500 B + Body-Länge (`min(1500)`, `HTTP {code} ({len} B): {head}`); `cargo check` grün.
- **Blockade:** kehrt der neue Lauf denselben HTML-400, nennt der erweiterte Body Gateway/WAF oder Parameter als Ursache.
- **Braucht:** nach Push `gh workflow run entsoe-cdn.yml`; dann `ci_manage log <id>` den erweiterten Body lesen; bei Grün den `url`/`origin`/`compiler`/`sha256`/`format entsoe_unavailability`/`ttl`-Block in `phi/sources.φ` eintragen.

### GRDC-Caravan — Resume-Fix gebaut; Verifikationslauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf `38234294097` endet
- **Lage:** (gemessen 2026-10-12) Resume-Fix in `src/archivar/fetch.rs` steht im Baum (`-C -` auf jedem Versuch, `--retry-max-time` = `transfer_bound_s`; `cargo check` grün; Commit `934c131f3`). Lauf `38234294097` **pending**; `38217882014` war `curl: (18) transfer closed with 6822877025 bytes remaining`.
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log 38234294097` (erwartet: der `(18)`-Abbruch wird fortgesetzt → finite streamflow-Samples → `grdc_caravan.bin`).

### M10↔MDIS-Raw-CDN — USGS-Mosaik-sha gemessen; Workflow vervollständigt; Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `m10-mdis-raw-cdn`-Lauf nach dem Push endet
- **Lage:** (gemessen 2026-10-12) **Der USGS-Mosaik-Riss ist geklärt:** `planetarymaps.usgs.gov` 302 → `asc-pds-services.s3.us-west-2.amazonaws.com`, `Content-Length: 1880562475`; voller Download + `sha256sum` = **324c0e497570f908db6a85d4e4823a9b76da1f676d0ecdb1ba8f67559e14e52f** (`bytes=1880562475 complete=yes`). Der in `mountain-folge326` genannte `sha256 1a90a8ff…` über 66 414 088 B ist **ungültig** (auf einem gekürzten Download berechnet). Die Matrix in `.github/workflows/m10-mdis-raw-cdn.yml` trägt jetzt 7 Assets (PSJ t1–t4 + camera_angles + LPSC-PDF + Mosaik, sha gated); `cargo check` n/a (YAML).
- **Blockade:** keine.
- **Braucht:** nach Push `gh workflow run m10-mdis-raw-cdn.yml`; dann `ci_manage log <id>`. USGS-Mosaik: die neue Matrixzeile auf grün lesen (1,88 GB Body gate). Der Registerblock `phi/sources.φ:31582-31586` trägt **keine** `sha256`-Zeile — Mountain setzt dort den gemessenen Wert `324c0e49…` (Quellen-Eigenschaft); url/origin/compiler stehen bereits.

### NIST-NCNR-NSE Daten-Arm — Caller gebaut; Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nist-ncnr-nse-data-cdn`-Lauf `38233919420` endet
- **Lage:** (gemessen 2026-10-12) `.github/workflows/nist-ncnr-nse-data-cdn.yml` gebaut; Lauf `38233919420` **in_progress** (Schritt „Compile and upload each NSE data set to the CDN"); Coverage-Block `phi/sources.φ:11721`.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38233919420`; die vom Compiler emittierten Daten-`url`-Blöcke in `phi/sources.φ` eintragen.

### INPOP/NOE4-Ephemeriden — CDN-Workflow gebaut; Lauf offen (mountain-folge326)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ephemeris-inpop-noe4-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-12) Mountain registrierte 12 Blöcke (`phi/sources.φ:2185-2285`, `terms etalab-2.0`); Origins verifiziert (`inpop19a_TDB_m100_p100_spice.tar.gz`, `ftp.imcce.fr/pub/ephem/satel/NOE/MARS/2020/NOE-4-2020.bsp`); Release `ftp.imcce.fr` existiert noch nicht. **In diesem Atom:** `.github/workflows/ephemeris-inpop-noe4-cdn.yml` gebaut (2 Jobs: `inpop_compiler` bzw. `noe4_compiler`, `--ci-mode`, Idempotenz auf `ephemeris_inpop_earth.bin`/`ephemeris_noe4_phobos.bin`).
- **Blockade:** keine.
- **Braucht:** nach Push `gh workflow run ephemeris-inpop-noe4-cdn.yml`; dann `ci_manage log <id>`.

### Fetch-Provenienz — `archive_search`-Sidecar steht; allgemeiner Wrapper-Pfad fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein kurzer Fetch-Weg (`bin/`-curl/Playwright-Wrapper der Taucher) schreibt den Quell-Sidecar
- **Lage:** (gemessen 2026-10-12) `write_fetched` (`tools/utils/src/bin/archive_search.rs`) schreibt `<datei>.src` (Quell-URL), nichts wenn URL fehlt (`d884a2e93`, `213d8937f`). Offen: allgemeine Wrapper-Route.
- **Blockade:** keine.
- **Braucht:** der curl/Playwright-Wrapper schreibt den Quell-Sidecar (Mycelium).

### archive_search --all-Spill — Werkzeug gebaut; die Linien setzen die Wurzel nicht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine Linien-Session setzt `OMEGAFLOW_SPILL_DIR`
- **Lage:** (gemessen 2026-10-12) `spill_dir()` honoriert `$OMEGAFLOW_SPILL_DIR` (sonst `temp_dir`) in `archive_search/net.rs`, `frbcat.rs`, `opensn.rs` (`03bb95e2e`). Keine Linien-Config setzt die Wurzel.
- **Blockade:** keine.
- **Braucht:** die Linien-Commands (`state/<line>/`-Preset bzw. `bin/`-Wrapper) exportieren `OMEGAFLOW_SPILL_DIR` auf einen kanonischen Spill-Ort (Mycelium).

### Offene-Prosa-Träger — Träger-Zeilen in die eigene Übergabe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Zensus
- **Lage:** (gemessen 2026-10-12) `docs/concepts/free-voices.md` (1 offener Marker) · `docs/concepts/tools-map.md` (2); grenznah: `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (1) · `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1) · `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` (2).
- **Blockade:** keine.
- **Braucht:** je Dokument eine Träger-Zeile (Mycelium für `free-voices`/`tools-map`; Rest Mountain) — `register_lookup --orphan-docs` als Nachweis.

### Struktur-Fix — CI-Messzeilen durabel in den Baum (Rat, 2026-10-12)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `fold`-Job eines messenden Workflows (`.github/workflows/field-te-query.yml`) läuft
- **Lage:** (gemessen 2026-10-12, Operator-Wort Session 325) Rat-Verdikt (5 Stimmen): neues getracktes append-only Register `phi/ci_measurements.φ`, ein terminaler `fold`-Job je Workflow (Commit über `block_commit` privat-Index + CAS); Block trägt `run_id`/`head_sha`/`descriptor` + rohe Beobachtung, nie ein Verdikt-Wort; Serien-Ergebnis → CDN-Asset.
- **Blockade:** neue getrackte Registerklasse → `REGISTER_CLASSES`-Eintrag (`src/gate/commit_gate.rs`) + Kanon-Wort (Architektur-Akt).
- **Braucht:** `REGISTER_CLASSES`-Eintrag + `phi/ci_measurements.φ`-Skelett + `fold`-Job in `field-te-query.yml` (Pilot); Verdikt liegt vor (Rat).

### M10-EDR/Ephemeride + DSC 0119 (mountain-folge325/326)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine M10-Ephemeriden-`url`/`origin`/`compiler`-Zeile steht
- **Lage:** (gemessen 2026-10-12) M10-Kamera-`url` steht im CDN-Workflow (`ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt`, sha `0a65df59…`); Mountain braucht für den M1-Kamera-Blindfleck die `url`-Zeile für `ser.im-ldi.com/M10/M10_ISIS/` sowie `spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/renamed_dataset_catalogs/mariner4_celestial-mechanics_64-077a-09a_DSC_0119.pdf` (sha `ec3c41eb…`).
- **Blockade:** `phi/sources.φ` trägt fremde uncommittete Berg-Hunks (Zeilenklassen-Verschränkung) — ein Fremd-Schreibakt würde den Block-Commit mit fremder Arbeit mischen.
- **Braucht:** die `url`/`origin`/`compiler`-Zeilen in `phi/sources.φ` setzen, sobald der Baum für Mycelium frei ist (oder per `block_commit` block-scoped); dann `terms`/`format` (Mountain).

### MCP no-leak · de441-de442-sha · SWPC/RTSW · Domain→UCD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart · Merge-Lauf (`phi:4321-4369`) · `matrix-vlies`-Lauf · messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11/12) MCP: Allowlist konsistent, Live-Seat-Restart offen; de441/de442: sha256 erst nach Merge-Lauf; SWPC/RTSW: `matrix-vlies`-Wiederholung (River); Domain→UCD: keine messbare Quelle.
- **Blockade:** je der genannte Lauf/Quelle.
- **Braucht:** je der genannte Schritt (`ci_manage log <id>`; Merge-Lauf; messbare Domain-Quelle).

### wy-max-t-Artefakt — Werte im Bestand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`
- **Lage:** (gemessen 2026-10-12, River-196) α=0.05, B=9999, (1-α)-Quantil 2.4467/2.4433/2.4831 (Artefakt `38166787414`); studentisiertes Moving-Block-Bootstrap benannt.
- **Blockade:** Siegel (Operator).
- **Braucht:** die gemessenen Werte ins Register/Artefakt (Mycelium); Siegel (Operator).

### ui-seats-Träger (`docs/concepts/ui-seats.md`) — offene `pending`-Seats
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Stimmen-Round in `mycelium-ui` (`open-weight-ui`-Lock) läuft
- **Lage:** (gemessen 2026-10-12) `register_lookup --orphan-docs` = 0; `pending`-Seats Sakana-Tempo · SciSpace (Login-Wall) · Gemini Notebook (kein freier Composer) noch nicht gemessen.
- **Blockade:** kein Stimmen-Round in dieser Session gefahren.
- **Braucht:** im nächsten Stimmen-Round die `pending`-Seats messen und den Stand fortschreiben.

### MLA-Register · SPT-3G-D1 · GloFAS-EWDS
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Lauf/Digest bzw. eigener Register-Schritt
- **Lage:** (gemessen 2026-10-12) MLA repointet (`phi:12100`; NAME-Riss); SPT-3G `38150991206` success (sha offen); GloFAS Daten-Arm grün (`phi:11297`; grib2→φ offen).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` bzw. der genannte Register-Schritt.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
