<!--
  title: Handover — Mountain-Folge 309 (2026-10-11)
  session: Mountain-Linie in einem Pass — GIC-Papier Zwei-Arm-Riss nachgezogen, ref_hz-Stadium gebaut, particle-cern-Risse korrigiert
  class: handover
  date: 2026-10-11
  sha256: 42208d364fed9a2a7346bad3f52038b3b822916550669543eecf0edc07882341
  status: live
-->
# Handover — Mountain-Folge 309 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge308.md` (→ `archiv/`).
flash only, kein pro/max. Operator-Trigger dieser Session: die Linien-Start-Direktive
(„Mountain in einem Pass, kein Planungstheater") — Bekanntes direkt bis zur Kante
gearbeitet, 3 begrenzte Dispatches (general ×3) + 1 Bau-Dispatch (grind-flash ×1).

## Burn: open 0.0 · close 0.17 · cap 0.5 — Grund: 3× general (~$0.18) + 1× grind-flash (~$0.005) + 1× line; `session_burn`-Snapshot am Schluss 19 Sessions $0.9507; deepseek-flash, kein pro/max

## Offen (aufgeschlüsselt)

### Beobachtungsoperator + Fit — Schritt 2 (Residuum verhören)
- **Status:** eigen (Bau/Messung) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt — die `ref_hz`-Stadien in den Konsumenten verdrahten
- **Lage:** (gemessen 2026-10-11 via general + dieser Atom) Der `ref_hz`-Riss ist **erklärt**: `bits(words,179,224)` = ODF **Reference Frequency** (mHz; high Bits 179–200 ×2²⁴ + low 201–224), `ref_hz/1000.0` ist dt-unabhängig korrekt. Der ~104×-Spread ist **echter Feldinhalt**: dt11 trägt den S-Band-Beacon (2 292 035 640 Hz, projektkonstant — alle 29 712 dt11-Records identisch), dt12/13 den **Uplink** — Träger ~2,110e9 Hz und TSF = Träger/96 (~2,2e7 Hz); Band-ID in Bits 158–159. Quellen: PDS `odf_unpack.pdf` + TRK-2-18 Rev.E A.2/A.3 (`pds-geosciences.wustl.edu`); gemessen an `86334o97343_sc23.odf` und `73288o74360_bj_sc23.odf`. **Gebaut in diesem Atom:** `ReferenceStage` + `reference_stage`/`ref_uplink_hz` (`src/archivar/odf.rs:2218-2242`), Test `:3188-3203`, `cargo check` 0/0. Der Konsument `pioneer10_odf_residuum.rs:502,510` behandelt `ref_hz` noch als einen Skalar.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** die zwei `pub`-Funktionen in `tools/measure/src/bin/pioneer10_odf_residuum.rs` verdrahten (ref_hz je dt-Stadium trennen); den `pending`-Diskriminator TSF vs. Uplink-Träger bei dt12/13 gegen eine Quelle bestätigen (Kandidat Magnituden-Schwelle); danach die Medien (Troposphäre/Ionosphäre) in die Kette (`fit-residuum`).

### particle-cern — Baskets offen
- **Status:** eigen (Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (oder ein Sample-Byte-Read)
- **Lage:** (gemessen 2026-10-11 via general) Die zwei folge308-Risse sind **korrigiert**: (a) der Element-Präfix als TNamed ist **byte-exakt korrekt** (`TStreamerElement::Streamer`); eine `:829-833`-TObjArray-Vorlage existiert dort nicht — der Anker ist gedriftet (Riss Register↔Baum, benannt). Der echte Defekt: `parse_streamer_elements:882-888` verliert die Klassen-Referenz bei `tag != K_NEW_CLASS_TAG` (kein class-ref-map wie `parse_tree:707`), und der Element-Body endet bei `fSize` (`:899`) — der Tail `fArrayLength/fArrayDim/fMaxIndex[5]/fTypeName` (`:862-865`) wird nie gelesen. (b) Der Frame-Advance ist ROOT-nativ korrekt; der echte Defekt ist der zu kurze Body + der `>`-Guard `:900` (fängt nie ein kurzes Body).
- **Blockade:** kein lokales `.root`-Sample (die Sample-Bytes nie gelesen); das Layout ist aus ROOT-/jsroot-Master gelesen, nicht am Sample gemessen.
- **Braucht:** begrenzter Schritt — `StreamerElement` (`:52-59`) um `array_length/array_dim/max_index/type_name` erweitern, den Tail nach `fSize` parsen, Guard `>` → `!=` (ein File-Abschnitt, `cargo check` 0/0); dann class-ref-map, dann baskets (TBranch v>5: `fBasketBytes`/`fBasketEntry`). Vorher ein Sample-Byte-Read (`AliVSD_Masterclass_6.root`, opendata.cern.ch/record/1120).

### ci-gate — Trigger: nächster abgeschlossener Lauf
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am HEAD nach diesem Commit
- **Lage:** (gemessen 2026-10-11 via `ci_manage status`) am HEAD `5247e9873` laufen `ci-gate 38117459724` (dropped-gate) + `38117428860`/`38117378965` (subset) — `in_progress`; kein abgeschlossener Lauf. Die 18 Lints sind in `77a7f7e71` getilgt.
- **Blockade:** kein abgeschlossener Lauf am neuen HEAD.
- **Braucht:** `ci_manage log <ci-gate-id>` auf dem nächsten abgeschlossenen Lauf → grün/rot; bei rot die verbliebenen Lints tilgen (kein `#[allow]`).

### Mycelium-φ-Blöcke — Serien-Manifest-Format offen
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Entwurf des Serien-Manifest-Formats
- **Lage:** (gemessen 2026-10-11) der **ecad-Riss ist gelöst** (netloc = Quellen-Identität `ecad.eu`, origin = Distributionshost). Der Mycelium-adressierte Block gegengeprüft: `terms` aller 6 Klima-Quellen deckt die Behauptung (PD SURFRAD; non-commercial ECAD/WorldClim; CC-BY-4.0 DWD/AODN/KNMI); `ttl` 5/6 mit Compiler-`TTL_S` belegt (SURFRAD 604800 · ECAD 604800 · DWD 2592000 · WorldClim 315360000 · KNMI 86400). **AODN-Riss:** `ttl 1` (`phi/sources.φ:8`) = Datensatz-Kadenz (`aodn_compiler.rs:586`), kein fester Harvest-Refresh.
- **Blockade:** ohne Serien-Manifest trägt jeder Block nur einen Default-Tag.
- **Braucht:** Serien-Manifest-Format (Mountain) — die Compiler erzeugen je-Akt-Instanzen. Der CI-Wait je Block steht in `state/zustand/wartend.φ`.

### PEP + Tudat — Schritt 3 (mehrere Bahnen) offen
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Schritt 3 — weitere Zeugen (Merkur/Venus Radar+VEX-Ranging) durch die Kette
- **Lage:** (gemessen 2026-10-11, folge306) Schritt 1 gefahren (Pioneer-10 durch die Kette); vier Häuser (DE440/INPOP19a/EPM2021/PETREL19) als `ephemeris_*` registriert; Träger-Doks `docs/concepts/eigene-ephemeride.md`, `docs/surveys/survey-2026-10-10-ephemeris-quellen.md`.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Schritt 3 — weitere Zeugen durch die Kette, Kette CI-nah verankern.

### FMHY-Routing — Riss 4 bot-gated
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss — Overpass bot-gated (gemessen 2026-10-10)
- **Lage:** (gemessen 2026-10-10, unverändert) Grenze ist die Manifestations-Achse: Query → `archive_search`-Arm; Messwert → `phi/sources.φ` + Compiler; Operator-Werkzeug → `tools/`; Blick/Portal → Lead. Riss 4 (Overpass — HTTP 406 direkt+Proton → bot-gated `pending`).
- **Blockade:** Riss 4 bot-gated.
- **Braucht:** Riss 4-Abschluss oder Descope.

### phi/pipeline nach Verbraucher trennen — Register + Vorrat offen
- **Status:** eigen (Bau/Register) | **Bindung:** eigen
- **Trigger:** je Klasse der Konsumenten-Umbau (`cargo check` 0/0) → dann verschieben/entfernen
- **Lage:** (gemessen 2026-10-11) 3 Naturen getrennt; descoped: `research/`·`stage/`·`queue/`·`leads.φ`·`tap_index_*`·`tap_index_merge.rs`. Offen: (a) **Verdikt-Register** `ledger.φ`/`index.φ`/`decline_lens.φ`; (b) **Kandidaten-Vorrat** `probe_*.φ`, `library.φ`, `frame_registry.φ`, `master_urls.txt`.
- **Blockade:** (a)/(b) brauchen den Konsumenten-Umbau voraus; (b) ist ein Live-Arm-Umbau (`discovery.rs`).
- **Braucht:** je Klasse **ein Atom**: Konsument umbauen, dann verschieben/entfernen.

### phi/ Ordnung — harvest.φ auflösen
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** `harvest.φ`-Auflösung
- **Lage:** (gemessen 2026-10-11) `phi/` = 8 Dateien; Gate trägt `REGISTER_CLASSES` fest (`src/gate/commit_gate.rs:2115`). Offen: `harvest.φ` (77 Formate) → ~60 `*-cdn.yml`; 533 `terms unknown` in `sources.φ`; `docs/SOURCE_PORT.md` beschreibt noch die descopte `phi/pipeline/`-Maschinerie.
- **Blockade:** keine.
- **Braucht:** je Format den Workflow-Input setzen; `SOURCE_PORT.md` nachziehen.

### queue/stage + ledger.φ — Descope-Zeile je Klasse
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** das Descope-Verdikt je Klasse
- **Lage:** (gemessen 2026-10-11) `phi/pipeline/queue/` 2, `stage/` 33, `tap_index_*` 72, `ledger.φ` 6 offene Zeilen. Ohne Konsumenten-Umbau hinterlässt Löschen schreibende Pfade (`port.rs`, `leads_merge`, `probe_sweep`).
- **Blockade:** schreibende Pfade.
- **Braucht:** je Klasse Messung + Descope-Zeile; `phi/canon.φ` von den `tap_index_*`-Zeilen lösen.

### Keine Leads-Datei — live suchen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** `discovery.rs` live + `leads_merge` entfernt
- **Lage:** (gemessen 2026-10-11) `phi/pipeline/leads.φ` + `leads_merge.rs:70/90/91` + `discovery.rs`. `leads_merge` liest den Kandidaten-Vorrat.
- **Blockade:** `leads_merge` liest gerade den Kandidaten-Vorrat.
- **Braucht:** `discovery.rs` auf live (`archive_search`), `leads_merge` + `leads.φ` entfernen.

### Flyby-Kette — termin 2026-11-01
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09) 157 ODF-Referenzen; `doppler.rs` absent; Wahrheit `state/zustand/wartend.φ`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### canon.φ fällt — Release greift (Dependency erfüllt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die frische `commit_check`-Release lokal (nicht mehr stale)
- **Lage:** (gemessen 2026-10-11) `tools-build 38103741757` = success — die Release mit `REGISTER_CLASSES` (7 lebende Register, `src/gate/commit_gate.rs:2115`) liegt vor. `phi/` = 8 Dateien (7 Register + `canon.φ`); `bin/.tools_ensure --all` = `pending` (HTTP 302, absent from manifest).
- **Blockade:** die lokale `commit_check`-Binary greift die frische Release nicht.
- **Braucht:** `bin/.tools_ensure --all` erneut messen; bei frischer Binary `phi/canon.φ` entfernen → `phi/` = 7 Register.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt ist Annahme-Akt
- **Status:** wartend | **Bindung:** eigen (Mountain-Feder)
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile (Operator-Hand; Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-10) kein offener Datensatz trägt eine im Datensatz gemessene Koordinate; das medizinische Resultat (`docs/surveys/survey-2026-10-09-domaenen.md:93`) gehört hierher (TE-Lauf).
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt — dann `phi/sources.φ`-Zeile, dann Te-Paar-CI-Feed (Mycelium).

### iEEG — registriertes Wort maßgeblich
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort, das den Riss über 2026-10-06 hebt
- **Lage:** (gemessen 2026-10-11) iEEG = privates Experiment (`state/zustand/wartend.φ`), kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es.

### api.sensor.community — zulassen, Zielzeile unbenannt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das wörtliche Future-Wort (welcher Arm/welche Zeile)
- **Lage:** (gemessen 2026-10-11) `https://api.sensor.community/` HTTP 200, sha256 `ad345805…`; `sensor.community` bereits registriert (`phi/sources.φ`, `dead_sources.φ`, `declined_sources.φ` SPS30-Parser-Gap).
- **Blockade:** „zulassen" nennt keinen Arm/keine Zeile; `…/v1/now` wäre ein neuer Arm — ungemessen.
- **Braucht:** das wörtliche Future-Wort, sonst Doppel-Block.

### Asservatenkammer-Survey-Marker — Prosa-Reconciliation
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** die Marker-Reconciliation des Surveys
- **Lage:** (gemessen 2026-10-11, aus Mycelium-303 adressiert) `docs/surveys/survey-2026-10-08-research-api-mcp.md`: der Status-Absatz (`:31-38`) trägt den gemessenen Stand (Consensus live · Perplexity live · Elicit descoped · SciSpace pending); die `Top-Schritte` (`:55-74`) tragen noch die stale „Braucht: Key anlegen … Arm bauen"-Sprache.
- **Blockade:** keine.
- **Braucht:** die `Top-Schritte` auf den gemessenen Status ziehen (Prosa-Reconciliation, Header-sha neu); Adressat Mycelium, da der Survey dort lebt.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf lokal/silent,
  nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen:
  der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)
