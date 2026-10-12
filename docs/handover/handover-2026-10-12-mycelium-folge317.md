<!--
  title: Handover — Mycelium-Folge 317
  session: Mycelium-Linie in einem Pass (Mariner-CDN-Manifestation, M4-Register-Riss gelöst)
  class: handover
  date: 2026-10-12
  sha256: 77f381b840583854d5d5acf45a31dd386298634d2e2fd5a86661f4186a97dcfb
  status: live
-->
# Handover — Mycelium-Folge 317 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge316.md` (→ `archiv/`) und faltete die zwei
adressierten Blöcke (future-240, mountain-321). Gebaut dieses Atom: der
`mariner_rst`-Origin-Riss gelöst (M4-Tar heraus), ein eigener `mariner4_odg`-Block
(`url`/`format`/`origin`/`compiler`) angelegt und die drei fehlenden Mariner-CDN-
Workflows (`mariner4-odg-cdn`, `mariner-rst-cdn`, `mariner-m10-edr-cdn`) gebaut;
Register via `register_sort` canonical (2814 Blöcke).

## Burn: open 0.0 · close 0.055 · cap 0.20 — Grund: Meta-Pass in einem Atom (flash-first, kein Diver, kein pro/max). (close = gemessener Delta `session_burn` total `$1.4823`→`$1.5370`; die Burn-DB wurde zwischen den Sessions rotiert — kein sauberer Session-Startwert.)

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „kannst du dich bitte um deinen teil kümmern? Klare Trennung — und ich habe meinen Teil gleich gebaut." | 2026-10-12 | Operator (Session 317) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 315/316) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |
| Sensory: `{cmb_planck,mla,ola}_compiler.rs` = Referenz, kein Träger (a) | 2026-10-11 | Operator/Sensory (Session 310) |

## Offen — eigen

### Mariner-CDN-Arme — Register + Workflows gebaut; Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `mariner4-odg-cdn`/`mariner-rst-cdn`/`mariner-m10-edr-cdn`-Läufe enden
- **Lage:** (gemessen 2026-10-12) Riss gelöst: `phi/sources.φ:31390-31394` trägt den neuen `mariner4_odg`-Block (`url …/spdf.gsfc.nasa.gov/mariner4_odg.bin` · `format mariner4_odg` · `origin` beide M4-Tars · `compiler mariner4_odg_compiler.rs` · `no-cadence`); der `mariner_rst`-Origin (`:31398`) führt die M4-Tar **nicht** mehr. Drei Workflows gebaut (`.github/workflows/{mariner4-odg,mariner-rst,mariner-m10-edr}-cdn.yml`, Muster `mariner-occlt-cdn.yml`); `register_sort` canonical. `mariner_m10_edr` (`ser.im-ldi.com`) und `mariner_rst` waren registriert, aber ohne Workflow (Register-Schuld).
- **Blockade:** Lauf-Ergebnis + Asset-sha256; `terms NASA-PD/CC0` im M4-Block (Mountain).
- **Braucht:** `gh workflow run mariner4-odg-cdn.yml` (nach Push, alle drei); `ci_manage log <id>`; sha256 nach grünem Lauf; `terms` in den M4-Block (Mountain).

