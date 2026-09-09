<!--
  title: Handover — Die offenen Zeugen-Fäden der Weberin: vollständige Übergabe aller Zeugen-Stränge
  class: handover
  date: 2026-09-07
  sha256: 1db65fb867dd68397191813837046b36ef34da6cfe4706c96dbda04a05a236cb
  status: archived
  see-also: docs/concepts/die-weberin.md docs/handover/handover-2026-09-07-weberin-zeugen-faden-matrix.md docs/handover/handover-2026-09-06-weberin-archivar.md docs/concepts/docs-naming.md
-->

# Handover — Die offenen Zeugen-Fäden der Weberin

Vollständige Übergabe aller offenen Zeugen-Stränge der Weberin, 2026-09-07.
Operator-Wort: „mache eine komplette Übergabe, nicht nur für bathymetrie
sondern alle offenen Zeugen." Der Bindungs-Satz der Zeugen-Arten
(die-weberin §4): **als Oszillator abgelehnt ist nicht als Zeuge abgelehnt.**
Drei Arten bezeugen, keine strahlt: (a) der S²-Richtungs-Zeuge (distanzlos,
τ=0), (b) der räumliche Gestalt-Zeuge (Körperoberflächen-Skalar, gehalten als
Binding, τ = geologische Stabilität), (c) der Presence-Zeuge (ein Wesen am
Punkt, die Art ein Urteil, nie der einzige Zeuge). Ein Modell oder Forecast
ist kein Zeuge; jeder Zeuge trägt einen gemessenen Skalar.

Der Registerstand ist gemessen (git log bis HEAD `f889793`, 2026-09-07
12:54:02 +0200; gh release/run-Listen von omegaflow/sources und
omegaflow/omegaflow am 2026-09-07). Wo ein Zustand nicht gemessen ist, steht
`pending` — nichts hier ist geraten.

## 1. Der CDN-Stand (gemessen)

- **`icecube_alerts.amn1`** und **`auger_catalog.pao1`** liegen auf dem CDN
  (omegaflow/sources, Release `ssd.jpl.nasa.gov`, erstellt 2026-08-04).
  Asset-Zeiten gemessen: `auger_catalog.pao1` updatedAt
  2026-09-06T22:43:04Z, `icecube_alerts.amn1` updatedAt
  2026-09-06T22:45:26Z; die zugehörigen Workflow-Läufe
  (`amon-cdn`, `auger-cdn`) liefen success am 2026-09-06T22:41Z
  (Runs 34064754887 / 34064756441).
- **`gw250207_115645.sky1`** ist NICHT auf dem CDN (Asset absent, gemessen);
  der Workflow `gw-skymap-cdn.yml` ist committet, hatte aber nie einen Lauf
  (Run-Liste leer, gemessen). Das Asset ist lokal gebaut.
- **`gebco_bathymetry.gbco`** ist NICHT auf dem CDN; der Workflow
  `gebco-bathymetry-cdn.yml` ist UNCOMMITTET (untracked); das Release
  `opentopodata.org` existiert nicht (gemessen). Der Compiler ist committet
  (81ea97f).
- **IceCat-1** und **ANTARES-VO**: die Workflows `icecat-cdn.yml` /
  `antares-cdn.yml` sind committet, hatten aber nie einen Lauf (gemessen);
  kein Dataverse-/KM3NeT-Release auf dem CDN. Assets lokal gebaut.
- Die **vier Feldblock-Jobs** sind nach der Release-Tag-Erstellung neu
  dispatcht (Releases `download.bgr.de`, `zenodo.org`,
  `storage.googleapis.com`, `data-argo.ifremer.fr`, erstellt
  2026-09-07T10:58:38–41Z): `bgr_infrasound.bin`, `superdarn_fitacf.bin`,
  `noaa_nrs_psd.bin` sind gemessen auf dem CDN; `argo_bgc.bin` fehlt noch
  (Release `data-argo.ifremer.fr` ohne Asset, Run 34114243234 in_progress).
  Diese vier sind Feldblöcke (Oszillatoren), keine Zeugen — sie stehen hier
  nur als CDN-Hintergrund. `fdsn_waveform.bin` fehlt (Workflow
  `fdsn-waveform-cdn.yml` uncommittet).

