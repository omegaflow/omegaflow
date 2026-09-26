<!--
  title: Handover — Sensory-Folge 179 (Stand 2026-09-26)
  session: Sensory-Folge 179
  class: handover
  date: 2026-09-26
  sha256: 8dcf73fada6bff5292ba02d4de6e1843b9211fed82933b6cef34e91b499c5842
  status: live
-->
# Handover — Sensory-Folge 179 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur
die eigenen Hunks — committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr
von HEAD ist.

Es gibt keinen `härtesten Punkt` — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit Messstempel) / **Blockade** /
**Braucht**. Sortierung: **umsetzbar zuerst**; der Akteur steht pro Punkt in `Bindung`.

Die [redacted] ist das persönliche Gerät des Operators; ihre Kennung (MAC) und ihre Daten
bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN.

**Audit-Regel (2026-09-26):** vor jedem `operator-gebunden`-Tag wird `.secrets.local`
auf den nötigen Key geprüft — `[redacted]_MAC` und `OMEGAFLOW_CA_BUNDLE` standen bereits
dort, während die Punkte als „wartet auf Operator" gebucht waren.

## Stehender Pass (gemessen 2026-09-26, Session-Beginn)

- **HEAD** `352a56d556ea8a298ddcf59a7386ea39e42e3174`, `origin/main == HEAD`.
- **Postfach** — `state/mail/mail_ledger.φ` absent → `pending` (Aufbau gehört CI);
  CSES-Limadou-Antwort (Sotgiu „wait a few weeks") ist Wiedervorlage; DSN-Briefe in Flug.
- **CI** — `meteo-cdn 36259731023` success, `charm2-cdn 36259779508` success,
  `allwise-cdn 36263500650` success, `aia-cdn 36268729035` success (Asset
  `aia2014_lines.bin` bereits im Release → `manifest skipped`); `ps1-cdn 36267466693`
  in_progress; `modis-cdn`/`eht-uvfits-cdn` rot (fremde Linien); `ci-check 36271257595`
  pending. `volume-cdn`/`isc-ehb-cdn`/`emc-cdn`/`cometels-cdn` liegen außerhalb des
  `ci_manage list`-Fensters (per_page=100, `ci_manage.rs:72`) → Status **ungemessen**, nicht null.

## Korrektur am Voratom (folge178 war stale)

folge178 trug fünf Punkte als „Braucht: fix committen", deren Fixes bereits im Commit
`d766ed9a5` (Vorfahre von HEAD) lagen. Gemessen 2026-09-26 (`git log -L`, `sed`):
- `src/mathematikerin/te.rs:4306` trägt bereits `true_total.div_ceil(2)` — der
  `manual_div_ceil`-clippy-Fund ist gegen HEAD stale.
- `slab2_compiler.rs:556-564` `create_dir_all` vor Download, `lsst_color_coupling_probe.rs:46`
  `-c.u=deg`, `naif_body_ids.tsv:82-90` (9 TNO-Zeilen), `kernel-flatten.yml:105`
  (`--bodies` mit 9 TNO-Ids), `lsst-live-scan.yml:33` (2. Kontroll-Cone
  `150.22889,1.39466,260,24`) — alle im HEAD vorhanden; `cargo check` (core),
  `-p omegaflow-harvest --bin slab2_compiler`, `-p omegaflow-measure --bin {lsst_color_coupling_probe,cross_te_screen}`
  je 0 Fehler / 0 Warnungen.
Der offene Schritt dieser Punkte ist damit allein der **Workflow-Lauf**, nicht der Fix.

## Sofort abarbeitbar (eigen, dispatchbar)

### 1. Katalog-Dispatches — in diesem Atom gestartet
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) alle fünf `workflow_dispatch` ohne Pflicht-Inputs; die
  Fixes liegen im gepushten HEAD.
- **Braucht:** Ergebnis via `ci_manage view <id>` lesen, Träger nachziehen:
  - `slab2-cdn` (Positive Maske, `create_dir_all`-Fix) → `36272877915`,
  - `lsst-live-scan` (Nadel V, `-c.u=deg`-Fix) → `36272880382`,
  - `kernel-flatten` (Weberin TNO, 9 `naif_body_ids`) → `36272882020`,
  - `aia-ladder-probe` (Korona, voller 613-Event-Korpus) → `36272883282`,
  - `corona-conditional-probe` (Solar-Seconds 211A→193A) → `36272884808`.

