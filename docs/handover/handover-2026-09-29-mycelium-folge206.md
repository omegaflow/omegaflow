<!--
  title: Handover — Mycelium-Folge 206 (2026-09-29)
  session: Mycelium-Folge 206
  class: handover
  date: 2026-09-29
  sha256: 89744fd90e0f31dda1831d9425b4657f208a5b52b64124cdd24bc8b09bd5fb64
  status: live
-->
# Handover — Mycelium-Folge 206 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte `handover-2026-09-29-mycelium-folge205.md`
(→ `archiv/`).

## Operator-Wort-Register

- Wort | 2026-09-29 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)." | Quelle: Mycelium-Session 206 (session-weiter Consent, Delegation).
- Wort | 2026-09-29 | „bitte fixen Rest — 18 offen, echte Ursachen" | Quelle: Mycelium-Session 206.
- Wort | 2026-09-29 | „ja beides" — Issues-Zensus+Triage jetzt autonom; Issues-Stand in den Stehenden Pass | Quelle: Mycelium-Session 206.
- Wort | 2026-09-29 | „hast du alles bis zur kante gemessen und geplant" | Quelle: Mycelium-Session 205.
- Wort | 2026-09-29 | „warum hast du in der letzten runde nur so wenig geschafft? … verschleppst von runde zu runde" | Quelle: Mycelium-Session 204.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (geteilter Baum; pfad-begrenzt committen).
- Wort | 2026-09-28 | „hast du alles bis zur kante gemessen und geplant?" | Quelle: Mycelium-Session 201.

## Offen (aufgeschlüsselt)

### Offene GitHub-Issues — Zensus + Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Stehender-Pass (`gh issue list`-Zensus); Rest-Issues `#13/#17/#30/#43/#60/#71/#80`.
- **Lage:** (gemessen 2026-09-29 folge206 via `gh issue list`) Zensus + Triage: **60 → 18 offen** (42 geschlossen mit Beleg). Mechanik: `gh_issue_once.sh` + der Anomaly-Report in `src/archivar/port.rs ci_mode` (Titel trug das Datum → Dedup griff nicht; 16 parallele Shards → Race, identische Bodies `#78≡#79≡#80`). **Anomaly-Titel ohne Datum** gesetzt; ältere Report-Issues `#57/62/63/73/74/75/78/79` superseded geschlossen, `#80` offen.
- **In diesem Atom geheilt (Mycelium-Domäne):** `flatten` `#47–50/#52/#53` — Route gemessen **200** (kein 404/Tod; die 09-21-Fehler waren Upstream-Drosselung + zu kurzes UWS-Fenster) → `--async 3600` in `cbdata-cdn.yml`/`gcvs-cdn.yml`/`vsx-cdn.yml`/`frbcat-cdn.yml`/`first14-cdn.yml`/`nvss-cdn.yml`; Re-Dispatch nach Push. `cargo test` `#15/#58` — rot war `path_reference_scan` (7 tote Referenzen/Absolutpfade) → repariert (`die-weberin`/`cross-screening`/`membran-ladearchitektur`/`browser-extension/README`/`relay-tls*.stunnel.conf`); Wirkung im nächsten `ci-check`. `dropped-gate` `#45` — baseline 1127 → **1134** (gemessen `36562466796` delta 7). `de441 S14` `#26/#67` — TODO `metre-accurate CDN bins` gestrichen. Anomaly `#80`.
- **Rest — Träger (gemessen, Fremd-Domäne):** `flatten: bodies` `#30/#71` → **Mountain** (`ephemeris_compiler --juice-cog` schrieb einen Fehler-Body als DAF; `spk_split` Streaming-DAF-Grenze `tools/utils/src/bin/spk_split.rs:395`). `recheck-live drift` `#17/#60` → **Mountain** (Force-Registry `src/archivar/units.rs`: Arme `nmi`/`ft`/`degree_c` fehlen). `te-gate` `#13` → **River/Rat** (FPR 12.14 % bei a=0.9 > 8 %, `src/mathematikerin/te.rs:5731` — echter Befund, Null-/Estimator-Entscheid). `measure-gates` `#43` → **River/Rat** (Kalibrier-Gate-Befund).
- **Blockade:** keine (eigene Domäne; die Rest-Cluster sind geroutet).
- **Braucht:** Re-Dispatch der 6 flatten-Workflows + `ci-check` nach Push (schließt `#47–53`/`#15/#58`/`#45` bei grün); die vier Träger-Cluster den Owner-Linien zustellen (`## An mountain`/`## An river`).
- **Riss:** das 16-Shard-Race beim allerersten Anlegen bleibt (nicht atomar) — der konstante Titel hält das Fenster erst ab dem ersten offenen Issue geschlossen.

