<!--
  title: Handover — Sensory-Folge 184 (2026-09-27)
  session: Sensory-Folge 184
  class: handover
  date: 2026-09-27
  sha256: 0b9c504a763f9670f5de38be3436326ff46d35869d5f99048fb85b1d7ddd9c48
  status: live
-->
# Handover — Sensory-Folge 184 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht erklärt; git trägt,
was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks —
committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie überschrieben;
gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Sortierung: **umsetzbar zuerst**; der Akteur
steht pro Punkt in `Bindung`.

Diese Session konsumierte `handover-2026-09-27-sensory-folge183.md` (nach `archiv/`).

## Abarbeitbar (eigen, dispatchbar)

Der gemessene Rundenzustand: `state/zustand/standing-pass.md` (zitieren, nie kopieren); Operator-Akte in der Future-Queue, Dritt-Waits in `state/zustand/wartend.φ`.

### paper-check — Lauf am HEAD lesen (DOI-Parser geheilt)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** paper-check-Lauf nach diesem Push (geänderte Pfade `docs/paper/**`, `tools/register/src/bin/reference_verify.rs`).
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36312779071` @`375ec132`) Steps 4/5 (Export-Gates title/abstract/numbers/sha) waren **green** — kein Header-sha-Mismatch. Rot war Step 6 „Reference gate": die DOI `10.3321/j.issn:0001-5733.2007.01.001` in `docs/paper/yu-tong-fang-hu-2022-transfer-entropy-solar-wind-drivers.md:149` wurde am ersten `:` abgeschnitten → `10.3321/j.issn` → doi.org 404. `extract_dois` hatte `:` nicht im Suffix-Charset; Char-Set um `:` erweitert, lokal `./target/debug/reference_verify` auf dem Paper: alle 22 DOIs `resolved` (2026-09-27).
- **Blockade:** keine.
- **Braucht:** nach dem Push `ci_manage view`/`ci_manage log <paper-check-id>` lesen; eine verbleibende benannte Differenz benennen, nie still glätten.

### ci-check — eigene Arm-Warnungen am HEAD messen (ble.rs geheilt)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ci-check-Lauf am HEAD nach diesem Push.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36284835355` @`ee715257b`) `src/archivar/ble.rs:1271` clippy `get_first` (`*rest.get(0)?` → `*rest.first()?`), geheilt; `cargo check` 0 Warnungen, `cargo build -p omegaflow-register --bin reference_verify` grün. Die übrige clippy-Wand betrifft Fremddateien (`aia.rs`, `eve.rs`, `hdf4.rs`, `channels.rs`, …).
- **Blockade:** keine (eigener Teil erledigt).
- **Braucht:** `ci_manage log <ci-check-id>` am HEAD; verbleibende eigene Arm-Warnungen heilen — fremde Datei-Owner bleiben ihre Linien.

