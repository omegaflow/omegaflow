<!--
  title: Handover — Mycelium-Folge 151 (Planungs-Pass: Tafel nach Handlungsfähigkeit geordnet, Bayestar19-Post gefaltet) (Stand 2026-09-24)
  session: Mycelium-Folge 151
  class: handover
  date: 2026-09-24
  sha256: 83cca255d80575f400ff60ed2eea70fbfb047396cf237da6bb8cca9644d06b43
  status: live
-->
# Handover — Mycelium-Folge 151 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Sortierung von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit: `autonom` →
`operator-gebunden` → `blockiert` → `wartend` → `termin` → `LOCK`.

Diese Session hat `handover-2026-09-24-mycelium-folge150.md` konsumiert.

## Stehender Pass (gemessen 2026-09-24)

- **HEAD** `15ca40c`; Arbeitsbaum zum Beginn trägt zwei fremde Modifikationen
  (`opencode.json`, `src/archivar/port.rs` — andere Linie, nicht Teil dieses
  Handovers); `git_safety --snapshot` = `refs/safety/1790253455`.
- **Postfach** — `state/mail/mail_ledger.φ` absent (`mail_digest` pending, Bau
  gehört zu CI); kein handlungsrelevanter Eingang.
- **CI am HEAD** — Watchdog-Snapshot 2026-09-24T14:21:57: aktiv = `tools-build
  35998483156`, `te-ncurve 35994664173`, `ci-check 35989139086`, `health-check
  35949827023`; `failed` attempt 1: `ci-check 35983942672` (jüngste), `…35973038431`,
  `…35971097226`, `…35930077037`, `…35924695548`.
- **`open_points_check` folge150:** 27 Pfad-Refs, 1 absent = Backtick-Fehlalarm
  (`phi/canon.φ`-Deklaration, Z. 24) — kein echter Stale-Punkt.
- **`register_lookup --open`:** 116 Docs, 574 offene Zeilen; pipeline: `ledger` 2,
  `index` 9, `witnesses` 4, `candidates` 1; 18 `zustand`-Einträge due, 2 Post offen.

## Diese Session (git trägt es)

- **Planungs-Pass** (read-only): Tafel strikt nach Handlungsfähigkeit geordnet;
  Post-Zeile `An mycelium` (Bayestar19) in den offenen Punkt gefaltet und aus
  `post.md` gelöscht; Status-Glättung: `Beat-Paar` und `DEMETER` sind im Register
  `blockiert`, tragen aber je einen **eigenen** Schritt → `autonom`.
- Keine Messung außerhalb des Passes, kein Bau.

## Offen (aufgeschlüsselt)

### GitHub-Release-Asset-Cap
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Bau des Folge-Atoms.
- **Lage:** 1000 Assets/Release erreicht (`src/archivar/cdn.rs:41` `CAPPED_RELEASE`);
  Council-Verdikt 2026-09-24: **per-caller family tag, Rotation vollenden** — Rest
  ~24 Writer-Sites, deren `CDN_TAG` auf das gekappte Release zeigt (RPW/ps1/eve
  bereits rotiert) (gemessen 2026-09-24).
- **Blockade:** keine (Architektur entschieden).
- **Braucht:** `grind-pro`-Atom „the capped release carries no writer": je Site
  Familien-Tag setzen, `cargo check` 0/0, CI-Manifestation verifizieren.

### Bayestar19-Asset-Manifestation
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Workflow-Netloc-Fix + CI-Lauf.
- **Lage:** Rust-Kette gebaut (`Buffer.bayestar` `src/archivar/spatial.rs:38`,
  Loader `main_flow.rs:2653`, `sightline_ebv` `membrane.rs:133`); CDN-Asset
  `…/dataverse.harvard.edu/bayestar2019.be19` **404** in allen 3 Stufen
  (`archive_search --verdict`, 2026-09-24); `src/archivar/bayestar.rs:352` trägt
  `chunks_exact` (clippy) und `archivar::bayestar::tests::load_map_leaf_record_finds_the_pixel`
  rot; `bayestar-cdn.yml:23` prüft fälschlich `ssd.jpl.nasa.gov` (soll
  `dataverse.harvard.edu`). (Post `An mycelium` 2026-09-24 gefaltet.)
- **Blockade:** keine.
- **Braucht:** `bayestar.rs:352` auf `as_chunks` heilen + Leaf-Test grün;
  Workflow-Netloc fixen; `gh workflow run bayestar-cdn.yml`; sha256 in `phi/sources.φ`
  nachtragen.

### sources.φ dr3_stars netloc-Drift
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Register-Korrektur.
- **Lage:** `sources.φ:8327` registriert `dr3_stars.bin` unter Tag `gea.esac.esa.int`
  → HTTP 404; dieselbe Datei (75 001 828 B, sha256 `fb9a1408…`) liefert
  `ssd.jpl.nasa.gov/dr3_stars.bin` 200 (auch `frame_registry.φ:157`, `ztf-cdn.yml:22`)
  (gemessen 2026-09-24).
- **Blockade:** keine.
- **Braucht:** `url`-Zeile auf den tragenden Tag umstellen (Netloc-Konvention prüfen).

### pre-cdn Stage-Regeneration
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Merge-Atom.
- **Lage:** Queue gitignored → CI sieht sie nicht; 887 Blöcke, 141 keyless HAPI
  (gemessen 2026-09-24).
