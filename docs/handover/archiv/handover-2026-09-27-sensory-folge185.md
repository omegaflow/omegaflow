<!--
  title: Handover — Sensory-Folge 185 (2026-09-27)
  session: Sensory-Folge 185
  class: handover
  date: 2026-09-27
  sha256: 5e029d51c98e219e29be833fd7d0a579a2c02304b81fa7e54821ae6e3fb8c319
  status: live
-->
# Handover — Sensory-Folge 185 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht erklärt; git trägt,
was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks —
committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie überschrieben;
gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Sortierung: **umsetzbar zuerst**; der Akteur
steht pro Punkt in `Bindung`.

Diese Session konsumierte `handover-2026-09-27-sensory-folge184.md` (nach `archiv/`).
Geschlossen in diesem Atom: der paper-check-Punkt — `36316318565` @`5cdf92677` =
**success** (DOI-Suffix-Fix bestätigt).

## Abarbeitbar (eigen, dispatchbar)

Der gemessene Rundenzustand: `state/zustand/standing-pass.md` (zitieren, nie kopieren); Operator-Akte in der Future-Queue, Dritt-Waits in `state/zustand/wartend.φ`.

### ci-check — HEAD-Lauf lesen (eigener Arm geheilt)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ci-check-Lauf `36320549136` @`edfcedad0` completed.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36314070355` + `view`) genau **eine**
  Sensory-eigene Warnung — `src/archivar/ble.rs:1271` `clippy::get_first` — am HEAD geheilt
  (`rest.first()`, `sgrep "rest.get(0)" src/archivar/ble.rs` → 0 Treffer). Die übrigen 29
  clippy-Sites sind fremder Datei-Owner (`extract.rs` 11, `main_flow.rs` 4, `hdf4.rs` 3,
  `actuators.rs` 3, `channels.rs`/`tests.rs` je 2, `aia.rs`/`eve.rs`/`uws.rs` je 1).
  HEAD-Lauf `36320549136` **pending** (created 12:54Z); `36317126936` @`5728b5424` in_progress.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36320549136` einmal lesen (kein Poll); verbleibende eigene
  Arm-Warnung heilen — fremde Dateien bleiben ihre Linien.

### survey-2026-09-06-codestruktur — confirm-rendering ist headless verifizierbar
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster **completed** matrix-rotor-Lauf mit Artefakt `matrix-rotor`.
- **Lage:** (gemessen 2026-09-27 via `sgrep`/`sread`) die folge184-Blockade war unpräzise:
  `docs/concepts/archivar-mathematikerin.md:78` nennt den read-only-Weg selbst —
  `OMEGAFLOW_HIDDEN=1 cargo run` fährt den vollen ω-Loop, die `φ window:`-stderr-Zeile ist
  der maschinenlesbare HUD-Zwilling; Print-Site `src/mathematikerin/omega.rs:1779`
  (unbedingtes `eprintln!` → `matrix-rotor.txt`). Der Datenvertrag ist zentral (Wire
  `src/archivar/relay.rs:825-886`, JS `static/constants.js:1,4,5,74-149`, WGSL
  `src/mathematikerin/shaders.rs:19-20,173-208`; alle Parser auf `Sample`,
  `src/archivar/types.rs:41`). Lauf `36320306259` wurde extern (Runner-Shutdown) abgebrochen
  und lud kein Artefakt — daher kein Log-Beleg.
- **Blockade:** keine (Weg gemessen); es fehlt der nächste nicht-extern abgebrochene Lauf.
- **Braucht:** `gh run download <matrix-rotor-run-id> -n matrix-rotor -D /tmp/opencode/mr-art`
  → `sgrep "window:" /tmp/opencode/mr-art/matrix-rotor.txt` — die `φ window:`-Zeile ist der Beleg.

### probe-front-dark-matter — NTRS-Route gemessen negativ (0 honored)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** neue Volltext-Route (Rasterizer oder NTRS-Antwort).
- **Lage:** (gemessen 2026-09-27 via `archive_search --playwright`/`--pdf-text`/`--tavily`)
  Citation-Seite + API-JSON von `19850019987/19991/20940` rendern 200, erklären aber
  `METADATA_ONLY` (`downloads: []`, `downloadsAvailable:false`); die einzige Scan-URL
  (`ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19850019987.pdf`) trägt keinen Textlayer
  (`--pdf-text` → pending); kein `MDA`/`resolver`/`Mark IV` in irgendeinem Output
  (`--root` matched 0); Tavily 0 relevant. Paper §5.4 auf **v10** nachgezogen
  (`docs/paper/probe-front-dark-matter.md`, Header-sha `6447b059…`, `version: 10`).
- **Blockade:** kein Rasterizer in `archive_search --pdf-image`; kein API-PDF der 1985er.
- **Braucht:** Rasterizer-Modus in `archive_search --pdf-image` (eigener Weg) **oder** NTRS
  Document-Inquiry (`sti.nasa.gov/doc-inquiry/?DocID=19850019987`, im Citation-HTML
  verlinkt) — der Dritt-Akt liegt in Future's Operator-Queue.

### matrix-rotor — Muster (externer Runner-Shutdown)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster matrix-rotor-Schedule-Lauf (`43 */6 * * *`) rot.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36318994937`/`36298095390`/`36282378750`)
  exit 143 ×3 = externes Runner-Shutdown-Signal (`The runner has received a shutdown signal`),
  Slice 43/52 s — **nicht** Timeout (18000-s-Deckel), **nicht** OOM; der Rotor lief still
  (`OMEGAFLOW_HIDDEN=1`). Instrumentierung steht (`matrix-rotor.yml:62` trap + `timeout --verbose`).
