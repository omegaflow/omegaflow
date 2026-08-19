# Source-Port: Der eine Pfad

Selbsttragend. Interpretierbar von einer Session mit null Vorkontext. Dieses
Dokument ist der einzige Einstiegspunkt für alle Source-Arbeit (Grind, Port,
Kuration). Eine neue Session liest genau dieses eine Dokument.

## 1. Was die Arbeit ist

Die gold/pre-cdn-Korpora (alte Grammatik, teilweise gitignored-historie) werden
mechanisch nach v6 überführt (Konvertierung), gegen die lebenden APIs
verifiziert (Probe + Extract) und in eines der zwei Register disponiert.
Daneben: neue Kandidaten aus dem Recherche-Bestand grinden. Der Reichtum sind
nicht nur die URLs — Katalog-Inventare, Domänen-Maps und Kandidaten-Listen
liegen im Bestand und sind im Ledger registriert. Kein offener Posten ohne
registrierten Ort.

## 2. Pfadkarte

| Pfad | Was es IST |
|------|------------|
| `phi/pipeline/queue/master.φ` | DIE eine Master-Datei: 13 alte Korpora dedupliziert gemergt (7.430 Blöcke). |
| `phi/pipeline/queue/sources_potential_*` | Join-Paar (reichste Extract-Parameter) für die Lost-Blocks. |
| `phi/pipeline/queue/grind_*` | Offene Block-Drafts (ArcGIS, VirES, ESA, TerraPulse, NASA/DONKI, …) mit Disposition ausstehend. |
| `phi/pipeline/stage/` | Konvertierungs-Ausgänge `<korpus>_converted.φ` + Sweep-Ergebnisse `staging_verified.φ` / `staging_void_ledger.txt`. |
| `phi/pipeline/ledger.φ` | DAS Zustands-Register. Jeder offene Posten mit Zustand. |
| `phi/pipeline/index.φ` | Der Index der zu portierenden Dateien (Zustand + Blockzahl). Regenerierbar. |
| `phi/pipeline/master_urls.txt` | Die deduplizierte URL-Liste (ohne live/declined) + `netloc.txt` (Domänen-Counts). |
| `phi/pipeline/prompt.φ` | Port-Vorlage für Agenten (Korpus → Disposition). |
| `phi/pipeline/katalog/` | Katalog-Inventare (tap_index*, b2find, eogateway, VirES, ArcGIS, TerraPulse, ESA, archeology_gaps). |
| `phi/sources.φ` | Das kanonische Register (Annahme-Ziel). |
| `phi/dead_sources.φ` | Dispositionen: `dead`/`decline`/`integrated`. |
| `phi/blocked_sources.φ` | Dispositionen: `key-needed`/`parser-def` — blockiert, gewollt. |
| `/home/johannes/projects/archive/` | Externes Archiv: `archeology/` (pre-cdn) + `phi-research/` (batches, probe_batches, Dispositionen). |
| `docs/concepts/SOURCES_V2_SPEC.md` | Die Kontroll-Spec (Grammatik, τ-Gate, Force-Unit-Registry, File-Regeln). |

## 3. Die zwei Register

`phi/sources.φ`: kanonisches Format nach SOURCES_V2_SPEC §1.0 — Blöcke
sortiert nach `ttl` (aufsteigend), dann nach `url`; keine Kommentare; eine
Leerzeile zwischen Blöcken; Direktivenreihenfolge: `url`, Blockzustand
(`ttl`, `format`, `header`, `post_body`, `at`/`on`), Fanout-Familie, dann
Extracts in Abhängigkeitsreihenfolge.

`phi/dead_sources.φ`: Einträge sortiert nach `url`; eine Dispositionszeile,
eine `url`-Zeile, eine `note`-Zeile; Leerzeile zwischen Einträgen; keine
Duplikate pro URL. Enthält NUR `dead` (abgeschaltet) + `decline`
(nicht-physikalisch/kommerziell) + `integrated` (tote URL, live via
Fanout-Route in `sources.φ`).

