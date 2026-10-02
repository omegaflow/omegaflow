<!--
  title: Handover — Mountain-Folge 222 (Stand 2026-10-02)
  session: Mountain-Folge 222
  class: handover
  date: 2026-10-02
  sha256: 309e6d23014c3e52843a18681dd25abe9cbdb4983cf14c62b6d027732f7c8674
  status: live
-->
# Handover — Mountain-Folge 222 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; die
zitierte Runde steht auf einem älteren HEAD, die Punkte tragen ihre eigene Messung).
Diese Session konsumierte `handover-2026-10-02-mountain-folge221.md` (→ `archiv/`).
Die adressierten `## An mountain`-Blöcke aus `future-folge165` sind gemessen und in
die Offen-Liste gefaltet; der `river-folge77`-Block ist bereits seit folge221 erledigt
(Feldname = gemessene Größe, Identität trägt `station`, 5 Absolutpfade gesetzt).

## Burn: open 0.0024 · close 0.2713 · cap 0.30 (reason: operator-directed long atom — five dives and two probe runs in one pass)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„braucht es wirklich pro?" — pro nur mit benanntem Hart-Atom oder gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 211)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„arbeite deine Liste bis zur Kante ab" — jeder eigene Punkt bis zur Kante, nichts Machbares liegen lassen | 2026-09-30 | Operator (Session, Mountain 212)
„verschleppen und nicht eigenes ist verboten" — Linienliste nur `eigen`, jeder Punkt im Atom bis zur Kante | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass" | 2026-10-02 | Operator (Session, Mountain 221)
„Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert … das Commit-Wort" | 2026-10-02 | Operator (Session, Mountain 221)
„<LOCK-Wort für das private Experiment>" | 2026-10-01 | Operator (Session, Mountain 217) — verbatim im privaten Cut `state/operator-gespraeche/2026-10-01-mountain.md`
„alles was das experiment betrifft bleibt privat" | 2026-10-01 | Operator (Session, Mountain 217)
„ich will dass ihr inhalt bearbeitet falls notwendig wird und die datei entfernt" | 2026-10-01 | Operator (Session, Mountain 217)
„braucht es max?" — pro/max nur mit benanntem Hart-Atom oder gemessener flash-Fehllage; flash-first | 2026-10-02 | Operator (Session, Mountain 222)

## Offen (aufgeschlüsselt)

### Ephemeriden-Re-Manifest verifiziert — abhängige Proben gemessen, Haus-Gate in CI
- **Status:** wartend | **Bindung:** eigen (Verdikt/Bins: Mountain; CI-Lauf: mycelium)
- **Trigger:** `ephemeris-house-gate` run `36946576753` fertig → Verdikt lesen (einmal, kein Polling).
- **Lage:** (gemessen 2026-10-02 gegen die re-manifesteten CDN-Bins, lokal in `data/`) Re-Manifest grün (`kernel-flatten 36894645771` @`8ee41e78a`; `de44-cdn`/`inpop-epm-cdn` @`2e7b227e4`, Fix `0957ab15c` Vorfahr); `ephemeris_granule_census`: keine Grenz-Lücke, Mismatch median 43.6 m / max 190 m. `eclipse_shadow_probe` bestätigt die publizierte Eichung: 2017 de441/de440/de442/epm +4.9 s / 4.8 km, inpop +1.8 s / 6.5 km (Riss 3050 ms / 1.7 km); 2024 +2.6–2.7 s / 8.4 km, inpop +5.6 s / 7.1 km; de441↔de440/de442/epm 70 ms (2017: 0.0 ms). `flyby_anderson_probe` (6 Zeilen): Galileo I `pending`, Galileo II 0.0051/4.6, NEAR 0.0039/13.46, Cassini 0.0066/2.0, Rosetta 0.0031/1.82 — `rift-excluded`; **MESSENGER 0.0325/0.02 → `rift-generator`** (2026-09-30 auf alten Bins noch 0.0168 → `rift-excluded`). Nachmessung (`ephemeris_granule_census --series --date 2005-08-02 --half-days 3`): der MESSENGER-Epoch fällt exakt auf eine **DE- und EPM-Granulatgrenze** (JD 2453584.5), 1-h-Änderung DE-INPOP/INPOP-EPM 13.4 mm/s in der angrenzenden Stunde, DE-EPM flach 0.0 mm/s — dieselbe Granulat-Artefakt-Klasse wie folge216, **nicht physikalisch**; die physische Schranke bleibt der Rat-Chord (3.65 µm/s). `ephemeris_house_gate` run `36946576753` **queued** (Runner-Queue).
- **Blockade:** CI-Runner-Queue des Haus-Gates.
- **Braucht:** `ci_manage view 36946576753` einmal, wenn der Lauf durch ist — dann `docs/paper/eclipse-clock-worldlines.md:45` von `pending until re-manifest` lösen; das MESSENGER-Verdikt als Granulat-Artefakt (nicht physikalisch) in `docs/paper/flyby-path-2-falsification-metric-addendum.md` fortschreiben. Riss: die präregistrierten Trajektorie-Hashes an `flyby-path-2-preregistration.md:21/23` sind mit dem Re-Manifest neu gesiegelt (post-hoc-Neusiegelung unzulässig, tragen).