- **Blockade:** keine.
- **Braucht:** bei Rot `ci_manage log <id>` — der gemessene Shutdown-Grund liegt vor.

## Termin

- **JUICE-Erdpassage 28./29.09.2026** | termin:2026-09-29 | Trigger: 28./29.09. | Lage
  (gemessen 2026-09-27): die Kp-Route ist in `phi/sources.φ` **wiederhergestellt** (folge182) |
  Braucht: Epoche ernten.
- **Europa-Clipper** | termin:2026-12-03 | Trigger: 02./03.12. | Lage (gemessen 2026-09-27):
  Dritt-Wait in `state/zustand/wartend.φ:15` (Aufnehmer sensory) | Braucht: Epoche ernten.

## Träger-Zeilen (Orphan-Faltung)

Der Dateiname in dieser Übergabe ist der Träger. Je Zeile ein zuletzt trägerloses
Dokument: `Pfad` (offene Marker) → Trägerpunkt oder descoped-Befund. Die Marker in den
Punkten #1–#7 sind **echte Messgrenzen** (konditionale Prüfung, Remessungen), keine
stale Reste — sie werden getragen, nie geglättet (0 honored).

- **#1 corona (success):** `docs/blatt/blatt-solar-seconds-matrix.md` (1),
  `docs/paper/solar-seconds-matrix.md` (3), `docs/paper/corona-heating-ladder.md` (2),
  `docs/surveys/survey-ein-blatt-korona-heizung.md` (1).
- **#1 lsst (success):** `docs/paper/nadel-v-fresh-area-dip-scan.md` (1).
- **#2 causal-arrow:** `docs/concepts/fuenf-funken-anomalie-suche.md` (4),
  `docs/paper/causal-arrow-preregistration.md` (1), `docs/concepts/ein-blatt-papier.md` (2).
- **#4 galileo:** `docs/paper/galileo-rotor-spin-era-floor.md` (1).
- **#5 Seismik:** `docs/concepts/die-akteure-im-boden-und-wasser.md` (7),
  `docs/paper/depth-phase-echo-fleet.md` (5),
  `docs/surveys/axiom-gate-depth-phase-echo-fleet.md` (1),
  `docs/paper/sturzflut-tibet-pfeil.md` (23).
- **#6 Bojen-Matrix:** `docs/concepts/blatt-papier-resultat.md` (1).
- **#7 Kreuz-Screening:** `docs/blatt/blatt-der-grat.md` (2),
  `docs/paper/cross-screening-tibet.md` (1),
  `docs/blatt/blatt-kreuz-screening-gyirong.md` (1).