`phi/blocked_sources.φ`: gleiches Format; `key-needed` (Key frei
registrierbar, `.secrets.local`) + `parser-def` (Gap-Verweis) — blockiert,
gewollt, nicht tot.

## 4. Zustandsmaschine + ledger.φ

Zustände: `ausstehend | verifiziert | void | geparkt | disponiert`.

- `ausstehend` — unberührt, offene Arbeit
- `verifiziert` — Sweep lief, Samples extrahiert, Merge in die Register steht aus
- `void` — Sweep lief, alle URLs void (diagnostiziert) — Korpus ist erschöpft
- `geparkt` — Kandidat wartet auf einen Parser-Gap (Gap im Eintrag benannt)
- `disponiert` — in `sources.φ` / `dead_sources.φ` / `blocked_sources.φ` eingegangen; Eintrag wird entfernt (Git trägt ihn)

Ledger-Format (wie dead_sources.φ, φ-Textformat):

```
<zustand>
<art> <wert>
note <was IST — Herkunft, Umfang, Gap-Verweis>
```

Arten: `queue` (Korpus-Datei), `kandidat` (URL), `bestand` (Recherche-Asset),
`nachlauf` (verlorenes Session-Artefakt), `parser-gap` (P-Liste-Eintrag).
Beispiel:

```
ausstehend
queue phi/pipeline/queue/sources_astro_untested_30-astro.φ
note 30 Blöcke, alte Grammatik — kein URL im kanonischen Register; --gold + Sweep, dann Disposition
```

## 5. Workflow-Prozedur (Trichter: Linse → Probe → Review)

Pro Korpus:

1. `cargo run -- --port phi/pipeline/queue/<korpus>.φ phi/pipeline/stage/<korpus>_converted.φ`
   — mechanische Quellgrammatik→kanonisch-Konvertierung (Lautsignal: der
   Konverter meldet Blöcke, die nicht parse-fähig sind; `source`-Köpfe,
   `method`, `pos` und unbekannte Direktiven fallen — siehe §9).
2. Linse: `cargo run --bin source_scanner -- phi/pipeline/library.φ <katalog.φ>`
   — die gewichtete Tag-Library (Ratssitzungen, 2.358 Tags) wiegt die
   Kandidaten; positive Gewichte sind die Probe-Kandidaten.
3. Probe: `cargo run -- --probe <blöcke.φ>` — fetch → parse →
   `walk_json_probe`-Auto-Draft → `extract`-Verdict. Überlebende (echte
   Samples) nach `phi/pipeline/probe_survivors.φ`, Diagnosen nach
   `phi/pipeline/probe_void.txt`. Befunde: `phi/pipeline/probe_comparison.txt`
   (Linse 57% vs 2% Survivor-Rate, fetch_one == fetch_raw_probe für
   unmanifestierte Kandidaten).
4. Review (Mensch): Survivor-Duplikate gegen `phi/sources.φ` prüfen;
   Force-Gate; echte Neue nach §1.0 in `phi/sources.φ`, Varianten/Modelle/
   Tote nach `phi/dead_sources.φ`; `ledger.φ` aktualisieren; den nächsten
   Batch in `phi/pipeline/probe_batch.φ` nachrücken.
5. `cargo check` 0/0; ein Commit, der TODO.md im selben Schritt aktualisiert.

**CI-Schleife:** `.github/workflows/probe_sweep.yml` (wöchentlich + manuell)
läuft die mechanischen Stufen — Linse über die Kataloge + `--probe` über
`phi/pipeline/probe_batch.φ` — und lädt `probe_survivors.φ` / `probe_void.txt` /
`weights_*.txt` als Artefakte hoch. Die Review bleibt in der Session: Artefakt
herunterladen → Schritt 4 → Commit. Die CI probt, der Mensch prüft.

Per-Block-Kuration (neue Kandidaten, nicht mechanisch):
URL-Templates füllen → `curl`-Erreichbarkeit → Struktur prüfen (200er-HTML ist
kein Daten-JSON) → Force-Gate → Klassifikation → Disposition.