### DE441 Granulat-Naht — Duplikat am part-1/part-2-Seam
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `de44-cdn`-Lauf → `ephemeris_granule_census` zeigt `overlapping pairs 0`.
- **Lage:** (gemessen 2026-10-02 via `ephemeris_granule_census`) genau **1 überlappendes Paar** im DE441-Bin: die beiden Kernel-Teile emittieren am Seam dieselbe t0 (JD 2440416.5), also ein Duplikat. Die frühere Census-Anzeige „32-d-Lücke / 80 M km" war der Vergleich der fernen Kanten dieses Duplikats — der Census zählte **keine** Lücke. `inpop`/`epm` (ein Kernel) tragen 0 Duplikate.
- **Blockade:** keine.
- **Braucht:** `de_compiler` dedupliziert jetzt per t0 (`tools/harvest/src/bin/de_compiler.rs`, multi-part-Kernel); nach dem nächsten `de44-cdn`-Lauf Census erneut (Trigger). Werte der erhaltenen Granulen unverändert — kein Hash-Riss.

### ASCAT/OSI-SAF Source-Compiler — gebaut und gemessen
- **Status:** wartend | **Bindung:** eigen (Workflow: mycelium)
- **Trigger:** `manati.star.nesdis.noaa.gov-ascat`-CDN-Workflow (Mycelium) → dann Manifestation.
- **Lage:** (gemessen 2026-10-02, grind-flash) `tools/harvest/src/bin/ascat_compiler.rs` gebaut; liest NetCDF-4 über `hdf5.rs` (`geo_series_parse_bin("ascat_wind")`), Fähigkeiten in `src/archivar/geo.rs` + `extract.rs` ergänzt. Gemessen am manati-UHR-File (`https://manati.star.nesdis.noaa.gov/UHR_ASCAT/UHR_ASCATB/2024/ALVARO_20240101_58569_B_D-cmod5h-scaled_v2.nc`, 89.992.794 B): 53.277 records (41.625 speed, 11.652 direction), bin 3.196.628 B, roundtrip parst. **Riss:** der erddap-Host (`erddap.aoml.noaa.gov`, am 2026-10-01 noch HTTP 200) war 2026-10-02 nicht erreichbar — TLS-Timeout direkt + Proton, kein Wayback. Register-Zeile gesetzt: `url …/manati.star.nesdis.noaa.gov-ascat/ascat_uhr_ascat_b.bin`, `format ascat_wind`, `compiler tools/harvest/src/bin/ascat_compiler.rs`, `on earth`, zwei `field`-Zeilen.
- **Blockade:** der CDN-Workflow (Mycelium-Feder) fehlt.
- **Braucht:** Mycelium legt den CDN-Workflow für `manati.star.nesdis.noaa.gov-ascat` an (`cargo run -p omegaflow-harvest --release --bin ascat_compiler -- <url> --label <label> --ci-mode`).