### 2. Funken — beide Proben gebaut, kein CI-Workflow
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `broker_difference_probe.rs` (965 Zeilen, 15 Tests,
  Funke 3) und `tdb_coincidence_probe.rs` (756 Zeilen, 7 Tests, Funke 5; TDB über
  `naif0012.tls`, Rømer via Ephemeride) existieren und kompilieren 0/0; `sgrep` über
  `.github/workflows/` → **kein Workflow** für beide.
- **Blockade:** keine.
- **Braucht:** einen Funken-Workflow bauen (oder die Bins in einen bestehenden Lauf
  hängen), dann dispatchen.

### 3. Galileo CK-Kerne `_rtr` — erreichbar, Harvest offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26, research-max) die 8 CK-Kerne `ck90341a`–`ck90344b`
  liegen **nicht** unter `.../ck/ck9034*.bc` (404), sondern unter
  `https://naif.jpl.nasa.gov/pub/naif/GLL/kernels/ck/prime_mission/unvalidated/rtr/`
  (`ck9034*_rtr.bc`, HTTP 200; `ck90341a_rtr.bc` 3863552 B sha256
  `2bf5551a…a84170`, `ck90344b_rtr.bc` 3598336 B sha256 `2ebd138a…a64a27`). Frame −77000
  inhaltlich ungeprüft (kein SPICE-Reader im Tool-Set).
- **Blockade:** keine.
- **Braucht:** die 8 `_rtr.bc` per `sfetch`/Compiler harvesten; `galileo-rotor-spin-era-floor.md:1`
  nachziehen.

### 4. Seismik-Flotte — Paper-Pendings sind eigene Schritte
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `volume-cdn`/`isc-ehb-cdn`/`emc-cdn` außerhalb des
  Fensters (Status ungemessen); Arme LLNL-G3D-JPS/ISC-EHB/EMC gebaut.
- **Blockade:** keine.
- **Braucht:** die drei Träger-Pendings als getrennte Schritte abarbeiten —
  `depth-phase-echo-fleet.md:35-38` Dual-Phase-Fit, `die-akteure-im-boden-und-wasser.md:85`
  Gravimeter-SFTP, `axiom-gate-depth-phase-echo-fleet.md:42` CMT-Term/Kalibrier-Gate;
  `ci_manage list` für die CDN-Läufe nachziehen.

### 5. Blatt-1-Bojen-Matrix — Matrix-Rotor offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) Träger `docs/concepts/blatt-papier-resultat.md:63-71`.
- **Braucht:** Bojen-Matrix-Rotor laufen lassen, Matrix-Zeile Σ p̂·M nachtragen.

### 6. causal-arrow — `te_pair_probe` ohne Workflow
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `te_pair_probe` hat **keinen** CI-Workflow (null Treffer
  in `.github/`); Lag-Sweep {1,3,6,12,24,48} gegen Rasuwa-Regen; DAHITI Koshi via `sfetch`
  (api_key).
- **Braucht:** `te_pair_probe` + `kbo_residue_probe` in CI verankern, dann dispatchen;
  Träger `causal-arrow-preregistration.md:1`, `ein-blatt-papier.md:2`.

### 7. Kreuz-Screening — `meteo-cdn` grün, Kopplung offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `meteo-cdn` success; `is_day`/Selbstpaar-Fix in HEAD;
  Träger `cross-screening-tibet.md:42/:44`, `blatt-kreuz-screening-gyirong.md:71/:201`
  räumliche Kopplung Rasuwa→Gyirong bleiben offen.
- **Braucht:** Kopplung messen (`cross_te_screen`), Träger nachziehen.

### 8. Sieben Sphären — Sphäre I closed, Δz gemessen `absent`
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `charm2-cdn 36259779508` success; Sphäre I
  (Stern-Winkeldurchmesser-Feld) closed. Δz je Okkultation: **absent** in ADS/OpenAlex/
  Crossref; gemessener Kandidat PDS `EAR-A-3-RDR-OCCULTATIONS-V12.0`
  (`https://sbn.psi.edu/pds/resource/occ.html`, 9729 Okkultationen bis 2023-06; Δz-Spalte
  ungeprüft).
