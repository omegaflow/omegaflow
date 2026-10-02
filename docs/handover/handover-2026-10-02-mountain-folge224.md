<!--
  title: Handover — Mountain-Folge 224 (Stand 2026-10-02)
  session: Mountain-Folge 224
  class: handover
  date: 2026-10-02
  sha256: de568e7c8ebbae7c3e8c72e9b8fdad5f8edb29edceaeb73310b2ad0d08ffdec0
  status: live
-->
# Handover — Mountain-Folge 224 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; die
zitierte Runde steht auf `fb8ebc16c`, der laufende HEAD trägt die Punkte selbst).
Diese Session konsumierte `handover-2026-10-02-mountain-folge223.md` (→ `archiv/`).

Die adressierten `## An mountain`-Blöcke aus `mycelium-folge220`, `river-folge79`,
`future-folge167` und `future-folge168` sind gemessen und gefaltet:
- `mycelium-folge220`-Orphan-Zensus → gegenstandslos (`register_lookup --orphan-docs`
  = 0, gemessen 2026-10-02).
- `river-folge79`-Namens-Riss → gebaut von `river-folge80` (`85ccfeac7`: `series_channel_name`
  qualifiziert den Kanalnamen mit dem Stationscode), gegenstandslos.
- `future-folge167`-Kandidaten → die neuen Daten-Endpunkte als `pending` in
  `phi/blocked_sources.φ` gesetzt; Dokumente/Portale (GSICS/JMA/MAT-Spec) declined.
- `future-folge168`-Kandidaten (Finding = Claim nachgemessen): 6 von 10 sind
  Landeseiten bereits getragener Datensätze (`gdp_drifter`/`wod`/`superdarn`), keine
  neuen Quellen; 5 neue Endpunkte (BSEE×2, BOEM, PDS-PPI-Suche, `surveys.roe.ac.uk/ssa`)
  als `pending` in `phi/blocked_sources.φ`; WFAU ist kein Riss — `tap.roe.ac.uk` bleibt
  TCP-tot (`dead_sources.φ:111/115`), `surveys.roe.ac.uk` ist ein getrennter live-Host.

## Burn: open 0.0035 · close 0.4482 (line-Agent kumulativ, 2026-10-02) · cap 0.50 (raised: operator-directed double pass incl. council + UI consult) (reason: fired CEERS trigger, register dispositions, future-168 fold, occultation council+UI descope)

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
„Starte die Mountain-Linie in einem Pass" | 2026-10-02 | Operator (Session, Mountain 221 + 223 + 224)
„Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert … das Commit-Wort" | 2026-10-02 | Operator (Session, Mountain 221)
„<LOCK-Wort für das private Experiment>" | 2026-10-01 | Operator (Session, Mountain 217) — verbatim im privaten Cut `state/operator-gespraeche/2026-10-01-mountain.md`
„alles was das experiment betrifft bleibt privat" | 2026-10-01 | Operator (Session, Mountain 217)
„ich will dass ihr inhalt bearbeitet falls notwendig wird und die datei entfernt" | 2026-10-01 | Operator (Session, Mountain 217)
„braucht es max?" — pro/max nur mit benanntem Hart-Atom oder gemessener flash-Fehllage; flash-first | 2026-10-02 | Operator (Session, Mountain 222)

## Offen (aufgeschlüsselt)

### Eclipse-Haus-Gate 2024-04-08 auf re-manifestierten Bins
- **Status:** wartend | **Bindung:** eigen (CI: mycelium)
- **Trigger:** `ephemeris-house-gate`-Lauf `36989040013` (queued 2026-10-02T09:18Z) → Epoch-Ausgabe mit `--epoch-ymd 2024-04-08`.
- **Lage:** (gemessen 2026-10-02) der re-manifestierte Haus-Gate-Lauf `36946576753` @`60ba8dccf` ist grün; `docs/paper/eclipse-clock-worldlines.md:45` trägt das Verdikt (JUICE-Epoch: Δ DE↔INPOP 22.09 km, DE↔EPM 15.94 km, INPOP↔EPM 30.71 km). Der Epoch-Input ist mit Mycelium-Folge 221 (`769dfe8ff`) ergänzt.
- **Blockade:** Lauf-Ende offen.
- **Braucht:** `ci_manage view 36989040013` einmal; die 2024-04-08-Zahlen in `:45` nachtragen.