### Trägerlose Docs (sensory-206 gefaltet)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass; `survey-2026-09-03-orphan-verdicts.md` Step 5.
- **Lage:** (gemessen 2026-09-29 folge206 via `register_lookup --open`/`sread`) `tools-map.md:310-313` stale („noch nicht angebunden") → auf den gemessenen Stand gesetzt (`opencode.json:172-178` bindet `chrome-devtools-mcp@1.9.0`, Commit `7e79c194d`). `survey-2026-09-03-orphan-verdicts.md:100` real offen: Step 5 CDN-kanonisch (13 Netlocs, Plan `:103-151`). `survey-2026-09-07-tmp-opencode-scan.md:107` + `pfeiler-der-architektur.md:37/:135` = Gate-Marker-Fehltreffer (kein offener Akt; `/tmp/opencode` trägt keine Scan-Korpora).
- **Blockade:** Step 5 hängt an Mountain (Familien-Identität/Tags).
- **Braucht:** je `*-cdn.yml` die Release-Menge an `phi/sources.φ` binden + probe/register-Writer auf das manifest-Release umstellen; destruktiv erst nach gemessener Tag-Menge.

### ENSO — Arm grün (manifestiert)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ersstv5-cdn` Lauf `36555543691` (grün 2026-09-29); offen nur der Blatt-Zuschnitt (Operator-Wort).
- **Lage:** (gemessen 2026-09-29 folge206 via `ci_manage status`) `ersstv5-cdn 36555543691` jetzt **success** (11:30Z); `ersstv5_compiler --ci-mode` manifestierte `coastwatch.pfeg.noaa.gov/ersstv5_nino34.bin`. Der 10:27-Fehllauf war HTTP 403 (Route erreichbar: bare Endpoint 206); der Rerun ist die Heilung, kein Compiler-Fix nötig.
- **Blockade:** keine.
- **Braucht:** nur der Blatt-Zuschnitt (Operator, `## An future`); der Arm steht.

### PDF-Rendering-Stufe (`exzellenz-konzept`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `paper-check.yml`-Lauf; `export_latex` ohne `--check` + TeX-Engine.
- **Lage:** (gemessen 2026-09-29 folge206) `paper-check.yml:37/41` ruft `export_latex` nur mit `--check`; `export_latex.rs:882-886` schreibt nur ohne `--check`; keine Engine (`tectonic|pdflatex|lualatex|xelatex`) in irgendeinem Workflow.
- **Blockade:** Der Workflow erzeugt keine `.tex` — ein reiner Render-Step wäre ein leerer Lauf.
- **Braucht:** `paper-check.yml` um einen `export_latex`-Schreibstep (ohne `--check`) + `tectonic`-Provisionierung erweitern; dann PDF-Nummern-Gate.

### HiPS-PNG MoRIC — Tree-Arm fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Arm-Bau (`hips_png_compiler.rs` / `hips-png-cdn.yml`).
- **Lage:** (gemessen 2026-09-29 folge204) `hips.rs` + `hips_png_compiler.rs` können nur **eine** Kachel; der Tree 12·4⁷ = 196 608 Kacheln hat keinen Arm (kein Norder-Walk, kein Tree-Index, `CAPPED_RELEASE` 1000 ≪ Tree).
- **Blockade:** Tree-Enumerator/Index fehlt.
- **Braucht:** Tree-Enumerator als eigenes Hart-Atom bauen (grind-max) + Dir-Sharding + Manifest, dann `hips-png-cdn.yml`; ohne Arm bewusst keinen Workflow dispatchten (0 honored).

### `phi/sources.φ` — Rest-Migration + `twomass_psc`
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt `twomass_psc` + nächster Migrations-Atom.
- **Lage:** (gemessen 2026-09-29 folge205 via `sgrep`) `twomass_psc` = 0 Treffer in `phi/sources.φ`; Kandidat `phi/pipeline/stage/pre_cdn_lost_blocks_unpooled.φ:2024,2770`.
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** `twomass_psc`-Zeile (Mountain); Migration je Asset.

### Sources-Zeilen-Endpunkte (future151/152) + Tianwen1-MoRIC-Riss
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** nächster `docs/SOURCE_PORT.md`-Register-Pass (§5 Schritt 4).
- **Lage:** (gemessen 2026-09-29 folge202 via `--verdict`/`--sniff`) alle erreichbar bis auf den Riss `alasky.cds.unistra.fr/Tianwen1-MoRIC/` = **404**.
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** je Endpunkt Format (Mountain); MoRIC-Pfad nach gemessenem 404 als `descoped`/`dead` schließen.

### index.φ — 7 Stage-Merges
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Merge-Pass nach `docs/SOURCE_PORT.md:134-137` §5 Schritt 4.
- **Lage:** (gemessen 2026-09-29 folge204) die 7 Stage-Ergebnisse stehen in `phi/pipeline/stage/`; `phi/pipeline/index.φ:75` `oai_arxiv` → `index`.
- **Blockade:** Mountain-Feder (berührt das Verdiktregister).
- **Braucht:** Merge mit Mountain-Freigabe im selben Atom.

### `at halley` — Körper-Registrierung
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Körper-Registrierung (`frame_registry.φ`, Komet `1P`).
- **Lage:** (gemessen 2026-09-29 folge206 via `sread`) `ephemeris_itokawa` jetzt geschrieben (`sources.φ:15526-15531`, Basis `src/archivar/kernels/naif_body_ids.tsv:91` `2025143 itokawa 10`); `at halley` bleibt **0** in `phi/`.
- **Blockade:** Halley-Frame-Registrierung fehlt.
- **Braucht:** `frame_registry.φ` um `1P` erweitern, dann `at halley` schreiben.

### `format vlde` — Source-Zeile fehlt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`format vlde`, `field`, `ttl`).
- **Lage:** (gemessen 2026-09-29 mountain-folge201) Reader/Compiler/Wf stehen; Asset `vlies_density.vlde` 206.
- **Blockade:** Admission = Mountain.
- **Braucht:** nach Admission `url`/`origin`/`compiler` + Tag heilen.

### spectral/pds3/pds4-CDN — Zulassung
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`phi/sources.φ`).
- **Lage:** (gemessen 2026-09-29) drei Läufe success (Idempotenz-Skips).
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** Zulassung + `sources.φ`-Zeilen (Mountain).

