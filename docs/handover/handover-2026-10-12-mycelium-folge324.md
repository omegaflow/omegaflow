<!--
  title: Handover — Mycelium-Folge 324 (2026-10-12)
  session: Mycelium-Linie in einem Pass (ENTSO-E-Arm dispatcht, M10-raw-CDN-Caller + GRDC-Resume gebaut, NIST-NSE-Daten-Caller)
  class: handover
  date: 2026-10-12
  sha256: 68ee8e0eff2c53b1b3b35dba281597d0981eee489fd8e779995a06347859f99c
  status: live
-->
# Handover — Mycelium-Folge 324 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge323.md` (→ `archiv/`) und faltete die
adressierten Blöcke `future-folge241` und `mountain-folge325`.

## Burn: open 0.00 · close 0.0915 · cap 0.15 — Grund: Mycelium-324 in einem Atom (deepseek-flash; 3 Dispatches: alle grind-flash); kein pro/max. Gemessen via `session_burn --top 50`: Linie `0.0521` + `Diagnose GRDC 7GB curl truncation` `0.0209` + `Build NIST NSE data-arm CI caller` `0.0119` + `Build M10 raw CDN caller workflow` `0.0066`.

In diesem Atom gemessen/gebaut: **Z-Minuten-Null gemessen** (`bz-dbdt-delay-probe`
`38226083266` success, ABK 2024): studentisiertes Null **Z = max |T| = 6.4601**
(block-jackknife SE, block = 1386 min) · p_adj = 0.0050 · Kalibrier-Schwelle
M_(1−0.05) = 2.8972 über 200 Surrogate · beobachtetes max |r| = 0.31053 · n = 523621;
die `tee`-Umstellung macht das Ergebnis künftig im Job-Log sichtbar. ·
**GRDC-Caravan-Fehlgrund gemessen** (`38217882014`): `curl: (18) transfer closed
with 6822877025 bytes remaining` — 7 GB Single-Connection-Abbruch; **Resume-Fix
gebaut** (`src/archivar/fetch.rs`: `fetch_raw_to_file_with` läuft eine äußere
Schleife mit `-C -` ab dem zweiten Versuch, `FILE_RESUME_ATTEMPTS = 4`;
`cargo check` grün). · **M10↔MDIS-Raw-CDN-Caller gebaut**
(`.github/workflows/m10-mdis-raw-cdn.yml`, 6 sha256-gated Assets; USGS-Mosaik
ausgenommen — kein Register-sha256). · **NIST-NSE-Daten-Caller gebaut**
(`.github/workflows/nist-ncnr-nse-data-cdn.yml`, walkt das JSON-LD-Katalog,
kompiliert + lädt je `.nxs.nse`-Satz). · **ENTSO-E-Arm dispatcht**
(`entsoe-cdn` `38233635426`); `entsoe-token` in `state/zustand/wartend.φ`
geschlossen (Repo-Secret `ENTSOE_API_TOKEN` steht, gemessen via `gh secret list`).
· `archive_search --all`-Spill honoriert bereits `$OMEGAFLOW_SPILL_DIR` (gebaut
`03bb95e2e`, im Baum) — die Linien setzen es noch nicht (offener Punkt unten).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt, unsere arbeitsweise muss sich ändern ihr dürft euch nicht mehr länger blockieren entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du bitte deinen und mountains teil bearbeiten mountain ist beschäftigt" | 2026-10-12 | Operator (Session 318) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du dich bitte um deinen teil kümmern? Klare Trennung — und ich habe meinen Teil gleich gebaut." | 2026-10-12 | Operator (Session 317) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 324) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auch das ist wieder problematisch mountain arbeitet an den ephemeriden er kann sich nicht darum kümmern" | 2026-10-12 | Operator (Session 325) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |

## Offen — eigen

