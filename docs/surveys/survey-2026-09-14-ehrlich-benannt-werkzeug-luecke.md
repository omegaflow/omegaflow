<!--
  title: Survey — Ehrlich benannt: wo das Werkzeug fehlte (Stand 2026-09-14)
  class: survey
  date: 2026-09-14
  sha256: d57f4eb4138291d1e6461d3bb47281bea05622ba21256c75ffe54cd25d15def6
  status: live
  see-also: docs/specs/ref-auth-apis.md docs/concepts/docs-naming.md
-->
# Ehrlich benannt — wo das Werkzeug fehlte (Stand 2026-09-14)

Dieser Survey folgt dem Operator-Hinweis: das Wort **„ehrlich" (honest)** markiert
im Register die Stellen, wo ein Bau-Vorhaben mit dem Verdikt „Daten nicht
veröffentlicht / nicht öffentlich / request-only / gated / Paywall" abgebrochen
wurde. Die Messung zeigt: in vielen Fällen war nicht die Quelle verschlossen,
sondern das **Werkzeug** fehlte (Parser, Download, Reader, Token-Hook) — oder ein
Query-Parameter war falsch. Der Survey listet die Stellen und trennt
Werkzeug-Lücke von echter Absenz.

## Der Goldschatz (das Muster in Reinform)

`docs/auftrag/archiv/auftrag-flyby-doppler-rohdaten.md:78-84` — „Suchlauf-Befund
(2026-09-03) — reine Web-Recherche. Vorbemerkung, **ehrlich benannt**: Dieser
Suchlauf verfügte nur über Web-Recherche — **kein Datei-Download, kein
ODF-Parser, kein N-Körper-Solver**. Der in der Abnahme geforderte Schritt
„geladener Tracking-Pass → Residuum → mm/s-Wert" ist daher **nicht** vollzogen.
0 honored: keine einzige mm/s-Zahl wird hier genannt oder re-deriviert."

Die Flyby-Doppler-Rohdaten (Galileo, NEAR, Cassini, Rosetta, Messenger, Juno)
wurden als `open`/`pending`/„nur intern JPL-NAV" registriert — die Tabelle selbst
zeigt je Zeile „RS/ODF nein" (nicht geladen). Die Absenz ist der fehlende
**ODF-Parser/Download**, nicht die Quelle.

Der kanonische Präzedenzfall steht im Haus selbst:
`docs/specs/ref-auth-apis.md` §A — **SuperMAG**: „funktioniert (gemessen
10.09.2026 …; der frühere ‚geht nicht'-Befund war der **falsche Parameter**
`user`/`username` statt `logon`)."

## Die Klassen

### 1. `request-only` / internes Archiv — Sonden-Rohdaten (DSN/JPL/NAV/ODF)

| file:line | Quelle | Claim (wörtlich) | blockierter Bau | Werkzeug-Lücke |
|---|---|---|---|---|
| `docs/auftrag/archiv/auftrag-flyby-doppler-rohdaten.md:78-84` | JPL/DSN ODF (Flybys) | „kein Datei-Download, kein ODF-Parser" | Flyby-Doppler-Residuum (mm/s) | **JA** (ODF-Parser/Download) |
| `docs/auftrag/archiv/auftrag-flyby-doppler-rohdaten.md:125,151-155,168-170,180` | Juno Earth-EDR / JPL-NAV | „pre-EFB-Merged-ODF, nicht öffentlich archiviert"; „Roh-Doppler ausschließlich über DSN-Datenanfrage" | Juno-Erd-Flyby ΔV∞ | teils (TRK-2-34-Parser vorhanden; Zugang = Anfrage) |
| `docs/auftrag/archiv/auftrag-voyager-roh-doppler-zugang.md:35,61-63,78-81` | JPL/DSN ODF/TDF (TRK-2-34) | „kein offener Endpunkt, `request-only`" | Voyager-Cruise-Doppler 1998–2002 | NEIN (DSN-Anfrage) |
| `docs/auftrag/archiv/auftrag-quiet-zone-uebertragung.md:34,178-190` | NH/JPL/DSN-ODF | „Harvest nicht offen — `request-only`" | NH-Quiet-Zone-Harvest | NEIN (Anfrage) |
| `docs/auftrag/archiv/auftrag-quiet-zone-vorfilter.md:71,74,79-80` | NH Nav-Doppler / Mariner | „SPDF 404; Nav-Doppler `request-only`" | NH-Harvest | NEIN |
| `docs/handover/archiv/handover-2026-09-12-forschung-folge7.md:80-81` | Galileo ODF | „ODF-Format (1 vs 2) bleibt ungemessen, bis der ODF-Doppler-Extrakt etwas hält" | Galileo-Doppler | **JA** (ODF-Doppler-Extrakt) |
| `docs/auftrag/archiv/auftrag-lisa-pathfinder-psd.md:35,59-61` + `tools/measure/src/bin/lpf_psd_probe.rs:40` | ESA LPF-Legacy-Archiv | „behind CAS auth with the TAP interface disabled"; „`not-published`" | LISA-PF-Kreuzspektrum | NEIN (CAS-Auth) |
| `docs/auftrag/archiv/auftrag-extern-weberin-zweitlinien.md:73-77` | PRIDE ΔDOR / EVN | „Kein offenes VLBI/ΔDOR/Range gemessen" | Sonden-Positions-Zweitlinie | NEIN (Login-Gate) |

