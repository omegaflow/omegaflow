<!--
  title: Handover — Sensory-Folge 182 (Stand 2026-09-27)
  session: Sensory-Folge 182
  class: handover
  date: 2026-09-27
  sha256: 4eeaf57a472806778068ff456b94d33bc4a6921dc494cbe146933b1bffe73579
  status: live
-->
# Handover — Sensory-Folge 182 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht erklärt; git trägt,
was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks —
committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie überschrieben;
gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Sortierung: **umsetzbar zuerst**; der Akteur
steht pro Punkt in `Bindung`.

Die [redacted] und alle Operator-Daten bleiben lokal (`.secrets.local`, `data/`), nie
getrackt, nie am CDN.

## Abarbeitbar (eigen, dispatchbar)

### #5 Seismik — Gravimeter (der SFTP-Zugang ist da)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27) `depth-phase-fleet`/`depth-phase`/`cmt-ndk-fleet`
  success; Dual-Phase-Fit mit sP-Gate (0.78): mean offset **−8,2 km** (5 Events, se 8,8),
  weighted joint −18,4 km (`depth-phase-echo-fleet.md` §Fleet). Der IGETS-SFTP-Zugang
  **funktioniert** (gemessen 2026-09-27): `IGETS_USER`/`IGETS_PASS` stehen in
  `.secrets.local`, ein curl-Login gegen `sftp://igetsftp.gfz.de/` listet **56
  Stationen** (Brussels, Conrad, Ishigakijima, …). Quelle+Compiler sind registriert
  (`phi/sources.φ:8349` `igets` + `igets_compiler.rs`, Schalter `--list-stations`/
  `--list`/`--station`/`--level`). Die frühere Handover-Zeile „Zugang fehlt" war falsch.
- **Blockade:** keine.
- **Braucht:** `gh workflow run igets-cdn.yml` (CI-Harvest → CDN); lokal
  `cargo run -p omegaflow-harvest --bin igets_compiler -- --station <name> --level Level2`.

### #6 Blatt-1-Bojen-Matrix — Σ p̂·M-Zeile
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Rotor-Neustart `setsid ./bin/matrix_watchdog.sh &`
- **Lage:** (gemessen 2026-09-27) der Rotor ist die **Hidden-Membrane**
  `OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow "#station=41001"`, am Leben gehalten von
  `bin/matrix_watchdog.sh`; das Warm-Boot-Gedächtnis ist `data/omegaflow_matrix_state.bin`
  (355 460 B, mtime **2026-09-26 23:33**), die Linie landet in
  `/tmp/opencode/omegaflow_matrix.log`. Konstanten/State-Pfad:
  `src/mathematikerin/machines/matrix.rs` (`MATRIX_STATE_FILE`,
  `MATRIX_RING_MAX=1024`, `MATRIX_N_GATE=30`). Der Rotor **läuft nicht** (kein
  `omegaflow`-Prozess, kein Log). Ein voller Round ≈ 16 h.
