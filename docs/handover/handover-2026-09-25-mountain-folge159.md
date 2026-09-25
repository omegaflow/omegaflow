<!--
  title: Handover — Mountain-Folge 159 (2026-09-25)
  session: Mountain-Folge 159
  class: handover
  date: 2026-09-25
  sha256: f2ad75b54d4c568a925fd5b9b10387343e0d72b9f2365654402ef03f42e21a1e
  status: live
-->
# Handover — Mountain-Folge 159 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Geschichts-Abschnitte. Geteilter externer Zustand lebt in
`docs/zustand/external-state.md`, nie als Kopie hier. Keine Rangfolge — die offenen
Punkte werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Sortierung von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit.

Diese Session konsumierte `docs/handover/handover-2026-09-25-mountain-folge158.md`
(zum Session-Beginn noch in `docs/handover/`).

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### Port der 189 `gap`-Quellen nach `phi/sources.φ` (M1, geteilt)
- **Status:** autonom | **Bindung:** linie:mycelium
- **Trigger:** Mountain-Dispositions-Verdikt (steht)
- **Lage:** (gemessen 2026-09-25) die 5 Parser-Arme stehen; Klassen-Counts:
  `unit-auto-detect` ×167, `force-undetermined` ×16, `konverter` ×4,
  `votable-reader` ×1 (ALMA), `html-parser-arm` ×1 (AEC) = 189. Unit-Feld-Verdikte
  gefällt: 27 Felder (`port.rs`/`units.rs`), 9 DROP (Metadatum/String-Enum),
  2 außerhalb der ~30 (`declination_deg`, `sz_mass_10e14_msun`, Quelle noch nicht
  portiert). `parser-def`+`gap`-Direktive = Mountain (Verdikt steht); die `url`-Zeile
  = mycelium (Port nach `phi/sources.φ` + CDN).
- **Blockade:** keine.
- **Braucht:** Port je Klassen-Träger `phi/blocked_sources.φ::gap:<token> ×N` nach
  `phi/sources.φ`; die `gap`-Direktive fällt erst mit dem Port.

### health-check void-/empty-Klasse (GitHub-Issues ungelesen)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via Browser) 7 offene `health`-Issues
  (#15/#16/#17/#45/#51/#54/#55), teils Duplikate: `cargo test returned void`,
  `cargo clippy returned void`, `dropped-gate returned void`,
  `recheck-live carries drift/broken findings`. Der `reverify`-Job ist grün, aber
  `reverify.txt` (Run #113) trägt 141 rechecks: 78 drift-void, 35 quiet-void,
  7 key-void, 21 refused, **0 broken** — der Parser liefert für praktisch jede
  regeprüfte Quelle nichts. Die CI filed GitHub-Issues, die kein Session-Lesepfad
  (`register_lookup`/`git_safety`/Handover) liest → stiller Drop. `probe-full` ist
  im schedule skipped (nur workflow_dispatch), `pages-verify` ein curl.
- **Blockade:** keine.
- **Braucht:** die health-Issues in den stehenden Pass/Register aufnehmen
  (`ci_manage`-Snapshot erweitern oder Issues ins `external-state`/Handover routen);
  reverify-Mask neu bewerten (quiet-void/key-void/refused werden nicht geflaggt);
  Duplikate #15/#54, #16/#51, #17/#55 zusammenführen. Der `register_sort`-Fix schließt
  die Ursache der test-void-Issues.

### ALMA `declined_sources.φ:330` — Duplikat + Policy
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25) `phi/declined_sources.φ:330` trägt dieselbe falsch
  verdrahtete ALMA-URL (`ra,dec,source_name,project_code`); der `blocked_sources.φ:98`-
  Block ist auf `s_ra,s_dec,target_name,proposal_id` korrigiert (TAP Positiv HTTP 200 /
  Negativ `validateColumnNonAlias`).
- **Blockade:** keine.
- **Braucht:** Policy-Messung aus `korpora_heim.φ:93–96` (© ALMA, kein CC): trägt
  `declined` statt `blocked`? Danach Duplikat abgleichen.

### `arxiv` HTTP 406 — serverseitig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** arXiv schließt die API-Migration ab
- **Lage:** (gemessen 2026-09-25) 406 mit leerem Body, UA-unabhängig, für jede
  ungecachte Query; gecachte Queries liefern 200. Retry für 406 entfernt (~9s/Query
  gespart); Meldung benennt den gemessenen Zustand.
- **Blockade:** arXiv-Edge.
- **Braucht:** Wiedervorlage bei Trigger; kein Code.

#### Stufe 2 — operator-gebunden

### `OPENALEX_MAILTO` (Polite-Pool)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator setzt einen Kontakt-Mail-Key
- **Lage:** (gemessen 2026-09-25) `OPENALEX_MAILTO` fehlt in `.secrets.local`; der
  `mailto`-Param ist verdrahtet, aber inaktiv. `S2_API_KEY` liegt vor und greift
  (semanticscholar 200).
- **Blockade:** kein Kontakt-Mail-Key.
- **Braucht:** Operator-Key.

#### Stufe 3 — blockiert: keiner. / Stufe 4 — wartend: `arxiv` (s. o.). / Stufe 5 — termin: keiner. / Stufe 6 — LOCK: keiner.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort. Diese Session trägt:
die 5 uncommitteten Werkzeug-/Config-Dateien (`AGENTS.md`, `opencode.json`,
`bin/archive_search`, `tools/utils/src/bin/archive_search.rs`,
`register_sort.rs` — rustfmt + `url_violations`-Ordnungsfix),
die Unit-Feld-Verdikte (`src/archivar/{port,units,tests}.rs`), die ALMA-/AEC-Register-
Korrekturen (`phi/blocked_sources.φ`), die Rate-Limit-Arme
(`tools/utils/src/bin/archive_search/{net,openalex,semanticscholar}.rs`) und diese
Übergabe. Die CI-Prüfung im stehenden Pass liest die Job-Ebene direkt (Browser /
`ci_manage log <id>`), nicht das Run-Ende.
