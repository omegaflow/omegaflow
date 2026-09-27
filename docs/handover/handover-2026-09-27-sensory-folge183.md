<!--
  title: Handover — Sensory-Folge 183 (2026-09-27)
  session: Sensory-Folge 183
  class: handover
  date: 2026-09-27
  sha256: 4eee62a827f036da60c09bc6ced7ce9687ed30a3ff2d857ba118f7a07db714f8
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

### #5 Seismik — IGETS (Ernte dispatcht)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Lauf `36284373510` endet
- **Lage:** (gemessen 2026-09-27) Workflow `igets-cdn.yml` (getrackt) dispatcht →
  Lauf `36284373510` **queued** (2026-09-27T01:03:32Z); ein früherer Lauf
  `36282004600` war success @`8f0251ab`, nicht am HEAD. Lokaler Compiler targetiert
  gebaut, SFTP-Zugang grün: `--list-stations` = **47 Level2-Träger** (Brussels, Conrad,
  … Syowa). Die in folge182 genannten **56** sind die rohen SFTP-Verzeichnisse — die
  Differenz ist der Level2-Filter (`igets_compiler.rs:414-420`); `--level` ∈ {1,2,3},
  Default = kein Filter (alle Level).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36284373510`; bei success sha256 in den `igets`-Block
  `phi/sources.φ` (Zeile driften — Zeilennummer nicht eintragen).

### paper-check — terminologie-Header-sha (behoben; CI-Verifikation offen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächster paper-check-Lauf
- **Lage:** (gemessen 2026-09-27 via `export_latex --check`) `docs/paper/terminologie-der-gegenstroemung.md`
  Header-sha war veraltet — Body in `b30323daa` nach der Header-Setzung `b294ba4c5`
  geändert, ohne Nachziehen. Korrigiert: Header Zeile 5 `c4a98130…` → **`68c76aba…`**
  (`omega_sh sha`). Lokal `export_latex --check <datei>` grün.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <run>` am nächsten paper-check-Lauf; grün = geschlossen.

### #6 Blatt-1-Bojen-Matrix — CI-Rotor (Rerun dispatcht)
- **Status:** eigen (CI) | **Bindung:** eigen
- **Trigger:** Attempt-2-Ergebnis von `36282378750`
- **Lage:** (gemessen 2026-09-27) Run `36282378750` Attempt 1 rot = **exit 143
  Runner-Shutdown** (transient, kein Assertion-Rot) → Rerun, **Attempt 2 queued**
  (2026-09-27T01:03:31Z). Workflow `matrix-rotor.yml`: `cron "43 */6 * * *"`, Job-Timeout
  350 min, Slice 5 h (`timeout … 18000`), State-Asset Release `matrix-state`
  (`omegaflow_matrix_state.bin`, Warm-Boot geladen, `--clobber` zurückgeschrieben).
  Der lokale Rotor läuft nicht (kein `omegaflow`-Prozess); `data/omegaflow_matrix_state.bin`
  355 460 B (mtime 2026-09-26 23:33).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36282378750` (Attempt 2). Lokaler Rotor bleibt
  Operator-Hand (`setsid ./bin/matrix_watchdog.sh &`); beide Betriebsarten dürfen den
  State nicht zugleich schreiben.

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

## operator-gebunden (nur der Akt; Vorbereitung an der Kante)

- **RR-Kanal / Beat-to-beat** | operator | Trigger: Förderung gewährt | Lage: BLE-HR ohne
  RR; FIT `nn=0` | Braucht: Brustgurt (Polar H10/HRM-Dual) → `perm_tone_probe`.
- **Weberin-Quellen HAWC** | operator | Trigger: `OMEGAFLOW_CA_BUNDLE` gesetzt |
  Braucht: HAWC-Fetch mit lokalem 4-Zert-Bundle; CI-Secret separat.
- **Beat-Arbitrierung** | operator | Trigger: zweiter Beat-Kanal (Gurt) | Braucht:
  `OMEGAFLOW_HIDDEN=1`-Lauf → genau eine `beat source:`-Zeile.
- **Live-Sensor-Cluster** | LOCK | Trigger: Operator-Wort hebt LOCK | Braucht: BOM
  bestellen (inkl. H2).
- **ESP32-Puls-Knoten** | LOCK | Trigger: Operator-Wort hebt LOCK | Wort: ESP32 separat
  als eigener LOCK | 2026-09-25 | Operator (Session).

## Termin

- **JUICE-Erdpassage 28./29.09.2026** | termin:2026-09-29 | Trigger: 28./29.09. | Lage
  (gemessen 2026-09-27): die Kp-Route ist in `phi/sources.φ` **wiederhergestellt**
  (folge182) — der frühere Riss ist aufgelöst | Braucht: Epoche ernten.
- **Gaia DR4 + Europa-Clipper** | termin:2026-12-02 | Trigger: 02./03.12. | Braucht:
  Epoche ernten.

## Wartend / blockiert (mit Trigger)

- **#3 O1 DEMETER/CDPP** | wartend | dritter | Trigger: CDPP-Antwort auf die Mail
  (2026-09-27) | Lage: Mail in Flug (`code@omegaflow.space` → `cdpp@cnes.fr`, DEMETER
  Order 18387); metalink gesichert (`data/cdpp-archive.cnes.fr/metalink_18387.xml`,
  97 078 URLs); die File-URLs weist der F5-WAF ab | Blockade: Quellenseite | Braucht:
  CDPP-Kontakt (ein neuer Order wiederholt nur den fehlschlagenden Lauf).
- **M2c — Rust-ZNSP-Host BL808/H2** | wartend | dritter | Trigger: Ox64 angekommen
  (`LZ473049629CN`) | Blockade: Carrier.
- **Das eine Instrument — Anomalie** | blockiert | dritter | Trigger: zweiter Messkanal
  (VLBI-Beacon) | Blockade: kein Instrument misst den vollen Pionier-Phasenraum.
- **LAIC/Causal-Arrow (CSES)** | wartend | dritter | Trigger: 2026-10-02 | Lage:
  ASI-SSDC-Weg gemessen, PI-Berechtigung offen.

## Extern (Dritter)

- **Postfach** | wartend | Trigger: neuer Eingang | Lage: `state/mail/mail_ledger.φ`
  **169 Zeilen** (gemessen 2026-09-27, `wc`); jüngster Eingang = DEMETER-Send in Flug,
  davor 5× Meta-Login-Codes + OpenAlex magic-link (2026-09-26) | Braucht: `smail_recv`/Ledger
  bei Trigger.
- **Ox64-Lieferung** | termin (Carrier) | Trigger: Ankunft (`LZ473049629CN`) | Braucht:
  quittieren → M2c.
- **BGR-Matched-Filter (Tonga)** | dritter (vDEC) | Trigger: vDEC-Zugang | Lage: vDEC
  descoped → BGR-Weg neu bewerten | Braucht: Arrival messen.
- **DSN-Briefe in Flug** | dritter | Trigger: DSN-Antwort | Braucht: quittieren.

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

Fremd (nicht committen, nicht überschreiben): `docs/auftrag/auftrag-flyby2-kette.md`
(uncommittete River-Arbeit), `docs/handover/handover-2026-09-27-river-folge41.md`
(Träger-Eintrag des yu-tong-Punkts, s. u.), `bin/relay-tls-1620.stunnel.conf`.

Der `paper-check`-Riss `yu-tong-fang-hu-2022-…` (title=94 > 75) ist **River-Werk**
(Commit `314fea2b5`); als Punkt in River-Folge 41 getragen, nicht in Sensory gefixt.
