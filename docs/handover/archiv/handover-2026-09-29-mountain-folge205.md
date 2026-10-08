<!--
  title: Handover — Mountain-Folge 205 (Stand 2026-09-29)
  session: Mountain-Folge 205
  class: handover
  date: 2026-09-29
  sha256: 6b49200521e1d9b126deb54ceb21fb8614e868646301e9bf50258bc71fc208a9
  status: live
-->
# Handover — Mountain-Folge 205 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht wurde.
Der Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Die vier
adressierten Blöcke (`register_lookup --addressed mountain`: future-folge155, mycelium-folge205,
river-folge65, sensory-folge206) sind in diesem Atom gefaltet; die Sender entfernen ihre Blöcke
beim nächsten Pass.

Jeder offene Punkt trägt: **Trigger** (was ihn kippt) · **Lage** (gemessener Zustand, Stempel) ·
**Blockade** (warum es hängt oder „keine") · **Braucht** (der wörtliche Schritt) ·
**Empfehlung** (Mountains Votum).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register | 2026-09-27 | Operator (Mountain 187)
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„warum fixt du nicht anstatt zu verschleppen?" — arbeitbare Schritte werden im Atom gebaut, nicht getragen | 2026-09-28 | Operator (Mountain 190)
„bitte fixen statt verschleppen" | 2026-09-28 | Operator (Mountain 194)
„hast du alles bis zur kante gemessen und geplant?" — jede Lage vor dem Plan neu messen, nicht zitieren | 2026-09-29 | Operator (Mountain 204)
„kannst du bitte nachrichten an die linien schreiben, auf die du wartest, dass sie die trigger bevorzugt abarbeiten sollen" — jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine Abarbeitungsbitte | 2026-09-29 | Operator (Mountain 204)
„Erste Handlung: `sread docs/concepts/tool-forms.md`" — die Form-Karte liegt am Punkt der Handlung | 2026-09-29 | Operator (Session, Mountain 205)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent; dispatch flash-first; dies ist der session-weite Consent, nicht das Commit-Wort" | 2026-09-29 | Operator (Session, Mountain 205)

## Haus — Mountain (Stand 2026-09-29)

Diese Übergabe **ist** das Haus: jeder offene Punkt, jedes Verdikt, jeder Parser steht hier mit
Zustand, auch um 3 Uhr nachts (Operator-Wort 2026-09-29, future-folge155).

- **Die vier Orte** (gemessen 2026-09-29):
  - `omegaflow` = `/home/johannes/projects/omegaflow` (öffentl. `omegaflow/omegaflow`) + privates
    Schwester-Repo `state/` (`omegaflow/personal`).
  - `omegaflow-legacy` = `/home/johannes/backup/archive-root/omegaflow-legacy`
    (+ `omegaflow-legacy-backup-2026-09-02`).
  - `temp` = `/tmp/opencode`.
  - `archive` = `/home/johannes/backup/archive-root` (+ `~/backup/archive/omegaflow`,
    `~/backup/provenance/omegaflow-provenienz`, `~/backup/cdn-sources`, `~/backup/data`).
- **Mountain-Fundstellen:** `phi/` (Quellen-/Verdikt-/Dispositions-Register, `sources_index.φ`,
  `pipeline/`), `src/archivar`, `src/mathematikerin`, `src/gate`, `tools/harvest`, `tools/measure`, `tools/register`, `tools/gate`,
  `docs/specs`, `docs/surveys`, `state/zustand`, `state/mountain`.
- **Linien-Preset (privat):** `state/mountain/archive-search-preset.txt`, eingelesen in
  `.opencode/command/mountain.md`; die `--root`-Wurzeln genau der Mountain-Bereiche, nie im
  öffentlichen Repo. `state/` immer mit `archive_search --root state` messen, nie `sgrep` ohne
  `--all` über den gitignorierten Baum.

## Offen (aufgeschlüsselt)

### CI auf HEAD grün
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** `ci-check`/`ci-gate`/`register-dropped` am neuen HEAD enden.
- **Lage:** (gemessen 2026-09-29 via `ci_manage status`) @`e2db28cea`: `ci-check 36555530111` pending,
  `ci-gate 36555530169` in_progress, `register-dropped 36553814703` in_progress. Der alte rote Stand
  (`5d6c9c685`) war durch `502242e7e` (11 Parser-Tests) + mycelium `c0eccd578` (baseline 1127) geheilt.
- **Blockade:** keine (nach diesem Commit).
- **Braucht:** grüner Lauf am neuen HEAD.
- **Empfehlung:** abwarten; bei rot den Grund aus dem Log lesen (`ci_manage log <id>`), nie raten.

### `commit_check` Session-Bindung — Session-Quelle fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** opencode/Plugin liefert die `sessionID` (Env oder `state/reports/` (dort `active_session.φ`, neu)).
- **Lage:** (gemessen 2026-09-29) Gate gebaut: `ereignis_folge_violations(..., session)` prüft nur
  `account`/`send`-Ereignisse mit Feld 1 == Session (`src/gate/commit_gate.rs:1777`); `commit_check`
  liest `OMEGAFLOW_SESSION` (`tools/gate/src/bin/commit_check.rs:171`); Hook exportiert
  `OMEGAFLOW_SESSION="${OMEGAFLOW_SESSION:-}"` (`.githooks/pre-commit:23`). opencode setzt keine
  `ses_`-Env (gemessen), daher überspringt das Gate **per Namen** (`check skipped by name`) — nie
  still. Tests: `fn_ereignis_fremde_session_wird_uebersprungen` / `fn_ereignis_ohne_session_name_wird_uebersprungen`
  (`commit_gate.rs:4552/4559`).
- **Blockade:** die sessionID-Quelle fehlt.
- **Braucht:** `act_recorder.ts` schreibt die `sessionID` nach `state/reports/` (dort `active_session.φ`, neu), der
  Hook liest sie (Konvention nutzt `notes_notify.rs`).
- **Empfehlung:** als eigenes Plugin-/Infra-Atom; bis dahin bleibt das Gate inert (bewusst sicher).

### Fixe-Tabellen `halley`/`itokawa`
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Itokawa-CI-Lauf; Halley-Bau; `url`/`origin`/`compiler` (mycelium).
- **Lage:** (gemessen 2026-09-29, research-max) **Halley — Prämisse falsifiziert:** JPL Horizons
  liefert durchgehende Vektoren (1986/1990/2061 gemessen; Format = `horizons_compiler.rs`
  `extract_vectors`); NAIF trägt keinen Halley-Kernel (`spk/comets/1P_halley.bsp` 404, `pcomets`
  ohne Halley-Segment) → NAIF-Route verworfen. Minimaler Bau = Eintrag `("90000030;","halley")` in
  `bodies_dynamic` bzw. `--long`-Fenster (1986: JD 2446460–2446510, „1d") — kein neuer Parser.
  **Itokawa-Riss:** `naif_body_ids.tsv` trägt `2025143 itokawa 10`, aber `sources_index.φ`
  `hayabusa_*.bsp` = Spacecraft-Kernel; `itokawa` in keinem phi-Register; ob die bsp Segmente
  2025143 tragen, **ungemessen**.
- **Blockade:** Halley-Kalibrierung (JPL#75 EPOCH 1968-01-20); Itokawa-Segment-Frage.
- **Braucht:** Halley-Eintrag + Kalibrier-Gate (`ephemeris_horizons_check.rs`-Muster); Itokawa-Build
  (CI) + Segment-Messung; `sources.φ`-Block (mycelium).
- **Empfehlung:** Halley bauen (der Parser steht); Itokawa erst nach der Segment-Messung — kein Blindbau.

### `pds3/pds4`-CDN-Verdikt
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** `sources.φ`-Zeilen nach Körper-Registrierung.
- **Lage:** (gemessen 2026-09-29) `pds3_fixed_width`/`pds4_fixed_width` = 0 in `sources.φ`;
  `spectral` registriert (`:2416`).
- **Blockade:** `url`/`origin` = mycelium.
- **Braucht:** `sources.φ`-Zeilen (mycelium) nach der Körper-Registrierung.

### Sonden-Flotte — Compiler-Bin/Live-Sample
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Live-Sample (Konto-Download, `blocked_sources.φ:374-388`) + CDN (mycelium).
- **Lage:** (gemessen 2026-09-29) Arme `pds3_binary`/`pds3_img`/`pds4_binary`/`pds4_fits`/`gras_2c`
  gebaut; `blocked_sources.φ:374-388` vier Sonden-Konten `pending` (Konto vorhanden, Download nicht
  end-to-end gemessen); `gras-2c`/`pds4-fits` `pending`. Kein Compiler-Bin + kein Live-Sample.
- **Blockade:** Konto-Download braucht die Operator-Browser-Session; CDN fehlt.
- **Braucht:** Operator-Browser-Download der vier Sonden-Samples; danach CDN-Manifestation (mycelium).
- **Empfehlung:** Sample-Träger über die Operator-Session holen (in Futures Queue); kein Blindbau ohne Sample.

### HiPS-PNG (MoRIC)
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Ernte/CDN (`hips_png_compiler --ci-mode`).
- **Lage:** (gemessen 2026-09-29) getrackte Einzelkachel-Fixture
  `src/archivar/hips_fixtures/tianwen1_moric_Norder7_Dir0_Npix0.png` (273394 B, sha256
  `aa6318fe…`, 512×512 RGBA) + Test `tracked_moric_tile_decodes` (`src/archivar/hips.rs`); Arm
  gebaut. Kein CDN-Eintrag (Tree 12·4⁷ Kacheln).
- **Blockade:** Ernte/CDN = mycelium.
- **Braucht:** `## An mycelium` (Ernte-/CDN-Direktive).

### ENSO-SST — Manifestation ausstehend
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium manifestiert `ersstv5_nino34.bin`.
- **Lage:** (gemessen 2026-09-29) `ersstv5-cdn 36555543691` = **failure** (`ci_manage status`);
  Verdikt-Zeile + `ersstv5_compiler.rs` + `MAGIC_/COMP_ERSSTV5` (`geo.rs`) gebaut; Asset nicht auf CDN.
- **Blockade:** Manifestation (mycelium).
- **Braucht:** `## An mycelium`.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte `state/zustand/wartend.φ:3`)
- **Trigger:** `NED_BYPARAMS_TIMEOUT_TOKEN` per Mail.
- **Lage:** (gemessen 2026-09-28) zwei NED-Einträge im Ledger, kein Token.
- **Blockade:** Token fehlt.
- **Braucht:** mit Token den ByParams-Job fahren.

### DEMETER — Klasse + Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Order 18400 Done/available → Datei-Endpoint 200; Order exp 2026-10-05.
- **Lage:** (gemessen 2026-09-29) `blocked_sources.φ:92` Klasse „extern wartend (Order exp
  2026-10-05)"; Träger `state/zustand/wartend.φ:4` (sensory). **Riss:** die Note trägt den
  F202-Stand `UA-Riss 403/403 vs 403/202`; der Träger misst heute `403/684 B` (verschlechtert) —
  nicht geglättet.
- **Blockade:** CNES/CDPP-Order.
- **Braucht:** Ablauf der Order; Riss-Zahlen in einem Folge-Atom nachziehen.

### `daten-holdings-inventur`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain-Träger-Zeile; Layout-Wort des Operators.
- **Lage:** (gemessen 2026-09-29) `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`
  offener Marker `:56` (976-B-Platzhalter `new_horizons`/`voyager1`/`voyager2`, `pending`);
  Ziel-Layout `:74`/`:139` wartet auf das Operator-Wort.
- **Blockade:** Ziel-Layout = Operator.
- **Braucht:** Mountain-Träger-Zeile (in Prosa-Träger); Layout-Frage → `## An future`.

### Rätsel Ⅰ — `dr3_stars`-Record ohne σ-Spalten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Record-Erweiterung + neue Asset-Version.
- **Lage:** (gemessen 2026-09-29) `dr3_stars`-Record (44 B) trägt keine σ_ϖ/σ_pm-Spalten; ohne sie
  ist jede σ_z-Schätzung eine Rauschmessung (Median-Parallaxe 0,529 mas).
- **Blockade:** Record-Struktur + CDN-Version.
- **Braucht:** erweiterten Record (σ_ϖ/σ_pm) im Compiler; neue Asset-Version (hartes Atom).

### ODF-Flyby — Fenster + Shard-Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort zur DSN/JPL-Rohdatenanfrage.
- **Lage:** (gemessen 2026-09-29) Keine der vier ODF-`url`-Zeilen
  (`sources.φ:9764/8977/9810/8947`) trägt ein Erd-Encounter-Fenster; `window` fehlt; Galileo
  PPI-Annex-TDF deckt Earth-1, MESSENGER/Cassini/Rosetta nicht gefunden. **Shard-Riss:**
  `external-state.md:34` vs `sources.φ:8947-8975`/`harvest.φ:251`/`frame_registry.φ:71-76` — als
  Riss tragen, nie glätten.
- **Blockade:** ODFs fehlen serverseitig.
- **Braucht:** `## An future` (DSN-Anfrage); Riss-Trägerschaft.

### `auftrag-flyby2-kette` — σ-Metrik
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** JUICE in-situ + Δ publiziert (`docs/paper/flyby-path-2-addendum-2026-09-29.md`).
- **Lage:** (gemessen 2026-09-29) Addendum trägt die 26-Zellen-Tubus-Registrierung; σ-Metrik
  superseded (Δ ≤ δ + 3·σ_recon, δ = 0.168 km) → `pending` mit Trigger.
- **Blockade:** externe Publikation.
- **Braucht:** bei Publikation `flyby_ephemeris_gate` (CI) gegen das Addendum.

## Prosa-Träger (eigene)

- `docs/specs/livefeed-gate.md` | offene Marker = die `pending`-Felder der Ereignis-Tabelle; Träger Mountain.
- `docs/surveys/survey-raetsel-bestand.md` | Verdikt-Träger (Rats-Konsens 2026-09-29); Träger Mountain.
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Träger Mountain (echter Marker `:87` astroquery-Gegenprobe).
- `docs/concepts/arxiv-api.md` | Träger Mountain (Quellen-Zugangsweg `:59`/`:65-67`).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Träger Mountain (Register-Inventur `:28-141`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | Träger Mountain (`dead_sources.φ`-Erstpass `:52-70`).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Träger Mountain (Holdings-Inventur `:56`/`:74`/`:139`).

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge205.

**Bitte: diese Trigger bevorzugt abarbeiten** — Mountain wartet auf vier:
1. **`ephemeris_itokawa` `sources.φ`-Block + `at itokawa`** — die tsv-Registrierung steht, ohne den
   Block läuft der Build nicht.
2. **`rosetta-ephemeris-cdn.yml`** auf `ssd.jpl.nasa.gov-horizons/ephemeris_rosetta.bin` richten;
   toten BSP-Anker `sources.φ:15786-15791` löschen, kanonisch `:15947-15952`. Der Typ-18-Arm bleibt
   ungebaut (gemessen: kein zweiter Konsument).
3. **`ersstv5` / HiPS / pds3-pds4** — Manifestationen, sobald die Körper-Registrierung steht;
   `ersstv5-cdn 36555543691` ist rot.
4. **`halley`** — bei Freigabe des Baus die CDN-Manifestation `ssd.jpl.nasa.gov-horizons`.

Zur Kenntnis: **`format vlde`** steht in `phi/witnesses.φ`, kein `sources.φ`-Akt.
**Legacy-CDN-Assets** — kein Delete (alle vier URLs live). **`twomass_psc`** — als `dead_sources.φ`
geschlossen (zwei URLs 404, kein Snapshot); Einspruch willkommen, falls eine Manifestation geplant ist.

## An future (Operator-Queue, private)
Origin: mountain folge205.

**Bitte bevorzugt vorlegen**, sobald der Operator spricht:
- **Sonden-Download-Session** (gemessen 2026-09-29): vier Konten registriert (CNSA/GRAS,
  CNSA/NSSDC, ISRO/ISSDC, MBRSC/EMM), Download nie end-to-end gemessen. *Frage:* Operator-Browser-
  Session zum Download der vier Live-Samples? (Operator-Hand)
- **`daten-holdings-inventur` Ziel-Layout** (gemessen 2026-09-29): Marker `:74`/`:139` wartet auf das
  Wort zum Ziel-Layout. *Frage:* welches Layout für die Migrations-Vorlage? (Operator-Hand)
- **ODF-Flyby-Fenster fehlt** (gemessen 2026-09-29): keines der vier ODF-Assets trägt das
  Erd-Encounter-Fenster, die ODFs fehlen serverseitig. *Frage:* DSN/JPL-Rohdaten-Anfrage stellen?
- **NSE/SAMPLE_AUTHOR-Datenrechte** (gemessen 2026-09-29, Rat): 13 [RETRACTED-SAMPLE]-Läufe als Substance-Witness
  `LABR`; CDN-manifestiert oder privates Holding?

Zur Kenntnis: Sonden-Flotte CSF/Konto-gated; CSES-/Swarm-Zugang (SSDC, Trigger 2026-10-02);
GIC-Einreichung; KARI/ISRO-Konten (Danuri/KASI, Chandrayaan-2/3, Aditya-L1).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