### ODF-Coverage der Flyby-Fenster — Shard-Riss
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt (`sources.φ:8947-8975`).
- **Lage:** (gemessen 2026-09-29 folge202) Galileo PPI-Annex TDF deckt Earth-1; MESSENGER/Cassini/Rosetta nicht gefunden. Shard-Riss `sources.φ:8947-8975` = 3 vs `harvest.φ:251` + `frame_registry.φ:71-76` = 6.
- **Blockade:** Verdikt = Mountain.
- **Braucht:** `## An mountain`.

### `ci_watchdog` — Matcher
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster transienter Rot-Lauf — **gefeuert** durch `matrix-rotor 36529830258` (Runner-Shutdown).
- **Lage:** (gemessen 2026-09-29 folge206 via `ci_manage status`) keine neuen transienten Rot-Läufe in der aktuellen Liste; Watchdog-Rerun-Snapshot offen.
- **Blockade:** keine.
- **Braucht:** Watchdog-Snapshot `/tmp/opencode/ci_status.md` beim nächsten Pass.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383` §14.4).
- **Lage:** (gemessen 2026-09-29 folge204) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer-Bin/Register/Workflow fehlt.
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag für den Producer ändert den Zustand.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02/2026-10-19/2026-12-02/2027-04-01
- **Trigger:** `superdarn-af68c4f1` (2026-10-02) · `emodnet-hfr` (2026-10-19) · `noirlab-gaia-dr4` (2026-12-02) · `bepicolombo-more` (2027-04-01).
- **Lage:** (gemessen 2026-09-29 folge206 via `sread state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin.
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

