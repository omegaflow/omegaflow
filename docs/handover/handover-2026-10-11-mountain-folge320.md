<!--
  title: Handover — Mountain-Folge 320 (2026-10-11)
  session: Mountain-Linie in einem Pass — zwei Register-Form-Risse durch Rat + Research + UI-Roster entschieden (McPherron-Onset-Liste → witness point-event, Riss gegen River gemessen: die Liste trägt SML-Werte/Bay-Properties, nicht nur Onsets; THEMIS-ASI-Stationstabelle → Compiler-Eingabe, Riss asilib-23 vs Berkeley-20) + 7 Quellen-Terms/Verdikte gemessen (OE-417/THEMIS-ASI/PCN/CSES/BGS-GIC/NERC-GMD/McPherron) + McPherron-Schema exakt gemessen
  class: handover
  date: 2026-10-11
  sha256: c4d806f33293b4f58e41817254f3da2644f0894cc51aa5a2c177f8ee6f8d738c
  status: live
-->
# Handover — Mountain-Folge 320 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; der dort
genannte HEAD war `537c5dc09`, aktueller HEAD dieser Session `7517e6016`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge319.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.06 · cap 0.50 · reason: Linien-Session + 1 general-Taucher (Quellen-Terms) + 1 grind-flash-Taucher (McPherron-Schema) + 1 council (2 Form-Risse) + 2 UI-Seats (Claude Sonnet 5.5 Extra hoch · GLM-5.3 Deep Think Max) + `archive_search --all`/Fach-Modi als Research-Schicht — gemessen via `session_burn`, deepseek-flash, kein pro/max.

## Erledigt in diesem Atom (gelöscht aus dem Register; git trägt es)

- **Register-Form Q1 entschieden — McPherron-Onset-Liste als `witness point-event`.** Durch Rat (5 Stimmen) + Research-Schicht + UI-Roster konvergent: **kein neues Grammatik-Arm**. Der Oszillator ist das Ereignis an seinem Epoche (τ=0, extent=0, 0 honored); Träger = `witness point-event`, `record mcon`, `force em`, `no-cadence`. Der Compiler bleibt `mcpherron_compiler.rs`; `MCON` tritt in den Point-Event-Arm (`src/archivar/witness.rs:20-22`) + Gate-Test im selben Atom.
- **Riss gegen River-187 gemessen und benannt (McPherron-Inhalt).** Die Behauptung „reiner Onset-Zeit-Katalog, keine Feldwerte" ist **widerlegt**: die drei Dateien tragen SML-Werte und Bay-Properties (s. u.). Damit entfällt die council-seitige Bedingung „per-Bin-Zählung via `.te`-Deskriptor" — **jedes Ereignis trägt seinen gemessenen SML-Skalar**, kein Bin nötig.
- **Register-Form Q2 entschieden — THEMIS-ASI-Stationstabelle ist Compiler-Eingabe**, kein `record`/`witness` (nackte Koordinate ohne Wert bleibt verworfen, `die-weberin.md:127-129`). Compiler-Defekt benannt: `themis_asi_keogram_compiler.rs:112` muss `on earth <lat> <lon> <alt>` + `station <CODE>` emittieren (MIRACLE-Form, `phi/sources.φ:20287`). Riss a (asilib `asi_locations.csv`, 23) vs b (Berkeley `thm_asi_stations.pro`, 20, abweichende Menge, lon 0–360): **a ist autoritativ** (gleicher `<code>`-Raum wie die UCalgary-Labels), b als zweite Zeugenlinie getragen; fehlendes `alt` → Station bleibt `pending`, nie `0.0`.
- **7 Quellen-Terms/Verdikte gemessen (general-Taucher, `--verdict`/`curl`/Zenodo-API):**
  - **OE-417** (`doe417.pnnl.gov`) — **not-admit**: die 27-Box-Cause-Taxonomie trägt **keine** geomagnetische Kategorie; Rohdatensätze hinter Microsoft-Entra (OIDC). Nicht als GMD-Quelle führen.
  - **THEMIS-ASI** (aurorax + UCalgary) — **admit**: data.phys.ucalgary.ca keyless lesend, offen mit Zitatpflicht (kein SPDX-String); `ttl 604800`. Bestehender Berkeley-Block `phi/sources.φ:20889-20895` (`terms free-open`).
  - **PCN** (DTU WDC) — **admit**: CC BY 4.0 (im Datei-Header `ftp.space.dtu.dk/WDC/indices/pcn/PCN_definitive/`), `ttl 31536000` (definitive, 1975–2024) / `86400` (quicklook).
  - **CSES** Zenodo `13329652`/`8082201` — CC BY 4.0, `ttl 31536000`.
  - **BGS-GIC** — **not-admit**: nur der B_GIC-Index als Bild + modelliertes UK-Feld (PNG), **keine** maschinenlesbare Messreihe.
  - **NERC-GMD** — **not-admit / blocked-account**: `eroportal.nerc.net/gmd-event-list` liefert die ERO-Portal-„System Error"-Seite, Konto-Gate + BotDetect; keine offenen Records.
  - **McPherron** — cc-by-4.0, `ttl 31536000` (statisches Archiv), **admit als Daten**.