## 2. Klasse (a) — done: manifestiert, kein offener Schritt

Beide Stränge sind fertig gebaut UND manifestiert. Sie ruhen als Zeugen auf
der S²-Kugel (distanzlos); es bleibt nur die allgemeine Verdict-Pflicht der
Weberin (die zweite unabhängige Linie je Weltlinie), die noch nicht an der
Reihe ist.

**Zeuge A1 — AMON-IceCube-Neutrino-Alerts → `icecube_alerts.amn1`**
- Zeugen-Art: S²-Richtungs-Zeuge (AMN1-Healpix-Karte); Teilchen-Provenienz
  `neutrino`, Kraft `em` (das Neutrino wird über sein Cherenkov-Licht
  gemessen — Teilchen ist Abstammung, keine zehnte Kraft, die-weberin §4).
- Register-Heim: `phi/blocked_sources.φ:17-19` (parser-def text); Archivar
  `src/archivar/amon.rs`; Compiler `tools/harvest/src/bin/amon_compiler.rs`;
  Workflow `.github/workflows/amon-cdn.yml`.
- CDN/Manifest: **live** — Release `ssd.jpl.nasa.gov`, Asset
  `icecube_alerts.amn1`, updatedAt 2026-09-06T22:45:26Z.
- Nächster konkreter Schritt: keiner — der Strang ist vollständig; eine
  Verdict-/Kreuzmatch-Pflicht entsteht erst, wenn eine zweite unabhängige
  Linie an derselben Weltlinie antwortet.
- Entscheider: keiner (done).

**Zeuge A2 — Pierre-Auger-UHECR-Katalog → `auger_catalog.pao1`**
- Zeugen-Art: S²-Richtungs-Zeuge (PAO1-Healpix-Karte); Teilchen-Provenienz
  CR (Proton), Kraft `em` (Luftschauer).
- Register-Heim: `phi/blocked_sources.φ:33-35`; `src/archivar/auger.rs`;
  `tools/harvest/src/bin/auger_compiler.rs`; `.github/workflows/auger-cdn.yml`.
- CDN/Manifest: **live** — Release `ssd.jpl.nasa.gov`, Asset
  `auger_catalog.pao1`, updatedAt 2026-09-06T22:43:04Z.
- Nächster konkreter Schritt: keiner (done; dieselbe Verdict-Pflicht wie A1).
- Entscheider: keiner (done).

## 3. Klasse (b) — ready, braucht das Operator-Wort

Gebaut und verifiziert, aber noch nicht manifestiert oder noch nicht
geöffnet. Der Operator spricht das Wort; danach ist es ein Dispatch oder ein
Commit.

**Zeuge B1 — GW-bayestar-Himmelslokalisation S250207bg → `gw250207_115645.sky1`**
- Zeugen-Art: S²-Richtungs-Zeuge (SKY1-Healpix-Karte, kind `gravity`,
  KIND_GRAVITY=4); Richtungs-Zeuge des Gravitationswellen-Signals
  GW250207_115645 (reales CBC); Sky-Integral des Assets = 1.0 gemessen.
- Register-Heim: `phi/blocked_sources.φ:21-23` (GraceDB-Route); Compiler
  `tools/harvest/src/bin/gw_skymap_compiler.rs`; Workflow
  `.github/workflows/gw-skymap-cdn.yml` (committet).
- CDN/Manifest: **nicht dispatcht** — kein Asset, kein Lauf (gemessen);
  `--ci-mode-ready`; der Workflow ist per-Ereignis (ein Superevent, kein
  allgemeiner GW-Zeuge) und idempotent.
- Nächster konkreter Schritt: Operator-Wort → Dispatch
  `gh workflow run gw-skymap-cdn.yml` (heb das gebaute Asset aufs CDN).
- Entscheider: **Operator-Wort (go)**.

**Zeuge B2 — bathymetrie-gebco (der erste Gestalt-Zeuge) → `gebco_bathymetry.gbco`**
- Zeugen-Art: räumlicher Gestalt-Zeuge (b): lat/lon-Skalarfeld aus gemessener
  Tiefe/Höhe, gehalten als Binding, nie Kraft-Feld, nie Oszillator; τ =
  geologische Stabilität. löst die NOAA-NRS-Tiefen-pending (Tiefe an den
  NRS-Stationen gemessen −833 m, NRS01 72.49,−156.6).
