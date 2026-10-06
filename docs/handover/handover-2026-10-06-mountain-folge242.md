<!--
  title: Handover — Mountain-Folge 242 (2026-10-06)
  session: Mountain-Folge 242
  class: handover
  date: 2026-10-06
  sha256: f7590296f3afc4402ff9a5ebbbb7a74443636a06726e2552a244dddc57827b0b
  status: live
-->
# Handover — Mountain-Folge 242 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`, gelesen
2026-10-06). Diese Session konsumierte `handover-2026-10-06-mountain-folge241.md`
(→ `archiv/`) und maß die adressierten Blöcke `future-183`, `mycelium-236` gegen den
Baum: **alle erledigt** (iEEG-Bindung `sources.φ:3584-3589`; AGrav/CEEIN registriert
`sources.φ:9071`/`:9239`; gistemp/godas `sha256` `sources.φ:18870`/`:17141`;
Chandrayaan-Felder `sources.φ:10025-10027`; JAXA-Reader descoped
`blocked_sources.φ:96`; clippy `units.rs` committet) — nichts zu falten, die Sender
können die Blöcke streichen. Zwei `grind-flash`-Dispatches, kein pro/max.

**Nachtrag (Operator: „kannst du das bitte fixen?").** Der Fink-`blockiert`-Stand war ein
**Messfehler** (falscher Host `api.fink-portal.org`); `api.lsst.fink-portal.org` antwortet
2026-10-06 HTTP 200 für `/`, `/schema`, `/sources`, `/objects`, `/cutouts`, `/conesearch`.
Die allgemeine Query ist `/api/v1/conesearch` (nicht `/objects` — das ist `diaObjectId`-
gebunden) und wird schon von `skydirection_compiler` (`FINK_CONE`) geerntet, als Zeuge
`s2-direction` (`witnesses.φ:11`) registriert — **kein `sources.φ`-Block nötig** (Fink ist
ein Skydirection-Zeuge, kein `sources.φ`-Ursprung). **Cutout-FITS-Reader gebaut:**
`tools/measure/src/bin/fink_cutout_probe.rs`, live gemessen (`POST /api/v1/cutouts`,
`diaSourceId=314002968168367863` → 34560 B FITS, NAXIS 30×30, BITPIX −32, Apertur-Summe).
trishuli: Live-Route gemessen — `POST /site/getRiverWatchBySeriesId_Single`
(csrf + `seriesid=23251`) → `status:success` mit **leerer** Serie (`river=[]`) über
Perioden 1–4; Kontrolltest Devghat 265 (`seriesid=4140`) → 24 Zeilen ⇒ Abfrage valid,
4913 stationstod (Flood 26.08.2026). **Operator-Wort danach: die zwei Live-Nachbarn als
Triangulation.** Gebaut: `dhm_gauge_compiler.rs` (gemessen station 4657 → 60 Zeilen,
`cargo build` 0/0), Arm `dhm_stage` (`extract.rs`), zwei `phi/sources.φ`-Blöcke
(Dhunche `4657` 2.16 m / Bhorle `4661` 3.80 m); die tote 4913-Quelle entfernt. Der
CDN-Workflow für die zwei Assets steht adressiert unter `## An mycelium`.

## Burn: open 0.0000 · close 0.1894 · cap 0.25 (Operator eröffnete das Atom erneut: Fink+trishuli, dann Schwarm) · Grund: flash-first — Line-Session $0.1387 + zwei `grind-flash`-Dispatches ($0.0325 fink-Arm, $0.0182 cutout-Reader) + Schwarm (12 Stimmen, frei), kein pro/max

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„verschleppen und nicht eigenes ist verboten" | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-02 | Operator (Session, Mountain 225)
„1 ja bitte" — privater TE-Pfad, Lauf lokal/silent, nie CI | 2026-10-02 | Operator (river-folge82)
„ja bitte" — `descoped` aus `blocked_sources.φ` auflösen | 2026-10-03 | Operator (Session, Mountain 229)
„kannst du dich bitte darum kümmern? 9 blocked parser-def" — als Weberin-zweite-Linie führen | 2026-10-03 | Operator (Session, Mountain 229)
„fixe die aktuellen Medizinische Datenquellen aber setze den rest auf on hold" | 2026-10-04 | Operator (Session, Mountain 230)
„Macht EFD/HPM/SCM Sinn? — Ja." | 2026-10-04 | Operator (Session, Mountain 230)
„also bitte alles umsetzen ich möchte nicht dass du etwas in die nächste runde nimmst was jetzt von agenten bearbeitet werden kann" | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es dafür wirklich pro?" — flash-first; der Katalog-Rest per flash geschlossen | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es pro?" — flash-first bestätigt | 2026-10-04 | Operator (Session, Mountain 233)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-05 | Operator (Session, Mountain 235)
„was fehlt hast du in die secrets local geschaut?" — vorhandene Keys nutzen; kein „Operator-Hand" ohne Messung | 2026-10-05 | Operator (Session, Mountain 234)
„braucht es max?" — flash-first erneut bestätigt; ExoMars-Parser per grind-flash gebaut | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und kannst du dir das bitte ansehen?" — flash-first; die zwei Punkte-Listen gegen den Baum messen | 2026-10-05 | Operator (Session, Mountain 235)
„messe nochmal den aktuellen zustand dann commit" — Atom 2: neue adressierte Blöcke falten, FMI-GIC-fein-grain bauen, em-Apertur messen, committen | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und max?" — flash-first bestätigt: alle Kanal-/Serien-Arme per grind-flash/general geschlossen, kein pro/max | 2026-10-05 | Operator (Session, Mountain 236)
„bitte gib das dem rat den tauchern für wissenschaft und forschung und den 3 online stimmen" — Contract-Frage (Sentinel vs. Presence-Bit) an Rat + research-max + UI-Stimmen | 2026-10-05 | Operator (Session, Mountain 236)
„brauchen wir überhaupt pro für den rat/council … in dateien steht veraltet wann pro angebracht ist" — Council → flash/low; pro/max nur noch Eskalation nach gemessener flash-Fehllage | 2026-10-05 | Operator (Session, Mountain 236)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-05 | Operator (Session, Mountain 237)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 238)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 239)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 240)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 241)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 242)
„kannst du das bitte fixen? Fink … trishuli …" — Fink-Route (Messfehler) + trishuli klären | 2026-10-06 | Operator (Session, Mountain 242)
„bitte lasse darauf nochmal den schwarm los und gib mir die frage für glm und claude" | 2026-10-06 | Operator (Session, Mountain 242)
„die beiden live nachbarn wären doch als triangulierung gut?" — Dhunche 4657 + Bhorle 4661 als Trishuli-Ingestion | 2026-10-06 | Operator (Session, Mountain 242)

## Offen (aufgeschlüsselt)

### Exposom-x-Homes (Anfrage Mycelium 240) — 4 registriert, 2 feld-descoped
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine — Bau (autonom)
- **Lage:** (gemessen 2026-10-06) 6 x-Homes aus `handover-2026-10-06-mycelium-folge240.md:52/55`.
  **4 feld-aufgenommen** (per-Zelle-Geo-Arm + `phi/sources.φ`-Block, Asset pending, kein `sha256`
  bis zur Manifestation): Licht `black_marble_vnp46a3_nightlight` (MAGIC `NTL1`, Epoche = Granuledatum),
  Pollen `openmeteo_pollen_axis_value_text` (73 Zeilen gemessen), gebaute Umwelt `ghsl_built_s`
  (MAGIC `GHS1`, Wert m² nicht Fraktion, Epoche 2020), Ernährung `usda_fara_low_access` (MAGIC `FAR1`,
  Census-Tract-Centroide gejoint, 46 205 Tracts, Epoche 2019). **2 feld-descoped:** O*NET
  `work_context.csv` (Date-Spalte, aber keine Position — occupations-level) und Exposome-Explorer
  `concentrations.csv.zip` (Population/Biomarker, keine Zeit/kein Ort) — **ohne Position kein 4D-Sample**;
  sie bleiben Referenz, kein `sources.φ`-Ursprung (Verdikt Mountain 2026-10-06).
- **Blockade:** keine für die 4 aufgenommenen; offen ist nur die CDN-Manifestation (`## An mycelium`).
- **Braucht:** die vier CDN-Workflows + `sha256`-Nachzug; die zwei Descopen sind das Verdikt.

## An mycelium

Origin: mountain-folge242. **DHM-Pegel-CDN (die zwei Trishuli-Nachbarn):** die neuen
Quellen `dhm_4657_stage.txt`/`dhm_4661_stage.txt` (Tag `dhm.gov.np`) brauchen einen
Workflow `dhm-gauge-cdn.yml`, der `dhm_gauge_compiler --station 4657|4661 --period 1
--ci-mode` **täglich** fährt (curl-Cookie-Jar + csrf-freier POST intern). Compiler
gebaut (`tools/harvest/src/bin/dhm_gauge_compiler.rs`), Lauf station 4657 → 60 Zeilen,
`cargo build -p omegaflow-harvest --bin dhm_gauge_compiler` 0/0; Arm `dhm_stage` deckt.

**VNP46A3-Nachtlicht-CDN:** für die registrierte Licht-Quelle
(`format black_marble_vnp46a3_nightlight`) braucht es `vnp46a3-cdn.yml`, das eine
VNP46A3-Granule mit `EARTHDATA_EDL_TOKEN` holt und `vnp46a3_compiler --label
allangle_composite_snow_free --ci-mode` fährt (Tag `data.laadsdaac.earthdatacloud.nasa.gov`).
Der `sha256`- und `url`-Nachzug in `phi/sources.φ` folgt nach dem ersten `--ci-mode`-Lauf.

**Drei weitere Exposom-CDN-Workflows:** `openmeteo-pollen-cdn.yml`
(`openmeteo_pollen_compiler --lat 52.52 --lon 13.405 --variable birch_pollen --ci-mode`,
Tag `air-quality-api.open-meteo.com`, stündlich), `ghsl-cdn.yml`
(`ghsl_compiler --stride <N> --ci-mode`, Tag `jeodpp.jrc.ec.europa.eu`; Achtung: das
GeoTIFF ist ein 2.14 GB deflate-Member → einmaliger Jahreslauf), `usda-fara-cdn.yml`
(`usda_fara_compiler --ci-mode`, Tag `ers.usda.gov`). Die `sources.φ`-Blöcke stehen;
`sha256` folgt je nach erstem Lauf.

## Träger (Prosa, eigene)

- `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (`class: survey`, Header-sha `feb28078…`) — Pool disponiert.
- `docs/specs/sources-v2-spec.md` (`class: concept`, Header-sha `11c6db3c…`).
- `docs/surveys/survey-raetsel-bestand.md` (`class: survey`, Header-sha `524d61dc…`).
- `docs/blatt/blatt-pioneer-floor-falsifikation.md` (`class: sheet`, Header-sha `5bb1696b…`).
- `docs/concepts/kybernetische-astrophysik.md` (`class: concept`).
- `docs/concepts/tools-map.md` (`class: concept`).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (`class: survey`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (`class: survey`, Header-sha `ac672e3e…`).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern
  (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI.
  Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün;
  offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz. Der
  private Wort-Laut nur im privaten `state/operator-gespraeche/`.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und
Push. `/consent` ist der session-weite Consent, nie das Commit-Wort.