- **McPherron-Schema exakt gemessen (grind-flash-Taucher, `unzip`/`tr`/`awk`):** `CorrOnsFiles.zip` (md5 `52951d852f4d5bf7ac092a7d304f2b26` bestätigt) → `ftims.txt` (11 Spalten: 6 Zeit + 5 Relativzeiten in min), `fvals.txt` (11 Spalten: 6 Zeit + **SML-Werte** an den 5 Zeitpunkten), `fadd.txt` (10 Spalten: 6 Zeit + **Amplitude nT, Bay-Depth nT, Bay-Duration min, Bay-Width min**); **314 560 Ereignisse**, 1966-01-01→2024-05-29, Zeit = `[year month day hour minute second=00]` (Kalender-Vektor, **kein** doy/Epoche), `NaN`-Sentinel für unbestimmte Werte → `Option`, nie `0.0`. Kein MLT/MLAT/GLON/GLAT, kein Onset-Typ-Flag.

## Offen (aufgeschlüsselt)

### McPherron-Compiler — Bau der `MCON`-Form steht (eigen, nächster begrenzter Schritt)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Bau-Schritt
- **Lage:** (gemessen 2026-10-11 via `sread`/grind-flash) `mcpherron_compiler.rs` existiert, nutzt aber `FORMAT="substorm"` und `SubstormOnset{..None}` — `substorm::parse_series` (`substorm.rs:178`) verwirft Records ohne Komponentenwert → **0 Oszillatoren** (stiller Nicht-Ertrag, nicht Fehler). Verdict steht: point-event mit gemessenem SML-Skalar.
- **Blockade:** die `MCON`-Bin-Form ist ein neues Format-Modul (analog `quake_event.rs`: MAGIC + Header + Fest-Record mit `present`-Bits + Komponenten SML/Amplitude/Bay); das ist ein Architektur-Akt (neuer Format-Arm in `extract.rs`, Punkt-Event-Arm in `witness.rs`, Gate-Test).
- **Braucht:** `src/archivar/mcon_event.rs` (Punkt-Event, Komponenten `mcon_sml`/`mcon_amplitude`/`mcon_bay_decrease`/`mcon_bay_duration`/`mcon_bay_width`) + `extract.rs`-Arm `"mcon_event"` + `witness.rs`-Arm `MCON` + Compiler-Umbau (ftims/fvals/fadd zeilenweise joinen, `NaN`→`None`) + `cargo check` 0/0; danach Register-Block (Mycelium) + CDN-Workflow (Mycelium).

### THEMIS-ASI-Keogramm — Compiler-Fix blockiert am aktiv gehaltenen Dateiset
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** die Freigabe von `tools/harvest/src/bin/themis_asi_keogram_compiler.rs` durch die haltende Session
- **Lage:** (gemessen 2026-10-11 via `git status`/`stat`) die Datei ist **uncommittet modifiziert** (mtime 2026-10-11 20:5x, nach HEAD); Form-Verdikt steht (`on earth …` + `station …`, a=asilib autoritativ).
- **Blockade:** die Datei hält eine parallele Session (uncommitteter Hunk).
- **Braucht:** nach Freigabe `emit` in `themis_asi_keogram_compiler.rs:112` auf `on earth <lat> <lon> <alt>` + `station <CODE>` umstellen (Alt aus gemessener Spalte oder `pending`); `cargo check` 0/0.

### Quellen-Terms schreiben — `phi/sources.φ` aktiv gehalten
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** die Freigabe von `phi/sources.φ` durch die haltende Session
- **Lage:** (gemessen 2026-10-11 via `git status`/`git diff --stat`) `phi/sources.φ` trägt **70 uncommittete Zeilen** (url/origin/compiler-Manifest-Bindungen, Mycelium-Anteil) plus `src/mathematikerin/{mod,receiver,tropo}.rs`, `cdn_reconcile.rs`, `themis_asi_keogram_compiler.rs` — ein aktiver Fremd-Halter. Die 7 Terms/Verdikte (s. Erledigt) sind gemessen, aber noch nicht als Verdikt-Zeilen geschrieben.
- **Blockade:** ein pfad-begrenzter Commit über `phi/sources.φ` würde die fremden Hünke mitnehmen; „Commit als Letzter" verbietet den Schreibakt in eine aktiv gehaltene geteilte Datei.
- **Braucht:** nach Freigabe je Block die `terms`/`ttl`-Zeile setzen (THEMIS-ASI/PCN/CSES admissibel; OE-417/BGS-GIC/NERC-GMD als not-admit-Verdikt; McPherron als point-event-Block nach `MCON`-Bau), `terms` = Mountain, `url`/`origin`/`compiler` = Mycelium.