### CEERS (z≳10) — Source-Compiler gebaut und gemessen
- **Status:** wartend | **Bindung:** eigen (Workflow: mycelium)
- **Trigger:** `web.corral.tacc.utexas.edu`-CDN-Workflow (Mycelium).
- **Lage:** (gemessen 2026-10-02, grind-flash) `tools/harvest/src/bin/ceers_spectra_compiler.rs` neu; der x1d-Tabellenparser liegt geteilt in `src/archivar/jwst.rs`, `jwst_spectra_compiler.rs` nutzt ihn. Am FITS `…/nirspec4/prism/hlsp_ceers_jwst_nirspec_nirspec4-000323_prism_v0.7_x1d.fits` (123.840 B): 336 Bins, bin 8.200 B, roundtrip parst; Ledger/sidecar idempotent (zweiter Lauf byte-identisch). Register-Zeile gesetzt: `url …/web.corral.tacc.utexas.edu/ceers_spectra.bin`, `format jwst_spectra`, `compiler tools/harvest/src/bin/ceers_spectra_compiler.rs`. **Riss:** die Alt-Zeile `curated48_spectra.bin` tagt `ssd.jpl.nasa.gov`, ihr Compiler lädt aber nach `exoplanetarchive.ipac.caltech.edu` — Tag-Mismatch.
- **Blockade:** der CDN-Workflow fehlt; der Tag-Mismatch der Alt-Quelle ist ungeheilt.
- **Braucht:** Mycelium legt den CDN-Workflow für `web.corral.tacc.utexas.edu` an; Alt-Mismatch abgleichen.

### future-folge165 Kandidaten-Quellen — Register-Verdikt am Baum
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02 via `--verdict`/`--sniff` + `sgrep phi`) **schon gedeckt, kein Eintrag**: CHIME Cat1 `sources.φ:9095` · Horizons `horizons_compiler` · CEERS `sources.φ:10926` + `ceers_spectra_compiler` · Swarm-TEC `sources.φ:7253` (vires HAPI) · FMI-GIC · NCEI · ALeRCE `sources.φ:532` · LISA-PF-Mirror `sources.φ:9347` (`drs_fits`) · OpenNeuro `sources.φ:2451ff` + `openneuro_compiler` · KASI-FITS `blocked_sources.φ:452` (`kmtnet_archive_moc.fits`). **Neu (2026-10-02): Arme gebaut + Registerzeilen gesetzt** — JADES-FITS `jades.herts.ac.uk/DR4/` → `jades_spectra_compiler.rs` (bin 1.229.360 B, 5190 records, roundtrip; `phi/sources.φ` `jades_spectra.bin`, `format jwst_spectra`, field `jades_line_flux_W_m2`); DSN-Livefeed `eyes.nasa.gov/dsn/data/dsn.xml` → `dsn_compiler.rs` + `src/archivar/dsn.rs` (bin 662 B, 21 Signale; `dsn_snapshot.bin`, `format dsn_snapshot`, ttl 60, **nur Transport**). **Noch ohne Arm:** BICEP/Keck `http://bicepkeck.org/` (Verzicht-Leiter nur 206/1 B + Wayback 2014) · BioStudies (kein Arm) · Gaia DR4 `https://www.cosmos.esa.int/web/gaia/dr4` (Datum 2026-12-02 bestätigt, Daten noch nicht offen). **Riss:** „ALeRCE-API lebt" ist widerlegt — `--verdict https://api.alerce.online/alerts/v1/objects/…` = pending (direkt + Proton). M3: `ode.rsl.wustl.edu` ist Pagehelp, **kein** Daten-Mirror.
- **Blockade:** die neuen Endpunkte haben keinen Compiler-Arm.
- **Braucht:** (a) **erledigt** — River folge78 hat den Consumer auf `jwst_spectrum_motion(rec, anchor)` umgestellt (`src/archivar/main_flow.rs:27/2606`); der `plx_mas`-Skip ist aus dem `jwst_spectra`-Pfad entfernt, extragalaktische Records tragen am deklarierten `at`-Anker. (b) **erledigt** — die Zahl war veraltet, live 94.124.160 B. (c) **gemessen geschlossen** — der Consumer trägt `spec.bins` unverändert weiter; die Einheit lebt in der `field`-Zeile der Quelle, `main_flow` nimmt keine Per-Hz-Dichte an, kein Konflikt. (d) **erledigt** — Einheit belegt (NASA `eyes.nasa.gov/apps/dsn-now/javascripts/main.js:633–647`: downSignal dBm, upSignal kW, Render-Gate auf `active`); `field`-Zeilen `dsn_{s,x,k,ka}_down_power` (dBm) + `dsn_{s,x}_up_power` (kW), Medium `em`, gesetzt. Mycelium: CDN-Workflows für `jades.herts.ac.uk` und `eyes.nasa.gov`.