- **Braucht:** Δz-Spalte in V12.0 prüfen; sonst `absent` in `the-seven-spheres.md:148` halten.

### 9. Trishuli — S1-Footprint (operator-nahe Session)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) 6 post-event S1D-IW-GRDH-Frames (2026-08-28…2026-09-24),
  Bahrabise im Footprint; Download auth-gated (Earthdata-OAuth, `EARTHDATA_EDL_TOKEN` da).
- **Blockade:** Earthdata-OAuth-Session (kein Key-Gap).
- **Braucht:** Szene über die Session laden, Footprint/Alignment messen.

## Operator-gebunden (Vorbereitung an der Kante)

- **RR-Kanal / Beat-to-beat** | operator | Trigger: Förderung gewährt (2026-09-26) | Lage: Live-BLE HR aber `NotSupported` → keine RR; FIT `nn=0` bei 25 Aktivitäten | Braucht: Brustgurt (Polar H10 / HRM-Dual) → `perm_tone_probe`.
- **DEMETER/CDPP-Order-Flow** | operator | Trigger: Operator-Browser exportiert Order-JSON | **Ablauf 2026-09-28** | Braucht: `regards_order_read <order.json>`.
- **Weberin-Quellen HAWC** | operator | Trigger: `OMEGAFLOW_CA_BUNDLE` gesetzt | Braucht: HAWC-Fetch mit lokalem 4-Zert-Bundle prüfen; CI-Secret separat.
- **Weberin-Quellen vDEC** | operator | Trigger: Operator-Wort | Lage: IGETS/ONC/TNS per Credential gelöst; vDEC 403 direct+Proton, Wayback 200 | Braucht: vDEC-Antrag (Webform + Projekttext, Entwurf `state/mail/weberin-quellen-konten-2026-09-26.md`).
- **Onboard-/CIQ-Bedarf** | operator | Trigger: Operator-Wort | Braucht: Bedarf Ja/Nein; Nein → released.
- **Beat-Arbitrierung** | operator | Trigger: zweiter Beat-Kanal (der Gurt) | Braucht: `OMEGAFLOW_HIDDEN=1`-Lauf → genau eine `beat source:`-Zeile.
- **Live-Sensor-Cluster** | LOCK | Trigger: Operator-Wort hebt LOCK | Braucht: BOM bestellen (inkl. H2).
- **ESP32-Puls-Knoten** | LOCK | Trigger: Operator-Wort hebt LOCK | Wort: ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session).
- **JUICE-Erdpassage 28./29.09.2026** | termin:2026-09-29 | Trigger: 28./29.09. | **Riss:** Kp-Route in `61e272ab0` aus `sources.φ` entfernt → Kp-Zelle kann nicht füllen | Braucht: Siegel-Wort vor dem 28.09.; danach In-situ-Messung.

## Blockiert / wartend (mit Trigger)

- **Commit ohne Operator-Wort `c6edfbe45`** | eigen | Trigger: sofort | Riss, im Abschluss-Check getragen.
- **M2c — Rust-ZNSP-Host BL808/H2** | wartend | Trigger: Ox64 angekommen (`LZ473049629CN`) | Blockade: Carrier.
- **BL808-eigenes 802.15.4-Radio** | wartend | Trigger: öffentlicher BL808-Stack | Lage (gemessen 2026-09-26, research-max): Zephyr PR #112921 deckt BL616/618, BL702/704/706, BL702L/704L — **BL808 nicht**; `bouffalo_sdk_bl808` ohne `lmac154` | Braucht: PR-Suche beobachten.
- **Galileo Borduhr-Sprung A/B** | wartend | Trigger: öffentliche Absolutfrequenz-Reduktion über 1995-11-30/12-01 | Lage (gemessen 2026-09-26): NTRS/ADS `absent` (Fundstücke 1993/1994 datieren vor dem Sprung); arXiv-Route gepcapped (HTTP 406, OAI-PMH-Bulk als Alternative) | Braucht: OAI-PMH-Route oder andere Reduktion.
- **JWST Biosignatur-Kanäle** | wartend | Trigger: JWST-Detektion+Spektrum (`10.1073/pnas.2416188122`) | Lage: nur Modelle/Feasibility.
- **Das eine Instrument — Anomalie** | blockiert | Trigger: zweiter Messkanal (VLBI-Beacon) | Blockade: kein Instrument misst den vollen Pionier-Phasenraum.
- **LAIC/Causal-Arrow (CSES)** | wartend | Trigger: CSES-FTP-Zugang | Lage (gemessen 2026-09-26, research-max): Live-Host TLS-broken (`NET::ERR_CERT_AUTHORITY_INVALID`), Wayback 200 (Snapshot 2025-10-14), Datenservice per FTP-Registrierung | Braucht: Registrierung; `laic-arrow-direction.md:3` nachziehen.

