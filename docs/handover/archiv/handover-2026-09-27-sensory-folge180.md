<!--
  title: Handover — Sensory-Folge 180 (Stand 2026-09-27)
  session: Sensory-Folge 180
  class: handover
  date: 2026-09-27
  sha256: 46247d493f79226cadbb8cf13ba2b764c0bafe10f60e3153ab0f6cb53272a8f2
  status: live
-->
# Handover — Sensory-Folge 180 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur
die eigenen Hunks — committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr
von HEAD ist.

Es gibt keinen `härtesten Punkt` — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit Messstempel) / **Blockade** /
**Braucht**. Sortierung: **umsetzbar zuerst**; der Akteur steht pro Punkt in `Bindung`.

Die [redacted] und alle Operator-Daten bleiben lokal (`.secrets.local`, `data/`), nie
getrackt, nie am CDN.

## Stehender Pass (gemessen 2026-09-27, Session-Beginn)

- **HEAD** `517d587e9`, `origin/main == HEAD`.
- **Postfach** — `state/mail/mail_ledger.φ` absent → `pending` (Aufbau gehört CI).
- **CI (die fünf Katalog-Dispatches aus folge179):** `slab2-cdn 36272877915`
  **success**, `lsst-live-scan 36272880382` **success**, `aia-ladder-probe
  36272883282` **success**, `corona-conditional-probe 36272884808` **success**,
  `kernel-flatten 36272882020` **in_progress** (gemessen 2026-09-27 via
  `ci_manage list`).
- **Geteilter Baum:** eine fremde Linie (Mountain/Mycelium) arbeitet **uncommittet**
  an `phi/sources.φ`, `phi/dead_sources.φ`, `src/archivar/main_flow.rs` und weiteren
  Quelldateien sowie Handover-Dateien → nicht anfassen (siehe O7).

## Abarbeitbar (eigen, dispatchbar)

### 1. Funken-Workflow — gebaut, Dispatch offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27) `.github/workflows/funken-probe.yml` neu angelegt
  (63 Z.); beide Bins liegen in `tools/measure` (`omegaflow-measure`):
  `broker_difference_probe` (Funke 3), `tdb_coincidence_probe` (Funke 5); Workflow
  dispatcht beide.
- **Blockade:** keine.
- **Lage (Nachtrag):** dispatched 2026-09-27 → Run `36276735080` (queued).
- **Braucht:** Ergebnis via `ci_manage view 36276735080` einmalig lesen (nächster Pass).

### 2. causal-arrow-Scan — Workflow gebaut, Dispatch offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27) `.github/workflows/causal-arrow-scan.yml` neu
  angelegt (67 Z.); `te_pair_probe` (Lag-Sweep {1,3,6,12,24,48}, Gyirong→Rasuwa) +
  `kbo_residue_probe` (--check-bins/--cluster-only/Sweep), beide `tools/measure`.
- **Blockade:** keine.
- **Lage (Nachtrag):** dispatched 2026-09-27 → Run `36276736870` (in_progress).
- **Braucht:** Ergebnis via `ci_manage view 36276736870` lesen; Träger
  `causal-arrow-preregistration.md:1`, `ein-blatt-papier.md:2`.

### 3. O1 DEMETER/CDPP — Order nutzbar, Ernte offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort, **Frist 2026-09-28**
- **Lage:** (gemessen 2026-09-27, Browser-Session `johannes.tyroller@proton.me`) Order
  **18387** ist **DONE_WITH_WARNING**, `filesInErrorCount 96978`, `availableFilesCount 0`
  — **aber** das metalink `…/orders/18387/metalink/download` ist **befüllt**: echte
  `DMT_N1_1144_*.DAT`-Dateien mit Download-URLs (`…/orders/public/files/<id>?orderToken=…`)
  und Größen. Der `availableFilesCount`-Wert ist irreführend, die Order ist abholbar.
  Datensätze `DMT_N1_1143` (39318) + `DMT_N1_1144` (57760).
- **Blockade:** der metalink-Abruf ist groß (Stream-Timeout) und braucht die
  SPA-Token-URL; `regards_order_read` ist nicht auf PATH (Bin noch nicht gebaut).