### ENTSO-E A78 — Token da; Lauf dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `entsoe-cdn` `38233635426` endet
- **Lage:** (gemessen 2026-10-12 via `gh secret list` + dispatch) Repo-Secret `ENTSOE_API_TOKEN` steht (gesetzt 2026-10-11T19:36:15Z); Ticket `#9473923` gelöst (future-241); `entsoe-token` in `state/zustand/wartend.φ` geschlossen; Lauf `38233635426` dispatcht (Workflow materialisiert den Token aus dem Secret in `.secrets.local`).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38233635426`; bei grün den vom Compiler emittierten Register-Block (`url`/`origin`/`compiler`/`sha256`/`format entsoe_unavailability`/`ttl`) in `phi/sources.φ` eintragen.

### GRDC-Caravan — Resume-Fix gebaut; Verifikationslauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf nach dem Push endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38217882014`) `38217882014` **failure** — `curl: (18) transfer closed with 6822877025 bytes remaining` (7-GB-Zip, Single-Connection-Abbruch nach 86,6 min); `38220536486` war noch `in_progress`. Resume-Fix in `src/archivar/fetch.rs` gebaut (`-C -` äußere Schleife, `cargo check` grün).
- **Blockade:** keine im Code.
- **Braucht:** nach Push `gh workflow run grdc-caravan-cdn.yml`; dann `ci_manage log <id>` (erwartet: finite streamflow-Samples → `grdc_caravan.bin`).

### M10↔MDIS-Artefakte — Raw-CDN-Caller gebaut; Lauf + Mosaik-sha offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `m10-mdis-raw-cdn`-Lauf nach dem Push endet
- **Lage:** (gemessen 2026-10-12) `.github/workflows/m10-mdis-raw-cdn.yml` gebaut — 6 sha256-gated Assets (PSJ t1–t4, `mariner_m10_camera_angles.txt`, `lpsc2024_2663.pdf`) spiegeln byte-für-byte via `raw_mirror`. Das USGS-Mosaik (`phi/sources.φ:31486`) ist ausgenommen: kein `sha256` in seinem Block (`raw_mirror` verweigert ein unverifiziertes Body), und die Größe ist gemessen widersprüchlich (66 414 088 B vs. 1,88 GB).
- **Blockade:** USGS-Mosaik-sha256 fehlt.
- **Braucht:** nach Push `gh workflow run m10-mdis-raw-cdn.yml` + `ci_manage log <id>`; USGS-Mosaik: Größe + sha256 messen (`archive_search --sniff <url>`) → Register-`sha256`.

### NIST-NCNR-NSE Daten-Arm — Caller gebaut; Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nist-ncnr-nse-data-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-12) `.github/workflows/nist-ncnr-nse-data-cdn.yml` gebaut — walkt `https://ncnr.nist.gov/pub/ncnrdata/nse/?format=jsonld`, kompiliert + lädt je `.nxs.nse`-Satz (`nist_ncnr_nse_nexus_compiler --url … --ci-mode`); der Coverage-Manifest-Block steht (`phi/sources.φ:11721`).
- **Blockade:** keine.
- **Braucht:** nach Push `gh workflow run nist-ncnr-nse-data-cdn.yml`; die vom Compiler emittierten Daten-`url`-Blöcke in `phi/sources.φ` eintragen.

### Fetch-Provenienz — `archive_search`-Sidecar steht; der allgemeine Wrapper-Pfad fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein kurzer Fetch-Weg (`bin/`-curl/Playwright-Wrapper der Taucher) schreibt den Quell-Sidecar
- **Lage:** (gemessen 2026-10-12) `write_fetched` (`tools/utils/src/bin/archive_search.rs`) schreibt `<datei>.src` (Quell-URL) und nichts, wenn die URL fehlt — gebaut (`d884a2e93`, `213d8937f`). Offen bleibt die allgemeine Wrapper-Route.
- **Blockade:** keine.
- **Braucht:** der curl/Playwright-Wrapper schreibt den Quell-Sidecar (Mycelium).

### archive_search --all-Spill — Werkzeug gebaut; die Linien setzen die Wurzel nicht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine Linien-Session setzt `OMEGAFLOW_SPILL_DIR`
- **Lage:** (gemessen 2026-10-12) `spill_dir()` honoriert `$OMEGAFLOW_SPILL_DIR` (sonst `temp_dir`) in `archive_search/net.rs`, `frbcat.rs`, `opensn.rs` — gebaut `03bb95e2e`, im Baum. Keine Linien-Config setzt die Wurzel (sgrep in `.opencode`/`bin` leer).
- **Blockade:** keine.
- **Braucht:** die Linien-Commands (`state/<line>/`-Preset bzw. `bin/`-Wrapper) exportieren `OMEGAFLOW_SPILL_DIR` auf einen kanonischen Spill-Ort (Mycelium).