## Extern (Dritter)

- **Postfach** | wartend | Trigger: neuer Eingang | **Blockade:** Ledger absent (Aufbau gehört CI) | Braucht: `smail_recv`/Ledger bei Trigger.
- **Ox64-Lieferung** | termin (Carrier) | Trigger: Ankunft (`LZ473049629CN`) | Braucht: quittieren → M2c.
- **BGR-Matched-Filter (Tonga)** | dritter (vDEC) | Trigger: vDEC-Zugang | Braucht: Arrival messen.
- **DSN-Briefe in Flug** | dritter | Trigger: DSN-Antwort | Braucht: quittieren.
- **Gaia DR4 + Europa-Clipper** | termin:2026-12-02 | Trigger: 02./03.12. | Braucht: Epoche ernten.

## Träger-Zeilen (Orphan-Faltung, Rest 2026-09-26)

Der Dateiname in dieser Übergabe ist der Träger. Je Zeile ein zuletzt trägerloses
Dokument: `Pfad` (offene Marker) → Trägerpunkt oder descoped-Befund.

- `docs/blatt/blatt-der-grat.md` (2) → ENSO-Pfeil: `cross_te_screen` (Punkt 1/Kreuz-Screening).
- `docs/blatt/blatt-kreuz-screening-gyirong.md` (3) → Punkt 7 Kreuz-Screening.
- `docs/blatt/blatt-solar-seconds-matrix.md` (1), `docs/paper/solar-seconds-matrix.md` (3) → Punkt 1 `corona-conditional-probe`.
- `docs/blatt/blatt-thuan-fragesteller.md` (4) → `termin:2026-12-02` (Gaia DR4).
- `docs/concepts/arxiv-api.md` (2) → Mountain-Linie.
- `docs/concepts/blatt-papier-resultat.md` (1) → Punkt 5 Blatt-1-Bojen-Matrix.
- `docs/concepts/das-eine-instrument.md` (2) → Punkt „Das eine Instrument".
- `docs/concepts/der-paradigmenwechsel.md` (9) → Punkt JUICE.
- `docs/concepts/die-akteure-im-boden-und-wasser.md` (7) → Punkt 4 Seismik-Flotte.
- `docs/concepts/fuenf-funken-anomalie-suche.md` (4) → Punkt 2 Funken.
- `docs/concepts/recherche-galileo-kadenz-reconciliation.md` (1) → Galileo NTRS 19930010224 = TDA PR 42-110, keine Kadenz-Aussage (gemessen); PDF 16 MB erreichbar.
- `docs/concepts/the-seven-spheres.md` (2) → Punkt 8 Sieben Sphären.
- `docs/paper/causal-arrow-preregistration.md` (1), `docs/concepts/ein-blatt-papier.md` (2) → Punkt 6 causal-arrow.
- `docs/paper/corona-heating-ladder.md` (2), `docs/surveys/survey-ein-blatt-korona-heizung.md` (1) → Punkt 1 aia-ladder.
- `docs/paper/cross-screening-tibet.md` (1) → Punkt 7.
- `docs/paper/depth-phase-echo-fleet.md` (5), `docs/surveys/axiom-gate-depth-phase-echo-fleet.md` (1) → Punkt 4.
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4), `docs/paper/flyby-path-2-preregistration.md` (1) → JUICE.
- `docs/paper/galileo-rotor-spin-era-floor.md` (1) → Punkt 3 CK-Kerne.
- `docs/paper/jwst-disequilibrium-survey.md` (7) → JWST.
- `docs/paper/laic-arrow-direction.md` (3) → LAIC/CSES.
- `docs/paper/nadel-v-fresh-area-dip-scan.md` (1) → Punkt 1 lsst-live-scan.
- `docs/paper/probe-front-dark-matter.md` (2) → Pioneer/Dark-Matter **gemessen geschlossen**.
- `docs/paper/sturzflut-tibet-pfeil.md` (23) → Punkt 9 Trishuli.
- `docs/paper/tonga-lamb-crosscheck.md` (3) → BGR-Matched-Filter.
- `docs/surveys/axiom-gate-broken-null-control.md` (1) → geschlossen (te-gate 36228804363 success).
- `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` (17) → Pioneer/Dark-Matter (closed) + DSN-Briefe.
- `docs/surveys/survey-2026-09-16-sonden-flotte.md` (4) → DSN-Briefe.
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (9) → HRV/Puls→Strahlung (wartend).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` (2) → River-/Browser-Linie.
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (7) → FIT-Verifikation [redacted] (operator) + BLE-HR-Live.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` (1) → Postfach.
- `docs/concepts/archivar-mathematikerin.md` (1) → Archivar-Pending (Mountain-Linie).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (2) → River-Linie.
- **descoped (gemessen 2026-09-26):** `docs/concepts/exzellenz-konzept.md`, `docs/concepts/kybernetische-astrophysik.md`, `docs/concepts/zeugnis.md`, `docs/paper/asmar-2005-spacecraft-doppler-tracking-noise-budget.md`, `docs/paper/planet-nine-kbo-residue.md`, `docs/paper/terminologie-der-gegenstroemung.md`, `docs/concepts/blatt-papier-beweis.md`, `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`, `docs/surveys/survey-messpunkt-verteilung.md`, `docs/concepts/glossar.md`, `docs/concepts/pfeiler-der-architektur.md`, `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md`.
- `docs/surveys/survey-fortschritt.md` (1) → Träger Mycelium.