### future-folge165 Mail-Routen — Verdikt bzw. als überholt schließen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02 via `--verdict`/curl) CSES `/query.php` = SSDC-CAS-Login (`<title>Login</title>`) → **Wall**; DEMETER Order-API `regards.cnes.fr/api/v1/rs-order` 403 → **blocked account** (Order 18400 läuft, `state/zustand/wartend.φ:4`); DEMETER SPASE **offener korrigierter Pfad** `https://cnes.github.io/CDPP/CNES/NumericalData/CDPP-Archive/DEMETER/IAP/DMT_N1_1140.xml` (200, echtes SPASE 2.4.0) — der Claim-Pfad ist 404; Roh-ODF Cassini `…/titan/s34/O050/` **offen** (200, 51 Links); LISA-PF Science-Archive = COSMOS Sign-in → Wall, Mirror `heasarc…/FTP/lpf/` **schon registriert** (`sources.φ:9347`); ShadowCam `pds.shadowcam.im-ldi.com/derived/` **offen** (200, PDS4-Listing; `blocked_sources.φ:459` pending); Juno-ODF `pds-atmospheres.nmsu.edu/…/JUNO/logs/…csv` **offen**; NSSDCA `obtain?PSNO-…` 404 → Navigation nötig.
- **Blockade:** keine.
- **Braucht:** die zwei offenen Cassini-/Juno-Listings als `url` nach Arm-Prüfung; den DEMETER-SPASE-Pfad als `origin` an `blocked_sources.φ:99`; die CSES/DEMETER-Mail-Walls an `Future` (DEMETER-Order Ablauf 2026-10-05).

### future-folge165 Datenbestand-Move
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02 via `glob`) `who_flunet_influenza_api_full.json` und `nbp_Lmon_CanESM5-CanOE_historical_r1i1p2f1_gn_185001-201412.nc` liegen bereits unter `~/archive/archive-root/declined/`. Der WMM/Kernel-Satz ist noch nicht geprüft.
- **Blockade:** keine.
- **Braucht:** WMM/Kernel-Satz lokalisieren (`glob **/WMM*`, `glob **/*.COF`); vorhandenes als `descoped`/`disponiert` benennen, fehlendes als `pending` registrieren.

### JWS1-Kontrakt — extragalaktische Spektren ohne Richtungs-Slot
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Contract-Entscheid, ob JWS1 einen `z`/Distanz-Slot trägt.
- **Lage:** (gemessen 2026-10-02, river-folge78 + `src/archivar/main_flow.rs:27`) der Wire hat ohne Abstand keinen Richtungs-Slot; extragalaktische Spektren (JADES/CEERS) sitzen am deklarierten `at`-Anker (sun), die eigentliche Messung (`spec.bins`) ist vollständig geführt. Eine echte Richtungs-/Distanz-Führung bräuchte ein `z` im JWS1-Bin.
- **Blockade:** kein Record trägt eine Distanz (Parallaxe absent, kein `z`).
- **Braucht:** Mountain-Contract-Entscheid: `z`-Slot im JWS1-Bin (`src/archivar/jwst.rs`) + Compiler (`jades_spectra_compiler`/`ceers_spectra_compiler`) + `phi/harvest.φ`; danach Mycelium-Manifestation.