### Band-Paar-Atom — CI-Idempotenz-Sperre (unverändert aus 319)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `spdf.gsfc.nasa.gov`-Asset-Refresh + `witness-odf-residuum`-Lauf
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert) `pioneer-odf-cdn.yml:21-31` überspringt Compile+Upload, wenn das Release-Asset existiert (zuletzt 2026-09-18) → Witness `model: 0 samples modeled`.
- **Blockade:** Manifestations-Recht (Mycelium).
- **Braucht:** ODF-Workflows mit erzwungenem Re-Manifest (`--force`/clobber) neu dispatchten; danach Witness-Lauf lesen.

### ci-gate — rot am HEAD; format/register fremd
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf
- **Lage:** (gemessen 2026-10-11 via `ci_manage list`) `ci-gate` Läufe `38164577203`/`38164341503`/`38164276418` = **failure** (18:32–18:37Z); clippy in vmf3 geheilt (Vorgänger-Atom). `data.fingrid.fi` erscheint jetzt als Block `phi/sources.φ:11133-11136` (uncommittet).
- **Blockade:** `src/mathematikerin/*`-Format-Hünke hält eine andere Session; register/netloc-Bindung Mycelium.
- **Braucht:** die haltende Session formatiert `src/mathematikerin/*`; danach einen `ci-gate`-Lauf lesen (`ci_manage log <id>`).

### Eigene Ephemeride (PEP/Tudat) — Step 4 blockiert am aktiven Dateiset (unverändert)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Freigabe von `src/mathematikerin/{mod,receiver,tropo}.rs`
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert) Rat-Votum C; Research + UI konvergent.
- **Blockade:** fremd-aktiv gehaltenes Dateiset.
- **Braucht:** nach Freigabe die drei Rat-Schritte 1+2 als begrenzten Bau.

### Teil C — A5 offen (Galileo TDF), A6 descoped
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert) A5 `GO-SS-RSS-1-TDF-V1.0` (PDS PPI); A6 descoped.
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** A5 die TDF-Route als `url`-Zeile (Mycelium) + `terms`/`format` (Mountain), oder `descoped` mit Befund.

### opentopography_coverage — per-Dataset-Lizenz (Rat-Entscheid steht)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Coverage-Manifest auf dem CDN + Block registrieren
- **Lage:** (gemessen 2026-10-11 via 319-Register) per-Dataset-Lizenz; Portal-Manifest ohne `terms` (0 honored); `shard`-Emit nicht registriert.
- **Blockade:** Manifest-Asset (Mycelium).
- **Braucht:** Manifest (Mycelium) + Block (`url /origin /compiler /format coverage_manifest`, keine `terms`).

### SWPC/RTSW-Kurzreihen — nicht alignbar (river-187, gefaltet)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via river-187, Lauf `38119733631` Job `matrix-vlies`) 126/210 Matrix-Zellen `alignment pending` — SWPC-Livereihen decken ~1 Tag, OMNI endet 2026-10-05, keine Überlappung.
- **Blockade:** fehlende statische überlappende Archivreihe.
- **Braucht:** Verdikt + `ttl` für eine statische SWPC/RTSW-Archivreihe (URL) **oder** die Kurzreihen als nicht-alignbar führen (descoped mit Befund).

### `api.sensor.community` — eigener Register-Punkt (future-235, gefaltet)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via future-235) **kein** Future-Wort; Ursprung `mycelium-158…162` (403) + `mountain-306`.
- **Blockade:** Arm/Zielzeile selbst zu benennen.
- **Braucht:** Arm/Zielzeile benennen oder `descoped` mit Befund.

### Sechs Klima-Quellen / series-manifest · copernicus B1–B6 · NOAA S3 B7/B8 · SPT-3G D1 (unverändert aus 319)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je der erste CDN-/Manifest-Lauf bzw. gewählte Eingaben
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert) ecad/aodn brauchen gewählte Station/URL; die übrigen Läufe laufen/fehlen.
- **Blockade:** kein Lauf / wissenschaftliche Wahl ohne Default.
- **Braucht:** die Läufe lesen (`ci_manage log <id>`) + sha256 je Familie.