Die kanonischen Register werden nie gemirrort — nur `--verify phi` spiegelt
(CI). `phi/pipeline/` und `phi/pipeline/research/` sind fetch-only (kein Mirror, Quota).

## 6. Grammatik-Kurzfassung

Bindend: SOURCES_V2_SPEC §1. Die lebenden `field`-Formen: 5 Token
(`field <key> <force> <unit> <tau>`), 6 Token (mit Kernel), 9 Token
(`field <key> <name> <kernel> <force> <unit> <tau> <absorption> <advection>`).
Der 3-Token-`field` und die `force`-Direktive werden laut abgelehnt (P01).
τ = 0 schließt das Gate (kein Oszillator, 0 honored). `on <body> <lat> <lon>
[alt]` (Meter), `at <body>`, `body <body>` + `lat`/`lon`-Keys (datengetragen).
Extracts: `map/cmap/rows/flatten`, `first/last/lastrow/lastline/lastobj/
objlast/path/deep/regex/count`, `geojson/cmrpolygon/celestialpolygon/
keplermap/hapi/ephemeris/vectors`; cmap: `ra/dec/plx/pmra/pmdec/radvel/
dist/dist_scale/z`.

## 7. Referenz-Karte

| Dokument | Rolle | Status |
|----------|-------|--------|
| `docs/concepts/SOURCES_V2_SPEC.md` | Kanonische Grammatik, τ-Gate, Force-Unit-Registry, File-Regeln | bindend |
| `docs/reference/FORCE_SYSTEM.md` | 9 Kraftkanäle, IDs, Ausbreitungsgeschwindigkeiten | bindend |
| `docs/reference/URL_TEMPLATES.md` | Template-Variablen (Spatial/Temporal) | bindend |
| `docs/reference/CONSTANTS.md` | Φ, c, Konstanten | bindend |
| `docs/reference/TIME.md` | Zeit-/Epoch-Handling | nachschlagen |
| `docs/concepts/SI_UNITS.md` | Force-Unit-Matrix (SUPERSEDED als Kontrolle, physische Referenz) | nachschlagen |
| `docs/reference/NIST_SP330_tables.md`, `NIST_SP811_units.md`, `ucum-essence.xml` | Unit-Identität (BIPM/UCUM) | nachschlagen |
| `docs/concepts/PARSER_MAGIC.md` | Parser-Intelligenz + offene Lücken (P05/P07/P08) | nachschlagen |
| `docs/reference/naif_body_ids.tsv` | Bodynamen für Frames | nachschlagen |
| `docs/concepts/DOMAIN_COVERAGE.md` | Host-Manifest der Grind-Wellen | nachschlagen |
| `docs/reference/EXTRACT_TYPES.md` | Extract-Enum (veraltet) | SUPERSEDED |
| `docs/concepts/PARSER_EVALUATION_MATRIX.md` | 4-Token-Behauptung (widerspricht P01) | SUPERSEDED |

## 8. Force-Gate + τ + Klassifikation

Force-Gate: `force` deklariert die physikalische Ausbreitung DER MESSUNG
selbst, nicht das Transportmedium der API. Litmus: könnte ein
nicht-menschlicher Organismus ein Sinnesorgan für diese Messung evolvieren?
Automatisch decline: nackte Zählwerte ohne Ereignis-Records, Stationslisten/
Register/Kataloge ohne Messwert, Modell-Forecasts/Reanalysen, Referenz-
Konstanten, aggregierte Indizes (skalar, positionslos), Text-Warnungen,
abgeleitete Satellitenprodukte, geographische Infrastruktur, position-only.

τ: temporal decay des Prozesses in Sekunden; schnell wechselnd → ttl/10,
stabil (Kataloge, Geologie) → ttl; explizites Prozesswissen schlägt beides.
Kernel-Vorgabe pro Force (Konverter): em/gravity → inverse-square, acoustic/
seismic-body/diffusion → gaussian-inverse-square, seismic-surface → erfc,
thermal → exponential-decay, advective → patch-levy.

