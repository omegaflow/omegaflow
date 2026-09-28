<!--
  title: Handover — Mycelium-Folge 195 (2026-09-28)
  session: Mycelium-Folge 195
  class: handover
  date: 2026-09-28
  sha256: 0fb59420a2dc87e90e6a839a51257caaa5cf279b1de4daf0ab512af7f664bc4c
  status: live
-->
# Handover — Mycelium-Folge 195 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`;
Operator-Akte leben in Futures Operator-Queue, Dritt-Waits in
`state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge194.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-28 | „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte … damit die erlaubte Form am Punkt der Handlung steht." | Quelle: Mycelium-Session 195.
- Wort | 2026-09-28 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher … Dispatch flash-first …" — session-weiter Consent (Delegation), **nicht** das Commit-Wort | Quelle: Mycelium-Session 195.

## Offen (aufgeschlüsselt)

### Register-Träger — Ledger-Ports (11) + index.φ-Kandidaten (10)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port-Pass — `phi/pipeline/ledger.φ` trägt 11 `ausstehend`, `phi/pipeline/index.φ` 10 `verifiziert` (owner mycelium).
- **Lage:** (gemessen 2026-09-28 via `sread` + `archive_search --verdict`) Ledger-`ausstehend`: die neun aus sensory-folge195 (Akatsuki RS, Hayabusa, Kaguya/SELENE LRS, Chandrayaan-1, Venera 15/16, Vega 1/2, Phobos 2 KRFM, ExoMars TGO ACS, Danuri/KPLO) **+ `pithia.cbk.waw.pl` (Z.10)** + **SSDC/limadou (Z.14, note 2026-09-26: SSDC-CAS-Umbau, „wait a few weeks")**. Vega 1/2 Halley (UDSSR) 7 Instrument-Sets + Ballons atmos.nmsu.edu/PDS/data/vega_5001/ (Z.51): Pfad korrigiert — alte Form 404, real `https://pds-smallbodies.astro.umd.edu/holdings/vega2-c_sw-mischa-3-rdr-original-v1.0/` (206). Parser-def-Fälle (→ `phi/blocked_sources.φ` gap-Arm): Akatsuki (PDS4-Binary), Chandrayaan (pds3-image/envi-cube), Phobos (unit-auto-detect/pds3-fixed-width). `index.φ` trägt 10 `verifiziert` (Z.37/39 richest/params; Z.75–97 oai_arxiv, b2find_intermagnet, grind_vires, grind_arcgis, terrapulse, esa_geomagnetic, archeology, copernicus) — teils Ports gemergt (grind_vires/arcgis), teils Live-Kandidaten offen (terrapulse 33, archeology 35).
- **Blockade:** keine.
- **Braucht:** Port über `docs/SOURCE_PORT.md` + Disposition (Register); Parser-Fälle als `gap` melden.

### register-coverage — mycelium Feder (Workflow-Verifikation)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain committet den Arm `UNVERIFIABLE_PRIVATE`.
- **Lage:** (gemessen 2026-09-28 via `git diff --stat` + `sread`) der Arm steht **fremd uncommittet** im Arbeitsbaum (`tools/register/src/bin/register_lookup.rs`, +52 Zeilen; Test `orphan_report_marks_absent_private_carrier_as_unverifiable`, Z.3787); CI `register-coverage` rot (`36417886014` 11:49) weil HEAD ihn nicht trägt. Nach dem Commit läuft `target/release/register_lookup --orphans --fail` (Workflow Z.23) grün.
- **Blockade:** Mountain-Commit (fremde uncommittete Arbeit — nicht angefasst).
- **Braucht:** nach dem Arm-Commit `.github/workflows/register-coverage.yml:23` + ein Lauf verifizieren.