„bitte die grossen punkte parallel mit agenten abarbeiten und ABSCHLIESSEN, NICHT VERSCHLEPPEN" | 2026-10-11 | Operator (Session, Mountain 305)
„was ist mit unseren ephemeriden und warum fängst du die grossen punkte immer wieder an anstatt sie fertig zu machen" | 2026-10-11 | Operator (Session, Mountain 306)
„bypass-mirror 23 (UrhG-§95a- was bedeutet das die möchte ich bitte raus haben keine fragwürdigen links" | 2026-10-10 | Operator (Session, Mountain 303)
„können wir nun eine untersuchung machen was davon als arme in archive search sollte, was in tools und was in phi dateien?" | 2026-10-10 | Operator (Session, Mountain 303)

## An Mycelium (gefaltet aus Mycelium-303)

Origin: mycelium-303 (2026-10-11). Gefaltet in diesem Atom:
- **te-ground-truth** → **erledigt**: das Artefakt ist am Papier (`docs/paper/gic-causal-driver.md`
  §3.5/Abstract/§7/§6) nachgezogen (Zwei-Arm-Riss). Die Block-Zahlen sind am Artefakt
  gegengeprüft: Behauptung 2 **korrigiert** — bei c=0 cleart nur `TE(Y→X)=-6.5337e-3` die
  fam `-8.5123e-3`, `TE(X→Y)=-1.1298e-2` nicht; der eine fehlende Term ist `te_yx0 <= fam`.
  Zwei Block-Zitate waren **stale**: `omega.rs:489` (Riss-Formatter, nicht der Schätzer) und
  `gic-causal-driver.md:655-660` (Yearly-fam-Werte, nicht der skalare Arm) — benannt, nicht
  geglättet.
- **VNP46A3 φ-Block** → **erledigt**: vollständig registriert (`phi/sources.φ:2036-2044`:
  url/terms/format/origin/compiler/header/on/ttl/field).
- **open-lidar-data φ-Block** → **erledigt**: vollständig registriert (`phi/sources.φ:11711-11723`:
  url/terms/format/origin/compiler/sha256/at/ttl/fields).
- **Sechs Klima-Quellen** → **erledigt** (folge308): terms 6/6 gedeckt, ttl 5/6 mit Compiler-
  `TTL_S`; AODN-Riss benannt.
- **Asservatenkammer-Survey** → **offen**: als eigener Punkt oben geführt (Prosa-Reconciliation
  der `Top-Schritte`).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch",
2026-10-07) trägt Commit und Push. **Dieses Atom (Mountain 309):**
- **GIC-Papier Zwei-Arm-Riss nachgezogen** — `docs/paper/gic-causal-driver.md` §3.5/Abstract/§7/§6
  tragen jetzt das gemessene `te-ground-truth`-Artefakt (Produktionsarm nullt den Rückkanal bei
  starker Kopplung, rekonstruiert aber den c=0-Null-Control nicht; skalarer Silverman-KDE-Arm
  nullt den Rückkanal nicht) — Am Artefakt gemessen (sha256-verifiziert, CI 38038722712,
  Artefakt 11664873110). Header-sha neu.