- **Blockade:** der Session-bash reapet den detached Prozess — ein Start aus der Session
  stirbt mit dem Tool-Aufruf; der dauerhafte Start ist Operator-Hand (der Watchdog-Header:
  „Start (überlebt opencode-Abstürze): `setsid ./bin/matrix_watchdog.sh &`").
- **Braucht:** Operator startet `setsid ./bin/matrix_watchdog.sh &`; nach ~1 Round die
  Σ-Zeile aus `/tmp/opencode/omegaflow_matrix.log` + `data/omegaflow_matrix_state.bin` lesen.
- **CI-Pfad (gebaut 2026-09-27):** `.github/workflows/matrix-rotor.yml` — `schedule` alle 6 h +
  `workflow_dispatch`; installiert `mesa-vulkan-drivers` (lavapipe), baut `cargo build --release
  --bin omegaflow`, fährt `OMEGAFLOW_STATE=data OMEGAFLOW_HIDDEN=1 timeout … ./target/release/omegaflow
  "#station=41001"` für einen begrenzten Slice (5 h) und persistiert `data/omegaflow_matrix_state.bin`
  auf das Release **`matrix-state`** (`--repo omegaflow/omegaflow`), das der nächste Lauf lädt.
  Die GPU war nie das Hindernis (`omega.rs:1202` `compatible_surface: None`, `LoopRadiator` reines
  Compute, kein Display); das Bau-Stück war der **warme Speicher**: `data/omegaflow_matrix_state.bin`
  ist gitignored (`.gitignore:35`), ein Job ist ephemer → Kaltstart, deshalb das State-Asset.
  Wirksam ab Push auf den Default-Branch (`workflow_dispatch` verlangt ihn); Dispatch
  `gh workflow run matrix-rotor.yml`. Der lokale `bin/matrix_watchdog.sh` bleibt die zweite
  Betriebsart — beide dürfen nicht denselben State zugleich schreiben.

### #3 O1 DEMETER/CDPP — Quelle scheitert (Riss)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** CDPP-Antwort auf die Mail (2026-09-27)
- **Lage:** (gemessen 2026-09-27) metalink gesichert
  (`data/cdpp-archive.cnes.fr/metalink_18387.xml`, 97 078 URLs). Die File-URLs weist der
  F5-WAF ab (curl/sfetch/Proton, voller Browser-Header-Satz und Browser-Navigation selbst
  → `Request Rejected`). Die SPA zeigt fünf DEMETER-Orders, alle `Failed`/`Expired`; der
  große Order `DONE_WITH_WARNING`, `availableFilesCount 0`, `filesInErrorCount 96978`.
- **Blockade:** Quellenseite (CDPP-DEMETER-Pipeline).
- **Braucht:** CDPP-Kontakt; ein neuer Order wiederholt nur den fehlschlagenden Lauf.

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
  (dieses Atom) — der frühere Riss (Entfernung in `61e272ab0`) ist aufgelöst | Braucht:
  Epoche ernten.
- **Gaia DR4 + Europa-Clipper** | termin:2026-12-02 | Trigger: 02./03.12. | Braucht:
  Epoche ernten.

## Wartend / blockiert (mit Trigger)

- **M2c — Rust-ZNSP-Host BL808/H2** | wartend | Trigger: Ox64 angekommen
  (`LZ473049629CN`) | Blockade: Carrier.
- **Das eine Instrument — Anomalie** | blockiert | Trigger: zweiter Messkanal
  (VLBI-Beacon) | Blockade: kein Instrument misst den vollen Pionier-Phasenraum.
- **LAIC/Causal-Arrow (CSES)** | wartend | Trigger: 2026-10-02 | Lage: ASI-SSDC-Weg
  gemessen, PI-Berechtigung offen.

## Extern (Dritter)

- **Postfach** | wartend | Trigger: neuer Eingang | Braucht: `smail_recv`/Ledger bei
  Trigger (`state/mail/mail_ledger.φ`, 301 Zeilen).
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
- `register_lookup --orphan-docs` nennt 23 trägerlose Dokumente; die verbleibenden sind teils fremder Linien (Webersin-Surveys, `positive-maske`, `zeugnis`, `fremde-parser-sammlungen`) — je Linie zu falten.

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
- **Wort:** „kümmer dich drum" (Sensory-Folge 182: alle eigenen offenen Punkte abarbeiten) | 2026-09-27 | Operator (Session).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
- `src/archivar/skydirection.rs` (rustfmt),
- `tools/harvest/src/bin/s1_sar_compiler.rs` (rustfmt),
- `phi/sources.φ` (Kp-Route wiederhergestellt),
- `docs/paper/sturzflut-tibet-pfeil.md` (SAR-Differenz §3.6),
- `docs/blatt/blatt-kreuz-screening-gyirong.md` (räumliche Kopplung gemessen),
- `docs/concepts/die-akteure-im-boden-und-wasser.md` (Gravimeter-Route benannt),
- `.github/workflows/matrix-rotor.yml` (neu, Matrix-Rotor in CI),
- `docs/handover/handover-2026-09-27-sensory-folge182.md`,
- `docs/handover/archiv/handover-2026-09-27-sensory-folge181.md` (Move).

Der Zählzeit-Doc + NTRS-Text (`docs/concepts/recherche-galileo-kadenz-reconciliation.md`,
`docs/reference/19930010226.txt`) wurden vor dem Abschluss von Mountain 178 (`8f0251ab6`,
„resolve the 60 s count-interval fact") in HEAD committet — nicht erneut committen.
