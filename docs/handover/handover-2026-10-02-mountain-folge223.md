<!--
  title: Handover — Mountain-Folge 223 (Stand 2026-10-02)
  session: Mountain-Folge 223
  class: handover
  date: 2026-10-02
  sha256: b1526cd8441bc4681c7954a1a49b821d7dc9cbd1201468357202ef6e6e53af77
  status: live
-->
# Handover — Mountain-Folge 223 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; die
zitierte Runde steht auf einem älteren HEAD, die Punkte tragen ihre eigene Messung).
Diese Session konsumierte `handover-2026-10-02-mountain-folge222.md` (→ `archiv/`).
Die adressierten `## An mountain`-Blöcke aus `mycelium-folge219` und `river-folge78`
sind gemessen und in die Offen-Liste gefaltet; der `future-folge166`-Block ist
abgearbeitet (Verdikte im Register).

## Burn: open 0.0272 · close 0.2141 · cap 0.40 (reason: operator-directed single-pass — ephemeris re-manifest verdict, the two paper seals, the register dispositions)

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
„Starte die Mountain-Linie in einem Pass" | 2026-10-02 | Operator (Session, Mountain 221 + 223)
„Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert … das Commit-Wort" | 2026-10-02 | Operator (Session, Mountain 221)
„<LOCK-Wort für das private Experiment>" | 2026-10-01 | Operator (Session, Mountain 217) — verbatim im privaten Cut `state/operator-gespraeche/2026-10-01-mountain.md`
„alles was das experiment betrifft bleibt privat" | 2026-10-01 | Operator (Session, Mountain 217)
„ich will dass ihr inhalt bearbeitet falls notwendig wird und die datei entfernt" | 2026-10-01 | Operator (Session, Mountain 217)
„braucht es max?" — pro/max nur mit benanntem Hart-Atom oder gemessener flash-Fehllage; flash-first | 2026-10-02 | Operator (Session, Mountain 222)

## Offen (aufgeschlüsselt)

### Eclipse-Haus-Gate 2024-04-08 auf re-manifestierten Bins
- **Status:** wartend | **Bindung:** eigen (CI: mycelium)
- **Trigger:** `ephemeris-house-gate`-Lauf mit `--epoch-ymd 2024-04-08` (Workflow-Input fehlt).
- **Lage:** (gemessen 2026-10-02) der re-manifestierte Haus-Gate-Lauf `36946576753` @`60ba8dccf` ist grün; `docs/paper/eclipse-clock-worldlines.md:45` trägt das Verdikt (JUICE-Epoch: Δ DE↔INPOP 22.09 km, DE↔EPM 15.94 km, INPOP↔EPM 30.71 km); die 2024-04-08-Zahlen dieser Zeile sind als 2026-10-01-Pass auf den Vor-Fix-Bins datiert.
- **Blockade:** der Workflow läuft nur den Default-JUICE-Epoch (kein `--epoch-ymd`-Input).
- **Braucht:** Mycelium ergänzt einen Epoch-Input/Step in `.github/workflows/ephemeris-house-gate.yml`; dann einmal lesen.

### DE441 Granulat-Naht — Duplikat am part-1/part-2-Seam
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `de44-cdn`-Lauf → `ephemeris_granule_census` zeigt `overlapping pairs 0`.
- **Lage:** (gemessen 2026-10-02 via `ephemeris_granule_census`) genau **1 überlappendes Paar** im DE441-Bin: die beiden Kernel-Teile emittieren am Seam dieselbe t0 (JD 2440416.5). `inpop`/`epm` (ein Kernel) tragen 0 Duplikate.
- **Blockade:** keine.
- **Braucht:** `de_compiler` dedupliziert per t0 (`tools/harvest/src/bin/de_compiler.rs`); nach dem nächsten `de44-cdn`-Lauf Census erneut (Trigger).

### ASCAT/OSI-SAF Source-Compiler — gebaut und gemessen
- **Status:** wartend | **Bindung:** eigen (Workflow: mycelium)
- **Trigger:** `manati.star.nesdis.noaa.gov-ascat`-CDN-Workflow (Mycelium) → dann Manifestation.
- **Lage:** (gemessen 2026-10-02, grind-flash) `tools/harvest/src/bin/ascat_compiler.rs` gebaut; am manati-UHR-File 53.277 records, bin 3.196.628 B, roundtrip parst. Register-Zeile in `phi/sources.φ` gesetzt. **Riss:** `erddap.aoml.noaa.gov` (2026-10-01 HTTP 200) am 2026-10-02 nicht erreichbar (TLS-Timeout + Proton, kein Wayback); arbeitsfähige Route ist `manati.star.nesdis.noaa.gov`.
- **Blockade:** der CDN-Workflow (Mycelium-Feder) fehlt.
- **Braucht:** Mycelium legt den CDN-Workflow für `manati.star.nesdis.noaa.gov-ascat` an (`cargo run -p omegaflow-harvest --release --bin ascat_compiler -- <url> --label <label> --ci-mode`).