### Offene-Prosa-Träger — Träger-Zeilen in die eigene Übergabe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Zensus
- **Lage:** (gemessen 2026-10-12) `docs/concepts/free-voices.md` (1 offener Marker) · `docs/concepts/tools-map.md` (2); grenznah/other: `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (1) · `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1) · `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` (2).
- **Blockade:** keine.
- **Braucht:** je Dokument eine Träger-Zeile (Mycelium für `free-voices`/`tools-map`; Rest Mountain) — `register_lookup --orphan-docs` als Nachweis.

### Struktur-Fix — CI-Messzeilen durabel in den Baum (Rat, 2026-10-12)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `fold`-Job eines messenden Workflows (`.github/workflows/field-te-query.yml`) läuft
- **Lage:** (gemessen 2026-10-12, Operator-Wort Session 325) eine CI-Messzeile (`witness`/`riss`, z. B. der `intermagnet_dbdt`-Riss oder das `bz_dbdt`-Z = 6.4601) lebt heute nur im Job-Log/Artefakt. Rat-Verdikt (5 Stimmen): ein neues getracktes append-only Register `phi/ci_measurements.φ`, ein Writer (terminaler `fold`-Job je Workflow, Commit über `block_commit` privat-Index + CAS); der Block trägt `run_id`/`head_sha`/`descriptor` + die rohe Beobachtung — nie ein Verdikt-Wort; das volle Serien-Ergebnis → CDN-Asset.
- **Blockade:** neue getrackte Registerklasse → `REGISTER_CLASSES`-Eintrag (`src/gate/commit_gate.rs`) + Kanon-Wort (Architektur-Akt).
- **Braucht:** `REGISTER_CLASSES`-Eintrag + `phi/ci_measurements.φ`-Skelett + `fold`-Job in `field-te-query.yml` (Pilot); das Verdikt liegt vor (Rat).

### M10-EDR/Ephemeride + DSC 0119 (mountain-folge325)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine M10-Ephemeriden-`url`/`origin`/`compiler`-Zeile steht
- **Lage:** (gemessen 2026-10-12) mountain-folge325 bittet um eine breite M10-Ephemeride (JPL Horizons `-76` via `horizons_compiler.rs` oder `ser.im-ldi.com/M10/MVM/`) und eine `url`-Zeile für `spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/renamed_dataset_catalogs/mariner4_celestial-mechanics_64-077a-09a_DSC_0119.pdf`.
- **Blockade:** keine.
- **Braucht:** `url`/`origin`/`compiler`-Zeilen in `phi/sources.φ` (Mycelium), dann `terms`/`format` (Mountain).

### MCP no-leak · de441-de442-sha · SWPC/RTSW · Domain→UCD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart · Merge-Lauf (`phi:4321-4369`) · `matrix-vlies`-Lauf · messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11/12) MCP: Allowlist konsistent, Live-Seat-Restart offen; de441/de442: sha256 erst nach Merge-Lauf; SWPC/RTSW: `matrix-vlies`-Wiederholung (River); Domain→UCD: keine messbare Quelle.
- **Blockade:** je der genannte Lauf/Quelle.
- **Braucht:** je der genannte Schritt (`ci_manage log <id>`; Merge-Lauf; messbare Domain-Quelle).

### wy-max-t-Artefakt — Werte im Bestand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`
- **Lage:** (gemessen 2026-10-12, River-196) α=0.05, B=9999, (1-α)-Quantil 2.4467/2.4433/2.4831 (Artefakt `38166787414`); die Intervall-Konstruktion ist benannt (studentisiertes Moving-Block-Bootstrap).
- **Blockade:** Siegel (Operator).
- **Braucht:** die gemessenen Werte ins Register/Artefakt (Mycelium); Siegel (Operator).

### ui-seats-Träger (`docs/concepts/ui-seats.md`) — offene `pending`-Seats
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Stimmen-Round in `mycelium-ui` (`open-weight-ui`-Lock) läuft
- **Lage:** (gemessen 2026-10-12) `register_lookup --orphan-docs` = 0; die `pending`-Seats Sakana-Tempo · SciSpace (Login-Wall) · Gemini Notebook (kein freier Composer) sind noch nicht gemessen.
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