Klassifikation: (accepted) → `sources.φ`; `parser-def` (Format
unkonsumierbar) → `blocked_sources.φ` mit Gap-Verweis (oder `park/` bei
Block-Draft); `key-needed` → `blocked_sources.φ` mit Key-Marker;
`decline` (Force-Gate) → `dead_sources.φ`.

**Toter Endpoint ist kein Endzustand.** Funktioniert der Endpoint nicht, wird
erst recherchiert: alternative Endpoints, URL-Änderungen (API-Versionen,
Redirects, Pfad-Renames) und Misspellings. `dead 404/400/5xx/dns/timeout`
ist ein Recherche-Auftrag, keine Disposition. Wirklich declined sind nur
drei Fälle: (a) der Anbieter ist komplett abgeschaltet (kein öffentlicher
Nachfolger), (b) die Quelle ist nicht-physikalisch (Force-Gate), (c) der
Anbieter ist kommerziell (Bezahl/Proprietär-Zugang). Erst wenn die
Recherche leer bleibt, wird `dead` mit `note` festgeschrieben, die den
Recherche-Stand nennt (Alternativen geprüft, Fund: keine).

## 9. Fix-Rezepte + API-Fakten (hart erworben)

- `map` braucht datengetragene Position: `lat`/`lon`-Keys oder GeoJSON
  `geometry.coordinates.N`. Positionslose Feeds → skalar (`last <dot.path>`,
  `lastrow`, `lastobj`) an einem Frame.
- `map`-Extract mit nicht-surface Frame (`at sun`) ist eine Fehlkennzeichnung
  — Migration hatte `wgs84`-Blöcke falsch gelabelt; surface-Frame heißt
  `on <body>`.
- ArcGIS: HTTP 200 mit `{"error":{"code":400}}` bei unbekannten outFields →
  `outFields=*` zuerst; Schema-Epochen wechseln (`_1_` vs `_0_`-Felder).
- Open-Meteo: Split-Hosts (`archive-api.`/`flood-api.`/`marine-api.`/
  `air-quality-api.`/`ensemble-api.`), nie `api.open-meteo.com/v1/<x>/v1/<x>`.
- TAP: `FORMAT=csv/json` nicht überall; VOTable-only-Kataloge = parser-def;
  `SELECT TOP 1 *` vor jedem Query; TAPVizieR kennt kein OFFSET/LIMIT.
- HAPI: BGS braucht `format=json` + `Z`-Suffix an Timestamps; VirES kennt
  `id=/start=/stop=` UND `dataset=/time.min=`; Fenster innerhalb der
  Abdeckung wählen (`{yesterday}`-Fenster + ttl 86400 bei Lag).
- ERDDAP: Constraint-Syntax des jeweiligen Servers prüfen (`[(last)]`-Formen
  400en heute oft).
- Header-Werte sind whitespace-gesplittet (keine Spaces); Secrets als
  Einzel-Marker (`header user-agent {TNS_UA}`) — Braces nicht mischen.
- Refused-Befunde des Ladens sind Source-Befunde, nie Parser-Fixes:
  `pos without body directive` → Block trägt `body <body>`; `no reference
  frame` → Block braucht `on`/`at`.
- **Auth ist kein Hindernis** solange die Registrierung frei, public und
  nicht-kommerziell ist (NASA Earthdata, NOAA, Copernicus, GBIF, …). Der
  Parser greift bereits auf `.secrets.local` zu — Key dort eintragen, als
  `{MARKER}` in `url`/`header` referenzieren; `resolve_secret`/`render_headers`
  lösen ihn auf. Kein Code nötig. Kommerzielle/private Keys bleiben declined.
  Re-review-Kandidaten stehen in `phi/pipeline/review_kandidaten.txt`.
- **Jina-Reader ist eingebaut**: URL mit `https://r.jina.ai/` präfixen — der
  Reader umgeht Netzwerk-/Git-Blocks (raw.githubusercontent, Geo-Blockierung),
  und `parse_json` überspringt den Jina-Header (`Title/URL Source/Markdown
  Content`) bereits (Test `test_parse_json_skips_jina_header`). `dead
  dns-unresolved/timeout/unreachable/ssl`-Einträge über das `r.jina.ai/`-
  Präfix erneut prüfen, bevor sie endgültig bleiben.