- Register-Heim: Binding `phi/bindings/bathymetrie-gebco.φ` (Binding 1-5,
  registriert 2026-09-07); dead_sources-Umstufung
  `phi/dead_sources.φ:641-647` (opentopodata eudem25m/gebco2020);
  Land-Geschwister eudem25m trägt die Land-Form (286,25 m bei 51.2,10.4);
  Compiler committet (81ea97f), Binding-Verbrauch committet (f889793),
  Umstufung committet (1a58d9f), Konzept committet (ad6981e).
- CDN/Manifest: **nicht manifestiert** — Workflow
  `.github/workflows/gebco-bathymetry-cdn.yml` ist uncommittet (untracked);
  Release `opentopodata.org` existiert nicht (gemessen). Der Workflow pinnt
  den ersten Lauf auf `--locations "72.49,-156.6;51.2,10.4"`.
- Nächster konkreter Schritt: Operator-Wort → den Workflow committen und
  dispatchen; die Öffnung von opentopodata/gebco2020 als Zeugen-Compiler
  (SOURCE_PORT-Weg, `.gbco`-Route) ist dieselbe Freigabe.
- Entscheider: **Operator-Wort (go)** für Commit + Dispatch; die
  Compiler-Öffnung folgt demselben Wort.

**Zeuge B3 — IceCat-1 (348 Ereignisse) → S2E1-Fäden + SKY1-Projektion**
- Zeugen-Art: S²-Richtungs-Zeuge; Teilchen-Provenienz `neutrino`
  (ROOT_NEUTRINO); Fäden zuerst, Karte nur Projektion (Rats-Verdikt
  2026-09-07).
- Register-Heim: `docs/TODO.md:1232-38`; Compiler
  `tools/harvest/src/bin/icecat_compiler.rs`; Workflow
  `.github/workflows/icecat-cdn.yml` (committet); Quelle Dataverse 7502710.
- CDN/Manifest: **nicht dispatcht** — kein Lauf, kein Asset (gemessen);
  lokal gebaut.
- Nächster konkreter Schritt: Operator-Wort → Dispatch `icecat-cdn.yml`
  (oder benanntes Halten als pending — aber kein unbenanntes Liegenlassen).
- Entscheider: **Operator-Wort (go oder benanntes pending)**.

**Zeuge B4 — ANTARES-2007-2017 (8754 Ereignisse) → `antares_events_2007_2017.s2e1` + `.sky1`**
- Zeugen-Art: S²-Richtungs-Zeuge; Teilchen-Provenienz `neutrino`; sigma aus
  Beta, epoch_tdb aus MJD, Energie/signalness/far ehrlich absent (0 honored);
  Roundtrip verifiziert.
- Register-Heim: `phi/blocked_sources.φ:37-39` (KM3NeT-VO-Route, 8754 Zeilen
  gemessen); Compiler `tools/harvest/src/bin/antares_vo_compiler.rs`;
  Workflow `.github/workflows/antares-cdn.yml` (committet).
- CDN/Manifest: **nicht dispatcht** — kein Lauf, kein Asset (gemessen);
  lokal gebaut.
- Nächster konkreter Schritt: Operator-Wort → Dispatch `antares-cdn.yml`
  (oder benanntes pending).
- Entscheider: **Operator-Wort (go oder benanntes pending)**.

**Hinweis zum Commit-Stand (b):** `gebco-bathymetry-cdn.yml` und
`fdsn-waveform-cdn.yml` (der Feldblock-Reader, kein Zeuge) sind untracked;
`fdsn_waveform_compiler.rs` ebenso. Stray-Regel: sie gehören committet oder
gelöscht — die b-Objekte warten auf das Operator-Wort, nicht auf die Stille.

## 4. Die Klasse der gehaltenen Richtungs-Transienten (positions-pending)