### ci-check — clippy geheilt; Bestätigungslauf läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des lebenden `ci-check`-Laufs.
- **Lage:** (gemessen 2026-09-28 via `ci_manage status`) `36413456788` in_progress (test → `cargo test --release --features browser_relay`); der clippy-Rot `src/archivar/odf.rs:209` ist in `9f8debcc3` geheilt.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36413456788` — clippy/test grün.

### ci_watchdog — Matcher heilt echte Runner-Shutdowns
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster transienter Rot-Lauf, belegt in `ci_watchdog.log`.
- **Lage:** (gemessen 2026-09-28) `bin/ci_watchdog.sh` prüft transient zuerst; Assertion-Klasse auf echte Rot-Marker begrenzt; `bash -n` 0.
- **Blockade:** keine.
- **Braucht:** der nächste Shutdown-Rot trägt „rerun … measured transient cause".

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-27) `36344350143` rot, Job-Log `unread`; Vorlauf 8× `attempt stayed unready`, Auth 200.
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** `ci_manage jobs 36344350143` beim Trigger.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:288`).
- **Lage:** (gemessen 2026-09-27) kein Producer-Bin, keine Register-Zeile, kein `*-cdn.yml`; der generische Weg (`src/archivar/cdn.rs` `upload_release`) steht.
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

### Träger — Prosadokumente mit offenen Markern
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `register_lookup --orphan-docs`-Lauf meldet einen ORPHAN_DOC (0 gemessen 2026-09-28).
- **Lage:** (gemessen 2026-09-28 via `register_lookup --orphan-docs`) **0 orphan docs** — jeder offene Marker trägt einen Namenträger. Marker in `docs/concepts/arxiv-api.md` (2), `docs/concepts/exzellenz-konzept.md` (3), `docs/concepts/tools-map.md` (Session-Checks-Abschnitt), `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (Step 5 Familien-Identität), `…survey-2026-09-14-warteliste-offene-alternativen.md` (wartend Mail-Eingang), `…survey-2026-09-03-daten-holdings-inventur.md` (Layout-Wort), `…survey-2026-09-07-tmp-opencode-scan.md` (§7 Roh-Korpora), `…survey-2026-09-14-kapitulationen-pendings-inventur.md` (Wiedervorlage 2026-12-02), `…survey-2026-09-16-dead-sources-relevanz.md` (3 Force + 4 pending tot), `…survey-2026-09-20-browser-anbindung.md` (Fork-Build unpacked).
- **Blockade:** teils Operator-Wort/Trigger (siehe Weitergabe).
- **Braucht:** `register_lookup --orphan-docs` beim nächsten Pass; je Marker der nächste Schritt.

## Weitergabe (fremde Feder — Aufenthalt beim Eigentümer)

- **register-coverage-Arm** (gemessen 2026-09-28 via `git diff --stat`): der `UNVERIFIABLE_PRIVATE`-Arm steht uncommittet in `tools/register/src/bin/register_lookup.rs` (+52 Zeilen). Ziel: **Mountain** — committen, dann ist CI `register-coverage` grün. Quelle: Mycelium 195.
- **public_audit (Ganz-Baum, Credit/Egress)** (gemessen 2026-09-28): der scoped Lauf `docs/concepts/*` steht (1,38 Cr), der Ganz-Baum-Lauf ist ungemessen; er braucht das Operator-Wort (Credit-Deckel). Ziel: **Future** (Operator-Queue, eine Zeile in einfacher Sprache). Quelle: Mycelium 195.
- **dropped-gate-Baseline** (gemessen 2026-09-28 via `ci_triage 36409581203`): `dropped-gate: delta 6 > 0`; `--dropped --count` läuft in die >120-s-Last. Braucht das Operator-Wort zum `--dropped --count`-Baseline-Bump. Ziel: **Future** (Operator-Queue). Quelle: Stehender Pass 2026-09-28.
- **PII in der Git-Historie** (gemessen 2026-09-28 via `house_audit`): die private Adresse/Mail stand in getrackten Docs; HEAD ist redigiert, die Historie trägt sie weiter → GitHub-GC-Ticket #4761801. Ziel: **Future** (Operator-Akt History-Rewrite). Quelle: Mycelium 194.
- **Copilot-Streichung an die Linien**: `docs/handover/handover-2026-09-28-river-folge55.md` und `…mountain-folge194.md` nennen Copilot als Stimme; die Cloud-Grenze (private Daten nie) + public-CI-Ausnahme (`bin/ci_triage`) gilt für alle. Ziel: **river**, **mountain** (per Absender-Zeile). Quelle: Mycelium 194.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