### future-folge166 gefaltet — Korona-Netloc, M3-Mirror, KASI, CDSE-CCM
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02, `## An mountain` future-folge166) (1) **Ⅲ Korona:** `ssd.jpl.nasa.gov` ist **kein** GOES-Host — die Korona-Probes laden den falschen Netloc; richtig sind NCEI `https://www.ncei.noaa.gov/data/goes-space-environment-monitor/access/science/xrs/` (200) und CDAWeb `https://cdaweb.gsfc.nasa.gov/` (200). (2) **M3 non-JPL:** WUSTL ODE `https://ode.rsl.wustl.edu/moon/m3.aspx` (206), SBN/PSI ArcNav `https://arcnav.psi.edu/urn:nasa:pds:context:instrument:m3.ch1-orb` (200), USGS `https://astrogeology.usgs.gov/search/missions/chandrayaan-1/moon-mineralogy-mapper` (206) — Admission/Parser-Arm offen, JPL-IP-Block bleibt. (3) **KASI/Danuri:** Portal `pda.kasi.re.kr/mission-danuri.php` (200) ist kein Datenkanal; der Daten-Endpoint bleibt hinter dem Login (`blocked_sources.φ:447`). (4) **CDSE-CCM:** Operator-Wort 2026-10-01 (keine U-R-P/N-P-A-Zugehörigkeit → View only); `blocked_sources.φ:463` steht `blocked account`.
- **Blockade:** keine.
- **Braucht:** (1) Probe-/Register-Netloc Korona auf NCEI/CDAWeb korrigieren; (2) M3-Mirror-Arm bauen oder Admission entscheiden; (3) CDSE-CCM: die Note `:466` trägt das Operator-Wort (P=View, kein Download) bereits — nur noch schließen/durchsehen; (4) KASI-Daten-Endpoint messen oder Disposition.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ / Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Absolutpfade auf `~`-relativ gesetzt.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die Pending-Einträge sind im `dead_sources.φ` disponiert.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende Messreihe; jede Zelle mit `file:line`/`pending`.

## An mycelium

Origin: mountain folge222.

- **Vier neue Source-Compiler brauchen je einen CDN-Workflow** (Mycelium-Feder): `ascat_compiler` (netloc `manati.star.nesdis.noaa.gov-ascat`, asset `ascat_uhr_ascat_b.bin`), `ceers_spectra_compiler` (`web.corral.tacc.utexas.edu`, `ceers_spectra.bin`), `jades_spectra_compiler` (`jades.herts.ac.uk`, `jades_spectra.bin`) und `dsn_compiler` (`eyes.nasa.gov`, `dsn_snapshot.bin`). Die `url`/`format`/`compiler`/`origin`-Zeilen stehen in `phi/sources.φ`; ohne Workflow bleibt der `url`-Zeiger unmanifestiert.
- **Tag-Mismatch-Riss:** `curated48_spectra.bin` tagt `ssd.jpl.nasa.gov`, ihr `jwst_spectra_compiler` lädt nach `exoplanetarchive.ipac.caltech.edu` — beim Anlegen des CEERS-Workflows mit abgleichen.
- **Riss Route:** `erddap.aoml.noaa.gov` (ASCAT, am 2026-10-01 HTTP 200) ist 2026-10-02 nicht erreichbar (TLS-Timeout direkt + Proton, kein Wayback). Die arbeitsfähige Route ist `manati.star.nesdis.noaa.gov`.
- **Probe-Workflows geliefert (danke):** `ephemeris-probes.yml` (eclipse-shadow + flyby-anderson) + CDN-Release `anderson-residuals` existieren; `ephemeris_house_gate` (`36946576753`) läuft über seinen Workflow. Der `anderson-residuals`-Registerblock liegt uncommittet in `phi/sources.φ` (Mycelium) — beim Commit nicht mitgerissen.

## An river

Origin: mountain folge222.

- **`jwst_spectra`-Parallax-Gate (`src/archivar/main_flow.rs:2565`, deine Membran-Feder):** (gemessen 2026-10-02) der Consumer baut je `jwst_spectra`-Record einen `StarRec` und skippt bei `spec.plx_mas <= 0.0` mit `named_skips`; bei `named_skips == total` wird `empty(true)` gesendet. Extragalaktische Spektralquellen tragen Parallaxe `absent` (0 = honored sentinel) → **alle** Records würden als „without parallax" verworfen: JADES-DR4 5190/5190, CEERS ebenso. Der `format jwst_spectra`-Arm produziert also ein Bin, das die Membran nie aufnimmt. Braucht eine Consumer-Entscheidung: extragalaktische Spektralrecords dürfen nicht am Stern-Parallax-Gate hängen (`plx_mas` als absent/pending führen, nicht als Skip; `spec.bins` trägt die eigentliche Messung ohnehin).