- **Braucht:** metalink als Datei sichern (SPA-Download-Link), `cargo build -p
  omegaflow-measure --bin regards_order_read` (bzw. harvest-Crate), dann
  `regards_order_read <metalink>` → Download-URLs → Ernte lokal nach `data/` **vor dem
  28.09.**. Kein CDN-Download (Order-Links sind token-gebunden, kurzlebig).

### 4. #3 Galileo CK-Kerne `_rtr` — Harvest offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26, research-max) 8 CK-Kerne `ck90341a`–`ck90344b` unter
  `https://naif.jpl.nasa.gov/pub/naif/GLL/kernels/ck/prime_mission/unvalidated/rtr/`
  (`ck9034*_rtr.bc`, HTTP 200).
- **Blockade:** keine.
- **Braucht:** die 8 `_rtr.bc` per `sfetch`/Compiler harvesten; `galileo-rotor-spin-era-floor.md:1`
  nachziehen.

### 5. #4 Seismik-Flotte — Paper-Pendings + CDN-Läufe
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `volume-cdn`/`isc-ehb-cdn`/`emc-cdn` außerhalb des
  `ci_manage list`-Fensters (Status ungemessen, nicht null); Arme LLNL-G3D-JPS/ISC-EHB/EMC gebaut.
- **Blockade:** keine.
- **Braucht:** `depth-phase-echo-fleet.md:35-38` Dual-Phase-Fit,
  `die-akteure-im-boden-und-wasser.md:85` Gravimeter-SFTP,
  `axiom-gate-depth-phase-echo-fleet.md:42` CMT/Kalibrier-Gate; `ci_manage list`
  für die drei CDN-Läufe.

### 6. #5 Blatt-1-Bojen-Matrix — Rotor offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) Träger `docs/concepts/blatt-papier-resultat.md:63-71`.
- **Blockade:** keine.
- **Braucht:** Bojen-Matrix-Rotor laufen lassen, Σ p̂·M-Zeile nachtragen.

### 7. #7 Kreuz-Screening — räumliche Kopplung offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `meteo-cdn` success; `is_day`/Selbstpaar-Fix in HEAD;
  Träger `cross-screening-tibet.md:42/:44`, `blatt-kreuz-screening-gyirong.md:71/:201`.
- **Blockade:** keine.
- **Braucht:** Kopplung mit `cross_te_screen` messen (Rasuwa→Gyirong), Träger nachziehen.

### 8. #9 Trishuli — S1-Footprint
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) 6 post-event S1D-IW-GRDH-Frames, Bahrabise im
  Footprint; Download auth-gated (`EARTHDATA_EDL_TOKEN` da).
- **Blockade:** Earthdata-OAuth-Session (kein Key-Gap).
- **Braucht:** Szene über die Session laden, Footprint/Alignment messen.

## Befund-Träger (gemessene Abschlüsse → Träger nachziehen)

### B1 Galileo Borduhr — gemessen `absent`
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27, research-max) keine veröffentlichte **absolute**
  Frequenz-Reduktion über 1995-11-30/12-01 auffindbar; arXiv-API 406, OAI-PMH
  erreicht die Ära nicht (`start date too early`), ADS/NTRS nur Vor-Sprung-Stücke
  (1993). Der SPICE-SCLK-Kernel selbst trägt an dem Datum keine Frequenzänderung.
- **Blockade:** keine.
- **Braucht:** Befund in `galileo-rotor-spin-era-floor.md` eintragen (absent, mit
  Route-Beleg); Punkt dann geschlossen.

### B2 BL808 802.15.4 — gemessen `absent`
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27, flash) kein öffentlicher 802.15.4-Stack für BL808;
  Bouffalo Issue #148: „All wireless doc in NDA"; `bouffalo_sdk_bl808` README:
  ZIGBEE ×; Zephyr-PR #112921 ohne BL808.
- **Blockade:** keine.
- **Braucht:** Befund als `descoped`/`absent` in `fuenf-funken-anomalie-suche.md` o.ä.
  eintragen; Punkt geschlossen.