### 2. Paywall (Paper/Volltext)

| file:line | Quelle | Claim | blockierter Bau | Werkzeug-Lücke |
|---|---|---|---|---|
| `docs/paper/woo-armstrong-1979-jgr-abstract.md:6,16` | Woo & Armstrong 1979 JGR | „abstract-only (full text paywalled)" | S-Band/Allan-Werte | NEIN (Verlag) |
| `docs/paper/armstrong-1998-phase-scintillation-abstract.md:6,16` | Armstrong 1998 Radio Science | „abstract-only; JPL preprint handle down" | Phasen-Szintillation | NEIN |
| `docs/concepts/recherche-extern-galileo-ruck-borduhr-modell.md:34,36,120,171-175` | Wohlmuth 1997 / Haw 1997 / Hinson 1997 | „Volltext Paywall (nicht gelesen)" | Galileo-Borduhr-Sprung A/B | **JA** (Schließer: „RSS-Datenköpfe der PDS-Sätze GO-J-RSS-* → Reader") |

### 3. Gated API / Login / SSO / SMS

| file:line | Quelle | Claim | blockierter Bau | Werkzeug-Lücke |
|---|---|---|---|---|
| `docs/specs/ref-auth-apis.md:136,137,138,140` | GRACE-FO/SWOT, SMAP, CDDIS IONEX, AppEEARS | „Earthdata vorhanden; S3-Scheme ungetragen"; „Live 200" | Gravity/EM/Thermal-Ports | **JA** (S3-Reader / EarthData-Token-Hook) |
| `docs/specs/ref-auth-apis.md:133,134,139` | NASA ADS, Space-Track, GES DISC | „Subendpoints dead 404"; „Query 401-auth"; „griddap dead 404" | EM-Katalog-Ports | teils |
| `docs/paper/laic-arrow-direction.md:224` | CSES (leos.ac.cn) | „login-gated SPA … requires a Chinese mobile number" | CSES-Ionosphärenkanal | teils (DEMETER-`.DAT`-Parser offen) |
| `docs/handover/archiv/handover-2026-09-13-ernte-folge11.md:45-46` | WWLLN | „Realtime-Roh ist `not-published` (Mitgliedschaft)" | Blitzortung | NEIN |

### 4. Teilchen-/Quantendaten

| file:line | Quelle | Claim | blockierter Bau | Werkzeug-Lücke |
|---|---|---|---|---|
| `phi/dead_sources.φ:415` | AMS-02 | „nur Papier-Abbildung, kein maschinenlesbarer Flux-Endpoint" | Teilchenfluss | **JA** (HEASARC AMS02SPEC FITS/TDAT) |
| `phi/dead_sources.φ:4803` | Super-Kamiokande | „ohne Roh-Messkanal; Teilchen-Kanal pending" | atmosph. Neutrino-Richtung | NEIN |
| `phi/dead_sources.φ:4807` | Telescope Array | „ein einzelnes UHE-CR-Ereignis, kein τ" | UHE-CR-Feld | NEIN |
| `docs/concepts/kybernetische-astrophysik.md:393-395` | Quantenschaum | „trägt keinen Tatort — Detektor-Klicks/Qubits gehören dem Labor" | Quantenschaum-Nadel | NEIN (Domänengrenze) |