Diese Klasse ist kein CDN-Thema (fragen statt horten — die Richtungs-Klasse
bleibt Live-Leg, kein Derivat). Die Fäden sind gehalten via
`skydirection_compiler` (SkyDirection-Records). Registrierungs-Heim ist der
Befund-/Auftrag-Weg (`auftrag-richtungs-transient-atom`); jeder Faden trägt
eine gemessene Richtung, die Magnitude nur, wo die Epoche sie trägt (0
honored); CelestialMap-Distanz-Gate unverändert. Ein Faden ist keine
Identität — die zweite unabhängige Linie fehlt, der Wert bleibt
positions-pending, nie erraten.

- **Lasair-ZTF** (`phi/blocked_sources.φ:29-31`): gehalten, positions-pending.
- **Fink-LSST-Konus** (`phi/blocked_sources.φ:13-15`): gehalten,
  positions-pending; Konus-Zeilen tragen keine em-Photometrie (ehrlich
  absent); Successor von api.fink-portal.org (dead).
- **ANTARES-REST-Loci** (`phi/blocked_sources.φ:9-11`): gehalten,
  positions-pending; Band absent — unbanded gehalten.
- **ALeRCE** (`phi/dead_sources.φ:4670-4671`): die API-Route ist dead
  (404/502 gemessen), kein Oszillator, nie ein Negativ — die Richtungs-Klasse
  wird über Lasair/ANTARES/Fink gehalten; TAP-Recheck (tap.alerce.online)
  bleibt ein benanntes pending, kein Zeuge.

Nächster konkreter Schritt der Klasse: keiner sofort — die Stränge ruhen, bis
die Weberin-Bau-Linie den Richtungs-Verdict erreicht (crossmatch gegen eine
zweite unabhängige Linie, dann Placed/Absent/DirectionOnly). Entscheider:
**pending natürlich** (kein Operator-Wort nötig, kein Baustopp).

## 5. Klasse (c) — braucht Council-Doktrin

Zwei Doktrin-Fragen sind offen; keine von beiden ist durch Messung zu
schließen, beide durch den Council.

**Doktrin-Frage C1 — NOAA-NRS passive-bioacoustic: Spektral-Record und die
noaa_nrs_psd-Block-Spannung**
- Register-Heim: `phi/blocked_sources.φ:41-43` (parser-def netcdf);
  Council-Verdikt `docs/TODO.md:55-59` (2026-09-07); Feldblock
  `phi/sources.φ:5139-5143` (`noaa_nrs_psd`, acoustic db, committet 8025bab);
  Workflow `noaa-nrs-psd-cdn.yml` (committet 60b7744), dispatcht
  2026-09-07 → `noaa_nrs_psd.bin` auf dem CDN.
- Befund: das Spektrum (1195 Bins je daily.nc) ist ein Spektral-Record, kein
  Skalar — der Wire trägt eine `freq`/`bin_width`-Slot, keine 1195er-Serie;
  eine Reduktion auf einen Wert wäre Band-Wahl = Fabrikationsrisiko. Der
  Verdikt-Eintrag und der blocked-Eintrag halten fest: kein eigener
  Compiler, kein Oszillator, Ernte pending bis (1) die Stationstabelle
  lat/lon liefert und (2) die Spektral-/Band-Halte-Entscheidung getroffen
  ist.