### B3 LAIC/CSES — Zugangsweg gemessen, Berechtigung offen
- **Status:** wartend | **Bindung:** eigen | **Trigger:** 2026-10-02 (Re-Messung)
- **Lage:** (gemessen 2026-09-27, research-max) China-LEOS-Portal TLS self-signed +
  chinesische Mobilnummer (blockiert); **ASI SSDC (Italien)** ist der begehbare Weg:
  Registrierung `tools.ssdc.asi.it/UserManager/requestUser.jsp` (kein Telefonfeld,
  frei), CAS-Login, Abfrage `limadou.ssdc.asi.it/query.php`; Hürde: Berechtigung
  fehlt (Mail an `alessandro.sotgiu@roma2.infn.it`), PI-Antwort „wait a few weeks".
  `SSDC_USER`/`SSDC_PASS` liegen in `.secrets.local`.
- **Blockade:** Berechtigung (PI-Verfahren nach CSES-02-Umstellung).
- **Braucht:** Re-Messung 2026-10-02; `laic-arrow-direction.md:3` nachziehen.

### B5 JWST Biosignatur — Detektionen gemessen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27, flash) der Auslöser-DOI `10.1073/pnas.2416188122`
  (Seager et al., PNAS 2025) ist eine **Perspektive/Modell**, keine Detektion.
  Echte JWST-Detektionen: WASP-39b CO2 `10.1038/s41586-022-05269-w` + photochem. SO2
  `10.1038/s41586-023-05902-2`; K2-18b CH4 5σ/CO2 3σ `10.3847/2041-8213/acf577`.
  Biosignatur-Gas (DMS) **absent**: tentative „potential signs" → `insufficient
  evidence` (`10.1051/0004-6361/202555580`, `10.3847/2041-8213/adc1c8`).
- **Blockade:** keine.
- **Braucht:** `jwst-disequilibrium-survey.md` auf Detektion vs. Modell nachziehen.

### #8 Sieben Sphären Δz — gemessen `absent`
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27, flash) PDS `EAR-A-3-RDR-OCCULTATIONS-V12.0`,
  `data/occlist.lbl` = 58 Spalten — **keine** Δz/vertical/altitude-Spalte; nur
  X/Y-Koordinaten. Auch der Nachfolger `smallbodiesoccultations 4.0` ohne Δz.
- **Blockade:** keine.
- **Braucht:** `the-seven-spheres.md:148` mit dem Befund nachziehen; Punkt geschlossen.

## Operator-gebunden (Vorbereitung an der Kante)

- **DEMETER/CDPP Order-Export** | operator | Trigger: sofort, Frist 2026-09-28 | Lage: metalink-Link in der SPA (geprüft, befüllt) | Braucht: SPA-Login → metalink herunterladen (oder der Maschine die Session zugänglich machen).
- **RR-Kanal / Beat-to-beat** | operator | Trigger: Förderung gewährt (2026-09-26) | Lage: BLE-HR ohne RR; FIT `nn=0` | Braucht: Brustgurt (Polar H10/HRM-Dual) → `perm_tone_probe`.
- **Weberin-Quellen HAWC** | operator | Trigger: `OMEGAFLOW_CA_BUNDLE` gesetzt (liegt) | Braucht: HAWC-Fetch mit lokalem 4-Zert-Bundle; CI-Secret separat.
- **Beat-Arbitrierung** | operator | Trigger: zweiter Beat-Kanal (Gurt) | Braucht: `OMEGAFLOW_HIDDEN=1`-Lauf → genau eine `beat source:`-Zeile.
- **Live-Sensor-Cluster** | LOCK | Trigger: Operator-Wort hebt LOCK | Braucht: BOM bestellen (inkl. H2).
- **ESP32-Puls-Knoten** | LOCK | Trigger: Operator-Wort hebt LOCK | Wort: ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session).
- **JUICE-Erdpassage 28./29.09.2026** | termin:2026-09-29 | Trigger: 28./29.09. | Lage: **Riss** — Kp-Route in `61e272ab0` aus `sources.φ` entfernt | Wort: „natürlich" (Route wieder aufnehmen) | 2026-09-26 | Operator (Session) | Braucht: **O7 unten**.

## Abgeschlossen / gemessen geschlossen (Befund)

- **O5 Weberin vDEC** — `descoped`. Befund: vDEC öffnet „to organizations rather than
  individual researchers", Vertrag braucht die „legally responsible person within your
  organisation", Webform verlangt Organisationsanschrift; Entwurf „Independent
  researcher" → für eine Einzelperson nicht zugänglich (gemessen 2026-09-26,
  ctbto.org vDEC-Policy + Webform). Rohwellenform bleibt `absent`. Wort: „nein dann
  passt es nicht" | 2026-09-26 | Operator (Session). Träger
  `state/mail/weberin-quellen-konten-2026-09-26.md`.
- **O6 Onboard-/CIQ-Bedarf** — `descoped`. Befund: Operator-Wort „kein onboard ciq"
  | 2026-09-26 | Operator (Session): CIQ bräuchte Monkey C + Garmin-SDK/Dev-Key +
  ANT-Hardware; BLE-Weg + eigener Sensor-Knoten decken die Kanäle. Nicht nötig. Träger
  `survey-2026-09-23-geraete-anbindung-radiatoren.md`.

## Wartend / blockiert (mit Trigger)

- **O7 Kp-Route wiederherstellen** | eigen | Trigger: sofort | Lage (gemessen
  2026-09-27): `phi/sources.φ` trägt **fremde uncommittete** Änderungen (Mountain/
  Mycelium) → mein Hunk darf nicht mit fremdem gestaged werden | Blockade: geteilter
  Baum | Braucht: nach fremdem Commit die vier Kp-Zeilen wieder aufnehmen
  (`last Kp magnetosphere_kp_index`, `…_a_running`, `url https://kp.gfz.de/app/json/?…index=Kp`,
  `last Kp magnetosphere_kp_3h`) — Referenz `61e272ab0^:phi/sources.φ`.