- **Blockade:** keine.
- **Braucht:** `--port` über `queue/sources_potential_pre-cdn_9k_richest.φ` +
  `…_params.φ`, gebunden im Merge-Atom.

### Witnesses absent (4)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Konsument-/Release-Entscheid.
- **Lage:** `witnesses.φ:7/13/37/91` `absent` (0 honored, regeneriert 2026-09-24):
  S²-Richtung/Energie-Feld real nicht geführt.
- **Blockade:** keine.
- **Braucht:** `declined`/`descoped`-Entscheid oder `absent` als Endzustand führen.

### Beat-Paar Datenquelle
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** eine gemessene Zwei-Ton-Rohquelle oder ein `descoped`-Befund.
- **Lage:** `beat_pair` (`shaders.rs:306`) verlangt zwei Oszillatoren im selben
  Kraft-Kanal, `df·dt<0.5`; Dual-Comb/Maser nur Paper bzw. `df·dt≥0.5`; kein
  Registry-Treffer (gemessen 2026-09-24). Register-Tag war `blockiert`, eigener
  Schritt vorhanden.
- **Blockade:** keine.
- **Braucht:** Zwei-Ton-Rohquelle finden oder `descoped mit Befund`.

### DEMETER Order 18387
- **Status:** autonom | **Bindung:** eigen (Tooling)
- **Trigger:** CI-Workflow/Binary verfügbar.
- **Lage:** Operator-Wort ja (2026-09-23); `demeter_harvest.rs` (BATCH=100,
  `DMT_N1_1144`) vorhanden, aber kein Binary (`tools-latest` absent) und kein
  CI-Workflow (gemessen 2026-09-24). Order RUNNING, 55 218 Fehler (56,9 %), Ablauf
  09-28; Neuordnung = 578 Orders, ~7 h CI-skalig. Register-Tag war `blockiert`,
  eigener Schritt vorhanden.
- **Blockade:** keine (Workflow-Bau ist ein Edit, kein lokaler Bau).
- **Braucht:** `demeter`-Workflow bauen (`demeter_harvest.rs` dispatchbar), dann
  `ci_manage view`.

### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf.
- **Lage:** Watchdog 2026-09-24: `ci-check` mehrfach rot (5× attempt 1, jüngste
  `35983942672`); folge150-Lauf `35924695548` zuletzt pending (gemessen 2026-09-24
  via Watchdog-Snapshot).
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`; Ergebnis ins nächste Handover.

### dropped-gate-Baseline
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf.
- **Lage:** `register-dropped 35922188109` success (2026-09-24); lokal
  `register_lookup --dropped --count` = 3258; `docs/zustand/dropped-baseline.md`
  Baseline 678 @`48ae5e734`.
- **Blockade:** keine.
- **Braucht:** prüfen, ob der nächste `ci-check` `dropped-gate` grün ist; sonst
  Baseline im annehmenden Commit heben.

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ps1-cdn`-Lauf bis `all_present`.
- **Lage:** `ps1_dr2_coverage.fp01` HTTP 404; Band-Parts 637–671, `band_max 2643`;
  Note in `footprints.φ:19` aktualisiert (2026-09-24).
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** bei `all_present` PS1-Note finalisieren.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin
- **Trigger:** `/tap/tables` 200.
- **Lage:** `ledger.φ:10` ausstehend; `/tap` 200, `/tap/tables` 500
  (`http://pithia.cbk.waw.pl/tap`, gemessen 2026-09-24).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** `blocked_sources.φ:3`; api 000 (direct) / 5xx (Proton), Frontend 200;
  Ein-Exit-Stichprobe 500 vs. notiertem 502 (gemessen 2026-09-24).
- **Blockade:** Broker-Backend.
- **Braucht:** Multi-Exit-Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Antwort / Freigabe.
- **Lage:** `blocked_sources.φ:21`; `release_date 2099-01-01`, `data?PRODUCT` 403
  (gemessen 2026-09-24).
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** Operator-Wort **nein** (2026-09-23); `ledger.φ:14` „Permission Denied"
  (Konto `omegaflow`), Host 200 (gemessen 2026-09-24).
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

### EMODNET HFRADAR NADR
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Trigger:** Datum 2026-10-19.
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

## Benchmark

- Planungs-Pass ohne Dispatch (read-only). Vorgeschlagene Ausführung: 7 parallele
  Punkte — 3–4× `grind-flash` (Bayestar19-Heilung dr3_stars, pre-cdn, Witnesses),
  2× `grind-pro` (Asset-Cap, DEMETER-Workflow), 1× `research-max` (Beat-Paar).
- **Befund (fortgeschrieben aus folge150):** zwei Operator-Worte (DEMETER,
  SuperDARN) waren trotz Wort **maschinell nicht ausführbar**; die Infrastruktur ist
  der zweite Gatter. DEMETER trägt jetzt einen eigenen Schritt (Workflow bauen).

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `docs/handover/handover-2026-09-24-mycelium-folge151.md`
  (neu), `docs/handover/post.md` (Post-Zeile gelöscht).
- **Move mit dem Commit:** `handover-2026-09-24-mycelium-folge150.md` → `archiv/`.
- **Fremd im geteilten Baum:** `opencode.json`, `src/archivar/port.rs` (uncommittet,
  andere Linie) — nicht Teil dieses Commits.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