### probe-front-dark-matter — MDA-Resolver-Frequenz bleibt offen (0 honored)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** neue NTRS-/Archiv-Route für die 1985-PTTI-Scans.
- **Lage:** (gemessen 2026-09-27 via `archive_search --pdf-text`/`--pdf-image`) `19860018816` trägt einen NTRS-API-Textlayer (`--pdf-text` liest „The Deep Space Network Tracking System, Mark IV-A, 1986, J. A. Wackley"); der Volltext nennt weder `MDA` noch `resolver` (0 Fund). Die drei 1985-Scans `19850019987/19991/20940` sind `METADATA_ONLY` (kein API-PDF) und bild-only; `--pdf-image` hebt kein eingebettetes Bild (Encoding nicht DCT/JPX/Flate). Paper §5.4 nachgezogen (Header-sha `fba9c3af…`, version 9, `d67cdafb…` → `fba9c3af…`).
- **Blockade:** kein Rasterizer in `--pdf-image`; kein API-PDF für die 1985er.
- **Braucht:** Rasterizer-Modus in `archive_search --pdf-image` oder andere Scan-Ablage (z. B. NTRS-Viewer via `--playwright`).

### survey-2026-09-06-codestruktur — confirm-rendering pending
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** benannter Hidden-/CI-Run (`OMEGAFLOW_HIDDEN=1`).
- **Lage:** (gemessen 2026-09-27 via `sread`/`sgrep`) der Datenvertrag ist **zentral**, nicht je Format-Modul: Wire `src/archivar/relay.rs:825-886`, JS `static/constants.js:1,4,5,74-149`, WGSL `src/mathematikerin/shaders.rs:19-20,173-208`, alle Parser auf `Sample` (`src/archivar/types.rs:41`); der Survey-`:111`-Hunk trägt den Messstempel.
- **Blockade:** `archivar-mathematikerin.md:78` confirm-rendering ist kein read-only-Lauf.
- **Braucht:** der Hidden-Run (Fenster/HUD) gehört in CI bzw. einen benannten Hidden-Lauf — hier als `pending` benannt, nicht gefälscht.

### matrix-rotor — Muster gemessen (externer Runner-Shutdown)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster matrix-rotor-Schedule-Lauf (`43 */6 * * *`) rot.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36298095390`/`36282378750`) exit 143 ×3 = externes Runner-Shutdown-Signal (SIGTERM, `The runner has received a shutdown signal`), Slice 43/52 s — **nicht** Timeout (18000-s-Deckel), **nicht** OOM; der Rotor lief still (`OMEGAFLOW_HIDDEN=1`).
- **Blockade:** keine.
- **Braucht:** falls der nächste Schedule-Lauf rot ist, `ci_manage log <id>` — der gemessene Shutdown-Grund liegt vor.

## Termin

- **JUICE-Erdpassage 28./29.09.2026** | termin:2026-09-29 | Trigger: 28./29.09. | Lage
  (gemessen 2026-09-27): die Kp-Route ist in `phi/sources.φ` **wiederhergestellt**
  (folge182) — der frühere Riss ist aufgelöst | Braucht: Epoche ernten.
- **Europa-Clipper** | termin:2026-12-02 | Trigger: 02./03.12. | Braucht:
  Epoche ernten.

## Träger-Zeilen (Orphan-Faltung)

Der Dateiname in dieser Übergabe ist der Träger. Je Zeile ein zuletzt trägerloses
Dokument: `Pfad` (offene Marker) → Trägerpunkt oder descoped-Befund.

- `docs/blatt/blatt-der-grat.md` (2) → #7 Kreuz-Screening (Blatt).
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
- `docs/paper/tonga-lamb-crosscheck.md` (3) → BGR/vDEC.
- `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` (17) → DSN-Briefe.
- `docs/surveys/survey-2026-09-16-sonden-flotte.md` (4) → DSN-Briefe.
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (9) → HRV/Puls (wartend).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` (2) → River-/Browser-Linie.
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (7) → [redacted]/BLE + O6-descoped.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` (1) → Postfach.
- `docs/concepts/archivar-mathematikerin.md` (1) → Archivar-Pending (Mountain-Linie; Riss `:74` an Mountain getragen).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (2) → River-Linie.
- `docs/surveys/survey-fortschritt.md` (1) → Träger Mycelium.
- `docs/concepts/the-seven-spheres.md` (2), `docs/concepts/recherche-extern-galileo-ruck-borduhr-modell.md` (1), `docs/paper/jwst-disequilibrium-survey.md` (7) → B1/B5/B8 (geschlossen in folge181; die Marker in den Docs sind nachzuziehen).
- **descoped (gemessen 2026-09-27):** `docs/paper/terminologie-der-gegenstroemung.md` (Definitions-Kopf), `docs/paper/planet-nine-kbo-residue.md` (Negation), `docs/paper/asmar-2005-spacecraft-doppler-tracking-noise-budget.md` (Scanner-False-Positive), `docs/surveys/axiom-gate-broken-null-control.md` (nur Pendings des Papers).
- **carrier (eigener Punkt oben):** `docs/paper/probe-front-dark-matter.md`, `docs/surveys/survey-2026-09-06-codestruktur.md`.
- `docs/blatt/blatt-kreuz-screening-gyirong.md` (1), `docs/paper/sturzflut-tibet-pfeil.md` (23) → #7/#5 (Sensory-eigen, folge182).
- `docs/concepts/glossar.md` (1) → Terminologie (Sensory).
- `docs/concepts/recherche-galileo-kadenz-reconciliation.md` (1) → Mountain-Linie.
- `register_lookup --orphan-docs` nennt **15** trägerlose Dokumente; die verbleibenden sind
  fremder Linien (Weberin-Surveys ×5, `zeugnis`, `fremde-parser-sammlungen`,
  `omegaflow-legacy-konzepte`, `messpunkt-verteilung`, `tools-map`, `positive-maske`,
  `kybernetische-astrophysik`, `pfeiler-der-architektur`, `blatt-papier-beweis`,
  `exzellenz-konzept`) — je Linie zu falten.

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
- **Wort:** „ich kanns echt nicht mehr hören seit wie vielen sessions schleppst du die offenen punkte durch" — die B-Befunde sind geschlossen | 2026-09-27 | Operator (Session).
- **Wort:** „kümmer dich drum" (Sensory-Folge 182: alle eigenen offenen Punkte abarbeiten) | 2026-09-27 | Operator (Session).
- **Wort:** „Du kannst" (`/consent`) — session-weiter Delegations-Consent, nicht das Commit-Wort | 2026-09-27 | Operator (Session).
- **Wort:** „die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session).
- **Wort:** ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | 2026-09-27 | Operator (Future-Session).
- **Wort:** RX100-Kalibrierer descoped — „über exif weg": Luminanz über den Kamera-EXIF-Weg (K=12.5) | 2026-09-27 | Operator (Future-Session).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
- `docs/paper/probe-front-dark-matter.md` (Header-sha `fba9c3af…`, version 9),
- `src/archivar/ble.rs` (clippy `get_first` geheilt),
- `tools/register/src/bin/reference_verify.rs` (DOI-Suffix-Charset um `:` erweitert; Mountain-Domäne, hier gemessen-heilend gefixt),
- `docs/handover/handover-2026-09-27-sensory-folge184.md`,
- `docs/handover/archiv/handover-2026-09-27-sensory-folge183.md` (Move).

Der Zustand-Ledger `state/zustand/external-state.md` (CI-Status- + Postfach-Zeile)
ist gitignored (`.gitignore:137 /state/`) — lokal fortgeschrieben, nicht committet.