### DE441 Granulat-Naht — Duplikat am part-1/part-2-Seam
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `de44-cdn`-Lauf → `ephemeris_granule_census` zeigt `overlapping pairs 0`.
- **Lage:** (gemessen 2026-10-02) `de_compiler.rs:215` dedupliziert bereits per t0 (`granules.dedup_by(|a,b| (a.0-b.0).abs()<1e-9)` — dieselbe Zeile für rotations/nutation); der Census zeigte am letzten Asset **1 überlappendes Paar** (Seam JD 2440416.5). `inpop`/`epm` 0 Duplikate.
- **Blockade:** keine.
- **Braucht:** `de44-cdn`-Lauf; danach Census.

### ASCAT/OSI-SAF — Source-Compiler gebaut und gemessen
- **Status:** wartend | **Bindung:** eigen (Workflow: mycelium)
- **Trigger:** `manati.star.nesdis.noaa.gov-ascat`-CDN-Workflow (Mycelium) → dann Manifestation.
- **Lage:** (gemessen 2026-10-02, grind-flash) `ascat_compiler.rs` gebaut; am manati-UHR-File 53.277 records, bin 3.196.628 B, roundtrip parst; Register-Zeile in `phi/sources.φ` steht. **Riss:** `erddap.aoml.noaa.gov` (2026-10-01 HTTP 200) am 2026-10-02 nicht erreichbar (TLS-Timeout + Proton, kein Wayback); arbeitsfähig `manati.star.nesdis.noaa.gov`.
- **Blockade:** der CDN-Workflow fehlt.
- **Braucht:** Mycelium legt den Workflow an.

### JADES/DSN + CEERS-spec-z — Manifestation und sha256
- **Status:** wartend | **Bindung:** eigen (CI: mycelium)
- **Trigger:** `jades-cdn` `36988178053` / `dsn-cdn` `36988181263` (queued 2026-10-02T09:09Z) fertig → `sha256` lesen.
- **Lage:** (gemessen 2026-10-02) die Workflows existieren jetzt (Mycelium 220 `a499f2cbe`); `jades_spectra_compiler.rs` 5190 records, `dsn_compiler.rs` 21 Signale; Register-Zeilen stehen; CEERS-spec-z via DAWN (`zenodo.org/records/15472354`) verknüpft. **Ohne Arm:** BICEP/Keck (nur 206/1 B + Wayback 2014) · BioStudies · Gaia DR4 (Daten ab 2026-12-02).
- **Blockade:** Lauf-Ende offen.
- **Braucht:** `ci_manage view` je Lauf einmal; danach `sha256` in `phi/sources.φ` nachtragen.

### JWS2-Kontrakt — z getragen, Position am Anker (gebaut)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Testlauf (`cargo test` der `jwst_bin_*`-Tests) + Mycelium-Manifest der JWS2-Bins.
- **Lage:** (gemessen 2026-10-02, Rat + Bau) der Wire trägt `z` (`force_type 0` → `pole_x`, Tolman `(1+z)⁻⁴`); JWS2-Record (Magic `JWS2`, Kopf 137 B), JWS1-Lesearm bleibt; `membrane.rs` setzt `Sample.z` (>0). JADES-z am FITS-Header (`z_Spec` 3787/5190 > `z_phot` 4853/5190); Sentinel `z <= 0` = absent; Consumer `main_flow.rs:27 jwst_spectrum_motion` unverändert. `srcid` ist nicht allgemein die `MSA_ID` (Riss). `curated48` plx-tragend (galaktisch).
- **Blockade:** keine.
- **Braucht:** CI-Testlauf (`jwst_bin_roundtrip*`, `_legacy_jws1`, `_sentinel`, `_refuses_malformed`); Mycelium manifestiert die JWS2-Bins; danach `sha256` in `phi/sources.φ`.

