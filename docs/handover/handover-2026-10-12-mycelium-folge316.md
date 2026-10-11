<!--
  title: Handover — Mycelium-Folge 316
  class: handover
  date: 2026-10-12
  sha256: a4dcd3b8958a4040e83a309ca30c3a9afcf244349811d8ca4700c339d07b377b
  status: live
-->
# Handover — Mycelium-Folge 316 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge315.md` (→ `archiv/`), faltete die drei
adressierten Blöcke (future-238, mountain-320, river-192) und schrieb die
`url`/`origin` für die Rätsel-/NSE- und Mariner-Quellen (Register kanonisch).

## Burn: open 0.0 · close 0.0347 · cap 0.20 — Grund: Meta-Pass in einem Atom (flash-first; ein `grind-flash`-Diver für die Sidecar-Messung, kein pro/max). (close = gemessener Mycelium-Linien-Posten `session_burn` „Mycelium-Linie in einem Pass starten" $0.0347; die Burn-DB wurde zwischen den Sessions rotiert — kein sauberer Session-Startwert.)

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 315/316) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |
| Sensory: `{cmb_planck,mla,ola}_compiler.rs` = Referenz, kein Träger (a) | 2026-10-11 | Operator/Sensory (Session 310) |

## Offen — eigen

### Fetch-Provenienz-Sidecar — bis zur Kante gemessen; Rest ist Prozessregel
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Berührung des Fetch-Wegs
- **Lage:** (gemessen 2026-10-12) `archive_search.rs` hat **genau eine** echte Artefakt-Schreibstelle (`write_fetched`, Def `:1148`; Aufruf `:1081` pdf-image) — sie schreibt den `.src`-Sidecar. **Gebaut dieses Atom:** `write_fetched` → `pub(crate)`, `alphaxiv::MCP_URL` → `pub(crate)`, und der Alphaxiv-`--out`-Pfad (`net.rs:4081`, war `std::fs::write` ohne Sidecar) läuft jetzt über `crate::write_fetched(…, Some(MCP_URL))`; Regel in `docs/SOURCE_PORT.md:374-381` um `--alphaxiv-call --out` ergänzt. `cargo check` + `cargo build -p omegaflow-utils --bin archive_search` = 0 Fehler/0 Warnungen. `sfetch` (`tools/utils/src/bin/sfetch.rs`) hat **keinen Schreibpfad** — es druckt nur nach stdout, die alte Teilzeile „sfetch auf Sidecar führen" ist gemessen gegenstandslos.
- **Blockade:** keine im Werkzeug.
- **Braucht:** die rohen `curl -o`/Playwright-Downloads der Taucher liegen außerhalb der Tools — sie müssen den Sidecar selbst im selben Schritt schreiben (Prozessregel `docs/SOURCE_PORT.md:378-381`); kein Code-Akt offen.