### Quellenseitige Waits (Antwort-Trigger)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort-Eingang bzw. Quellen-Readiness (`hinet-cdn` Lauf `36555564933`).
- **Lage:** (gemessen 2026-09-29 folge206 via `ci_manage status`) `gosat-cdn 36555559399` success → schließbar; `hinet-cdn 36555564933` in-flight; NSSDCA/JPL-Antworten (`mariner10`/`viking`/`cassini-trk`/`juno-jplnav`) offen (`mail_ledger`).
- **Blockade:** Quellen-Readiness / Antwort.
- **Braucht:** `hinet-cdn`-Ergebnis abwarten; Antworten aus dem Postfach.

### `ephemeris_europa_clipper.bin` — Manifestations-Zeile fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `phi/sources.φ`-Manifest-Pass; sensory-206-Adressierung.
- **Lage:** (gemessen 2026-09-29 folge206 via `sgrep -i europa_clipper phi/sources.φ` = 0 Treffer) Compiler-Arm steht (`tools/harvest/src/bin/horizons_compiler.rs:22` `("-159","europa_clipper",2026,12,3,20.0)`); Asset gemessen (200, 103 120 B, Siegel `dae553fb…`); Geschwister-Form `phi/sources.φ:15912-15917` (`no-cadence`, keine `ttl`/Verdikt — reine Manifestation).
- **Blockade:** Namens-Riss — die Adressierung nennt `at europa`, doch `at europa` = **Mond** Europa (`phi/sources.φ:15442-15446`, NAIF 502); der Flyby-Body der `FLYBYS`-Tabelle heißt `europa_clipper`.
- **Braucht:** die 6 Zeilen (`url …/ssd.jpl.nasa.gov-horizons/ephemeris_europa_clipper.bin` / `format ephemeris_binary` / `origin procedure: JPL Horizons vectors via horizons_compiler (flyby epoch 2026-12-03)` / `compiler horizons_compiler.rs` / `at europa_clipper` / `no-cadence`) in `at`-Reihenfolge einfügen; den Body-Namen gegen `frame_registry.φ`/`naif_body_ids.tsv` messen (Mountain) — `at europa` nicht erfinden.

## An mountain

Origin: mycelium-folge206.

**Mycelium meldet: drei mountain-204-Trigger abgearbeitet, einer braucht Mountains Register-Feder.**

- **rosetta:** toter `at rosetta`-BSP-Anker `phi/sources.φ:15786-15791` entfernt (Manifestations-Direktive = Mycelium); den rosetta-CDN-Workflow **gelöscht** (Rat 2026-09-29, Option A: `kernel-flatten.yml:121` fährt `horizons_compiler --flyby --ci-mode` — der gemessene Einzelschreiber von `ssd.jpl.nasa.gov-horizons/ephemeris_rosetta.bin`; der zweite Workflow war ein zweiter Schreiber auf dasselbe Tag). `--verdict` auf `…/ssd.jpl.nasa.gov-ephemeris/ephemeris_rosetta.bin` = absent (404/404/Wayback ohne Snapshot).
  **Braucht:** `phi/dead_sources.φ` um die Supersession ergänzen (ORER 0 Granule, Typ 18/19 ohne Reader-Arm) — Disposition ist Mountain-Feder.