- **ref_hz-Stadium gebaut** — `ReferenceStage` + `reference_stage`/`ref_uplink_hz`
  (`src/archivar/odf.rs:2218-2242`), Test `:3188-3203`, `cargo check` 0/0. Der ~104×-Riss ist
  als Feldinhalt (Beacon/Uplink-Träger/TSF) erklärt, nicht als Parser-Fehler.
- **particle-cern-Risse korrigiert** (general): Präfix-als-TNamed ist korrekt; der `:829-833`-
  Anker ist gedriftet; echte Defekte = Klass-Ref-Verlust + ungeparster Element-Tail.
- **Mycelium-303 addresster Block gefaltet** — te-ground-truth + VNP46A3 + open_lidar + Klima
  erledigt; Asservatenkammer-Survey als Punkt getragen.
- Standing-Pass-Bezug: `state/zustand/standing-pass.md` (am HEAD `5247e9873`; Standard-Pass
  zitiert, nicht kopiert). Geteilter Baum: `tools/gate/src/bin/commit_check.rs` ·
  `tools/harvest/src/bin/usgs_lidar_coverage.rs` fremd modifiziert — nicht angefasst.
- Eigene Pfade dieses Atoms: `docs/paper/gic-causal-driver.md` · `src/archivar/odf.rs` ·
  diese Übergabe · der Archiv-Move folge308.
