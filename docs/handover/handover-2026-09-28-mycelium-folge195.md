<!--
  title: Handover — Mycelium-Folge 195 (2026-09-28)
  session: Mycelium-Folge 195
  class: handover
  date: 2026-09-28
  sha256: 1711ed5d8cb18e1d90db815359024906f371be7481ace6ee15a6b9574baaa52c
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

### Register-Träger — index.φ-Kandidaten (8)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port-Pass — `phi/pipeline/index.φ` trägt 8 `verifiziert` (owner mycelium).
- **Lage:** (gemessen 2026-09-28 via `register_sort` + `sread`) der Ledger-Port ist **gebaut**: Venera 15/16 als zwei Akzepte in `phi/sources.φ` (Altimetrie `gravity` km / Radiometrie `thermal` K, je `no-cadence`; Register kanonisch über 1531 Blöcke); die 8 Sonden + pithia als `parser-def` in `phi/blocked_sources.φ` mit 5 neuen gap-Klassen (`pds3-fixed-width`, `pds3-img`, `pds3-binary`, `pds4-fixed-width`, `pds4-binary`); `phi/pipeline/ledger.φ` → `disponiert` (10 von 11; `limadou/SSDC` bleibt `ausstehend`, CAS-Login). `index.φ`: grind_vires + grind_arcgis → `erledigt` (Merges 2026-09-22), **8 offen**: `pipeline/queue/sources_potential_pre-cdn_9k_richest.φ`, `…_params.φ`, `pipeline/catalog/oai_arxiv.φ`, `pipeline/catalog/b2find_intermagnet_catalog.φ`, `pipeline/catalog/terrapulse_catalog.φ`, `pipeline/catalog/esa_geomagnetic_catalog.φ`, `pipeline/catalog/archeology_gaps_index.φ`, `pipeline/catalog/copernicus_catalog.φ`.
- **Blockade:** keine.
- **Braucht:** die 8 index-Inventare über `docs/SOURCE_PORT.md` portieren.

### Register-Riss — `parser-def`-Pen in `blocked_sources.φ`
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Ratifikation der neuen `parser-def`-Blöcke.
- **Lage:** (gemessen 2026-09-28) die Verfassung (2026-09-27) gibt die Verdikt-/Dispositions-Zeilen Mountain; der Port setzte 8 `parser-def`-Blöcke + 5 gap-Klassen-Deklarationen in `phi/blocked_sources.φ` (Mycelium), statt die Route zu verschleppen. Beide Enden genannt, kein stiller Schreibakt. Die 5 Reader-Arme fehlen.
- **Blockade:** Pen-Grenze Mountain/Mycelium (Riss).
- **Braucht:** Mountains Ratifikation; die 5 Reader-Arme (PDS3/PDS4/HTML) als `gap`-Arme.

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

- **5 parser-def-Reader-Arme** (gemessen 2026-09-28): `phi/blocked_sources.φ` trägt die neuen Klassen `pds3-fixed-width`/`pds3-img`/`pds3-binary`/`pds4-fixed-width`/`pds4-binary` für ExoMars, Akatsuki, Kaguya, Chandrayaan, Phobos, Vega, Hayabusa, Danuri — die Reader-Arme fehlen. Ziel: **Mountain** (`tools/harvest`/`src/archivar`). Quelle: Mycelium 195.
- **register-coverage-Arm** (gemessen 2026-09-28 via `git diff --stat`): der `UNVERIFIABLE_PRIVATE`-Arm steht uncommittet in `tools/register/src/bin/register_lookup.rs` (+52 Zeilen). Ziel: **Mountain** — committen, dann ist CI `register-coverage` grün. Quelle: Mycelium 195.
- **public_audit (Ganz-Baum, Credit/Egress)** (gemessen 2026-09-28): der scoped Lauf `docs/concepts/*` steht (1,38 Cr), der Ganz-Baum-Lauf ist ungemessen; er braucht das Operator-Wort (Credit-Deckel). Ziel: **Future** (Operator-Queue, eine Zeile in einfacher Sprache). Quelle: Mycelium 195.
- **dropped-gate-Baseline** (gemessen 2026-09-28 via `ci_triage 36409581203`): `dropped-gate: delta 6 > 0`; `--dropped --count` läuft in die >120-s-Last. Braucht das Operator-Wort zum `--dropped --count`-Baseline-Bump. Ziel: **Future** (Operator-Queue). Quelle: Stehender Pass 2026-09-28.
- **PII in der Git-Historie** (gemessen 2026-09-28 via `house_audit`): die private Adresse/Mail stand in getrackten Docs; HEAD ist redigiert, die Historie trägt sie weiter → GitHub-GC-Ticket #4761801. Ziel: **Future** (Operator-Akt History-Rewrite). Quelle: Mycelium 194.
- **Copilot-Streichung an die Linien**: `docs/handover/handover-2026-09-28-river-folge55.md` und `…mountain-folge194.md` nennen Copilot als Stimme; die Cloud-Grenze (private Daten nie) + public-CI-Ausnahme (`bin/ci_triage`) gilt für alle. Ziel: **river**, **mountain** (per Absender-Zeile). Quelle: Mycelium 194.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