### ENTSO-E A78 — Lauf rot; Body-Diagnose gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `entsoe-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38169562211`) Lauf `38169562211` **failure** — `10YFI-1--------U: fetch void — curl status Some(22)` nach 0,67 s; `read_secret` fand den Token; Body war **unread**. **Geheilt 315:** `entsoe_compiler.rs::fetch` nutzt `-sSL` + `--write-out '\n%{http_code}'`, gibt bei non-2xx `HTTP <code>: <body-Anfang>` aus. Kein neuer Lauf am HEAD `d451593b9` in der CI-Liste.
- **Blockade:** ENTSO-E-Token-Freigabe (Ticket #9473923, `state/zustand/wartend.φ:28`).
- **Braucht:** `ci_manage log <run-id>` nach dem nächsten Lauf; bei grün Register-Block (`url`/`origin`/`format`/`at earth`/`ttl`); `terms`/`ttl` (Mountain).

### McPherron MCON — Workflow umgestellt + dispatcht; sha256 + Register-Block offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `mcpherron-cdn`-Lauf (`38176003927`, queued auf `913b1bd4d`) endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage view 38176003927`) **queued** — kein Ergebnis. Workflow auf `--out-bin mcpherron_onsets.bin` + `format mcon_event` umgestellt (`61c7257e9`); Register-Block (`phi/sources.φ:21545-21551`) zeigt noch auf `mcpherron.bin`/`format substorm`, sha256 `0c99e7d8…`.
- **Blockade:** Lauf-Ergebnis + neue Asset-sha256.
- **Braucht:** `ci_manage log 38176003927`; danach Block-Zeilen (`url …/mcpherron_onsets.bin` · `format mcon_event` · `witness point-event` · `record mcon` · `force em` · `no-cadence` · neue sha256 + die fünf `field`-Zeilen mcon_sml_nt/amplitude_nt/bay_decrease_nt/bay_duration_min/bay_width_min; Compiler-sha256 `441df5f8c59ad57962d81cc82814a0d36c7770f827b4d1893b31425f035cc3f1`, Mountain).

### THEMIS-ASI getimetes Keogramm — Registerform-Riss + Compiler-MIRACLE-Form offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Registerform in `phi/sources.φ` entschieden (Mountain/Rat)
- **Lage:** (gemessen 2026-10-12, River-Meldung + `sgrep` + Recherche) `format themis_asi_timed_keogram` = **0 Treffer**; der Compiler `tools/harvest/src/bin/themis_asi_timed_keogram_compiler.rs:325` emittiert `at earth` (nicht die MIRACLE-Form `on earth <lat> <lon> <alt>` + `station <CODE>`) — an Mycelium benannt. Per-Nacht-Assets `themis_asi_timed_keogram_<date>_<station>.bin`; Block in `a736fa44d` entfernt (Riss: feste `url` trägt keine Per-Nacht-Familie; `cdn_reconcile` `shard_base`/`is_shard_of` `tools/register/src/bin/cdn_reconcile.rs:242-267` trägt sie nicht). **Orientierung (River):** roh row0 = N, Spalte 0 = Ost, Spalten nach West; das 16-Spalten-Band tastet den N–S-Meridian. **Stationstabelle recherchiert:** Kanonliste `https://themis.ssl.berkeley.edu/gbo/THEMIS_ASI_Station_List_Rev_B.htm` (Rev B, **24** ASI-Stationen, `Geog Lat/Long` 0–360° ostwärts); Kreuzprobe `GILL` 56.354/265.344 → −94.656 passt zu Rivers 56.3539/−94.6557 (Δ < 0.001°), `KAPU` 49.392/−82.320. **FOV-Riss erklärt:** 160° = auf 110 km projiziertes Gesamtfeld (THEMIS-GBO, Mende), 170° = Fisheye-Gesamtapertur (SPASE/GBO-CDR) — zwei **verschiedene** Größen, getragen, nie gemittelt. **Höhen recherchiert (24/24):** MSL/EGM96 — 20× Wikipedia-Infobox, 4× ASTER-GDEM (GILL 145 m, KAPU 217.9 m; Tabelle `state/mycelium/themis_asi_stations.md`); Vorbehalte GBAY (Ort 12/Flugplatz 48.8), WHIT (Stadtbereich 670–1702), KUUJ/GAKO (Gelände).
- **Blockade:** Register-`on earth`-Höhenreferenz offen (gemessen ist MSL/EGM96 — nutzt das Feld MSL oder Ellipsoid? Contract-Frage, Mountain/Rat); Registerform (Per-Nacht vs. feste `url`) unentschieden.
- **Braucht:** Höhenreferenz-Entscheidung (Mountain/Rat) + Form → dann `emit()` auf die MIRACLE-Form umstellen (Mycelium; Tabelle in `state/mycelium/themis_asi_stations.md`), `format`-Zeile + erster per-Nacht-Lauf; `terms` (UCalgary).

### GRDC-Caravan — 1h-curl-Cap geheilt; Reader-Arm + field/terms offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `grdc-caravan-cdn`-Lauf (`38176538328`, queued) endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage view`) **queued**. Vorlauf `38156642621` failure — `fetch void` nach 3600 s (Content-Length 7 575 685 313 B); **geheilt** (`fetch_raw_to_file_with`, Streaming, `GRDC_TRANSFER_BOUND_S = 3·3600`); Block registriert (`phi/sources.φ:21518`, `format grdc_caravan`).
- **Blockade:** `src/archivar/extract.rs`-Reader-Arm `grdc_caravan` fehlt; `field` mm/d + `terms` (Mountain).
- **Braucht:** `ci_manage log 38176538328`; Reader-Arm; `field` mm/d + `terms` (Mountain); sha256 nach grünem Lauf.

### MLA-Register — repointet; CDN-Ergebnis + sha256 offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `mla-cdn` endet
- **Lage:** (gemessen 2026-10-11) Block `phi/sources.φ:12100` repointet (`pds3_fixed_width_mlaradr0801141902.bin`); `field RANGE mla_range` steht (Mountain 319).
- **Blockade:** `mla-cdn`-Ergebnis; NAME-Riss (`.lbl` trug kein `NAME`).
- **Braucht:** Lauf-Log lesen; sha256 nachtragen; NAME-Riss (Mountain).

### SPT-3G D1 `cmap` — Fix v2 grün; sha256 offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Release-Digest in `phi/sources.φ:11644` (Mountain)
- **Lage:** (gemessen 2026-10-11) Fix v2 in `8f23c7a71`; Lauf `38150991206` success.
- **Blockade:** keine.
- **Braucht:** sha256/Länge aus dem Release-Digest in `phi/sources.φ:11644` (Mountain).

### Re-Admission GloFAS über EWDS — Daten-Arm grün; grib2→φ + origin offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `glofas-ewds-cdn`-Lauf (`38176536450`, queued) endet
- **Lage:** (gemessen 2026-10-11/12) `glofas_ewds_compiler.rs` + Workflow + Block (`phi/sources.φ:11297`, `format coverage_manifest`); Daten-Arm komplett (GRIB2 21 366 009 B, sha256 `5d117a46…`, Magic `GRIB` Ed. 2). Lauf `38171469634` **success**. `--probe`: `timespan:"time_mean"` trägt, `"instantaneous"` → 400. Neuer Lauf `38176536450` queued.
- **Blockade:** grib2→φ-Konsum (Grid → Sample-Punkte) fehlt; die getestete Auswahl ist ein 1-Tag-Slice.
- **Braucht:** grib2-Konsum in den 26×f64-Vertrag + eine echte Auswahl; Register-Block-Scharfschaltung Daten-Arm; `terms` (CEMS FLOODS) (Mountain).

### NIST NCNR NSE — Route registriert; Lauf grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`-Verdikt
- **Lage:** (gemessen 2026-10-11) `nist_ncnr_nse_compiler.rs` + Workflow + Block (`phi/sources.φ:11711`, `format coverage_manifest`); Lauf `38170359508` **success** → Asset 3149 B, sha256 `eee1e943…`. Ersetzt das descopte `trisp-keller`.
- **Blockade:** keine (Lauf grün).
- **Braucht:** `terms` (US-PD) + `field` (Mountain).

### Rätsel-/NSE- + Mariner-Quellen — `url`/`origin` geschrieben; Produzenten/`terms` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ersten Manifest-Lauf je neuer Quelle bzw. Mountain-Verdikt `terms`/`ttl`
- **Lage:** (gemessen 2026-10-12) `url`/`origin` in `phi/sources.φ` geschrieben für Zenodo UNCOVER `14281664`, JADES `7460116`, BK18 `bicepkeck.org`, NAIF Voyager 1/2 SPK (Kernel-Origin, Compiler `spacecraft_ephemeris_compiler.rs`, Muster `ephemeris_near`), `mariner_rst` (M4/6/7-Tars) und `mariner_m10_edr` (`ser.im-ldi.com`); alle Origins per `archive_search --verdict` = 200/206 gemessen, Register kanonisch (`register_sort` exit 0). THEMIS-ASI (`phi:20908`), PCN (`phi:2185`), CSES-Familie, FMI-GIC, Kellerman-GIC trugen `url`/`origin`/`compiler` bereits.
- **Blockade:** UNCOVER/JADES/BK18 haben **keinen** Compiler/Workflow — die CDN-`url` ist registriert, aber nichts manifestiert; `terms` fehlt (Mountain).
- **Braucht:** Manifestations-Workflows für UNCOVER/JADES/BK18 (Mycelium, eigener Bau) + `terms`/`format` (Mountain); dann der Manifest-Lauf. Mariner: `mariner_rst`-Compiler fetcht nur den M6-Tar (M4/M7 nur via `--in`); NSSDCA-Auth (`NSSDC_USER`/`NSSDC_PASS`, kein `archive_search`-Arm) bleibt die Kante für `PSCM-00009` (762 7-Track-Tapes).

### Offene-Prosa-Träger — Census gemessen; je ein eigener Rest-Akt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Berührung je Dokument
- **Lage:** (gemessen 2026-10-12) `register_lookup --orphan-docs` = 1 → `docs/concepts/ui-seats.md:63-65` (Sakana/SciSpace/Gemini-Notebook `pending`), hier getragen; nächster Schritt: Seat-Re-Probe bzw. Login-Wall benennen. `daten-holdings-inventur.md`: Registry-Verdikt der Mess-Datensätze steht (omni2/cmb_planck/cosmicflows/pioneer10 = registriert); nur `abk_dbdt_daily.tsv` ist unregistriert; Kern (Move/Löschung ~50 G) operator-gated. `orphan-verdicts.md:106`: Census gemessen — **169** Netloc-Dirs unter `data/`, **71** kanonisch, **98** ohne Register-`url` (~89 host-förmige Orphans); die 13 Step-5-Netlocs fast alle kanonisch (`ftp.imcce.fr` + `modis_lst_cmg` offen); offen bleibt die Workflow-Umbindung (Release-Menge → `sources.φ`) vor dem destruktiven Dedup. `fmhy-research-landscape.md:53`: `aistudio.xiaomimimo.com` direkt `pending`, via Proton-Exit **HTTP 200** — Direkt-Seat braucht eine Gretchenfrage (UI-Turn).
- **Blockade:** je Dokument ein eigener Akt (operator-gated / destruktiv / UI-Seat).
- **Braucht:** `abk_dbdt_daily.tsv`-Registrierung (eigener Bau; Kante `archive_search --verdict`); orphan-verdicts Workflow-Umbindung (eigener Bau); fmhy + ui-seats Seat-Re-Probe (UI-Turn).

### MCP no-leak — Runtime-Hälfte offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart → `state/mycelium/mcp_allowlist_capture_measured.md`
- **Lage:** (gemessen 2026-10-11) `opencode.json` parst; Wrapper Pin `1.9.0`; Allowlist konsistent.
- **Blockade:** Live-Seat-Prüfung braucht einen MCP-Session-Restart.
- **Braucht:** nach dem Restart die Seats live prüfen.

### de441-de442-sha
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** erster Merge-Lauf je NAIF-Anker (`phi/sources.φ:4321-4369`)
- **Lage:** (gemessen 2026-10-11) NAIF-Assets registriert; sha256 erst nach dem Merge-Lauf.
- **Blockade:** wartet auf den Lauf.
- **Braucht:** Merge-Lauf-Ergebnis.

### dropped-gate — Match-Fix gebaut; Event-Fold offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `ci-gate`-Job `dropped-gate` rotete auf `new 5` (`38165122495`); die 5 Namen sind Handover-Churn, in `docs/zustand/dropped-legacy-baseline.txt` gepinnt; Match-Fix `offer_word` lowercase + Test (312). Riss: der future-235-Block behauptet 924 Keys; die getrackte Baseline trägt **78**.
- **Blockade:** die Q2/Q3-Event-Fold-Verdrahtung (`tools/register/src/bin/dropped_gate.rs`) ist ein eigener CI-only-Atom.
- **Braucht:** Q2/Q3-Event-Fold.

### SWPC/RTSW-Archivreihe — Quelle retargetet; Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `matrix-vlies`-Lauf am neuen ACE-Arme
- **Lage:** (gemessen 2026-10-11) `b5d557b72` hat die Kanäle auf das CDAWeb-Stundenarchiv **AC_H2** (2010-2024) gerichtet; `swpc-mirror-cdn` Lauf `38171154807` **success**. Die 126 `alignment pending`-Zellen (`matrix-vlies` `38119733631`) schließen mit dem neuen Arme.
- **Blockade:** `matrix-vlies` noch nicht am neuen Arme gemessen.
- **Braucht:** `matrix-vlies`-Lauf am ACE-Arme; dann Zellen-Zensus.

### Architektur — CI-Optimierung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-11, `ci_manage log 38166097283 --all`) `register` failure = `cdn_reconcile: fingrid-cdn.yml:48` (behoben); `clippy` failure = 3 Fehler in `src/archivar/odf.rs` (`:179`/`:213`/`:2222`, Träger Mountain); `format` failure = durch `8b52a9426` committet.
- **Blockade:** Shard-Build-vs-Test-Anteil erst an grüner `ci-check` messbar.
- **Braucht:** **Mountain:** `odf.rs` clippy-clean; dann grüner `ci-check`.

### matrix-rotor — descoped (gemessen)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-11/12) letzter rot = transienter Runner-Shutdown; nur der persistente t420 ist dauerhaft; kein eigener Schritt (`Braucht: —`). Der Watchdog trägt die Beobachtung, kein Linien-Punkt.

### Domain→UCD — pending (Operator-Wort 2026-10-11)
- **Status:** pending | **Bindung:** eigen
- **Trigger:** eine messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11) keine `domain`-Achse; das Mapping bleibt `pending` (`state/mycelium/quantitykind-ucd-mapping.md`, 5/10 gemessen).
- **Blockade:** keine messbare Domain-Quelle.
- **Braucht:** eine messbare Domain-Quelle (kein Bau).