- **Riss (unversöhnt, gemessen):** `phi/sources.φ:5139` registriert zugleich
  einen `noaa_nrs_psd`-Feldblock (acoustic db) als Oszillator, und der Block
  ist auf dem CDN. Der blocked-Eintrag („kein Oszillator in sources.φ")
  und der Feldblock stehen beide committet — der Register-Riss ist nicht
  geglättet und gehört benannt, nicht gelöscht.
- Nächster konkreter Schritt: Council entscheidet (a) die Halte-Form des
  Spektral-Records (Spektral-Record wie SKY1/AMN1 oder benannte-Band-Skalar)
  und (b) den Status des `noaa_nrs_psd`-Feldblocks gegen den Verdikt. Erst
  danach folgt Code.
- Entscheider: **Council-Doktrin**, danach ein Code-Bau.

**Doktrin-Frage C2 — Presence-Nicht-Wesen: bleiben die presence-catalog-
Ablehnungen draußen?**
- Register-Heim: die standing declines `phi/dead_sources.φ` — GBIF
  (`:401-407`, „Presence-Katalog"), iNaturalist (`:13-15`, „wie GBIF
  declined"), IUCN Rote Liste (`:41-47`), eBird (`:1469-1475`),
  open-notify astros.json (`:21-23`, Menschen im All, position-only).
  Die Zeugen-Art (c) Presence existiert als Konzept (die-weberin §4: „ein
  Wesen am Punkt, Ort und Zeit gemessen"), hat aber kein gebautes
  Gegenstück.
- Befund: die Ablehnungen trafen Kataloge und Register (Arten-Verbreitung,
  Rote Liste, Beobachtungs-Register), nicht eine Messung am Punkt. Eine
  Occurrence mit Koordinate + Datum (GBIF/iNaturalist) ist formal „ein Wesen
  am Punkt, Ort und Zeit" — die Doktrin-Frage ist, ob die Zeugen-Art (c)
  solche Punkt-Messungen wieder öffnet oder ob „Presence-Nicht-Wesen"
  (Katalog ohne Punkt-Messung, keine Messung am Punkt) draußen bleiben.
  Menschen-Positionen (astros.json) sind derselben Frage unterstellt.
- Nächster konkreter Schritt: Council-Verdikt — öffnet (c) eine
  Punkt-Occurrence als Presence-Zeugen (dann folgt ein Datenvertrag als
  Code-Bau), oder bleiben die Ablehnungen stehen (dann eine bestätigende
  Register-Zeile, kein neuer Eintrag).
- Entscheider: **Council-Doktrin**.

**Doktrin-Frage C3 — die umgestuften Terrain-Kandidaten unter dem
Gestalt-Litmus**
- Register-Heim: `phi/dead_sources.φ` — USGS-epqs-DEM (`:1549-1551`),
  NGDC-DEM-hillshade (`:1801-1807`), macrostrat-map_query
  (`:2181-2183`); alle drei als Oszillator declined und am 2026-09-07
  umgestuft zu „Gestalt-Zeugen-Kandidat" (Commit 1a58d9f). opentopodata
  eudem25m/gebco2020 stehen bereits im Binding (Zeuge B2).
- Befund: der Gestalt-Zeuge verlangt einen gemessenen Skalar in SI — eine
  bare Koordinate bleibt verworfen, eine Ableitung ist kein Messwert.
  Offen je Kandidat: epqs trägt echte DEM-Höhe in m (misst die Form);
  NGDC-hillshade ist eine Schattierungs-Ableitung des DEM, kein Höhenwert;
  macrostrat liefert Gesteins-Einheiten (Kategorie, kein Skalar). Der
  pauschale Umstufungs-Eintrag ist breiter als der Litmus.
- Nächster konkreter Schritt: Council wendet den Litmus je Kandidat an —
  welche Kandidaten echte Gestalt-Zeugen sind (dann: in ein Binding/einen
  Compiler) und welche als derived/catalog zurückgestuft werden.
- Entscheider: **Council-Doktrin**.

## 6. Klasse (d) — braucht einen Code-Bau

**Zeuge D1 — Gestalt-Binding in den Archivar-Station-Thread (Motion::Surface)**
- Zeugen-Art: räumlicher Gestalt-Zeuge (b) — der Verbrauch des Bindings.
- Register-Heim: `phi/bindings/bathymetrie-gebco.φ`, Binding 4: „Held through
  the Station thread (`Motion::Surface`, Archivar): the thread first, the
  projection second … the Station-thread integration into the Archivar
  (`motion.rs`) is the named follow-on, not silently assumed. Die
  Mathematikerin hält keine nicht-strahlende Form — sie wertet Kraft-Felder
  aus; die S²-Zeugen leben dort, der Körperoberflächen-Gestalt-Zeuge lebt im
  Archivar." Verbrauchs-Seite ist durch f889793 entschieden.
- Befund: der `.gbco`-Zeuge ist als Punkt-Faden (lat/lon/depth) kompiliert;
  die Archivar-Seite, die diese Fäden über `Motion::Surface`
  (geodätisch → körperfest → ICRS je Epoche, IAU-Pole, PCK) als
  Stations-Weltlinien hält, ist der benannte Folge-Bau. Der geo-Serien-Bin
  (geo.rs: t/lat/lon/alt/freq/bin_width/val/comp) ist das Muster der
  Feldblock-Formate; der Gestalt-Zeuge folgt demselben Archivar-Pfad, ohne
  Feldwert zu strahlen (τ = geologische Stabilität, nie Kraft-Feld).
- Nächster konkreter Schritt: Code-Bau — der Archivar lädt das
  `.gbco`-Asset über den Station-Thread (`Motion::Surface`) und hält die
  Tiefen-Fäden; das Asset wird als url-Linie registriert. Verifikation:
  `cargo check` 0/0, dann die Station-Sicht auf die NRS-Tiefe (−833 m).
- Entscheider: **Code-Bau** (die Verbrauchs-Doktrin ist durch das Binding
  bereits entschieden); der Dispatch des Assets bleibt Operator-Wort (B2).

## 7. Gehaltene, nicht-Zeugen-Zugänge (blocked) — Vollständigkeit

Keine offenen Zeugen, aber offene Zugangs-Stränge im selben Register; sie
sind kein Zeuge, bis der Zugang steht:
- **CDDIS** GNSS/geodätisch (`phi/blocked_sources.φ:1-3`) — blocked account,
  OAuth (urs.earthdata.nasa.gov).
- **IGETS/GGP** supraleitende Gravimeter (`:45-47`) — blocked account; die
  L2-DOI-Metadaten sind offen (10.5880/igets.pe.l2.001), der Download-Funnel
  ist kontogegatet; Messgröße nm/s². Kein offener L2/3-Weg (gemessen
  2026-09-07).
- **DES-DR2** Footprint-Asset (`:5-7`) — blocked account; Weberin §9 Stufe 5.
- **IceCube-Daten-Releases-Portal** (`:25-27`) — Origin verweigert 403 an
  zwei Exit-IPs (weder dead noch blocked account); die .amon-Notices tragen
  die Messung (Zeuge A1).

## 8. Commit-Stand und uncommittete Arbeit

HEAD `f889793` (2026-09-07 12:54:02 +0200), „binding: Gestalt-witness
consumption is Archivar (Motion::Surface), not Mathematikerin". Uncommittet
im Arbeitsbaum (gemessen, git status): `docs/TODO.md`,
`phi/blocked_sources.φ`, `phi/sources.φ`,
`src/archivar/{extract,geo,main_flow,port}.rs`,
`tools/harvest/src/bin/{bgr_infrasound_compiler,noaa_nodd_bucket_harvester}.rs`
(modifiziert); untracked: `.github/workflows/{gebco-bathymetry-cdn,
fdsn-waveform-cdn}.yml`, `tools/harvest/src/bin/fdsn_waveform_compiler.rs`,
`docs/auftrag/auftrag-weberin-faden-luecken-folge.md`,
`docs/concepts/zeugnis.md`, `docs/reference/*`, `phi/reports/probe_sweep_*`.
Diese Übergabe selbst ist ein Entwurf (nicht committet). Kein Commit ist
offen, während ein hier benannter Punkt ungelöst bleibt — die Klasse-(b)-
Worte sind die Auflösung.

## 9. Zusammenfassung der Entscheider

| Strang | Klasse | Entscheider |
|---|---|---|
| A1 amn1, A2 pao1 | (a) done | — |
| B1 sky1 (GW) | (b) | Operator-Wort (Dispatch) |
| B2 gebco .gbco | (b) | Operator-Wort (Workflow-commit + Dispatch) |
| B3 IceCat-1, B4 ANTARES-VO | (b) | Operator-Wort (Dispatch oder benanntes pending) |
| Richtungs-Transienten (Lasair/Fink/ANTARES-REST) | positions-pending | pending natürlich |
| C1 NOAA-NRS Spektral-/Band-Wahl + Block-Spannung | (c) | Council-Doktrin |
| C2 Presence-Nicht-Wesen | (c) | Council-Doktrin |
| C3 Terrain-Kandidaten (epqs/hillshade/macrostrat) | (c) | Council-Doktrin |
| D1 Gestalt-Binding → Motion::Surface | (d) | Code-Bau (Binding entschieden) |