## Fremd im geteilten Baum (nicht anfassen)

- `src/archivar/spatial.rs` änderte `build_star_samples` auf 2 Argumente
  (`catalog_epoch_yr: Option<f64>`); die fremden Test-Bins `membrane_hull_probe.rs:245`
  und `star_dmax_probe.rs:155` rufen noch mit 1 Argument → kompilieren unter
  `cargo check --tests` nicht (fremde Linien, gemessen 2026-09-26).
- Offen im Baum (nicht eigene): `src/archivar/json.rs`, `main_flow.rs`, `skydirection.rs`, `tests.rs`.

## Operator-Wort-Register (Stand 2026-09-26)

- **Wort:** „ändere die agents — das Handover nach umsetzbar/nicht umsetzbar sortieren" | 2026-09-26 | Operator (Session) → ausgeführt: AGENTS.md `3672968d3`.
- **Wort:** „alles Offene bis zur Kante abarbeiten, gemessen abschließen — nicht verschleppen" | 2026-09-26 | Operator (Session).
- **Wort:** „die Compiler-Arme durch Agenten bauen lassen" | 2026-09-26 | Operator (Session) → ausgeführt: LLNL-G3D-JPS, ISC-EHB, Slab2, EMC, cometels.
- **Wort:** „alles Offene und Benannt-Ungemessene wird übernommen" | 2026-09-26 | Operator (Session).
- **Wort:** „jedes Operator-Wort steht im Handover; keine Session kaut es neu durch" | 2026-09-26 | Operator (Session).
- **Wort:** „deine Daten/Datei verlassen das Gerät nie" | 2026-09-25 | Operator (Session) — [redacted]/DEMETER.
- **Wort:** „beides" (BLE-Live + FIT-Probe) | 2026-09-26 | Operator (Session) → ausgeführt.
- **Wort:** „das mache ich erst, wenn ich gefördert werde" (Gurt-Beschaffung) | 2026-09-26 | Operator (Session).
- **Wort:** „DEMETER ist Sensory" | 2026-09-27 | Operator (Session) → DEMETER gehört zur Sensory-Linie (hier bereits geführt); GOSAT zurück an die Mycelium-Linie.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
`docs/handover/handover-2026-09-26-sensory-folge179.md`,
`docs/handover/archiv/handover-2026-09-26-sensory-folge178.md` (Move).