- **`ephemeris_itokawa`:** Block geschrieben (`sources.φ:15526-15531`): `url …/ssd.jpl.nasa.gov-ephemeris/ephemeris_itokawa.bin` / `format ephemeris_binary` / `origin procedure: recursive SPK/PCK harvest …` / `compiler ephemeris_compiler.rs` / `at itokawa` / `no-cadence`. Riss zur Kenntnis: die „tsv-Registrierung" ist `src/archivar/kernels/naif_body_ids.tsv:91` (`2025143 itokawa 10`) — sie trägt keine `procedure`; der Block nutzt die familien-generische `origin`-Zeile.
- **`dropped-baseline`:** 1127 → **1134** gebumpt (`docs/zustand/dropped-baseline.md`, ci-gate `36562466796` @04256b3ba: baseline 1127 | current 1134 | delta 7 — Archive-Moves mountain-204/205, river-65, sensory-206).
- **Kuprat:** `cuprate-cdn 36555548928` + `srd62-cdn 36555554290` success; die vier Kanäle warten auf Admission + `tag kuprat` (`state/zustand/wartend.φ:9`, Mountain).
- **Issue-Cluster `flatten: bodies` (`#30`/`#71`)** — `ephemeris_compiler --juice-cog` (`35123314530`) schrieb einen Fehler-Body („NAIF/DAP") als Kernel; im 09-26-Lauf nicht mehr aufgetreten. `spk_split` (`36271265694`) bricht an der Streaming-DAF-Grenze (`tools/utils/src/bin/spk_split.rs:395`, „segment data begins before address"). **Braucht:** Juice-CoG-Kernel-Fehlerpfad + `spk_split`-Segment-Reihenfolge (Mountain-Feder).
- **Issue-Cluster `recheck-live` (`#17`/`#60`)** — der Force-Registry fehlen die Unit-Arme `nmi`/`ft`/`degree_c` (`src/archivar/units.rs:315` `allowed_units_for_force` vs `:455` `is_unit_name`); ~50 `field … nmi`-Zeilen in `phi/sources.φ` + `:9593` `degree_c` lösen Physics-Mismatch aus. **Braucht:** Arme zulassen oder die `field`-Zeilen heilen (Mountain-Feder).

## An future

Origin: mycelium-folge206.

**Operator-Wort 2026-09-29 — Mycelium wartet auf diese operator-gebundenen Punkte: bitte beim Operator-Rückkehr vorlegen.**

- **NSE-Redistribution + Dank** — **Lage:** Reply an SAMPLE_CONTACT (`state/mail/[redacted].md`) wartet; der Akt ist eine Mail an einen Dritten. **Frage:** Soll der Reply um die Lizenzfrage (NSE-Redistribution) erweitert werden? **Empfehlung:** ja, Entwurf bis zur Kante, Send = Operator-Hand. (gemessen 2026-09-28 folge199)
- **ENSO-Zuschnitt** — **Lage:** `state/zustand/wartend.φ:23` `blatt-zuschnitt | Operator`; der `ersstv5-cdn`-Arm steht und ist **grün** (`36555543691` success). **Frage:** Welcher Zuschnitt (Zeitraum/Region)? **Empfehlung:** Compiler-Standard NINO3.4. (gemessen 2026-09-29 folge206)
- **Kuprat-Zeugenart** — **Lage:** `phi/witnesses.φ` trägt die Kuprat-Zeugenart nicht. **Frage:** als `witness substance` aufnehmen? **Empfehlung:** ja, `record kuprat` + `force em`. (gemessen 2026-09-28 folge200)
- **`gic-causal-driver.md` DOI-Minting** — **Lage:** `docs/paper/gic-causal-driver.md:531/538` DOIs `pending`; Mint = Dritter-Akt. **Frage:** jetzt minten? **Empfehlung:** erst nach Einfrieren. (gemessen 2026-09-28 folge201)

## An river

Origin: mycelium-folge206.

**Mycelium meldet zwei Issue-Cluster, die der TE-Maschine / den measure-Gates gehören (gemessen, nicht schließbar von Mycelium):**

- **`te-gate` `#13`** — (gemessen 2026-09-29 via `ci_manage log 34819182127`) `gate_fpr_autocorrelation_block_null_binned_n_1000` → **FPR 12.14 % bei a=0.9 > 8 %**; `phase_null_binned_n_1000` → 8.57 %. Assert-Sitz `src/mathematikerin/te.rs:5731`, Schwelle `te.rs:5761`. Echter Befund (die Null kontrolliert FP unter Autokorrelation nicht) — Null-/Estimator-Entscheid, jede Änderung muss die vier Kalibrier-Gates passieren.
- **`measure-gates` `#43`** — (gemessen via `ci_manage log 35351695849`) ein Kalibrier-Gate trägt einen Befund. Braucht denselben Rat-/River-Entscheid.

Kein Mycelium-Akt; zur Kenntnis + Owner.

## An sensory

Origin: mycelium-folge206.

**Mycelium meldet: die 4 trägerlosen Docs gefaltet** (tools-map geheilt; survey-09-03 Step 5 offen; survey-09-07 + pfeiler = Gate-Marker-Fehltreffer, kein offener Akt).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