### Frame/Epoche · particle-cern · Exposom-Matrix · iEEG · Flyby-Kette · FMHY (unverändert aus 319)
- **Status:** eigen/wartend/termin/blockiert | **Bindung:** eigen/termin:2026-11-01
- **Trigger:** der 319-Register-Trigger je Punkt (Frame/Epoche: CRS-VLR-Messung; particle-cern: `subset`; Exposom: extern gedeckter Repräsentativpunkt; iEEG: neues Operator-Wort über 2026-10-06 — **nicht erneut vorlegen**; Flyby: ESOC-Recon-Release; FMHY: Riss 4/5)
- **Lage:** (gemessen 2026-10-11 via 319-Register; unverändert)
- **Blockade:** s. je Punkt im 319-Register.
- **Braucht:** s. je Punkt; sonst der benannte Riss/Descope-Befund.

## An mycelium

Origin: mountain-folge320 (2026-10-11).

- **`phi/sources.φ` aktiv gehalten:** 70 uncommittete url/origin/compiler-Zeilen im Baum (plus `cdn_reconcile.rs`); bitte committen, damit Mountains Verdikt-Zeilen (7 Terms oben) ohne Hunk-Kollision geschrieben werden können.
- **`themis_asi_keogram_compiler.rs` aktiv gehalten:** uncommittet modifiziert; bitte committen (Fix `at earth`→`on earth` wartet).
- **THEMIS-ASI/PCN/CSES admissibel** — bitte `url`/`origin`/`compiler` je Quelle; `terms`/`ttl` schreibt Mountain nach Freigabe.
- **McPherron:** nach `MCON`-Bau (Mountain) Register-Block + CDN-Workflow `mcpherron-cdn.yml` (Mycelium); `url …/releases/download/zenodo.org/mcpherron_onsets.bin`, `origin https://zenodo.org/api/records/22711679/files/CorrOnsFiles.zip/content`, `compiler tools/harvest/src/bin/mcpherron_compiler.rs`.
- **MLA-Register:** Repoint + `field RANGE mla_range` stehen (319); nur noch CDN-Manifestation + sha256.
- **Re-Admission/GIC-Terms stehen** (319 + 320); es fehlen die `url`/`origin`-Zeilen.

## Operator-Wort-Register

- **„b"** | 2026-10-11 | Operator-Session (mountain) — Form **(b)** für das Band-Paar-Atom: Companion-Band-Ebene (Record bleibt 9 Felder), nicht das Feld-6-Packing.
- **„ja bitte mro-Schritt nachziehen?"** | 2026-10-11 | Operator-Session (mountain) — auch `mro_odf_compiler` (streaming/shard) auf das Band-Paar nachziehen.
- **„bitte befrage auch research und roster wenn du den rat befragst"** | 2026-10-11 | Operator-Session (mountain) — jede Ratsfrage durch Research-Schicht + Roster.
- **„nein bitte hole die stimmen erst ein hier geht es um etwas sehr existenzielles"** | 2026-10-11 | Operator-Session (mountain) — Stimmen vor dem Bau.
- **„hab glm abgeholt"** | 2026-10-11 | Operator-Session (mountain) — GLM 5.3-Antwort vom Operator nachgereicht.
- **„bitte bei ratfragen auch archive search all und roster (mind. claude und glm 5.3 max) befragen"** | 2026-10-11 | Operator-Session (mountain) — jede Ratsfrage zusätzlich über `archive_search --all` (Research-Schicht) und den UI-Roster, mindestens Claude und GLM 5.3 max. In diesem Atom angewandt (Q1/Q2): `archive_search --all` + Fach-Modi, Claude Sonnet 5.5 Extra hoch, GLM-5.3 Deep Think Max.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom: nur die eigenen
Dateien (`docs/handover/*`, `state/stimmen/*`) committen — `phi/sources.φ` und
`src/mathematikerin/*` sind fremd-aktiv gehalten und werden nicht angetastet
(„Commit als Letzter").

**Gemessener Zwischenfall (2026-10-11, 21:16):** der `git mv` der konsumierten
Übergabe (`folge319` → `archiv/`) wurde vom **parallel laufenden Mycelium-Commit
`9141bb97d`** („mycelium 311: fold the binding outcome…") mitgeschrieben — ein
geteilter Index zwischen zwei Linien-Sessions; die Umbenennung liegt jetzt unter
fremder Commit-Message. Bestätigt die „Commit als Letzter"-Regel: `git mv`/`git add`
in den geteilten Index während einer parallelen Session committen lässt. Der eigene
`folge320`-Commit bleibt davon getrennt.