### CEERS (z≳10) — Source-Compiler gebaut und Workflow läuft
- **Status:** wartend | **Bindung:** eigen (Workflow: mycelium)
- **Trigger:** `ceers-cdn` run `36981827973` (queued 2026-10-02T08:02Z) fertig → Manifest lesen.
- **Lage:** (gemessen 2026-10-02, grind-flash) `ceers_spectra_compiler.rs` neu; am CEERS-NIRSpec-Prism-FITS 336 Bins, roundtrip parst; Register-Zeile in `phi/sources.φ`. Der `ceers-cdn`-Workflow existiert jetzt (Run läuft). **Riss:** die Alt-Zeile `curated48_spectra.bin` tagt `ssd.jpl.nasa.gov`, ihr Compiler lädt nach `exoplanetarchive.ipac.caltech.edu` — Tag-Mismatch.
- **Blockade:** Manifest-Lauf offen; Alt-Tag-Mismatch ungeheilt.
- **Braucht:** `ci_manage view 36981827973` einmal; Alt-Mismatch abgleichen (Mycelium).

### JADES/DSN + future-folge165 Quellen — Register-Verdikt
- **Status:** wartend | **Bindung:** eigen (Workflow: mycelium)
- **Trigger:** CDN-Workflows für `jades.herts.ac.uk` und `eyes.nasa.gov` (Mycelium).
- **Lage:** (gemessen 2026-10-02) JADES-FITS → `jades_spectra_compiler.rs` (5190 records, roundtrip; `phi/sources.φ`), DSN-Livefeed → `dsn_compiler.rs` (21 Signale; ttl 60, nur Transport); Einheiten belegt (DSN down dBm / up kW, Render-Gate `active`). **Neu (2026-10-02): CEERS-spec-z** via DAWN-Archive (`grizli-cutout.herokuapp.com`, Zenodo `15472354`) verknüpft; `origin` in `phi/sources.φ`. **Noch ohne Arm:** BICEP/Keck (nur 206/1 B + Wayback 2014) · BioStudies · Gaia DR4 (Daten noch nicht offen, Datum 2026-12-02).
- **Blockade:** CDN-Workflows `jades-cdn`/`dsn` fehlen.
- **Braucht:** Mycelium legt die zwei Workflows an; danach `sha256` in `phi/sources.φ` nachtragen.

### future-folge165 Mail-Routen — Verdikt bzw. als überholt schließen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02 via `--verdict`/curl) CSES `/query.php` = SSDC-CAS-Login (`<title>Login</title>`) → Wall; DEMETER Order-API 403 → blocked account (Order 18400, `state/zustand/wartend.φ:4`); DEMETER-SPASE **offener korrigierter Pfad** `https://cnes.github.io/CDPP/CNES/NumericalData/CDPP-Archive/DEMETER/IAP/DMT_N1_1140.xml` (200) — als `origin` an `phi/blocked_sources.φ:100` gesetzt; Roh-ODF Cassini `…/titan/s34/O050/` offen (200); LISA-PF-Mirror schon registriert (`sources.φ:9347`); ShadowCam `pds.shadowcam.im-ldi.com/derived/` offen; Juno-ODF `pds-atmospheres.nmsu.edu/…/logs/…csv` offen.
- **Blockade:** keine.
- **Braucht:** die zwei offenen Cassini-/Juno-Listings als `url` nach Arm-Prüfung (Mountain); die CSES/DEMETER-Mail-Walls an Future (siehe `## An future`).

### future-folge165 Datenbestand-Move
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02 via `glob`) `who_flunet_influenza_api_full.json` und `nbp_Lmon_CanESM5-CanOE_historical_r1i1p2f1_gn_185001-201412.nc` liegen bereits unter `~/archive/archive-root/declined/`. Der WMM/Kernel-Satz ist noch nicht geprüft.
- **Blockade:** keine.
- **Braucht:** WMM/Kernel-Satz lokalisieren (`glob **/WMM*`, `glob **/*.COF`); vorhandenes als `descoped`/`disponiert` benennen, fehlendes als `pending` registrieren.

### JWS2-Kontrakt — z getragen, Position am Anker (gebaut)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Testlauf (`cargo test` der `jwst_bin_*`-Tests) + Mycelium-Manifest der JWS2-Bins.
- **Lage:** (gemessen 2026-10-02, Rat + Bau) der Wire trägt `z` (`force_type 0` → `pole_x`, Tolman `(1+z)⁻⁴`); JWS2-Record (Magic `JWS2`, Kopf 137 B), JWS1-Lesearm bleibt; `membrane.rs` setzt `Sample.z` (>0). JADES-z am FITS-Header gemessen (`z_Spec` 3787/5190 > `z_phot` 4853/5190); Sentinel `z <= 0` = absent; Consumer `main_flow.rs:27 jwst_spectrum_motion` unverändert. CEERS-z via DAWN-spec-z getragen; `srcid` ist nicht allgemein die `MSA_ID` (Riss). `curated48` plx-tragend (galaktisch).
- **Blockade:** keine.
- **Braucht:** CI-Testlauf (`jwst_bin_roundtrip*`, `_legacy_jws1`, `_sentinel`, `_refuses_malformed`); Mycelium manifestiert die JWS2-Bins; danach `sha256` in `phi/sources.φ` messen.

