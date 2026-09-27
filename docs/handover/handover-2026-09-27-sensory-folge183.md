<!--
  title: Handover — Sensory-Folge 183 (2026-09-27)
  session: Sensory-Folge 183
  class: handover
  date: 2026-09-27
  sha256: f3d667ce153a4c19604e7f792d766579a0a98fc30c2b6f7f1606ba5141e056b6
  status: live
-->
# Handover — Sensory-Folge 183 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht erklärt; git trägt,
was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks —
committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie überschrieben;
gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Sortierung: **umsetzbar zuerst**; der Akteur
steht pro Punkt in `Bindung`.

Diese Session konsumierte `handover-2026-09-27-sensory-folge182.md` (nach `archiv/`).

## Abarbeitbar (eigen, dispatchbar)

Der gemessene Rundenzustand: `state/zustand/standing-pass.md` (zitieren, nie kopieren); Operator-Akte in der Future-Queue, Dritt-Waits in `state/zustand/wartend.φ`.

### paper-check — Lauf am HEAD lesen (Header-shas nachgezogen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächster paper-check-Lauf (nach diesem Push)
- **Lage:** (gemessen 2026-09-27 via `export_latex --check`) vier Header-shas auf den Body gezogen,
  lokal `sha-match ok`: `docs/blatt/blatt-kreuz-screening-gyirong.md` Header war `c610dd0e…`,
  Body **`205e3152…`** (jetzt gezogen); `docs/paper/terminologie-der-gegenstroemung.md` `68c76aba…`
  (Vorgänger-Atom); `docs/paper/probe-front-dark-matter.md` `d67cdafb…`;
  `docs/surveys/survey-2026-09-06-codestruktur.md` `de35e451…` (Titel 97→54 gekürzt, H1-glatt).
- **Blockade:** keine.
- **Braucht:** nach dem Push den paper-check-Lauf lesen (`ci_manage view`/`log`); eine verbleibende
  benannte Differenz benennen, nie still glätten.

### probe-front-dark-matter — vision-Read der Scan-Quellen (GPS/DSN gemessen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-27 via `archive_search`) GPS-Timing 1983–85 in NTRS gefunden (6;
  `19830027104` trägt Textlayer und nennt das DSN-1985-Timing-Requirement); die 810-005-Doku
  (202E, 304D, 202 Rev. A) trägt **keine** MDA-Resolver-Frequenz; die MDA-Beschreibung liegt im
  Mark IV-A Tracking System 1986 (`19860018816`, TDA PR 42-85) — scan-only. Paper §5.4
  `:331-333`/`:362-364` mit dieser Messung aktualisiert (0 honored).
- **Blockade:** die Scan-Quellen (`19860018816`, 1985er) tragen keinen Textlayer.
- **Braucht:** `archive_search --pdf-image <pdf-url>` → `vision` für `19860018816` und die
  1985-Scans (`19850019987/19991/20940`); bei Fund die MDA-Resolver-Frequenz belegen.

### survey-2026-09-06-codestruktur — confirm-rendering pending
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** benannter Hidden-/CI-Run (`OMEGAFLOW_HIDDEN=1`).
- **Lage:** (gemessen 2026-09-27 via `sread`/`sgrep`) der Datenvertrag ist **zentral**, nicht je
  Format-Modul: Wire `src/archivar/relay.rs:825-886`, JS `static/constants.js:1,4,5,74-149`,
  WGSL `src/mathematikerin/shaders.rs:19-20,173-208`, alle Parser auf `Sample`
  (`src/archivar/types.rs:41`); der Survey-`:111`-Hunk trägt den Messstempel. Der Riss
  (`archivar-mathematikerin.md:74` `force_type`-Offset) ist an Mountain getragen.
- **Blockade:** `archivar-mathematikerin.md:78` confirm-rendering ist kein read-only-Lauf.
- **Braucht:** der Hidden-Run (Fenster/HUD) gehört in CI bzw. einen benannten Hidden-Lauf —
  hier als `pending` benannt, nicht gefälscht.

### clippy `-D warnings` — ble.rs (eigener Datei-Owner)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** `ci-check`-Lauf am HEAD lesbar.
- **Lage:** (gemessen 2026-09-25 via Stehender Pass, external-state.md:23) `src/archivar/ble.rs` ist der BLE-Reader (Sensory-Domäne); der clippy-Lauf (`mountain-folge181`) nennt ble.rs in der Wand.
- **Blockade:** Log noch pending.
- **Braucht:** `ci_manage log <id>` lesen; die ble.rs-Warnungen heilen (kein lokales clippy).

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
- `register_lookup --orphan-docs` nennt **13** trägerlose Dokumente; die verbleibenden sind
  fremder Linien (Weberin-Surveys ×5, `zeugnis`, `fremde-parser-sammlungen`,
  `omegaflow-legacy-konzepte`, `messpunkt-verteilung`) — je Linie zu falten.

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

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
- `docs/blatt/blatt-kreuz-screening-gyirong.md` (Header-sha `205e3152…`),
- `docs/paper/probe-front-dark-matter.md` (Header-sha `d67cdafb…`),
- `docs/surveys/survey-2026-09-06-codestruktur.md` (Header-sha `de35e451…`, Titel gekürzt),
- `docs/concepts/tool-forms.md` (neu, Header-sha `73586244…`),
- `opencode.json` (`line`-Prompt), `.opencode/command/consent.md`, `.opencode/command/start.md`,
- `.opencode/plugin/form-guard.ts` (neu), `tools/utils/src/bin/omega_sh.rs` (`perms`), `.gitignore` (Whitelist),
- `docs/handover/handover-2026-09-27-sensory-folge183.md`.

Getragen (Fremd-Übergabe, **nicht** in diesem Commit — fremde uncommittete Arbeit):
- `docs/handover/handover-2026-09-27-mycelium-folge179.md`: IGETS-sha256 (Block `phi/sources.φ:9941`,
  noch ohne `sha256`-Zeile), BoJen-Matrix-143 (gemessene Runner-Shutdown-Ursache), ci_watchdog-Matcher.

Der `force_type`-Offset-Riss (`archivar-mathematikerin.md:74`) wurde von Mountain in
`9d4f61c49` (folge181) **unabhängig geheilt** — mein getragener Punkt ist erledigt.

Infra-Atom (diese Session): die Form-Karte `docs/concepts/tool-forms.md` + der `line`-Prompt +
die Command-Verdrahtung + das Plugin `.opencode/plugin/form-guard.ts` wirken erst nach einem
**opencode-Neustart** (Config/Plugins laden einmal beim Start). `omega_sh perms [<agent>]` ist der
konfigurationsgestützte Druck der Deny→Ersatz-Karte; sein Test `gate_perms_matches_config` läuft in CI.

Der Zustand-Ledger `state/zustand/external-state.md` (CI-Status- + Postfach-Zeile)
ist gitignored (`.gitignore:137 /state/`) — lokal fortgeschrieben, nicht committet.

Der frühere `yu-tong`-Riss (title=94 > 75, River-Werk `314fea2b5`) ist im aktuellen
paper-check-Lauf nicht mehr rot (gemessen 2026-09-27: nur Gyirong) — kein offener Punkt.