- **LAIC/Causal-Arrow:** `docs/paper/laic-arrow-direction.md` (3).
- **BGR/vDEC:** `docs/paper/tonga-lamb-crosscheck.md` (3).
- **DSN-Briefe:** `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` (17),
  `docs/surveys/survey-2026-09-16-sonden-flotte.md` (4).
- **HRV/Puls (wartend):** `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (9).
- **[redacted]/BLE + O6-descoped:** `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (7).
- **Postfach:** `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` (1).
- **Terminologie (Sensory):** `docs/concepts/glossar.md` (1).
- **JUICE:** `docs/concepts/der-paradigmenwechsel.md` (9),
  `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4),
  `docs/paper/flyby-path-2-preregistration.md` (1).
- **Das eine Instrument:** `docs/concepts/das-eine-instrument.md` (2).
- **termin:2026-12-02:** `docs/blatt/blatt-thuan-fragesteller.md` (4).
- **B1/B5/B8 (geschlossen in folge181; die Marker in den Docs sind nachzuziehen):**
  `docs/concepts/the-seven-spheres.md` (2),
  `docs/concepts/recherche-extern-galileo-ruck-borduhr-modell.md` (1),
  `docs/paper/jwst-disequilibrium-survey.md` (7).
- **descoped (gemessen 2026-09-27):** `docs/paper/terminologie-der-gegenstroemung.md`
  (Definitions-Kopf), `docs/paper/planet-nine-kbo-residue.md` (Negation),
  `docs/paper/asmar-2005-spacecraft-doppler-tracking-noise-budget.md` (Scanner-False-Positive),
  `docs/surveys/axiom-gate-broken-null-control.md` (nur Pendings des Papers).
- **carrier (eigener Punkt oben):** `docs/paper/probe-front-dark-matter.md`,
  `docs/surveys/survey-2026-09-06-codestruktur.md`.
- **fremde Linien (nicht in dieser Übergabe getragen — Aufenthalt = Eigentum):**
  `docs/concepts/arxiv-api.md`, `docs/concepts/archivar-mathematikerin.md`,
  `docs/concepts/recherche-galileo-kadenz-reconciliation.md` → Mountain;
  `docs/surveys/survey-2026-09-20-browser-anbindung.md`,
  `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` → River;
  `docs/surveys/survey-fortschritt.md` → Mycelium. `register_lookup --orphan-docs`
  nennt sie als trägerlos; jede Linie faltet ihre eigene.

## Wer nicht senden darf

`smail --send` und jeder Formular-Absand bleiben die Hand des Operators; kein Consent
verschiebt einen Send auf die Maschine. DEMETER metalink ist Operator-Hand. Die
NTRS Document-Inquiry ist Operator-Hand.

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
- **Wort:** „ich kanns echt nicht mehr hören seit wie vielen sessions schleppst du die offenen punkte durch" — die B-Befunde sind geschlossen | 2026-09-27 | Operator (Session).
- **Wort:** „kümmer dich drum" (Sensory-Folge 182: alle eigenen offenen Punkte abarbeiten) | 2026-09-27 | Operator (Session).
- **Wort:** „Du kannst" (`/consent`) — session-weiter Delegations-Consent, nicht das Commit-Wort | 2026-09-27 | Operator (Session).
- **Wort:** „die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session).
- **Wort:** ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | 2026-09-27 | Operator (Future-Session).
- **Wort:** RX100-Kalibrierer descoped — „über exif weg": Luminanz über den Kamera-EXIF-Weg (K=12.5) | 2026-09-27 | Operator (Future-Session).
- **Wort:** „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus" (`/consent`, Sensory-Folge 185) | 2026-09-27 | Operator (Session).
- **Wort:** UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session, Mountain).
- **Wort:** D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | 2026-09-27 | Operator (Session, Mountain).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
- `docs/paper/probe-front-dark-matter.md` (v10, Header-sha `6447b059…`),
- `docs/handover/handover-2026-09-27-sensory-folge185.md`,
- `docs/handover/archiv/handover-2026-09-27-sensory-folge184.md` (Move).

Der Zustand-Ledger `state/zustand/external-state.md` ist gitignored (`.gitignore:137 /state/`) —
lokal fortgeschrieben, nicht committet.
