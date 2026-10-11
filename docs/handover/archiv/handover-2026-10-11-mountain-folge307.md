<!--
  title: Handover — Mountain-Folge 307 (2026-10-11)
  session: Mountain-Linie in einem Pass — GWOSC-sha256 gesetzt, particle-cern Elementliste gebaut, FMHY 54 Verdikte, Asservatenkammer-Träger gemessen
  class: handover
  date: 2026-10-11
  sha256: 567bb0b78551e03ac2b0b3d9eccc653c71f24a9ab9393863afd8b80f122ac24a
  status: live
-->
# Handover — Mountain-Folge 307 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge306.md` (→ `archiv/`).
flash only, kein pro/max. Operator-Trigger dieser Session: die Linien-Start-Direktive
(„Mountain in einem Pass, kein Planungstheater") — Bekanntes direkt bis zur Kante
gearbeitet, 4 begrenzte Dispatches (grind-flash ×2, general ×2).

## Burn: open 0.0 · close 0.41 · cap 0.5 — Grund: 4 Dispatches (grind-flash ×2 + general ×2); gemessen `session_burn`-Fenster 1.2828 (22 Sessions) → 1.6948 (26 Sessions), Delta +$0.41; die laufende line-Session ist noch nicht gezählt; deepseek-flash, kein pro/max

## Offen (aufgeschlüsselt)

### Beobachtungsoperator + Fit — Schritt 2 (Residuum verhören)
- **Status:** eigen (Bau/Messung) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt — Schritt 2 (Residuum verhören: Kette falsch / Modell trägt Rest / Beobachtung trägt Artefakt)
- **Lage:** (gemessen 2026-10-11, folge306) P11 24 808 modelliert / P10 59 363; A ≈ +f/c je Station (Vorzeichen gemessen), Downlink ≈2×; `ref_hz`-Skala gemischt (`odf.rs:61–86` trifft beide Zweige, ~100×); erstes Modell (`obs=A·ṙ+B_Pass`) trägt die Serie nicht bis Hz (RMS 1e2–1e5 Hz); das Tool führt Uplink/Station/Lichtzeit bereits — es fehlen die **Medien** (Troposphäre/Ionosphäre). `fit-residuum`, kein Blindtest.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** (1) den `ref_hz`-Skalen-Riss entscheiden — welcher Parser-Zweig die Pioneer-ODF trifft (`odf.rs:61–86`); (2) Schritt 2 — drei Lesarten (Kette/Modell/Beobachtung); (3) Medien in die Kette.

### particle-cern — Elementliste gebaut; Baskets offen
- **Status:** eigen (Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt
- **Lage:** (gemessen 2026-10-11) `parse_streamer_elements(obj, start, n_members)` gebaut — `src/archivar/root.rs:861–911`, `StreamerElement` (`:52–59`), Header trägt `elements_offset`; `cargo check` 0/0. **Zwei benannte Risse:** (a) das Element-Präfix ist als TNamed gelesen (zwei `read_version` + zwei tstrings) statt der `:829–833`-TObjArray-Vorlage — physisch die TNamed-Lage; (b) der Advance nutzt den `[byte-count|mask]`-Frame, **nicht** den vollen Body (Array-Tail-Member unhandled). TStreamerElement-Versionen < 2 unhandled.
- **Blockade:** keine.
- **Braucht:** die **Baskets** `fBasketBytes`/`fBasketEntry` (eigener Schritt, nicht mitbauen); der Branch-Decode (`gap:particle-cern`) bleibt offen.

### FMHY research-data — 54 Verdikte geschrieben; 6 Datenhosts für den Content-Check
- **Status:** eigen (Quellen-Verdikt) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Verdikt-Schritt je Resthost
- **Lage:** (gemessen 2026-10-11) die „Academic Papers"-Klasse (`state/future/source-kandidaten-fmhy-2026-10-10.md:7`) ist als **literature/discovery**-Klasse gemessen (`--verdict` 69 Hosts, 16 `--sniff`) — **54 Blöcke** `decline registry/katalog` in `phi/declined_sources.φ` geschrieben (canonical, **1522** Blöcke, `register_sort` exit 0); Paywalls (`sciencedirect`/`springer`/`link.springer`) im Block benannt; bereits registrierte Hosts (openalex/gbif/zenodo/re3data/openaire/crossref/dimensions/mendeley/medrxiv/huggingface/osf/worldbank/semanticscholar) übersprungen.
- **Blockade:** 6 Hosts sind **keine** Literatur-Portale — der Content-Check fehlt: `dataone.org`, `kaggle.com/datasets`, `dbpedia.org`, `rpubs.com`, `censoredplanet.org` (Netz-Zensur-Messung, evtl. echter Arm), `zooniverse.org`.
- **Braucht:** je der 6 ein Content-/Feld-Check (`--sniff`/`--playwright`) → Verdikt-Zeile oder `archive_search`-Arm.

### Asservatenkammer — Träger gemessen (5 Rest-Marker aufgelöst)
- **Status:** eigen (Register/Träger) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt je offenem Marker
- **Lage:** (gemessen 2026-10-11) je Rest-Marker entschieden: `globalfloods.eu`/GloFAS **CARRIED** (`phi/declined_sources.φ` `decline model-forecast`, `url …/glofas-forecasting/?format=json`) · `opensky-network.org` **CARRIED** (ebd. `decline no-physical-force`, `…/api/states/all`) · Airbyte-Totalzähler (`docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md:159`) **descoped** (Vergleichs-Untergrenze, kein Quellen-/Feld-Anspruch) · MiMo Studio (`docs/surveys/survey-2026-10-08-fmhy-research-landscape.md:53`) **descoped** (`aistudio.xiaomimimo.com` HTTP 000; anderer Host als der ui-seat) · 44-Domänen-Liste (`docs/surveys/survey-2026-10-09-domaenen.md:86`) **uncarried** (private Future-Dossier, am Baum nur 6 Trägerfamilien) · medizinisches Resultat (`docs/surveys/survey-2026-10-09-domaenen.md:93`) **gehört zur Exposom-Matrix** (TE-Lauf, kein Descope). Research-api-mcp vier Marker: §Status von `docs/surveys/survey-2026-10-08-research-api-mcp.md` trägt sie bereits gemessen (Perplexity live · Consensus live · Elicit descoped · SciSpace pending). **Träger:** die Asservatenkammer-Doks bleiben mit ihren offenen Markern getragen: `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` · `docs/surveys/survey-2026-10-08-open-sources-delta.md` · `docs/surveys/survey-2026-09-03-orphan-verdicts.md` · `docs/surveys/survey-2026-10-09-redistribution-alternativen.md` · `docs/surveys/survey-2026-10-09-domaenen.md` · `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` · `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` · `docs/surveys/survey-2026-10-07-fmhy-forschungsschicht.md` · `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`.
- **Blockade:** keine.
- **Braucht:** die zwei `uncarried`-Marker der `domaenen`-Survey tragen (Register/Träger) oder descopen; Doku-Annotation selbst ist der Rest.

### Mycelium-φ-Blöcke — ecad-Riss gemessen; Serien-Manifest offen
- **Status:** eigen (Register/Verdikt) | **Bindung:** linie:mycelium (CI-Lauf)
- **Trigger:** je Block der erste CI-Lauf (sha256)
- **Lage:** (gemessen 2026-10-11) der **ecad-Riss ist aufgelöst**: die Blöcke tragen netloc = Quellen-Identität (`ecad.eu`), `origin` = Distributionshost (`knmi-ecad-assets-prd.s3.amazonaws.com`) — die `--verdict`-Messung auf beide (Portal: direct no-response, wayback 200; S3: 403 direct, wayback ohne CDX-Snapshot) widerspricht dem nicht; der Block `phi/sources.φ` (`ecad`, sha256 gesetzt) ist vollständig. Offen bleibt das **Serien-Manifest**: die Compiler erzeugen je-Akt-Instanzen, die Blöcke sind Workflow-Default-Repräsentanten.
- **Blockade:** ohne Serien-Manifest trägt jeder Block nur einen Default-Tag.
- **Braucht:** Serien-Manifest-Format (Mountain) / je Block der erste CI-Lauf (Mycelium).

### ci-gate-Rot — Trigger: abgeschlossener Lauf
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am HEAD
- **Lage:** (gemessen 2026-10-11) am HEAD `c2c9a61ae` laufen `ci-gate` `38104340829` (queued), `38104044418`/`38103781955`/`38103736287` (in_progress) — **kein abgeschlossener Lauf**. Die Mountain-Heilung `mci.rs`/`parcorr.rs`/`receiver.rs` und die River-Heilung `pc.rs` sind committet.
- **Blockade:** kein abgeschlossener Lauf.
- **Braucht:** `ci_manage log <ci-gate-id>` auf dem nächsten abgeschlossenen Lauf → grün/rot; bei rot die verbliebenen Clippy-Lints unter `-D warnings` tilgen.

### canon.φ fällt — Release greift (Dependency erfüllt)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** die frische `commit_check`-Release lokal (nicht mehr stale)
- **Lage:** (gemessen 2026-10-11) `tools-build 38103741757` = **success** — die Release mit `REGISTER_CLASSES` (7 lebende Register, `src/gate/commit_gate.rs:2115`) liegt vor. `phi/` = 8 Dateien (7 Register + `canon.φ`); das Gate trägt die Klassen fest im Code, canon ist Tautologie. Lokal ist `tools/gate/src/bin/commit_check.rs` von einer parallelen Linie modifiziert (Formatter) — nicht angefasst.
- **Blockade:** die lokale `commit_check`-Binary muss die frische Release greifen (`bin/.tools_ensure`), sonst blockt der stale Hook das Entfernen.
- **Braucht:** `bin/.tools_ensure --all` erneut messen; bei frischer Binary `phi/canon.φ` entfernen → `phi/` = 7 Register.

### PEP + Tudat — Schritt 3 (mehrere Bahnen) offen
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Schritt 3 — weitere Zeugen (Merkur/Venus Radar+VEX-Ranging) durch die Kette
- **Lage:** (gemessen 2026-10-11, folge306) Schritt 1 gefahren (Pioneer-10 durch die Kette, `fit-residuum` gegen Horizons-DE440); vier Häuser (DE440/INPOP19a/EPM2021/PETREL19) als `ephemeris_*` registriert; Träger-Doks `docs/concepts/eigene-ephemeride.md`, `docs/surveys/survey-2026-10-10-ephemeris-quellen.md`.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Schritt 3 — der Operator auf weiteren Zeugen gegen weitere Häuser; Kette CI-nah verankern.

### FMHY-Routing — Riss 4 bot-gated
- **Status:** eigen (Architektur) | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss
- **Lage:** (gemessen 2026-10-10, unverändert) Grenze ist die Manifestations-Achse: Query → `archive_search`-Arm; Messwert → `phi/sources.φ` + Compiler; Operator-Werkzeug → `tools/`; Blick/Portal → Lead. Riss 4 (Overpass — HTTP 406 direkt+Proton → bot-gated `pending`). Mindat wartend.
- **Blockade:** Riss 4 bot-gated.
- **Braucht:** Riss 4-Abschluss (bot-gated) oder Descope.

### phi/pipeline nach Verbraucher trennen — Register + Vorrat offen
- **Status:** eigen (Bau/Register) | **Bindung:** eigen
- **Trigger:** je Klasse der Konsumenten-Umbau (`cargo check` 0/0) → dann verschieben/entfernen
- **Lage:** (gemessen 2026-10-11, folge306) 3 Naturen getrennt (Deskriptoren `d039f7618` · Compiler-Eingaben `dc9a2e792` · Test-Fixtures `01449065f`); descoped: `research/`(98) · `stage/`(33) · `queue/`(2) · `leads.φ` · `tap_index_*`(72) · toter Konsument `tap_index_merge.rs` (`1a885775f`). Offen: (a) **Verdikt-Register** `ledger.φ`/`index.φ`/`decline_lens.φ` (die `ledger.φ`-`pending`-Zeilen → Warte-Register); (b) **Kandidaten-Vorrat** `probe_*.φ`, `library.φ`, `frame_registry.φ`, `master_urls.txt`.
- **Blockade:** (a)/(b) brauchen den Konsumenten-Umbau voraus; (b) ist ein Live-Arm-Umbau (`discovery.rs`).
- **Braucht:** je Klasse **ein Atom**: Konsument umbauen (`cargo check` 0/0), dann verschieben/entfernen.

### phi/ Ordnung — harvest.φ auflösen (nächster Atom)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** `harvest.φ`-Auflösung
- **Lage:** (gemessen 2026-10-11) `phi/` = 8 Dateien; Werkzeug-Eingaben zu ihren Werkzeugen gezogen (`ff6f4315f`); Gate trägt `REGISTER_CLASSES` fest (`src/gate/commit_gate.rs:2115`). Offen: `harvest.φ` (77 Formate) → ~60 `*-cdn.yml` (Format/Timeout/Args/Idempotenz in die Jobs; `harvest-dispatch.yml` scannt statt zu lesen; heute deklarieren nur 4 Jobs ein `format:`). Zwei Risse: 533 `terms unknown` in `sources.φ` · `docs/SOURCE_PORT.md` beschreibt noch die descopte `phi/pipeline/`-Maschinerie.
- **Blockade:** keine.
- **Braucht:** je Format den Workflow-Input setzen; `SOURCE_PORT.md` nachziehen.

### queue/stage + ledger.φ — Descope-Zeile je Klasse
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** das Descope-Verdikt je Klasse
- **Lage:** (gemessen 2026-10-11, folge306) `phi/pipeline/queue/` 2, `stage/` 33, `tap_index_*` 72, `ledger.φ` 6 offene Zeilen. Ohne Konsumenten-Umbau hinterlässt Löschen schreibende Pfade (`port.rs`, `leads_merge`, `probe_sweep`).
- **Blockade:** schreibende Pfade.
- **Braucht:** je Klasse Messung + Descope-Zeile; `phi/canon.φ` von den `tap_index_*`-Zeilen lösen.

### Keine Leads-Datei — live suchen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** `discovery.rs` live + `leads_merge` entfernt
- **Lage:** (gemessen 2026-10-11, folge306) `phi/pipeline/leads.φ` + `leads_merge.rs:70/90/91` + `discovery.rs`. `leads_merge` liest den Kandidaten-Vorrat.
- **Blockade:** `leads_merge` liest gerade den Kandidaten-Vorrat.
- **Braucht:** `discovery.rs` auf live (`archive_search`), `leads_merge` + `leads.φ` entfernen.

### Flyby-Kette — termin 2026-11-01
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09) 157 ODF-Referenzen; `doppler.rs` absent; Wahrheit `state/zustand/wartend.φ`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### iEEG — registriertes Wort maßgeblich
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort, das den Riss über 2026-10-06 hebt
- **Lage:** (gemessen 2026-10-11) iEEG = privates Experiment (`state/zustand/wartend.φ`), kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es.

### GIC-Paper — Trigger: Mycelium-Artefakt
- **Status:** wartend | **Bindung:** linie:mycelium
- **Trigger:** `te-bias-n`-Lauf `38038722712` Abschluss → Mycelium meldet den Ground-Truth-Abschnitt
- **Lage:** (gemessen 2026-10-11) `te_ground_truth` in `.github/workflows/te-bias-n.yml:48`.
- **Blockade:** kein CI-Ergebnis.
- **Braucht:** nach Mycelium-Meldung — Paper §3.5/Abstract/§7 nachziehen.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt ist Annahme-Akt
- **Status:** wartend | **Bindung:** eigen (Mountain-Feder)
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile (Operator-Hand; Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-10) kein offener Datensatz trägt eine im Datensatz gemessene Koordinate; x-Kern-Serien stehen wie gemessen; das medizinische Resultat (`survey-2026-10-09-domaenen:93`) gehört hierher (TE-Lauf).
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt — dann `phi/sources.φ`-Zeile, dann Te-Paar-CI-Feed (Mycelium).

### api.sensor.community — zulassen, Zielzeile unbenannt
- **Status:** eigen (Register) | **Bindung:** eigen
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
2026-10-07) trägt Commit und Push. **Dieses Atom (Mountain 307):**
- **GWOSC-Strain:** `ci_manage view 38096865460` = success → `sha256 155df05741d0e79b11d228dd41f91b6c303a9dde82aea01fcd5bb953449beb14` in `phi/sources.φ` (losc-Block) nachgetragen; Register canonical (2730 Blöcke). Riss benannt: der Compiler druckt `format losc-strain`, der Block trägt `format losc` (Manifestations-Direktive, Mycelium).
- **particle-cern:** `parse_streamer_elements` gebaut (`src/archivar/root.rs:861–911`, +66 Z.), `cargo check` 0/0.
- **FMHY research-data:** 69 Hosts gemessen; 54 `decline registry/katalog`-Blöcke in `phi/declined_sources.φ` (canonical, 1522 Blöcke).
- **Asservatenkammer:** 5 Rest-Marker aufgelöst (2 CARRIED, 2 descoped, 1 → Exposom-Matrix).
- **ECAD-Riss:** aufgelöst (netloc=Identität, origin=Distribution).
Standing-Pass-Bezug: `state/zustand/standing-pass.md` (HEAD-Bewegung `6f0e902fd` → `c2c9a61ae`, Mycelium-302 lidar). Geteilter Baum: `tools/gate/src/bin/commit_check.rs` (Formatter) und die staged `tools/harvest/src/bin/*lidar_coverage.rs` + `.github/workflows/open-lidar-data-coverage-cdn.yml` gehören fremden Linien — nicht angefasst/committet.
Eigene Pfade dieses Atoms: `phi/sources.φ` · `phi/declined_sources.φ` · `src/archivar/root.rs` · diese Übergabe · der Archiv-Move folge306.