- **Toter Endpoint → Recherche-Rezept**: (1) Status-/Docs-Seite des Anbieters
  prüfen (Umzug, API-Version), (2) Sibling-Endpoints desselben Netloc,
  (3) URL-Pfad auf Versions-Bumps/Renames, (4) Misspelling gegen den
  Provider-Namen, (5) Jina-Präfix für Netzwerk-Blocks. Nur wenn all das leer
  bleibt → `dead` mit `note`, die den Recherche-Stand nennt.
- Der `--gold`-Konverter übernimmt: `url/format/header/target/catalog/
  flux_from_mag/abs_mag_from/catalog_epoch` direkt; `ttl`; `on/at`;
  numerisches `lat/lon/alt` → synthetisches `on earth`; `map/cmap/rows`;
  `lat_key/lon_key/alt_key/epoch_key` → `lat/lon/alt/epoch`; `field/field_in/
  first/last/count/path/deep` → 9-Token-`field`; `last_row`→`lastrow`,
  `last_line`→`lastline`, `last_obj`→`lastobj`, `regex` (Join), `geojson`
  (tau = ttl/10). **Fällt**: `source`, `method`, `body` (POST), `pos`,
  unbekannte Direktiven — solche Blöcke sind `park/`-Kandidaten mit
  Gap-Verweis (post_body-Migration ist offene Arbeit).
- **Session 2026-08-19 — Teleskop-Endpoints (Jina + Wayback verifiziert,
  volle Log §13):** `nhsa.esac.esa.int` tot → Herschel-Archiv lebt unter
  `archives.esac.esa.int/hsa` (HAIO: `/hsa/aio`); `svom.nscs.ac.cn` tot →
  Datenzugang `svom.ac.cn` (NSSDC), GRB-Notices via GCN
  `gcn.nasa.gov/missions/svom` + FSC `fsc.svom.org/alerts`;
  `www.lhaaso.ac.cn` tot → Site `english.ihep.cas.cn/lhaaso`, Daten-Seite
  `/lhaaso/pdl` (öffentlicher Bulk-Download unverifiziert → pending);
  CHIME-FRB-API retired → CANFAR-DOIs; NRAO `archive.nrao.edu`-TAP
  retired → REST `data.nrao.edu/archive-service/restapi_*`. Erreichbar
  (kein dead): MAGIC-Portal `magic.mpp.mpg.de/public/public-data/`
  (FITS + Low-level-Open-Data — „kein öffentliches Portal“ war falsch),
  HAWC `data.hawc-observatory.org` (Datasets), uGMRT
  `naps.ncra.tifr.res.in` (NAPS, CAPTCHA), AGILE `agile.ssdc.asi.it`
  (MMIA, AGILE-LV3, Kataloge 2AGL/MCAL-GRB/TGF).

## 10. Secrets

`.secrets.local` (Repo-Root, gitignored, 46 Keys, GitHub-Parität) — geladen
via `resolve_asset` (CWD-relativ). Probe-Läufe außerhalb des Repo-Roots
brauchen einen Symlink. `{MARKER}` in URLs wird case-insensitiv gegen die
UPPERCASE-Env-Vars aufgelöst; absent → void + stderr.

## 11. Regeln

- Name = Implementation. Keine Kommentare in φ-Dateien; Wissen lebt in
  Protokoll, Ledger und TODO.
- Ein Commit = ein Häkchen: TODO.md im selben Commit; Erledigtes entfernt.
- `cargo check` 0 Errors, 0 Warnings. Der Sweep-Test ist der Verifikator;
  `cargo check` prüft nur Syntax.
- Die Session ist das Atom: ein Korpus pro Session komplett durch die
  Prozedur — kein halber Port.
- 0 honored: eine void-Diagnose ist eine vollständige Disposition, kein
  Fehler. Absent ist erst nach Recherche absent (`ausstehend` bis dahin).

## 12. Teleskop-Inventar (Session 2026-08-19)

