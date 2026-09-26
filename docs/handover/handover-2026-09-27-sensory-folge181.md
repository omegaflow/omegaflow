<!--
  title: Handover — Sensory-Folge 181 (Stand 2026-09-27)
  session: Sensory-Folge 181
  class: handover
  date: 2026-09-27
  sha256: 82b16b0aea5856d90dd76d86a0b17c599966353b8916e7fda6ab1aaa5c0a8bbf
  status: live
-->
# Handover — Sensory-Folge 181 (2026-09-27)

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

- **HEAD** `c1fae4bb8`, `origin/main == HEAD` (folge180 trug noch `517d587e9`; main ist
  um 4 Commits vorangeschritten).
- **main GEHEILT** — `src/archivar/main_flow.rs` trägt den `| "gosat_tanso3"`-Arm aus
  `fae4a5081` (der am Dateiende statt im Match landete) jetzt korrekt im geo-series-Match;
  `cargo check` clean, Commit `cff336062`. Der Build ist für alle Linien frei.
- **Postfach** — `state/mail/mail_ledger.φ` absent → `pending` (Aufbau gehört CI);
  letzte 6 Eingänge im Snapshot (NED-Antwort, OpenAlex-Login, Meta-Codes).
- **CI:** die zwei folge180-Dispatches (`funken-probe 36276735080`,
  `causal-arrow-scan 36276736870`) scheiterten am main-Bruch. Nach Heilung neu dispatcht
  → `funken-probe 36277987363`, `causal-arrow-scan 36277989033` (gemessen 2026-09-27).
- **Geteilter Baum ist LIVE:** eine fremde Linie (mycelium/mountain) arbeitet gerade
  uncommittet an `.github/workflows/*-cdn.yml`, `phi/sources.φ`,
  `phi/blocked_sources.φ`, `src/archivar/main_flow.rs`, `src/archivar/skydirection.rs`,
  `tools/harvest/.../modis_lst_cmg_compiler.rs` u. a. → nicht anfassen.

## Flaschenhals — geheilt

- **main war rot an `main_flow.rs:5299`** (fremder Commit `fae4a5081`: die Zeile landete
  am Dateiende statt im geo-series-Match). Geheilt in `cff336062` (der eine Satz in den
  Match gesetzt; `cargo check` clean). mycelium hatte keine offene Session; der Hunk war
  strandet. Kein offener fremder Blocker.

## Abarbeitbar (eigen, dispatchbar)