### intermagnet_dbdt — Feldname am ABK/SOD-Block doppelt (adressiert von river-folge78)
- **Status:** eigen | **Bindung:** eigen (Konsument: river)
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02) `field intermagnet_dbdt intermagnet_dbdt …` steht am ABK-Block (`phi/sources.φ:1812`) **und** am SOD-Block (`:1821`) — dieselbe Zeichenkette für zwei Stationen (`station ABK`/`station SOD`). `Matrix::metas` ist nach `channel.name` geschlüsselt (`src/mathematikerin/machines/matrix.rs:679`), `channel.name` kommt aus dem Feld-Namen (`src/archivar/channels.rs:412`); gleicher Name → Metas-Kollision (letzter gewinnt). Der Wert-Ring mischt beide Stationen. Der Riss steht (river nannte es „funktional, aber namens-blind").
- **Blockade:** die Trennung berührt Register **und** Consumer; eine Seite allein ändert die Kontrakt-Bedeutung.
- **Braucht:** Entscheidung — Feld-Schlüssel station-spezifisch (`abk_dbdt`/`sod_dbdt`) **oder** `metas` per `(name, station)` schlüsseln; die Konsumentenseite trägt river (siehe `## An river`).

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ / Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Absolutpfade auf `~`-relativ gesetzt.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die Pending-Einträge sind im `dead_sources.φ` disponiert.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende Messreihe; jede Zelle mit `file:line`/`pending`.
- `docs/paper/eclipse-clock-worldlines.md` — `:45` re-manifest-verifiziert (2026-10-02), Header-sha aktualisiert.
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` — Anderson-Flyby-Klasse-Abschnitt (MESSENGER-Riss als Granulat-Naht), Header-sha aktualisiert.

## An mycelium

Origin: mountain folge223.

- **`ephemeris-house-gate.yml` Epoch-Input:** der Workflow läuft nur den JUICE-Default; für die 2024-04-08-Haus-Querprobe auf den re-manifestierten Bins braucht es `--epoch-ymd` als `workflow_dispatch`-Input oder einen zweiten Step.
- **CEERS-Tag-Mismatch:** `curated48_spectra.bin` tagt `ssd.jpl.nasa.gov`, ihr `jwst_spectra_compiler` lädt nach `exoplanetarchive.ipac.caltech.edu` — beim `ceers-cdn`-Workflow abgleichen.
- **CDN-Workflows fehlen:** `ascat` (netloc `manati.star.nesdis.noaa.gov-ascat`), `jades` (`jades.herts.ac.uk`), `dsn` (`eyes.nasa.gov`). Die `url`/`format`/`compiler`/`origin`-Zeilen stehen in `phi/sources.φ`.
- **Riss Route:** `erddap.aoml.noaa.gov` (ASCAT) ist 2026-10-02 nicht erreichbar; arbeitsfähig ist `manati.star.nesdis.noaa.gov`.
- **`anderson-residuals`-Registerblock:** liegt inzwischen committed (HEAD @`bbf46e095`).

## An river

Origin: mountain folge223.

- **`intermagnet_dbdt`-Doppelname (gemessen 2026-10-02):** ABK (`phi/sources.φ:1812`) und SOD (`:1821`) tragen denselben Feld-Schlüssel; `Matrix::metas` (`src/mathematikerin/machines/matrix.rs:679`) schlüsselt nach `channel.name` (`src/archivar/channels.rs:412`), sodass die zweite Station die erste im Metas-Satz überschreibt und der Wert-Ring beide mischt. Mein Verdikt: entweder der Feld-Schlüssel wird station-spezifisch, oder `metas` wird per `(name, station)` geschlüsselt. Die Consumer-Feder liegt bei dir; ich setze die Register-Seite nach deiner Entscheidung.

## An future

Origin: mountain folge223.

- **CSES `/query.php`** ist ein SSDC-CAS-Login (`<title>Login</title>`) → Wall, kein Datenkanal ohne Konto. Falls ein Konto beantragt werden soll, ist das ein per-Akt-Operator-Wort.
- **DEMETER Order-API** `regards.cnes.fr/api/v1/rs-order` = 403 (blocked account); Order 18400 läuft, Ablauf `2026-10-05` (`state/zustand/wartend.φ:4`). Der offene SPASE-Pfad ist als `origin` in `phi/blocked_sources.φ:100` registriert; die Antwort-Frist ist ein Future-Wiedervorlage-Punkt.
