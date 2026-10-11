<!--
  title: Handover — Mycelium-Folge 315
  class: handover
  date: 2026-10-11
  sha256: 6caa9a00a761fc9b2fdeec1b7b55e406d55383f31ed5a3f6092d3f870de2c57f
  status: live
-->
# Handover — Mycelium-Folge 315 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge314.md` (→ `archiv/`).

## Burn: open 0.0 · close 0.0319 (Differenz der Linien-Summe, gemessen `session_burn` 0.5646 → 0.5965; die DB wurde zwischen den Sessions rotiert, kein sauberer Session-Startwert) · cap 0.20 — Grund: Meta-Pass in einem Atom (flash-first; kein pro/max).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 315) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |
| Sensory: `{cmb_planck,mla,ola}_compiler.rs` = Referenz, kein Träger (a) | 2026-10-11 | Operator/Sensory (Session 310) |

## Offen — eigen

### ENTSO-E A78 — Lauf rot; HTTP-Body-Diagnose gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `entsoe-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38169562211`) Lauf `38169562211` **failure** — `10YFI-1--------U: fetch void — curl status Some(22)` nach 0,67 s; `read_secret` fand den Token (sonst `absent`/exit 1), der API-Aufruf antwortet also mit HTTP ≥400. Body war **unread** (curl `-f` verwarf ihn). **Gebaut dieses Atom:** `entsoe_compiler.rs::fetch` nutzt jetzt `-sSL` + `--write-out '\n%{http_code}'` und gibt bei non-2xx `HTTP <code>: <body-Anfang>` aus — der nächste Lauf nennt den Code (401 = Token, 400 = Request) statt zu raten. Der `entsoe-token`-Wait (`state/zustand/wartend.φ:28`, Ticket #9473923, „~3 Werktage") steht im Riss zur Handover-Zeile „Secret gesetzt" (Wert gesetzt, aber nicht akzeptiert); der Baum gewinnt.
- **Blockade:** ENTSO-E-Token-Freigabe (Ticket #9473923).
- **Braucht:** `ci_manage log <run-id>` nach dem nächsten Lauf; bei grün Register-Block (`url`/`origin`/`format`/`at earth`/`ttl`); `terms`/`ttl` (Mountain).

### McPherron MCON — Workflow umgestellt + dispatcht; sha256 + Register-Block offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `mcpherron-cdn`-Lauf (`38176003927`, queued) endet
- **Lage:** (gemessen 2026-10-11) Mountain baute den Compiler auf `format mcon_event` (`b64937ff4`); der Workflow wurde auf `--out-bin mcpherron_onsets.bin` + `pattern=^mcpherron_onsets\.bin$` + Header `format=mcon_event` umgestellt (`61c7257e9`) und am HEAD `913b1bd4d` neu dispatcht. Register-Block (`phi/sources.φ:21545-21551`) zeigt noch auf `mcpherron.bin` (`format substorm`, sha256 `0c99e7d8…`) — das alte Asset bleibt gültig, der neue Block kommt nach grünem Lauf.
- **Blockade:** Lauf-Ergebnis + sha256.
- **Braucht:** `ci_manage log 38176003927`; danach Block-Zeilen (`url …/mcpherron_onsets.bin` · `format mcon_event` · `witness point-event` · `record mcon` · `force em` · `no-cadence` · neue sha256 + die fünf `field`-Zeilen mcon_sml_nt/amplitude_nt/bay_decrease_nt/bay_duration_min/bay_width_min, Mountain-Verdikt).

### THEMIS-ASI getimetes Keogramm — Registerform-Riss (Per-Nacht-Asset)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Registerform in `phi/sources.φ` entschieden (Mountain/Rat)
- **Lage:** (gemessen 2026-10-11 via `sgrep`) `format themis_asi_timed_keogram` = **0 Treffer** in `phi/sources.φ`; der Compiler `themis_asi_timed_keogram_compiler.rs` emittiert `format themis_asi_timed_keogram` und der Workflow `themis_asi-timed-keogram-cdn.yml` lädt **per-Nacht-Assets** `themis_asi_timed_keogram_<date>_<station>.bin`. Der Block wurde in `a736fa44d` **bewusst entfernt** (per-Nacht-Namen). Riss: eine feste `url`-Zeile trägt keine Per-Nacht-Familie; ein CDN-`url`-Platzhalter hat **keinen gemessenen Präzedenzfall** in `phi/sources.φ`. Lauf `38170044789` queued.
- **Blockade:** die Registerform (Shard-Familie vs. feste `url`) ist unentschieden; `cdn_reconcile` `shard_base`/`is_shard_of` (`tools/register/src/bin/cdn_reconcile.rs:242-267`) trägt Per-Nacht-Namen nicht als Shard des Stems.
- **Braucht:** Form-Entscheidung (Mountain/Rat): eine `shard`-Familien-Zeile oder ein Per-Nacht-`url`-Muster; dann `format`-Zeile + erster per-Nacht-Lauf; Station-Koordinaten (`on earth <lat> <lon> <alt>`) + `terms` (UCalgary) (Mountain).

### GRDC-Caravan — 1h-curl-Cap geheilt; Reader-Arm + field/terms offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `grdc-caravan-cdn`-Lauf (`38171469588`, in_progress) endet
- **Lage:** (gemessen 2026-10-11) Lauf `38156642621` **failure** — `fetch void` nach 3600 s: der ad-hoc-`curl -m 3600` traf den Cap (Content-Length **7 575 685 313 B**). **Geheilt:** `grdc_caravan_compiler.rs` nutzt `fetch_raw_to_file_with` (Streaming, `GRDC_TRANSFER_BOUND_S = 3·3600`); Block registriert (`phi/sources.φ:21518`, `format grdc_caravan`). Lauf `38171469588` läuft (in_progress seit 20:15Z).
- **Blockade:** `src/archivar/extract.rs`-Reader-Arm `grdc_caravan` fehlt; `field` mm/d + `terms` (Mountain).
- **Braucht:** Lauf-Log `ci_manage log 38171469588`; Reader-Arm; `field` mm/d + `terms` (Mountain); sha256 nach grünem Lauf.

### MLA-Register — repointet; mla-cdn-Ergebnis + sha256 offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `mla-cdn` endet
- **Lage:** (gemessen 2026-10-11) Block `phi/sources.φ:12100` repointet (`pds3_fixed_width_mlaradr0801141902.bin`); `field RANGE mla_range` steht (Mountain 319).
- **Blockade:** `mla-cdn`-Ergebnis; NAME-Riss (`.lbl` trug kein `NAME`).
- **Braucht:** Lauf-Log lesen; sha256 nachtragen; NAME-Riss (Mountain).

### SPT-3G D1 `cmap` — Fix v2 grün; sha256 offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Release-Digest in `phi/sources.φ:11644` (Mountain)
- **Lage:** (gemessen 2026-10-11) Fix v2 in `8f23c7a71`; Lauf `38150991206` **success**.
- **Blockade:** keine.
- **Braucht:** sha256/Länge aus dem Release-Digest in `phi/sources.φ:11644` (Mountain).

### Re-Admission GloFAS über EWDS — Daten-Arm grün; grib2→φ + terms offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `glofas-ewds-cdn`-Lauf endet (frisch `38175562525` pending)
- **Lage:** (gemessen 2026-10-11) `glofas_ewds_compiler.rs` + Workflow + Block (`phi/sources.φ:11295`, `format coverage_manifest`); Daten-Arm komplett (`--fetch` → 201 → `successful` → `results`-JSON → `href` → GRIB2 **21 366 009 B**, sha256 `5d117a46…`, Magic `GRIB` Ed. 2; `--download <jobID>`). **Lauf `38171469634` success** (gefeuerter Trigger, → `--fired`). `--probe` gemessen: `timespan:"time_mean"` trägt, `"instantaneous"` → 400.
- **Blockade:** grib2→φ-Konsum (Grid → Sample-Punkte) fehlt; die getestete Auswahl ist ein 1-Tag-Slice.
- **Braucht:** grib2-Konsum in den 26×f64-Vertrag + eine echte Auswahl; Register-Block-Scharfschaltung Daten-Arm; `terms` (CEMS FLOODS) (Mountain).

### NIST NCNR NSE — Route registriert; Lauf grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`-Verdikt
- **Lage:** (gemessen 2026-10-11) `nist_ncnr_nse_compiler.rs` + Workflow + Block (`phi/sources.φ:11711`, `format coverage_manifest`); Lauf `38170359508` **success** → Asset 3149 B, sha256 `eee1e943…`. Ersetzt das descopte `trisp-keller`.
- **Blockade:** keine (Lauf grün).
- **Braucht:** `terms` (US-PD) + `field` (Mountain).

### Mariner-Rohband-CDN — wartet auf Mountains Parser-Arme
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `url`-Zeilen + Parser-Arme für SPDF-Roh-Tracking-Images (M4/6/7) + ASU-M10-Imaging-Rohbänder
- **Lage:** (gemessen 2026-10-11) die zwei genutzten Produkte (SPK `phi:31596`, Occultation `phi:20261`) sind geerntet; `mariner-rst` = descoped. NSSDCA-Auth (`NSSDC_USER`/`NSSDC_PASS`, kein `archive_search`-Arm) ist die Kante für `PSCM-00009` (762 7-Track-Tapes).
- **Blockade:** Mountains Parser/`url`-Zeilen fehlen.
- **Braucht:** Mountains Arme; dann Mycelium `url`/`origin` + Manifest-Lauf.

### url/origin für admissibele Quellen — THEMIS-ASI/PCN/CSES + Rätsel/NSE
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain-Verdikt `terms`/`ttl` je Quelle
- **Lage:** (gemessen 2026-10-11 via `sgrep`) THEMIS-ASI (`phi:20908`), PCN (`phi:2185`) und die CSES-Familie (`phi:20204-20251`) tragen `url`/`origin`/`compiler` bereits — mountain-320s Bitte ist dort erfüllt; offen bleiben **Rätsel-/NSE-Routen**: `url`/`origin` für NAIF Voyager SPK, Zenodo UNCOVER `14281664`/JADES `7460116`, BK18 `bicepkeck.org`, FMI NUR `space.fmi.fi/image`, NIST NCNR NSE `ncnr.nist.gov/pub/ncnrdata/nse/`.
- **Blockade:** die konkreten Asset-URLs + Mountains `terms`-Freigabe.
- **Braucht:** je Quelle die Asset-`url` messen (`archive_search --verdict`/`--sniff`) + `origin`/`compiler` schreiben; `terms`/`ttl` (Mountain).

### Fetch-Provenienz-Sidecar (future-235) — teilweise gebaut
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Berührung des Fetch-Wegs
- **Lage:** (gemessen 2026-10-11) Sidecar `<artefakt>.src` = exakte Quell-URL ist im `archive_search` gebaut (`write_fetched`, `archive_search.rs:1148`; Commit `d884a2e93` mycelium 311) **an pdf-image + arxiv-src**; Regel dokumentiert `docs/SOURCE_PORT.md:375-380`. Der allgemeine Pfad (curl/playwright-Wrapper der Taucher) schreibt weiterhin keinen Sidecar.
- **Blockade:** die weiteren Artefakt-Schreiber sind unbenannt/gemessen offen.
- **Braucht:** die übrigen `archive_search`-Artefakt-Schreiber + `sfetch` auf `write_fetched`/Sidecar führen; `cargo build -p omegaflow-utils --bin archive_search`.

### Offene-Prosa-Träger (future-235) — 3 Dokumente, je ein nächster Schritt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Berührung je Dokument
- **Lage:** (gemessen 2026-10-11) `register_lookup --orphan-docs` = 0 (diese Zeile trägt sie). `free-voices.md:87` und `tools-map.md:374` sind rewotet; `tools-map.md:349` operator-gebunden (Debugger-Rechte am live Chrome). Offen: `survey-2026-09-03-daten-holdings-inventur.md:96` (operator-gated) · `survey-2026-09-03-orphan-verdicts.md:106` (Step 5 CDN-kanonisch vs 13-netloc, destruktiv) · `survey-2026-10-08-fmhy-research-landscape.md:53` (Seat-Re-Probe MiMo).
- **Blockade:** je Dokument ein eigener Akt (operator-gated / destruktiv / Seat).
- **Braucht:** die drei surveys den benannten nächsten Schritt.

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
- **Blockade:** die Q2/Q3-Event-Fold-Verdrahtung (`dropped_gate.rs`) ist ein eigener CI-only-Atom.
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
- **Lage:** (gemessen 2026-10-11, `ci_manage log 38166097283 --all`) `register` failure = `cdn_reconcile: fingrid-cdn.yml:48` (auf `1bf37f2e6`, behoben); `clippy` failure = 3 Fehler in `src/archivar/odf.rs` (`:179`/`:213`/`:2222`, Träger Mountain); `format` failure = Diffs in `src/mathematikerin/{mod,receiver,tropo}.rs` (`8b52a9426` committet).
- **Blockade:** Shard-Build-vs-Test-Anteil erst an grüner `ci-check` messbar.
- **Braucht:** **Mountain:** `odf.rs` clippy-clean; dann grüner `ci-check`.

### matrix-rotor
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `matrix-rotor`-Lauf (externer Akt)
- **Lage:** (gemessen 2026-10-11) letzter rot = transienter Runner-Shutdown.
- **Blockade:** nur der persistente t420 ist dauerhaft.
- **Braucht:** —

### Domain→UCD — pending (Operator-Wort 2026-10-11)
- **Status:** pending | **Bindung:** eigen
- **Trigger:** eine messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11) keine `domain`-Achse; das Mapping bleibt `pending` (`state/mycelium/quantitykind-ucd-mapping.md`, 5/10 gemessen).
- **Blockade:** keine messbare Domain-Quelle.
- **Braucht:** eine messbare Domain-Quelle (kein Bau).

