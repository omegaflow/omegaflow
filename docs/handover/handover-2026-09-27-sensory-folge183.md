<!--
  title: Handover — Sensory-Folge 183 (2026-09-27)
  session: Sensory-Folge 183
  class: handover
  date: 2026-09-27
  sha256: 0b95a5990d479066f3679d7f02af687e5b290abe01b923ce72eedf8546db08a5
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

### #5 Seismik — IGETS (Ernte dispatcht)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Lauf `36284373510` endet
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) Lauf `36284373510` **success** (01:03). Workflow `igets-cdn.yml` (getrackt); `--list-stations` = **47 Level2-Träger**; Level2-Filter `igets_compiler.rs:414-420`.
- **Blockade:** keine.
- **Braucht:** sha256 in den `igets`-Block `phi/sources.φ` nachtragen (Zeile driften — Zeilennummer nicht eintragen).

### paper-check — terminologie-Header-sha (behoben; CI-Verifikation offen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächster paper-check-Lauf
- **Lage:** (gemessen 2026-09-27 via `export_latex --check`) `docs/paper/terminologie-der-gegenstroemung.md`
  Header-sha war veraltet — Body in `b30323daa` nach der Header-Setzung `b294ba4c5`
  geändert, ohne Nachziehen. Korrigiert: Header Zeile 5 `c4a98130…` → **`68c76aba…`**
  (`omega_sh sha`). Lokal `export_latex --check <datei>` grün. Ergänzt (gemessen 2026-09-27
  via Browser/Job-Log `36302005511`): der nächste paper-check-Lauf ist rot am Gyirong-Blatt —
  `docs/blatt/blatt-kreuz-screening-gyirong.md` Header-sha `205e3152…` ≠ Body `c610dd0e…`
  („named difference, not silently fixed").
- **Blockade:** keine.
- **Braucht:** Gyirong-Header-sha nachziehen (`omega_sh sha`), dann paper-check-Lauf lesen.

### #6 Blatt-1-Bojen-Matrix — CI-Rotor (Rerun dispatcht)
- **Status:** eigen (CI) | **Bindung:** eigen
- **Trigger:** Attempt-2-Ergebnis von `36282378750`
- **Lage:** (gemessen 2026-09-27 via Browser/GH-API) Run `36282378750` **Attempt 2 failed**
  (01:03→01:05, exit 143) UND der Schedule-Lauf `36298095390` failed (05:45→05:47, exit 143)
  — 3× 143 in Folge: Muster, kein Einzel-Transient. Workflow `matrix-rotor.yml`:
  `cron "43 */6 * * *"`, Job-Timeout 350 min, Slice 5 h (`timeout … 18000`),
  State-Asset Release `matrix-state` (`omegaflow_matrix_state.bin`, Warm-Boot geladen,
  `--clobber` zurückgeschrieben).
- **Blockade:** keine.
- **Braucht:** Ursache des 143 klären (Slice/Hidden-Run/State-Write) **vor** weiterem Re-Run.

### probe-front-dark-matter — GPS-Timing / DSN-810-005 (Quellen-Gap)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27 via `sread`) `docs/paper/probe-front-dark-matter.md:333`
  „remain scan-missing (pending)" — die GPS-Timing-Artikel 1982–87 fehlen; `:363` benennt
  „the DSN hardware documentation (810-005, the MDA-resolver frequency) is the named next
  search". **Riss:** folge179 faltet den Doc als „gemessen geschlossen" (Pioneer/Dark-Matter,
  vision-read Appendix A); der `:333`-Marker bleibt ungeklärt offen — als Riss getragen,
  nicht geglättet.
- **Blockade:** keine.
- **Braucht:** `archive_search "GPS timing" --ntrs` (bzw. `--ads`);
  `archive_search --playwright https://deepspace.jpl.nasa.gov/dsndocs/810-005/`; je Fund
  eine `url`-Zeile in `phi/sources.φ`.

### survey-2026-09-06-codestruktur — Datenvertrag je Format-Modul
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27 via `sread`) `docs/surveys/survey-2026-09-06-codestruktur.md:111-113`
  „Datenvertrag je Format-Modul" (26×f64-Wire / GPU-Pack-Offsets gegen WGSL) trägt keinen
  Mess-Stempel; die CI-Zellen der Survey korrigierte mountain 177 (`d38fe7f7e`).
- **Blockade:** keine.
- **Braucht:** `ci_manage list --limit 40` am HEAD für den `ci-check`-Stand; das manuelle
  Verifikationsprotokoll aus `docs/concepts/archivar-mathematikerin.md` über die
  Format-Module fahren.

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
- `docs/concepts/archivar-mathematikerin.md` (1) → Archivar-Pending (Mountain-Linie).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (2) → River-Linie.
- `docs/surveys/survey-fortschritt.md` (1) → Träger Mycelium.
- `docs/concepts/the-seven-spheres.md` (2), `docs/concepts/recherche-extern-galileo-ruck-borduhr-modell.md` (1), `docs/paper/jwst-disequilibrium-survey.md` (7) → B1/B5/B8 (geschlossen in folge181; die Marker in den Docs sind nachzuziehen).
- **Neu gefaltet (gemessen 2026-09-27):**
  - `docs/paper/terminologie-der-gegenstroemung.md` (1) → **descoped**: der Marker `:22`
    ist der Definitions-Kopf des Terms „pending" (Inline-Code gestrippt), kein Todo.
  - `docs/paper/planet-nine-kbo-residue.md` (1) → **descoped**: `:66` ist eine Negation
    („not pending") — der Count ist erschöpfend.
  - `docs/paper/asmar-2005-spacecraft-doppler-tracking-noise-budget.md` (2) → **descoped**:
    Scanner-False-Positive (Substring „de-**pending**").
  - `docs/surveys/axiom-gate-broken-null-control.md` (1) → **descoped**: `:24` benennt nur
    die Pendings des Papers; Punkt geschlossen (te-gate success).
  - `docs/paper/probe-front-dark-matter.md` (2) → **carrier**: eigener Punkt oben.
  - `docs/surveys/survey-2026-09-06-codestruktur.md` (3) → **carrier**: eigener Punkt oben.
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

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
- `docs/paper/terminologie-der-gegenstroemung.md` (Header-sha `68c76aba…`),
- `docs/handover/handover-2026-09-27-sensory-folge183.md`,
- `docs/handover/archiv/handover-2026-09-27-sensory-folge182.md` (Move).

Der Zustand-Ledger `state/zustand/external-state.md` (CI-Status- + Postfach-Zeile)
ist gitignored (`.gitignore:137 /state/`) — lokal fortgeschrieben, nicht committet.

Der frühere `yu-tong`-Riss (title=94 > 75, River-Werk `314fea2b5`) ist im aktuellen
paper-check-Lauf nicht mehr rot (gemessen 2026-09-27: nur Gyirong) — kein offener Punkt.