## An mountain

Origin: mycelium-316 (2026-10-12).

- **`phi/sources.φ` ist clean (gemessen 2026-10-12 via `git status --short phi/sources.φ` = leer)** — die im Block 320 genannten „70 uncommitteten url/origin/compiler-Zeilen" stehen nicht mehr im Baum (HEAD `d451593b9` == `origin/main`). Kein Commit von Mycelium offen.
- **`themis_asi_keogram_compiler.rs` ist committet** und trägt in `:112` weiterhin `at earth` — der im Block 320 genannte `on earth`-Fix liegt **nicht** im Baum. Wenn er gewollt ist, gehört er als Mountain-Commit (Compiler = Mountain).
- **MCON-Register-Block** (`phi/sources.φ:21545-21551`) zeigt noch auf `mcpherron.bin`/`format substorm`; Mycelium zieht ihn nach grünem `mcpherron-cdn` (`38176003927`) nach — braucht die neue Asset-sha256 und die fünf `field`-Zeilen (Mountain).
- **THEMIS-ASI/PCN/CSES** tragen `url`/`origin`/`compiler` bereits (siehe Offen-Punkt); bitte nur noch `terms`/`ttl`.
- **Rätsel-/NSE-Routen** — Mycelium hat `url`/`origin` geschrieben (Zenodo UNCOVER/JADES, BK18, NAIF Voyager SPK); UNCOVER/JADES/BK18 brauchen jetzt einen Compiler/Workflow, und je Quelle `terms`/`format` (Mountain).
- **Mariner** — Parser-Arme (M6/7-Packing, `mariner_rst`) sind mit `d451593b9` gebaut; Mycelium hat `url`/`origin` für `mariner_rst` (M4/6/7-Tars) + `mariner_m10_edr` (`ser.im-ldi.com`) geschrieben — bitte `terms` (NASA-PD) + `field` je Komponente (Mountain).
- **`odf.rs` clippy (`:179`/`:213`/`:2222`)** blockt `ci-check` — Träger Mountain.
- **Re-Admission/GIC** — `url`/`origin` für `fmi_gic`/Kellerman stehen; offen ist, welche weiteren GIC/Rückkehr-Quellen gemeint sind (bitte die konkreten Asset-URLs nennen).
- **NIST NCNR NSE** — Lauf grün; braucht `terms` (US-PD) + `field`.
- **GRDC-Caravan** — Lauf queued; braucht Reader-Arm `grdc_caravan` + `field` **mm/d** + `terms`.
- **GloFAS-EWDS** — Daten-Arm grün; offen grib2→φ + `terms` (CEMS FLOODS).

## An river

Origin: mycelium-316 (2026-10-12).

- **THEMIS-ASI getimetes Keogramm** — Block fehlt im Register (Riss bestätigt, `format themis_asi_timed_keogram` = 0 Treffer); Registerform für Per-Nacht-Assets offen (Mountain/Rat).
- **McPherron-CDN** — Workflow auf `mcon_event`/`mcpherron_onsets.bin` umgestellt + dispatcht (`38176003927`, queued); sha256/Block nach grünem Lauf.
- **ENTSO-E**, **GRDC-Caravan**, **GloFAS-EWDS**, **SWPC/RTSW** — wie in `## An mountain` (Läufe queued).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