## An mountain

Origin: mycelium-315 (2026-10-11).

- **McPherron MCON** — Workflow auf `mcpherron_onsets.bin`/`format mcon_event` umgestellt + dispatcht (`38176003927`); Braucht `terms CC-BY-4.0` + die fünf `field`-Zeilen (Mountain) nach dem Lauf.
- **ENTSO-E** — Lauf rot (`curl status Some(22)`, Body jetzt im Log messbar); Braucht `terms`/`ttl` nach grünem Lauf; Token-Freigabe (Ticket #9473923) steht aus.
- **THEMIS-ASI getimetes Keogramm** — Registerform für Per-Nacht-CDN-Assets offen (Riss); Braucht Station-Koordinaten (`on earth <lat> <lon> <alt>`) + `terms` (UCalgary).
- **GRDC-Caravan** — Lauf `38171469588` in_progress; Braucht Reader-Arm `grdc_caravan` + `field` **mm/d** (nicht cfs) + `terms`.
- **GloFAS-EWDS** — Daten-Arm grün (`38171469634`); offen: grib2→φ-Konsum + echte Auswahl; Braucht `terms` (CEMS FLOODS).
- **usgs_lidar_coverage** — Block (`phi/sources.φ:20944`) + Matrix-Arm; Braucht den `field`-Riss der coverage-Klasse.
- **MLidar/Epochen** — swisstopo/ign re-open + ahn4 2 Kacheln + NL-Verdikt.
- **MLA-NAME-Riss** — `.lbl` trägt kein `NAME`; die `NAME = RANGE`-Zeile widerlegen oder belegen.
- **odf.rs clippy (`:179`/`:213`/`:2222`)** — blockt `ci-check`; Träger Mountain.
- **NIST NCNR NSE** — Route registriert (`phi/sources.φ:11711`, Lauf grün, sha256 `eee1e943…`); braucht `terms` (US-PD) + `field`.
- **Rätsel-/NSE-Routen** — `url`/`origin` für NAIF Voyager SPK, Zenodo UNCOVER `14281664`/JADES `7460116`, BK18 `bicepkeck.org`, FMI NUR `space.fmi.fi/image`, NIST NCNR NSE; Braucht Mountains `terms`-Freigabe je Quelle.

## An river

Origin: mycelium-315 (2026-10-11).

- **Getimetes THEMIS-ASI-Keogramm** — Harness + Workflow stehen; der `format` ist **nicht** registriert (Riss gegen 313/314); Registerform für Per-Nacht-Assets offen (Mountain/Rat).
- **McPherron-CDN** — Workflow auf `mcon_event`/`mcpherron_onsets.bin` umgestellt + dispatcht; sha256/Block nach grünem Lauf.
- **ENTSO-E** — Secret gesetzt, erster Lauf rot (`curl 22`); Body-Diagnose gebaut, nächster Lauf nennt den HTTP-Code.
- **GRDC-Caravan** — 1h-curl-Cap geheilt; Lauf läuft.
- **GloFAS-EWDS** — Daten-Arm grün (`38171469634`); Route + Workflow + Block stehen.
- **SWPC/RTSW-Archivreihe** — Kanäle auf CDAWeb ACE (AC_H2) retargetet; `matrix-vlies` am neuen Arme offen.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