Recherche-Auftrag: alle aktiven terrestrischen + extraterrestrischen
Teleskope mit ihren APIs. Status-Legende: ✓ aktiv in `sources.φ` · ▣
Kandidat in `phi/pipeline/queue/master.φ` · ⛔ in `blocked_sources.φ` ·
● offen/unkuratiert · `pending` = Detail unverifiziert — der nächste
Schritt ist Probe/Klassifikation am vorliegenden Befund, keine neue
Adress-Recherche (0 honored). Force-Gate-Schnellurteil: alle
elektromagnetischen Teleskope messen `em`; GW-Detektoren `gravity`;
CR/ν-Detektoren (Auger, TA, IceCube, KM3NeT, Super-K/JUNO) — Teilchen-
Kanal: Registry-Frage, `pending` (die 9er-Force-Registry kennt kein
Teilchen-Medium; hier nicht entschieden).

### 12.1 Weltraum — Röntgen/Gamma

| Mission | Endpoint | Status |
|---------|----------|--------|
| Chandra | Chaser/Browse (`cxc.cfa.harvard.edu`) | ● |
| XMM-Newton | XSA-TAP (`nxsa.esac.esa.int`) | ● |
| NuSTAR | HEASARC-Browse | ● |
| Swift | ✓ Swift-GRB aktiv; weiteres via HEASARC | ✓/● |
| NICER | HEASARC | ● |
| MAXI | `maxi.riken.jp` / DARTS | ● |
| IXPE | HEASARC | ● |
| XRISM | DARTS | ⛔ |
| Einstein Probe | ESA-AMA-TAP + GCN | ● Lücke |
| SVOM | `svom.ac.cn` + GCN (§13) | ● |
| Fermi | FSSC-LAT | ⛔ |
| INTEGRAL | ISDC | ● |
| AGILE | `agile.ssdc.asi.it` (§13) | ● |
| AstroSat | ISSDC AstroBrowse | ● |
| HXMT/Insight | `hsuc.ihep.ac.cn` / DARTS | ● |
| eROSITA | `erosita.mpe.mpg.de`, DR2 (31.07.2026) | ● Lücke |

### 12.2 Weltraum — Optik/IR/Astrometrie

| Mission | Endpoint | Status |
|---------|----------|--------|
| Gaia | ESA-TAP + ARI; `dr3_stars.bin` | ✓ |
| JWST | MAST-CAOM-TAP (7 `mast_jwst_*` im Queue, §14) | ▣ |
| HST | MAST-CAOM-TAP | ● |
| TESS | MAST | ✓ |
| Euclid | Cosmos-TAP | ● Lücke |
| SPHEREx | IRSA-TAP | ● Lücke |
| CHEOPS | ESA-AMA-TAP | ● Lücke |
| Planck/Herschel-Legacy | PLA/IRSA; HSA-Umzug §13 | ● |
| Roman | Start 30.08.2026 — noch kein Bestand | ● |
| PLATO | Start 2027 — kein Bestand | ● |

### 12.3 Weltraum — Sonne/Heliosphäre

SOHO (VSO) ● · SDO (VSO/JSOC) ● · Parker ✓ (SPDF-HAPI aktiv) ·
Solar Orbiter ✓ Ephemeride aktiv, SOAR ● · STEREO (VSO) ● · Hinode
(VSO/DARTS) ● · IRIS (LMSAL/VSO) ● · Aditya-L1 (ISSDC) ● · GOLD ● ·
GOES/SWPC ✓ aktiv.

### 12.4 Boden — Optik/IR

Rubin/LSST ⛔ (TAP `data.lsst.cloud`, OAuth) · DESI ● · ESO
VLT/VISTA/VST ● (`archive.eso.org/tap_obs` + `tap_cat` — Lücke) · Subaru
SMOKA ● · Keck KOA-TAP ● (Lücke) · Gemini ▣ (`archive.gemini.edu`,
master.φ 40567) · NOIRLab AstroArchive ● · Pan-STARRS ● (Katalog-API —
Lücke) · ZTF ✓ (ALeRCE/Lasair aktiv; IRSA ●) · ASAS-SN ● (Skynet) ·
ATLAS ● (Forced-Photometrie `fallingstar.com` — Lücke) · SDSS ✓,
SDSS-V ● · LAMOST DR11 ● (Lücke) · 2MASS/WISE ⛔ (Gator/allwise).