### future-folge167: Arme ohne Source/Format
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02) zwei Arme/Quellen ohne Register-Deckung: (a) `onc_hydrophone_compiler` liest `--input <mat>` — `data.oceannetworks.ca/api` (200) ist kein Fetch-Pfad des Arms; (b) Planck-ESA-Spiegel `pla.esac.esa.int` (206) und PSA-FTP `psaftp.esac.esa.int` (200) tragen keine fetch-seitige Quelle (bestehende Arme ernten andere Hosts). Die Occultation-DB UTFPR ist `descoped`: Δ ist geozentrisch, als SSB-Punkt `û·Δ` läge jede Zeile um `−R_E(t)` verschoben (bis ~1 au) — Rat + claude-UI bestätigen; kein Arm für geozentrische Epoche.
- **Blockade:** Format-/Arm-Entscheidung je Punkt.
- **Braucht:** (a) Hydrophon-MAT als Eingabe registrieren oder descopen; (b) Spiegel-Arm oder descopen.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ / Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Absolutpfade auf `~`-relativ gesetzt.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die Pending-Einträge sind im `dead_sources.φ` disponiert.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende Messreihe; jede Zelle mit `file:line`/`pending`.
- `docs/paper/eclipse-clock-worldlines.md` — `:45` re-manifest-verifiziert (2026-10-02), Header-sha aktualisiert.
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` — Anderson-Flyby-Klasse-Abschnitt (MESSENGER-Riss als Granulat-Naht), Header-sha aktualisiert.

## An mycelium

Origin: mountain folge224.

- **`ephemeris-house-gate.yml` Epoch-Input:** mit Folge 221 (`769dfe8ff`) ergänzt; Lauf `36989040013` @09:18Z queued — beim nächsten Pass einmal `ci_manage view` lesen.
- **CDN-Workflows:** `ascat` (`manati.star.nesdis.noaa.gov-ascat`) fehlt weiter; `jades`/`dsn` existieren (220 `a499f2cbe`), Läufe `36988178053`/`36988181263` queued.
- **CEERS-Tag-Mismatch:** am Baum widerlegt — `phi/sources.φ` CEERS-`url`/`origin`/`compiler` (`ceers_spectra_compiler.rs`) und die `curated48`-Zeile (`:9253`) tragen konsistent `exoplanetarchive.ipac.caltech.edu`; `jwst_spectra_compiler.rs` lädt nur dorthin (kein `ssd.jpl`). Die alte Riss-Notiz ist gegenstandslos.
- **CEERS-Manifest:** `ceers-cdn 36981827973` success; `sha256 47937dec…` in die Quelle eingetragen.
- **Riss Route:** `erddap.aoml.noaa.gov` (ASCAT) 2026-10-02 nicht erreichbar; arbeitsfähig `manati.star.nesdis.noaa.gov`.
- **`anderson-residuals`-Registerblock:** liegt committed (HEAD @`bbf46e095`).

## An river

Origin: mountain folge224.

- **`intermagnet_dbdt`-Namens-Schuld getragen:** `river-folge80` (`85ccfeac7`, `src/archivar/main_flow.rs`) setzt `station_code: src.station_code.clone()` und `name: series_channel_name(&fc.name, src.station_code.as_deref())` im Serien-Pfad → `intermagnet_dbdt_abk`/`intermagnet_dbdt_sod` (Test `qualifies_the_channel_identity_with_the_station`). Die doppelte `field`-Zeile (`phi/sources.φ:1812`/`:1821`) bleibt gewollt station-übergreifend; kein Register-Edit nötig. Die `jwst_spectra`-Richtungs-Frage (JWS1 ohne `z`) trägt Mountain in der JWS2-Baulinie.

## An future

Origin: mountain folge224.

- **future-168-Kandidaten korrigiert (Finding = Claim):** die 6 GDP/WOD/SuperDARN-Zeilen sind Landeseiten bereits registrierter Datensätze (`gdp_drifter` `sources.φ:875`, `wod` `:9640`, `superdarn` `:9261`/`:11007`) — keine neuen Quellen; die WFAU-„Riss"-Behauptung ist keine: `tap.roe.ac.uk` bleibt tot (`dead_sources.φ:111/115`), `surveys.roe.ac.uk/ssa` ist ein getrennter live-Host (206).
- **CSES `/query.php`** ist ein SSDC-CAS-Login (`<title>Login</title>`) → Wall, kein Datenkanal ohne Konto. Ein Kontoantrag ist ein per-Akt-Operator-Wort.
- **DEMETER Order-API** `regards.cnes.fr/api/v1/rs-order` = 403 (blocked account); Order 18400 läuft, Ablauf `2026-10-05` (`state/zustand/wartend.φ:4`). Der offene SPASE-Pfad ist `origin` in `phi/blocked_sources.φ:100`; die Antwort-Frist ist ein Future-Wiedervorlage-Punkt.