### 3. O1 DEMETER/CDPP — Order nicht abholbar (Riss)
- **Status:** wartend | **Bindung:** operator | **Trigger:** Order neu erzeugen | **Frist 2026-09-28**
- **Lage:** (gemessen 2026-09-27) metalink gesichert
  (`data/cdpp-archive.cnes.fr/metalink_18387.xml`, 68.9 MB, 97 078 URLs; Frist-Ablauf
  09/28 12:46). **Aber:** die File-URLs (`/orders/public/files/<id>?orderToken=…`) weist
  der **F5-WAF** ab (`Request Rejected`) — curl/sfetch/Proton, mit vollständigem
  Browser-Header-Satz **und sogar die Browser-Session selbst** (Top-Level-Navigation →
  „Request Rejected", gemessen). Die Order steht `DONE_WITH_WARNING` mit
  `availableFilesCount 0`, `filesInErrorCount 96978` → die Dateien sind nicht servierbar.
- **Blockade:** Order fehlgeschlagen (0 verfügbare Dateien).
- **Braucht:** Order in der SPA **neu erzeugen** (die neuen Datei-URLs); dann erneut
  messen. Korrektur zu folge180 — `availableFilesCount 0` war schon da das Signal, der
  befüllte metalink kein Gegenbeweis.

### 5. #4 Seismik-Flotte — gemessen (Dual-Phase-Fit mit Gate), Gravimeter offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27) `depth-phase-fleet`/`depth-phase`/`cmt-ndk-fleet` alle
  **success**; Dual-Phase-Fit mit gesetztem sP-Gate (0.78): mean offset **−8.2 km**
  (5 Events, se 8.8), weighted joint −18.4 km; eingetragen in
  `depth-phase-echo-fleet.md` (§Fleet re-run).
- **Blockade:** keine (Gravimeter-SFTP separat).
- **Braucht:** offen nur noch `die-akteure-im-boden-und-wasser.md:85` Gravimeter-SFTP.

### 6. #5 Blatt-1-Bojen-Matrix — Hintergrundzyklus
- **Status:** wartend | **Bindung:** eigen | **Trigger:** Rotor-Zyklus (~16 h)
- **Lage:** (gemessen 2026-09-27) der Bojen-Matrix-Rotor ist ein **Hintergrundzyklus** im
  laufenden System (136 Paare × 366 Zellen ≈ 55 h/Station, voller Zyklus ≈ 16 h), kein
  Einmal-Bin; Träger `docs/concepts/blatt-papier-resultat.md:63-71`.
- **Blockade:** Laufzeit.
- **Braucht:** Rotor laufen lassen; Σ p̂·M-Zeile nachtragen.

### 7. #7 Kreuz-Screening — Workflow gebaut, Lauf offen
- **Status:** wartend | **Bindung:** eigen | **Trigger:** Lauf endet
- **Lage:** (gemessen 2026-09-27) Workflow **`.github/workflows/cross-screening-tibet.yml`**
  gebaut (Serien aus `phi/meteo/tibet-flut-2026.json`: gyirong/kollab/rasuwa) und
  dispatcht → Run `36278481962`.
- **Blockade:** keine.
- **Braucht:** Ergebnis via `ci_manage view 36278481962`; Träger
  `cross-screening-tibet.md:42/:44`, `blatt-kreuz-screening-gyirong.md:71/:201`.

### 8. #9 Trishuli — Post-Scene geladen (SAS-Bug gefixt), Alignment offen
- **Status:** eigen | **Bindung:** eigen | **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27) SAS-Bug in `s1_post_capture`/`s1_sar_compiler` gefixt
  (JSON-`\u0026` wird entschärft; Commit `046c3ee69`); Post-Event-Szene
  `S1D_…_20260828T122141` geladen → `tmp/s1post/…_vv.tif` (666 368 744 B).
- **Blockade:** keine (Pre-Baseline für die Differenz fehlt).
- **Braucht:** Pre-Event-Szene als `OMEGAFLOW_S1_VOR` setzen → `s1_raster_diff`
  (Vor/Nach-Differenz).

## Operator-gebunden (Vorbereitung an der Kante)

- **DEMETER/CDPP Order-Export** | operator | Trigger: sofort, Frist 2026-09-28 | Lage:
  metalink-Link in der SPA (geprüft, befüllt) | Braucht: SPA-Login → metalink
  herunterladen (oder der Maschine die Session zugänglich machen).
- **RR-Kanal / Beat-to-beat** | operator | Trigger: Förderung gewährt (2026-09-26) |
  Lage: BLE-HR ohne RR; FIT `nn=0` | Braucht: Brustgurt (Polar H10/HRM-Dual) →
  `perm_tone_probe`.
- **Weberin-Quellen HAWC** | operator | Trigger: `OMEGAFLOW_CA_BUNDLE` gesetzt |
  Braucht: HAWC-Fetch mit lokalem 4-Zert-Bundle; CI-Secret separat.
- **Beat-Arbitrierung** | operator | Trigger: zweiter Beat-Kanal (Gurt) | Braucht:
  `OMEGAFLOW_HIDDEN=1`-Lauf → genau eine `beat source:`-Zeile.
- **Live-Sensor-Cluster** | LOCK | Trigger: Operator-Wort hebt LOCK | Braucht: BOM
  bestellen (inkl. H2).
- **ESP32-Puls-Knoten** | LOCK | Trigger: Operator-Wort hebt LOCK | Wort: ESP32 separat
  als eigener LOCK | 2026-09-25 | Operator (Session).
- **JUICE-Erdpassage 28./29.09.2026** | termin:2026-09-29 | Trigger: 28./29.09. | Lage:
  **Riss** — Kp-Route in `61e272ab0` aus `sources.φ` entfernt | Wort: „natürlich" |
  2026-09-26 | Operator (Session) | Braucht: **O7 unten**.

## Abgeschlossen / gemessen geschlossen (Befund)