### 12.5 Radio

ALMA ⛔ (TAP) · NRAO VLA/GBT/VLBA: alter TAP tot, neuer REST
`data.nrao.edu/archive-service/restapi_*` ● · MeerKAT/SARAO ●
(`archive.sarao.ac.za/tap/` — Lücke) · CASDA ⛔ (ATCA/ASKAP/Parkes) ·
LOFAR ● (LTA) · FAST ● · CHIME: FRB-API tot → CANFAR-DOIs ● · HERA ● ·
MWA ● (ASVO) · uGMRT ● (NAPS, §13) · EVN/JIVE ● · e-MERLIN ● · MOJAVE ✓.

### 12.6 Hochenergie/CR/ν/GW + Sonne (Boden)

H.E.S.S. ● (Endpoint offen — nicht im Befund) · MAGIC ● (Portal §13) ·
VERITAS ● (Endpoint offen — nicht im Befund) · HAWC ● (Datasets §13) ·
LHAASO ● (Site §13; Bulk-Download unverifiziert — Klassifikation am
Befund) · CTAO ● · Pierre Auger ● (`opendata.auger.org`) · Telescope
Array ● · IceCube ● (Daten-Releases) · KM3NeT ● · Super-K/JUNO ● ·
LIGO/Virgo/KAGRA ✓ (GraceDB aktiv) · GONG ⛔ · DKIST ●
(`api.dkistdc.nso.edu`, Umzug von `api.nso.edu`).

### 12.7 Größte Lücken (Befund)

SPHEREx IRSA-TAP (offen), eROSITA DR2, ESA-AMA-TAP (Einstein Probe +
CHEOPS), Euclid-TAP, ESO-TAP, ALMA-TAP, MeerKAT-TAP, Rubin-TAP,
Pan-STARRS-Katalog-API, ATLAS-Forced-Photometrie, Keck-TAP, LAMOST DR11.
Stale Register-Einträge: NRAO-TAP (retired), CHIME-FRB-API (retired).

## 13. Jina-Verifikation (Session 2026-08-19)

Auftrag: nicht erreichbare Adressen mit Jina (Key in `.secrets.local`)
oder Web-Archive-Recherche prüfen. Methode: in der Session war kein
Bash-Tool verfügbar → anonyme `https://r.jina.ai/<url>`-Route via Fetch
(der Bearer-Header konnte nicht mitgeschickt werden; das eingebaute
`--probe`-Rezept mit Key steht in §9). 422 = Domain auch bei Jina
unauflösbar. Wayback-Fallbacks: `web.archive.org/web/2026/<url>` (404 =
kein Snapshot → Websearch löste die Domänen-Frage). Key:
`.secrets.local:46` (`JINA_API_KEY`); Muster: `src/main.rs:11405-11421`.

| Adresse | Befund | Korrektur/Neufund |
|---------|--------|-------------------|
| `magic.mpp.mpg.de` | erreichbar | Portal existiert: `/public/public-data/` (FITS, Low-level-Open-Data/VO) — „kein öffentliches Portal“ war falsch |
| `data.hawc-observatory.org` | erreichbar | „Datasets“-Seite; öffentliche Datensätze + Arbeitsgruppen-Kontakt |
| `naps.ncra.tifr.res.in` | erreichbar | NAPS (NCRA Archive and Proposal System), JS-App, CAPTCHA |
| `agile.ssdc.asi.it` | erreichbar | SSDC-Datenzentrum live: MMIA-Archiv, AGILE-LV3, 2AGL/MCAL-GRB/TGF |
| `mast.stsci.edu/docs` | erreichbar | reines JS-SPA — für den Reader inhaltsleer; MAST-API-Doku liegt anderswo |
| `nhsa.esac.esa.int` | tot (422) | HSA lebt unter `archives.esac.esa.int/hsa` (HAIO: `/hsa/aio`) — JS-App |
| `www.lhaaso.ac.cn` | tot (422, Wayback 404) | offizielle Site `english.ihep.cas.cn/lhaaso`; Daten-/Code-Seite `/lhaaso/pdl` |
| `svom.nscs.ac.cn` | tot (422, Wayback 404) | Datenzugang laut NSSDC: `svom.ac.cn`; GRB-Notices via GCN `gcn.nasa.gov/missions/svom` (FSC: `fsc.svom.org/alerts`) |