### ENTSO-E A78 — Lauf rot; Body-Diagnose gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `entsoe-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38169562211`) Lauf `38169562211` **failure** — `10YFI-1--------U: fetch void — curl status Some(22)`; `read_secret` fand den Token; Body war **unread**. **Geheilt 315:** `entsoe_compiler.rs::fetch` nutzt `-sSL` + `--write-out '\n%{http_code}'`. Kein neuer Lauf am HEAD in der CI-Liste.
- **Blockade:** ENTSO-E-Token-Freigabe (Ticket #9473923, `state/zustand/wartend.φ:28`).
- **Braucht:** `ci_manage log <run-id>` nach dem nächsten Lauf; bei grün Register-Block (`url`/`origin`/`format`/`at earth`/`ttl`); `terms`/`ttl` (Mountain).

### McPherron MCON — Workflow umgestellt + dispatcht; sha256 + Register-Block offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `mcpherron-cdn`-Lauf (`38176003927`, queued) endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage view 38176003927`) **queued**. Workflow auf `--out-bin mcpherron_onsets.bin` + `format mcon_event` umgestellt (`61c7257e9`); Register-Block (`phi/sources.φ:21545-21551`) zeigt noch auf `mcpherron.bin`/`format substorm`, sha256 `0c99e7d8…`.
- **Blockade:** Lauf-Ergebnis + neue Asset-sha256.
- **Braucht:** `ci_manage log 38176003927`; danach Block-Zeilen (`url …/mcpherron_onsets.bin` · `format mcon_event` · `witness point-event` · `record mcon` · `force em` · `no-cadence` · neue sha256 + fünf `field`-Zeilen mcon_sml_nt/amplitude_nt/bay_decrease_nt/bay_duration_min/bay_width_min; Compiler-sha256 `441df5f8c59ad57962d81cc82814a0d36c7770f827b4d1893b31425f035cc3f1`, Mountain).

### THEMIS-ASI getimetes Keogramm — Registerform-Riss + Compiler-MIRACLE-Form offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Registerform in `phi/sources.φ` entschieden (Mountain/Rat)
- **Lage:** (gemessen 2026-10-12) `format themis_asi_timed_keogram` = **0 Treffer**; `tools/harvest/src/bin/themis_asi_timed_keogram_compiler.rs` emittiert `at earth` (nicht die MIRACLE-Form `on earth <lat> <lon>` + `station <CODE>`) — als Mountain benannt. Per-Nacht-Assets `themis_asi_timed_keogram_<date>_<station>.bin`; Block in `a736fa44d` entfernt (Riss: feste `url` trägt keine Per-Nacht-Familie; `cdn_reconcile` `shard_base`/`is_shard_of` `tools/register/src/bin/cdn_reconcile.rs:242-267` trägt sie nicht). Stationstabelle 24/24 + `h = H + N` in `state/mycelium/themis_asi_stations.md`.
- **Blockade:** Registerform (Per-Nacht vs. feste `url`) unentschieden; Compiler-Form = Mountain.
- **Braucht:** Form-Entscheidung (Mountain/Rat) → `emit()` auf die MIRACLE-Form umstellen; `format`-Zeile + erster per-Nacht-Lauf; `terms` (UCalgary).

### GRDC-Caravan — 1h-curl-Cap geheilt; Reader-Arm + field/terms offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `grdc-caravan-cdn`-Lauf (`38176538328`, queued) endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage view`) **queued**. Vorlauf `38156642621` failure — `fetch void` nach 3600 s (Content-Length 7 575 685 313 B); **geheilt** (Streaming, `GRDC_TRANSFER_BOUND_S = 3·3600`); Block registriert (`phi/sources.φ:21518`, `format grdc_caravan`).
- **Blockade:** `src/archivar/extract.rs`-Reader-Arm `grdc_caravan` fehlt; `field` mm/d + `terms` (Mountain).
- **Braucht:** `ci_manage log 38176538328`; Reader-Arm; `field mm/d` + `terms` (Mountain); sha256 nach grünem Lauf.

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
- **Lage:** (gemessen 2026-10-11/12) `glofas_ewds_compiler.rs` + Workflow + Block (`phi/sources.φ:11297`, `format coverage_manifest`); Daten-Arm komplett (GRIB2 21 366 009 B, sha256 `5d117a46…`, Magic `GRIB` Ed. 2). Lauf `38171469634` **success**; `--probe`: `"time_mean"` trägt, `"instantaneous"` → 400.
- **Blockade:** grib2→φ-Konsum (Grid → Sample-Punkte) fehlt.
- **Braucht:** grib2-Konsum in den 26×f64-Vertrag + echte Auswahl; `terms` (CEMS FLOODS) (Mountain).

### NIST NCNR NSE — Route registriert; Lauf grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`-Verdikt
- **Lage:** (gemessen 2026-10-11) `nist_ncnr_nse_compiler.rs` + Workflow + Block (`phi/sources.φ:11711`, `format coverage_manifest`); Lauf `38170359508` **success** → Asset 3149 B, sha256 `eee1e943…`. Ersetzt das descopte `trisp-keller`.
- **Blockade:** keine (Lauf grün).
- **Braucht:** `terms` (US-PD) + `field` (Mountain).

### Architektur — `on earth alt` Bezug (Rat + Roster; Riss benannt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort bzw. Registerform-Entscheid
- **Lage:** (gemessen 2026-10-12) **aufgelöst durch Messung:** `alt` **ist** die ellipsoidische Höhe h; definitiv `h = H + N` (EGM2008, GeoidEval 2.5.2, ~1 mm RMS), 24/24 in `state/mycelium/themis_asi_stations.md` (RANK −17.15 m, SNKQ −11.22 m physikalisch). **kein Default, kein Pad**; `parse.rs`-`Option`-Pflicht entfällt (alt bleibt mandatory). Für die ASI (Quelle liefert nur MSL) ist `on earth <lat> <lon>` + `station <CODE>` kanonisch mit `alt` = h aus der festen Tabelle.
- **Blockade:** Registerform (Per-Nacht vs. feste `url`) → `emit()`.
- **Braucht:** Form-Entscheidung → `emit()` mit der festen Tabelle; WHIT bleibt `pending` (keine Punktmessung).

### Rätsel-/NSE-Routen — `url`/`origin` geschrieben; Produzenten/`terms` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ersten Manifest-Lauf je neuer Quelle bzw. Mountain-Verdikt `terms`/`format`
- **Lage:** (gemessen 2026-10-12) `url`/`origin` in `phi/sources.φ` geschrieben für Zenodo UNCOVER `14281664`, JADES `7460116`, BK18 `bicepkeck.org`, NAIF Voyager 1/2 SPK. Origins per `archive_search --verdict` 200/206.
- **Blockade:** UNCOVER/JADES/BK18 haben **keinen** Compiler/Workflow — die CDN-`url` ist registriert, nichts manifestiert; `terms` fehlt (Mountain).
- **Braucht:** Manifestations-Workflows für UNCOVER/JADES/BK18 (Mycelium, eigener Bau) + `terms`/`format` (Mountain); dann der Manifest-Lauf.

### Offene-Prosa-Träger — Census 0; je ein eigener Rest-Akt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Berührung je Dokument
- **Lage:** (gemessen 2026-10-12) `register_lookup --orphan-docs` = **0**. Reste: `daten-holdings-inventur.md` — nur `abk_dbdt_daily.tsv` unregistriert (Kern Move/Löschung ~50 G operator-gated); `orphan-verdicts.md:106` — Workflow-Umbindung (Release-Menge → `sources.φ`) vor destruktivem Dedup; `fmhy-research-landscape.md:53` — `aistudio.xiaomimimo.com` direkt `pending`, via Proton **HTTP 200**.
- **Blockade:** je Dokument ein eigener Akt (operator-gated / destruktiv / UI-Seat).
- **Braucht:** `abk_dbdt_daily.tsv`-Registrierung (eigener Bau; Kante `archive_search --verdict`); orphan-verdicts Workflow-Umbindung (eigener Bau); fmhy Seat-Re-Probe (UI-Turn).

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

### dropped-gate — Match-Fix gebaut; Event-Fold + Move-Disposition offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11/12) `ci-gate`-Job `dropped-gate` rotete auf `new 5` (`38165122495`); die 5 Namen sind Handover-Churn, in `docs/zustand/dropped-legacy-baseline.txt` gepinnt; Match-Fix `offer_word` lowercase + Test (312). **Riss:** der future-235-Block behauptet 924 Keys; die getrackte Baseline trägt **78** (future-240 nennt `--dropped-keys` + Baseline 924 keys `278ba2905` — zwei Linien, nie gemittelt).
- **Blockade:** die Q2/Q3-Event-Fold-Verdrahtung (`tools/register/src/bin/dropped_gate.rs`) + die Punkt-ID/Hash-Disposition sind je ein eigener CI-only-Atom.
- **Braucht:** Q2/Q3-Event-Fold; Disposition der 924 vs. 78 Keys (Riss benannt).

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

Origin: mycelium-317 (2026-10-12).

- **Mariner-CDN gebaut:** `mariner4_odg`-Block (`phi/sources.φ:31390-31394`) + drei Workflows (`mariner4-odg-cdn`/`mariner-rst-cdn`/`mariner-m10-edr-cdn`); der `mariner_rst`-Origin führt die M4-Tar nicht mehr. Bitte `terms` (NASA-PD/CC0) in den M4-Block und die Asset-sha256 nach grünem Lauf.
- **`themis_asi_keogram_compiler.rs`** — gemessen clean im Baum; der `on earth`-Fix ist ein Mountain-Commit (Compiler = Mountain).
- **`odf.rs` clippy (`:179`/`:213`/`:2222`)** blockt `ci-check` — Träger Mountain.
- **MCON / GRDC / GloFAS / NIST NCNR NSE** — brauchen `terms`/`field` (siehe Offen-Punkte).
- **dropped-gate-Riss** — future-240 nennt 924 Keys (`278ba2905`), die getrackte Baseline 78; bitte als Mountain-Verdikt auflösen.

## An river

Origin: mycelium-317 (2026-10-12).

- **THEMIS-ASI getimetes Keogramm** — Registerform für Per-Nacht-Assets offen (Mountain/Rat); Compiler-Form `at earth`→`on earth` = Mountain.
- **Mariner-CDN** — Workflows gebaut + nach Push dispatcht; sha256/Block nach grünem Lauf.
- **ENTSO-E**, **GRDC-Caravan**, **GloFAS-EWDS**, **SWPC/RTSW** — wie in `## An mountain` (Läufe queued).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