- **M2c — Rust-ZNSP-Host BL808/H2** | wartend | Trigger: Ox64 angekommen (`LZ473049629CN`) | Blockade: Carrier.
- **BL808-eigenes 802.15.4-Radio** | wartend | Trigger: öffentlicher BL808-Stack | Lage (B2): NDA-geschlossen | Braucht: PR-Suche beobachten.
- **Das eine Instrument — Anomalie** | blockiert | Trigger: zweiter Messkanal (VLBI-Beacon) | Blockade: kein Instrument misst den vollen Pionier-Phasenraum.
- **LAIC/Causal-Arrow (CSES)** | wartend | Trigger: 2026-10-02 | s. B3.

## Extern (Dritter)

- **Postfach** | wartend | Trigger: neuer Eingang | Blockade: Ledger absent (CI-Duty) | Braucht: `smail_recv`/Ledger bei Trigger.
- **Ox64-Lieferung** | termin (Carrier) | Trigger: Ankunft (`LZ473049629CN`) | Braucht: quittieren → M2c.
- **BGR-Matched-Filter (Tonga)** | dritter (vDEC) | Trigger: vDEC-Zugang | Lage: vDEC descoped → BGR-Weg neu bewerten | Braucht: Arrival messen.
- **DSN-Briefe in Flug** | dritter | Trigger: DSN-Antwort | Braucht: quittieren.
- **Gaia DR4 + Europa-Clipper** | termin:2026-12-02 | Trigger: 02./03.12. | Braucht: Epoche ernten.

## Träger-Zeilen (Orphan-Faltung)

Der Dateiname in dieser Übergabe ist der Träger. Je Zeile ein zuletzt trägerloses
Dokument: `Pfad` (offene Marker) → Trägerpunkt oder descoped-Befund.