## 14. JWST-MCT-Recherche (Session 2026-08-19)

Anlass: STScI-Artikel — Aufruf für JWST-Multi-Cycle-Treasury (MCT)
White Papers (>300 h, Frist 4.11.2026, jedes Thema). Auftrag: was misst
das JWST, das es als APIs gibt, die das Register noch nicht hat.

- Instrumente: NIRCam (Imaging 0,6–5 µm), NIRSpec (Multi-Objekt-Spektroskopie, MSAs), MIRI (5–28 µm), NIRISS (WFSS/SOSS), FGS.
- API-Landschaft: MAST CAOM-TAP (EDP-API, JWST-Metadata-API, HLSP, z.MAST), NExScI-TAP (`ps`/`pscomppars`, Atmospheric Spectroscopy Table AST, NExoList, Transit-Service), ESA JWST TAP, VizieR-TAP.
- Lücken: AST >1500 Spektren (hunderte JWST) = größte; JWST-Deep-Field-Photometrie/Redshifts (JADES DR5, COSMOS2025 `J/A+A/704/A339`); NExoList; EDP-Telemetrie; Metadaten geplanter Beobachtungen.
- Priorisierung: P0 AST-Spektren; P1 Deep-Field-Photometrie/Redshifts; P2 EDP/Metadaten; P3 NExoList.
- Disposition: „Nur Recherche, nichts umsetzen“ — keine Register-Schreibarbeit; die 7 `mast_jwst_*`-CAOM-Blöcke im Queue (master.φ ab 9059) sind älterer Bestand. Port-Entscheid P0 ausstehend.

## 15. Offene Posten aus der Session (hier registriert — kein Datei-Artefakt entstand)

Die Adressen sind recherchiert (Bestand §12–§13). Was aussteht, ist
Port-Arbeit an den vorliegenden Befunden — Block-Drafts, `--probe`,
Disposition — keine erneute Adress-Recherche.

1. SPHEREx IRSA-TAP — Block-Draft bauen (Adresse liegt vor).
2. eROSITA DR2 (31.07.2026) — Block-Draft bauen.
3. ESA-AMA-TAP (Einstein Probe, CHEOPS) — Block-Draft bauen.
4. Euclid-TAP — Block-Draft bauen.
5. ESO-TAP (`tap_obs`/`tap_cat`) — Block-Draft bauen.
6. ALMA-TAP, MeerKAT-TAP, Rubin-TAP, Keck-TAP, Pan-STARRS, ATLAS-Forced, LAMOST DR11 — Block-Drafts bauen.
7. NRAO: Register-Eintrag umschreiben auf REST `data.nrao.edu/archive-service/restapi_*` (Adresse recherchiert).
8. CHIME: Register-Eintrag umschreiben auf CANFAR-DOIs (Adresse recherchiert).
9. SVOM: Block-Draft aus `svom.ac.cn` + GCN-Notices (live GRB-Alerts, analog Swift-GRB).
10. LHAASO: `english.ihep.cas.cn/lhaaso/pdl` ist der Befund — offen ist nur die Klassifikation: öffentlicher Bulk-Download → Messung, Paper-only → Decline (§8). Kein neuer Recherche-Schritt.
11. MAGIC/HAWC: `--probe` auf die Portal-URLs (§13) — mechanisch, kein Recherche-Schritt.
12. Herschel: HSA-Umzug in Katalog/Register nachziehen (Adresse `archives.esac.esa.int/hsa` liegt vor).
13. JWST P0–P3: Port-Entscheid (§14).
