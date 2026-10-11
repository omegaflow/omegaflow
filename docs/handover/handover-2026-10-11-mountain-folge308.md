<!--
  title: Handover — Mountain-Folge 308 (2026-10-11)
  session: Mountain-Linie in einem Pass — ci-gate-Lints getilgt, omni2-1h-Kadenz geheilt, FMHY-6-Resthosts verdiktet, ref_hz-Riss gemessen
  class: handover
  date: 2026-10-11
  sha256: 391628149f18d0a9d6fd5fca26826205cbffd7ee74b001a1bc3d01c7bfb86a87
  status: live
-->
# Handover — Mountain-Folge 308 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge307.md` (→ `archiv/`).
flash only, kein pro/max. Operator-Trigger dieser Session: die Linien-Start-Direktive
(„Mountain in einem Pass, kein Planungstheater") — Bekanntes direkt bis zur Kante
gearbeitet, 4 begrenzte Dispatches (general ×4).

## Burn: open 0.0 · close 0.23 · cap 0.5 — Grund: 4× general ($0.1430) + 1× line (~$0.09); `session_burn`-Snapshot am Schluss 15 Sessions $0.7821; deepseek-flash, kein pro/max

## Offen (aufgeschlüsselt)

### Beobachtungsoperator + Fit — Schritt 2 (Residuum verhören)
- **Status:** eigen (Bau/Messung) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt — Schritt 2 (Residuum verhören: Kette falsch / Modell trägt Rest / Beobachtung trägt Artefakt)
- **Lage:** (gemessen 2026-10-11, folge306 + dieser Atom) P11 24 808 modelliert / P10 59 363; A ≈ +f/c je Station, Downlink ≈2×; erstes Modell (obs=A·ṙ+B_Pass) trägt die Serie nicht bis Hz. **ref_hz-Riss gemessen** (general, 2026-10-11): der Format-Zweig ist **nicht** der Diskriminator — alle 5 registrierten Pioneer-ODF sind `fmt=2` → else-Zweig (`src/archivar/odf.rs:83`); der ~104×-Riss sitzt **innerhalb** des else-Zweigs quer über `data_type` (P10-1996: dt11 → konstant `2 292 035 640` Hz = physikalische S-Band-Downlink-Frequenz, dt12/dt13 → ~`2,2e7` Hz). `bits(words,179,224)/1000` ist über alle dt identisch angewandt.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** die dt-spezifische ref_hz-Semantik aus TRK-2-18 (welches Feld `bits(words,179,224)` je `data_type` trägt) — dann dt-spezifische Dekodierung; danach die **Medien** (Troposphäre/Ionosphäre) in die Kette. Kein Blindtest, `fit-residuum`.

### particle-cern — Baskets offen
- **Status:** eigen (Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt
- **Lage:** (gemessen 2026-10-11) `parse_streamer_elements(obj, start, n_members)` gebaut — `src/archivar/root.rs:861–911`, `StreamerElement` (`:52–59`); `cargo check` 0/0. Zwei benannte Risse: (a) Element-Präfix als TNamed gelesen statt der `:829–833`-TObjArray-Vorlage; (b) Advance nutzt den `[byte-count|mask]`-Frame, nicht den vollen Body.
- **Blockade:** keine.
- **Braucht:** die **Baskets** `fBasketBytes`/`fBasketEntry` (eigener Schritt); der Branch-Decode (`gap:particle-cern`) bleibt offen.

### ci-gate — Trigger: nächster abgeschlossener Lauf
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am HEAD nach diesem Commit
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38105695194 --all`) am HEAD `c81493b1f` ci-gate **ROT** — die berührten Dateien sind bis `300ac20281` unberührt, die Lints stehen dort weiter: 18 Lints unter `RUSTFLAGS=-D warnings` — `src/archivar/root.rs:410` `manual_is_multiple_of`, `root.rs:632` `int_plus_one`, `src/mathematikerin/pc.rs:20` `type_complexity`, `src/mathematikerin/receiver.rs` 15× `excessive_precision` (Test-Referenzliterale). **In diesem Atom getilgt** (`is_multiple_of(31)`, `i + 4 + needle.len() < buf.len()`, `type ScreenedSkeleton`, f64-treue Literale); `cargo check` 0/0.
- **Blockade:** kein abgeschlossener Lauf am neuen HEAD.
- **Braucht:** `ci_manage log <ci-gate-id>` auf dem nächsten abgeschlossenen Lauf → grün/rot; bei rot die verbliebenen Lints tilgen.

### Mycelium-φ-Blöcke — Serien-Manifest-Format offen
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Entwurf des Serien-Manifest-Formats
- **Lage:** (gemessen 2026-10-11) der **ecad-Riss ist gelöst** (netloc = Quellen-Identität `ecad.eu`, origin = Distributionshost). Der Mycelium-adressierte Block gegengeprüft (general): `terms` aller 6 Klima-Quellen deckt die Behauptung (PD SURFRAD; non-commercial ECAD/WorldClim; CC-BY-4.0 DWD/AODN/KNMI); `ttl` 5/6 mit Compiler-`TTL_S` belegt (SURFRAD 604800 · ECAD 604800 · DWD 2592000 · WorldClim 315360000 · KNMI 86400). **AODN-Riss:** `ttl 1` (`phi/sources.φ:8`) = Datensatz-Kadenz (`aodn_compiler.rs:586`), kein fester Harvest-Refresh.
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

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch",
2026-10-07) trägt Commit und Push. **Dieses Atom (Mountain 308):**
- **ci-gate-Lints getilgt** — 18 Lints unter `-D warnings` am HEAD `c81493b1f` (gemessen `ci_manage log 38105695194 --all`): `root.rs` 2, `pc.rs` 1 (type-Alias `ScreenedSkeleton`), `receiver.rs` 15 Test-Referenzliterale auf f64-treue Werte. `cargo check` 0/0.
- **omni2-Register-Lüge geheilt** (River-adressierter Block): die sieben `field`-Zeilen des `omni2_serie_1h.bin`-Blocks tragen jetzt die native Kadenz `3600` statt `86400` (der Compiler baut mit `--decimate-min 60` = 3600 s; Beleg `.github/workflows/omni2-cdn.yml:59–62`).
- **FMHY research-data — 6 Resthosts verdiktet** (general-Messung, `--sniff`/`--playwright`): `decline registry/katalog` für `dataone.org`, `rpubs.com`, `www.kaggle.com/datasets`, `www.dbpedia.org`, `www.zooniverse.org`; `decline no-physical-force` für `censoredplanet.org` (Netz-Zensur-Messung, GraphQL). 6 Blöcke in `phi/declined_sources.φ` (canonical, url asc, 1528 `url`-Zeilen).
- **Klima-Blöcke gegengeprüft** (general): terms 6/6 gedeckt; ttl 5/6 mit Compiler-`TTL_S` belegt; AODN-Riss benannt.
- **ref_hz-Riss gemessen** (general): Format-Zweig ist nicht der Diskriminator; der ~104×-Riss sitzt im else-Zweig über `data_type`.
- **owner-drift geheilt**: die zwei `Bindung linie:mycelium`-Punkte aufgelöst — GIC-Paper + Serien-CI-Wait nach `state/zustand/wartend.φ`, das Serien-Manifest-Format bleibt `eigen`.
- Standing-Pass-Bezug: `state/zustand/standing-pass.md` (HEAD-Bewegung `6f0e902fd` → `300ac20281`); Stand zitiert, nicht kopiert.
- Geteilter Baum: `tools/gate/src/bin/commit_check.rs` (Formatter) ist fremd modifiziert — nicht angefasst/committet.
- Eigene Pfade dieses Atoms: `src/archivar/root.rs` · `src/mathematikerin/pc.rs` · `src/mathematikerin/receiver.rs` · `phi/sources.φ` · `phi/declined_sources.φ` · `state/zustand/wartend.φ` · diese Übergabe · der Archiv-Move folge307.