- **#1 Funken-Probe** — gemessen (run `36277987363`): Funke 5 (TDB/Rømer) `separated`
  (−900.883944 s gg. 60 s); Funke 3 (Broker-Differenz) `pending` (nur fink erreichbar,
  absent; lasair 401, alerce 404). Eingetragen in `fuenf-funken-anomalie-suche.md`.
- **#2 causal-arrow-Scan** — gemessen (run `36277989033`): Gyirong↔Rasuwa alle Lags
  `no finding` (Path-1 trägt nicht, n=241); `kbo_residue_probe` scheiterte am
  Ephemeriden-Batch **404** (11 Dateien, `bin parse void`) → Quelle `absent`. Eingetragen
  in `causal-arrow-preregistration.md` + `fuenf-funken-anomalie-suche.md`.
- **#3 Galileo CK-Kerne `_rtr`** — bereits geerntet (gemessen 2026-09-27): 8 Dateien
  `ck90341a`–`ck90344b_rtr.bc` vollständig in `data/naif.jpl.nasa.gov/`
  (`ck90341a_rtr.bc` = 3 863 552 B = NAIF-Live-Größe); der Punkt wurde als „offen"
  getragen. `galileo-rotor-spin-era-floor.md` trägt die lokalen Holdings bereits.
- **LLNL-G3D-JPS (Seismik-Rohdaten)** — bereits geerntet: `data/LLNL_G3D_JPS.volume.bin`
  + `data/gs.llnl.gov/llnl_g3d_jps.interpolated.zip` liegen lokal.
- **B1 Galileo Borduhr** — `absent`. Befund eingetragen in
  `docs/concepts/recherche-extern-galileo-ruck-borduhr-modell.md` (§Nachtrag
  2026-09-27): keine absolute Frequenz-Reduktion über 1995-11-30/12-01 (arXiv 406,
  OAI-PMH `start date too early`, NTRS/ADS nur 1993); der SPICE-SCLK-Kernel trägt keine
  Frequenzänderung. Punkt geschlossen.
- **B2 BL808 802.15.4** — `absent`. Befund eingetragen in
  `docs/specs/mantis-shrimp-bom.md` (Z. 116): kein öffentlicher 802.15.4-Stack für BL808
  (Bouffalo Issue #148 NDA, `bouffalo_sdk_bl808` README ZIGBEE ×, Zephyr-PR #112921 ohne
  BL808). Träger war in folge180 falsch gemappt (Funken-Doc); Korrektur auf die
  BOM-Spec. Punkt geschlossen.
- **B5 JWST Biosignatur** — Detektionen gemessen. Befund eingetragen in
  `docs/paper/jwst-disequilibrium-survey.md` (§6): Trigger-DOI `10.1073/pnas.2416188122`
  ist Perspektive/Modell; echte Detektionen WASP-39 b CO2 `10.1038/s41586-022-05269-w`
  + SO2 `10.1038/s41586-023-05902-2`, K2-18 b CH4/CO2 `10.3847/2041-8213/acf577`; DMS
  `absent` (`insufficient evidence`). Punkt geschlossen.
- **#8 Sieben Sphären Δz** — `absent`. Befund eingetragen in
  `docs/concepts/the-seven-spheres.md` (Sphäre Ⅶ): PDS `EAR-A-3-RDR-OCCULTATIONS-V12.0`
  (`occlist.lbl`, 58 Spalten) ohne Δz/vertical/altitude; Nachfolger
  `smallbodiesoccultations 4.0` ebenso. Punkt geschlossen.
- **O5 Weberin vDEC** — `descoped` (folge180): organisations-only. Träger
  `state/mail/weberin-quellen-konten-2026-09-26.md`.
- **O6 Onboard-/CIQ-Bedarf** — `descoped` (folge180): BLE + eigener Sensor-Knoten
  decken die Kanäle. Träger `survey-2026-09-23-geraete-anbindung-radiatoren.md`.

## Wartend / blockiert (mit Trigger)

- **O7 Kp-Route wiederherstellen** | eigen | Trigger: sofort | Lage (gemessen
  2026-09-27): `phi/sources.φ` trägt **fremde uncommittete** Änderungen → mein Hunk darf
  nicht mit fremdem gestaged werden | Blockade: geteilter Baum (live) | Braucht: nach
  fremdem Commit die vier Kp-Zeilen aus `61e272ab0^:phi/sources.φ` wieder aufnehmen.