- `docs/blatt/blatt-der-grat.md` (2) → #7 Kreuz-Screening.
- `docs/blatt/blatt-kreuz-screening-gyirong.md` (3) → #7.
- `docs/blatt/blatt-solar-seconds-matrix.md` (1), `docs/paper/solar-seconds-matrix.md` (3) → #1 corona (success, Träger nachziehen).
- `docs/blatt/blatt-thuan-fragesteller.md` (4) → `termin:2026-12-02`.
- `docs/concepts/arxiv-api.md` (2) → Mountain-Linie.
- `docs/concepts/blatt-papier-resultat.md` (1) → #6 Bojen-Matrix.
- `docs/concepts/das-eine-instrument.md` (2) → Punkt „Das eine Instrument".
- `docs/concepts/der-paradigmenwechsel.md` (9) → JUICE.
- `docs/concepts/die-akteure-im-boden-und-wasser.md` (7) → #5 Seismik.
- `docs/concepts/fuenf-funken-anomalie-suche.md` (4) → #2 causal-arrow + B2-Befund.
- `docs/concepts/recherche-galileo-kadenz-reconciliation.md` (1) → B1-Befund.
- `docs/concepts/the-seven-spheres.md` (2) → #8 Δz-Befund.
- `docs/paper/causal-arrow-preregistration.md` (1), `docs/concepts/ein-blatt-papier.md` (2) → #2.
- `docs/paper/corona-heating-ladder.md` (2), `docs/surveys/survey-ein-blatt-korona-heizung.md` (1) → #1 (success).
- `docs/paper/cross-screening-tibet.md` (1) → #7.
- `docs/paper/depth-phase-echo-fleet.md` (5), `docs/surveys/axiom-gate-depth-phase-echo-fleet.md` (1) → #5.
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4), `docs/paper/flyby-path-2-preregistration.md` (1) → JUICE.
- `docs/paper/galileo-rotor-spin-era-floor.md` (1) → #4 + B1-Befund.
- `docs/paper/jwst-disequilibrium-survey.md` (7) → B5-Befund.
- `docs/paper/laic-arrow-direction.md` (3) → B3.
- `docs/paper/nadel-v-fresh-area-dip-scan.md` (1) → #1 lsst (success).
- `docs/paper/sturzflut-tibet-pfeil.md` (23) → #8 Trishuli.
- `docs/paper/tonga-lamb-crosscheck.md` (3) → BGR/vDEC.
- `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` (17) → DSN-Briefe.
- `docs/surveys/survey-2026-09-16-sonden-flotte.md` (4) → DSN-Briefe.
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (9) → HRV/Puls (wartend).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` (2) → River-/Browser-Linie.
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (7) → [redacted]/BLE + O6-descoped.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` (1) → Postfach.
- `docs/concepts/archivar-mathematikerin.md` (1) → Archivar-Pending (Mountain-Linie).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (2) → River-Linie.
- `docs/surveys/survey-fortschritt.md` (1) → Träger Mycelium.

## Benchmark (gemessen)

- **Flash (general)** lieferte B2, B5, #8 beim ersten Lauf korrekt und vollständig →
  die Klassen „öffentlicher Stack-/Detektions-Fund" und „PDS-Spaltenprüfung" sind
  flash-tragend.
- **research-max** trug die mehrstufigen Register-/Routen-Atome B1 (OAI-PMH/ADS/NTRS
  20+ Queries) und B3 (drei Länderportale) — die Klasse „mehrstufige Quellen-Route"
  bleibt pro/max.

## Wer nicht senden darf

`smail --send` und jeder Formular-Absand bleiben die Hand des Operators; kein Consent
verschiebt einen Send auf die Maschine. vDEC-Webform (descoped) und DEMETER metalink
sind Netz-Lesen bzw. Operator-Hand.

## Operator-Wort-Register (Stand 2026-09-27)

- **Wort:** „nein dann passt es nicht" (vDEC descoped) | 2026-09-26 | Operator (Session).
- **Wort:** „kein onboard ciq" | 2026-09-26 | Operator (Session).
- **Wort:** „natürlich" (Kp-Route wieder aufnehmen — Siegel-Wort) | 2026-09-26 | Operator (Session).
- **Wort:** „du kannst die mail abschicken" → Maschine sendet nie (AGENTS.md); vDEC descoped | 2026-09-26 | Operator (Session).
- **Wort:** „ändere die agents — das Handover nach umsetzbar/nicht umsetzbar sortieren" | 2026-09-26 | Operator (Session) → ausgeführt: AGENTS.md `3672968d3`.
- **Wort:** „alles Offene bis zur Kante abarbeiten, gemessen abschließen — nicht verschleppen" | 2026-09-26 | Operator (Session).
- **Wort:** „alles Offene und Benannt-Ungemessene wird übernommen" | 2026-09-26 | Operator (Session).
- **Wort:** „jedes Operator-Wort steht im Handover; keine Session kaut es neu durch" | 2026-09-26 | Operator (Session).
- **Wort:** „deine Daten/Datei verlassen das Gerät nie" | 2026-09-25 | Operator (Session) — [redacted]/DEMETER.
- **Wort:** „das mache ich erst, wenn ich gefördert werde" (Gurt-Beschaffung) | 2026-09-26 | Operator (Session).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
`docs/handover/handover-2026-09-27-sensory-folge180.md`,
`.github/workflows/funken-probe.yml`, `.github/workflows/causal-arrow-scan.yml`,
`docs/handover/archiv/handover-2026-09-26-sensory-folge179.md` (Move).