### 5. Reader-/Werkzeug-Lücke (das „Werkzeug fehlte" wörtlich)

- S3-Scheme ungetragen (`ref-auth-apis.md:136`) — GRACE-FO/SWOT PODAAC.
- ODF-Doppler-Extrakt (Galileo, `handover-2026-09-12-forschung-folge7.md:80-81`).
- FITS/TDAT-Reader für AMS-02 (`dead_sources.φ:415`).
- Parquet/GRIB-2/OPeNDAP offen (`handover-2026-09-10-autonom.md:91-92`).

### 6. Query-/Parameter-Bug (SuperMAG-Muster)

- `docs/specs/ref-auth-apis.md:135` — SuperMAG: der frühere „geht nicht"-Befund
  war der falsche Parameter (`user`/`username` statt `logon`). Der Beleg, dass ein
  „nicht erreichbar" ein Werkzeug-/Query-Fehler sein kann, kein Wall.

## Re-Messung

Jede Werkzeug-Lücke (Klasse 5) ist der Auftrag der folgenden Party: für jede
Stelle messen, ob mit dem gebauten Werkzeug (EarthData-Token-Hook, S3-/Reader,
ODF-Parser) ein offener Weg existiert. Die `request-only`-Fälle (Klasse 1) sind
keine Mauern, sondern ungesendete DSN-Anfragen — eine Register-Pflicht, kein
Werkzeug. Die echten Absenzen (Klassen 2–4) bleiben benannt, bis eine Messung
sie öffnet.

Der Survey trägt die Stellen; die Party trägt die Messung.

### Messung 2026-09-25 — die vier Klasse-5-Stellen

Alle vier Stellen sind re-gemessen. Vier „Werkzeug fehlte"-Lücken — vier offene
Wege; keine der vier trägt `request-only`/`not-published`/`ip-blocked`/`key`.
Vorbemerkung: `phi/dead_sources.φ:415` trägt heute eROSITA (Registers drift) —
der AMS-02-Eintrag ist aus allen vier Registern verschwunden (`sgrep` über
dead/sources/blocked/declined: kein Treffer). Das alte „nur Papier-Abbildung"-
Verdikt existiert nicht mehr als Register-Zeile; die Lücke war real gemessen
**Werkzeug**, nicht Quelle.

- **S3-Scheme (GRACE-FO/SWOT PODAAC) — `open`; der alte 401 ist weg.**
  `curl -H "Authorization: Bearer <EARTHDATA_EDL_TOKEN>"
  https://archive.podaac.earthdata.nasa.gov/s3credentials` → **HTTP 200**, JSON
  mit vollem Temp-Credential-Satz (`accessKeyId` + `sessionToken`). Der Claim
  „s3credentials 401 ‚required client id missing'" (`ref-auth-apis.md:158`) ist
  überholt. CMR-Granule `C2263336836-POCLOUD` (HOMAGE GGFO L4) trägt explizit
  `s3://podaac-ops-cumulus-protected/HOMAGE_GGFO_L4_GOMA_Monthly_v01/goma_GGFO_MM_SHC_200204-202606_v01.nc`.
  HTTPS-Download mit Token: 303 → CloudFront-signierte S3-URL → **final 200**.
  Kein fehlender Arm — SigV4-Scheme gebaut (`546d39e`), Token-Handshake liefert
  Temp-Keys; die alte „scheme-unsupported/401"-Zeile ist ein
  Register-Umbuchungsfall. Die NSIDC-SMAP-Hälfte der toten Zeile ist in diesem
  Pass nicht re-gemessen.