- **M2c — Rust-ZNSP-Host BL808/H2** | wartend | Trigger: Ox64 angekommen
  (`LZ473049629CN`) | Blockade: Carrier.
- **BL808-eigenes 802.15.4-Radio** | geschlossen (`absent`, s. B2).
- **Das eine Instrument — Anomalie** | blockiert | Trigger: zweiter Messkanal
  (VLBI-Beacon) | Blockade: kein Instrument misst den vollen Pionier-Phasenraum.
- **LAIC/Causal-Arrow (CSES)** | wartend | Trigger: 2026-10-02 | Lage: ASI-SSDC-Weg
  gemessen, PI-Berechtigung offen.

## Extern (Dritter)

- **Postfach** | wartend | Trigger: neuer Eingang | Blockade: Ledger absent (CI-Duty) |
  Braucht: `smail_recv`/Ledger bei Trigger.
- **Ox64-Lieferung** | termin (Carrier) | Trigger: Ankunft (`LZ473049629CN`) | Braucht:
  quittieren → M2c.
- **BGR-Matched-Filter (Tonga)** | dritter (vDEC) | Trigger: vDEC-Zugang | Lage: vDEC
  descoped → BGR-Weg neu bewerten | Braucht: Arrival messen.
- **DSN-Briefe in Flug** | dritter | Trigger: DSN-Antwort | Braucht: quittieren.
- **Gaia DR4 + Europa-Clipper** | termin:2026-12-02 | Trigger: 02./03.12. | Braucht:
  Epoche ernten.

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
- `docs/concepts/fuenf-funken-anomalie-suche.md` (4) → #2 causal-arrow.
- `docs/paper/causal-arrow-preregistration.md` (1), `docs/concepts/ein-blatt-papier.md` (2) → #2.
- `docs/paper/corona-heating-ladder.md` (2), `docs/surveys/survey-ein-blatt-korona-heizung.md` (1) → #1 (success).
- `docs/paper/cross-screening-tibet.md` (1) → #7.
- `docs/paper/depth-phase-echo-fleet.md` (5), `docs/surveys/axiom-gate-depth-phase-echo-fleet.md` (1) → #5.
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4), `docs/paper/flyby-path-2-preregistration.md` (1) → JUICE.
- `docs/paper/galileo-rotor-spin-era-floor.md` (1) → #4.
- `docs/paper/laic-arrow-direction.md` (3) → LAIC/Causal-Arrow.
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

- **Flash (general)** lieferte B2, B5, #8 korrekt und vollständig → die Klassen
  „öffentlicher Stack-/Detektions-Fund" und „PDS-Spaltenprüfung" sind flash-tragend.
- **research-max** trug B1 (OAI-PMH/ADS/NTRS, 20+ Queries) und B3 (drei Länderportale) —
  die Klasse „mehrstufige Quellen-Route" bleibt pro/max.

## Wer nicht senden darf

`smail --send` und jeder Formular-Absand bleiben die Hand des Operators; kein Consent
verschiebt einen Send auf die Maschine. DEMETER metalink ist Operator-Hand.

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
- **Wort:** „ich kanns echt nicht mehr hören seit wie vielen sessions schleppst du die offenen punkte durch" — die B-Befunde sind geschlossen, nicht weitergetragen | 2026-09-27 | Operator (Session).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
`src/archivar/main_flow.rs` (main-Heilung, `cff336062`),
`docs/handover/handover-2026-09-27-sensory-folge181.md`,
`docs/handover/archiv/handover-2026-09-27-sensory-folge180.md` (Move),
`docs/concepts/recherche-extern-galileo-ruck-borduhr-modell.md`,
`docs/paper/jwst-disequilibrium-survey.md`,
`docs/concepts/the-seven-spheres.md`,
`docs/specs/mantis-shrimp-bom.md`,
`docs/concepts/fuenf-funken-anomalie-suche.md`,
`docs/paper/causal-arrow-preregistration.md`,
`.github/workflows/cross-screening-tibet.yml` (eigener Commit).