- **ODF-Doppler-Extrakt (DSN TRK-2-34 ODF/TDF) — `open`; Galileo-ODF `pending`.**
  Offener, maschinenlesbarer Korpus existiert: TRK-2-18 ODF
  `https://pds-geosciences.wustl.edu/lunar/lp-odf/lpod_case1.odf` (…case8) →
  **HTTP 200, 16,3 MB** Binär (`--sniff` 200). TRK-2-34 TNF
  `https://pds-ppi.igpp.ucla.edu/data/mess-rs-raw/data-tnf/2007/071550900sc236dss63_tnf.dat`
  → **HTTP 200, 30,9 MB**, SFDU-Kopf `NJPL2I00C123` (TRK-2-34);
  Jahres-Verzeichnisse 2007–2015 offen, MAVEN-TNF in PDS (LID
  `urn:nasa:pds:maven.rose.raw:data.tnf`). Web-Suche (`--tavily`): kein offener
  Galileo-ODF-Host gefunden. **Fehlender Arm: TRK-2-34/TNF-Parser** — Doppler-Phase;
  `odf.rs` trägt TRK-2-18 Format-1/2. Die Format-1-vs-2-Frage ist jetzt an echten
  offenen Dateien messbar; der Galileo-ODF selber bleibt die einzige ungefundene
  Datei-Klasse — `pending`, kein Wall.
- **FITS/TDAT-Reader AMS-02 — `open`; der alte dead-Eintrag ist falsifiziert.**
  `archive_search --heasarc table=ams02spec rows=5` → **live**: 10 Spalten, echte
  Zeilen (ENERGY 0.433–1800 GeV, RIGIDITY 1–1800 GV, MJD 55701–58266 =
  2011–2018 — der AMS-02-Protonenfluss-Messbereich).
  `https://heasarc.gsfc.nasa.gov/FTP/heasarc/dbase/tdat_files/heasarc_ams02spec.tdat.gz`
  → **HTTP 200, 174 884 B, gzip-Magic** (dazu `heasarc_ams02rates.tdat.gz`).
  Unkomprimiertes `.tdat` und `fits_files/*_fits.tar.gz` = 404 — das offene
  Artefakt ist das `.tdat.gz`; die W3Browse-Oberfläche ist heute Xamin/JS.
  **Fehlender Arm: TDAT-Reader** — HEASARC-TDAT = FITS-Variante, am ersten
  `.tdat.gz` messbar. Plus Register-Pflicht: AMS-02 als `live` (W3Browse+tdat)
  neu anlegen statt tote Zeile.
- **Parquet / GRIB-2 / OPeNDAP — `open`.** Alle drei Formate haben offene,
  anonyme bzw. mit vorhandenem Token offene Endpunkte. Parquet:
  `https://zenodo.org/api/records/22546478/files/cora_ar.parquet/content` →
  **HTTP 200, 90,3 MB, Magic `PAR1`** (cc-by-4.0); AIP-Gaia-XP-Bulk
  `gaia.aip.de/cms/data/gdr3-spectra/` = 404 (umgezogen; XP läuft dort über
  TAP/datalink). GRIB-2:
  `https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20260925/00/atmos/gfs.t00z.pgrb2.0p25.anl`
  → **HTTP 200, Magic `GRIB`**, S3-ListBucket live;
  `https://data.ecmwf.int/forecasts/20260925/00z/ifs/0p25/oper/20260925000000-0h-oper-fc.grib2`
  → **HTTP 200, Magic `GRIB`**, anonym. OPeNDAP:
  `https://opendap.earthdata.nasa.gov/collections/C2263336836-POCLOUD/granules/goma_GGFO_MM_SHC_200204-202606_v01.dds`
  + EDL-Token → **HTTP 200**, echter DDS (Grids `time_GFZ`/`time_CSR`/`goma_CSR`);
  anonym `coastwatch.pfeg.noaa.gov/erddap/info/index.json` → 200. GES-DISC-OPeNDAP
  von diesem Host aus direct+Proton `pending` — Route, kein Verdikt. **Fehlende
  Arme: Parquet-Reader, GRIB-2-Reader, OPeNDAP(DAP2)-Client** — die Quellen sind
  offen, diese drei bleiben der offene Bau.
