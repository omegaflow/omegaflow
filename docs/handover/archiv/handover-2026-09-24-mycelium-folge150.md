<!--
  title: Handover — Mycelium-Folge 150 (Operator-Wort abgearbeitet: Bayestar19 gebaut, DEMETER/SuperDARN blockiert, Council Asset-Cap; Witnesses/Footprints/Candidate gemessen) (Stand 2026-09-24)
  session: Mycelium-Folge 150
  class: handover
  date: 2026-09-24
  sha256: 0000000000000000000000000000000000000000000000000000000000000000
  status: live
-->
# Handover — Mycelium-Folge 150 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session hat `handover-2026-09-23-mycelium-folge149.md` konsumiert.

## Stehender Pass (gemessen 2026-09-24)

- **HEAD** `0c768291` (Base); Arbeitsbaum zum Beginn == HEAD, `git_safety --snapshot` nichts zu sichern.
- **Postfach** — `state/mail/mail_ledger.φ`: absent (nur CI-Bau); kein handlungsrelevanter Eingang.
- **CI am HEAD** — `ci_manage list` (2026-09-24): `ci-check 35924695548` **pending**; `tools-build 35924695592` **success**; `register-dropped 35922188109` **success**; `allwise-cdn`/`ps1-cdn`/`te-gate` aktiv.
- **`open_points_check` folge149:** 16 Pfade, 2 „absent" = Backtick-Fehlalarm (`phi/canon.φ`-Deklaration, Z. 34/131) — keine echten Stale-Punkte.

## Diese Session (git trägt es)

- **Operator-Wort 2026-09-23 abgearbeitet:** DEMETER **ja**, SuperDARN **ja**, Bayestar19 **ja**, Sicherheits-Befund **verfolgen**, SSDC **nein**, GitHub-Asset-Cap **ja**.
- **Bayestar19 gebaut** (`grind-pro`): neues Binding `phi/bindings/bayestar19.φ`, in `phi/canon.φ` deklariert, CDN-`url`-Block in `phi/sources.φ`, Rust-Wiring (`src/archivar/bayestar.rs` neu, `spatial.rs`/`membrane.rs`/`main_flow.rs`/Tests); `cargo check --all-targets --features browser_relay` **0/0** (Session verifiziert). Netloc = `dataverse.harvard.edu` (Compiler bereits korrekt; `ssd.jpl.nasa.gov` ist das gekappte Release).
- **DEMETER gemessen** (`grind-pro`): Order 18387 RUNNING, 55 218 Fehler (56,9 %), Ablauf 09-28; `demeter_harvest.rs` (BATCH=100, DMT_N1_1144) vorhanden, aber **kein Binary** (`tools-latest` absent) + **kein CI-Workflow** → Neuordnung verwehrt; Note in `blocked_sources.φ:49`.
- **SuperDARN gemessen** (`general`): kein Globus-Transfer-CLI/-Token am Host (nur GCP-Endpoint); Task `af68c4f1` Status ungemessen → `post.md`.
- **Sicherheits-Befund verfolgt** (`research-max`): Herkunft nicht pinbar — `opencode.db` der folge143-Session vollständig gelöscht (Backups 08-31); Repo/git/Tool-Archive/Snapshots/History null Treffer → `post.md`.
- **Asset-Cap-Rat** (`council`): Verdikt = **per-caller family tag, Rotation vollenden** (~24 Writer-Sites mit `CDN_TAG` = gekapptes Release); Folge-Atom für `grind-pro`.
- **Witnesses/Footprints + Candidate** (`grind-flash`): 4 Witnesses + 2 Footprints re-gemessen 2026-09-24 (`footprints.φ:12` absent→refused); Candidate `api.le-systeme-solaire.net` = false-open (bereits `sources.φ:8005/8006`), notiert.
- **Endpunkte/CDN** (`grind-flash`): src.pas `/tap/tables` weiter **500**; Lasair weiter 5xx (500/502, Ein-Exit-Stichprobe); dr3_stars/voyager/LIRA-RPW unverändert — keine Notiz-Änderung.
- **Post konsumiert** (6 `An mycelium`-Zeilen) und eigene `An future`/`An mountain`-Zeilen gesetzt; `index.φ:35` `master.φ` `pending`→`descoped` (Messung als Befund).

## Offen (aufgeschlüsselt)

### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check` auf dem folge150-Commit.
- **Lage:** `ci-check 35924695548` pending (gemessen 2026-09-24 via `ci_manage list`).
- **Blockade:** keine.
- **Braucht:** `ci_manage view <id>`; Ergebnis ins nächste Handover.

### pre-cdn Stage-Regeneration
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Merge-Atom.
- **Lage:** Queue gitignored → CI sieht sie nicht; 887 Blöcke, 141 keyless HAPI.
- **Blockade:** keine.
- **Braucht:** `--port` über `queue/sources_potential_pre-cdn_9k_richest.φ` + `…_params.φ`, gebunden im Merge-Atom.

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ps1-cdn`-Lauf bis `all_present`.
- **Lage:** `ps1_dr2_coverage.fp01` HTTP 404; Band-Parts 637–671, `band_max 2643`; Note in `footprints.φ:19` aktualisiert (2026-09-24).
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** bei `all_present` PS1-Note finalisieren.

### GitHub-Release-Asset-Cap
- **Status:** wartend | **Bindung:** eigen (Rat entschieden)
- **Trigger:** Bau des Folge-Atoms.
- **Lage:** 1000 Assets/Release erreicht (`src/archivar/cdn.rs:41` `CAPPED_RELEASE`); Council-Verdikt 2026-09-24: **per-caller family tag, Rotation vollenden** — Rest ~24 Writer-Sites, deren `CDN_TAG` auf das gekappte Release zeigt (RPW/ps1/eve bereits rotiert).
- **Blockade:** keine (Architektur entschieden).
- **Braucht:** `grind-pro`-Atom „the capped release carries no writer": je Site Familien-Tag setzen, `cargo check` 0/0, CI-Manifestation verifizieren.

### DEMETER Order 18387
- **Status:** blockiert | **Bindung:** eigen (Tooling)
- **Trigger:** CI-Workflow/Binary verfügbar.
- **Lage:** Operator-Wort ja (2026-09-23); `demeter_harvest.rs` (BATCH=100, `DMT_N1_1144`) vorhanden, aber kein Binary (`tools-latest` absent) und kein CI-Workflow (gemessen 2026-09-24). Order RUNNING, 55 218 Fehler, Ablauf 09-28; Neuordnung = 578 Orders, ~7 h CI-skalig.
- **Blockade:** kein Binary/CI-Workflow; lokaler Bau strukturell verwehrt.
- **Braucht:** `demeter`-Workflow bauen (`demeter_harvest.rs` dispatchbar), dann `ci_manage view`.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin
- **Trigger:** `/tap/tables` 200.
- **Lage:** `ledger.φ:10` ausstehend; `/tap` 200, `/tap/tables` 500 (`http://pithia.cbk.waw.pl/tap`, gemessen 2026-09-24).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** `blocked_sources.φ:3`; api 000 (direct) / 5xx (Proton), Frontend 200; Ein-Exit-Stichprobe 500 vs. notiertem 502 (gemessen 2026-09-24).
- **Blockade:** Broker-Backend.
- **Braucht:** Multi-Exit-Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Antwort / Freigabe.
- **Lage:** `blocked_sources.φ:21`; `release_date 2099-01-01`, `data?PRODUCT` 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### EMODNET HFRADAR NADR
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Trigger:** Datum 2026-10-19.
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** Operator-Wort **nein** (2026-09-23); `ledger.φ:14` „Permission Denied" (Konto `omegaflow`), Host 200.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

### Beat-Paar Datenquelle
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** eine gemessene Zwei-Ton-Rohquelle oder ein `descoped`-Befund.
- **Lage:** `beat_pair` (`shaders.rs:306`) verlangt zwei Oszillatoren im selben Kraft-Kanal, `df·dt<0.5`; Dual-Comb/Maser nur Paper bzw. `df·dt≥0.5`; kein Registry-Treffer.
- **Blockade:** Klasse existiert als Roh-Datenquelle nicht.
- **Braucht:** Zwei-Ton-Rohquelle finden oder `descoped mit Befund`.

### Bayestar19-Asset-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `bayestar-cdn.yml`-Lauf.
- **Lage:** Binding + `sources.φ`-Block + Rust-Wiring stehen (2026-09-24); `.be19`-Asset noch nicht manifestiert → kein `sha256` (0 honored, nie fabriziert).
- **Blockade:** CI-Lauf; `bayestar-cdn.yml:23` prüft fälschlich `ssd.jpl.nasa.gov` (soll `dataverse.harvard.edu`).
- **Braucht:** Workflow-Netloc fixen, `gh workflow run bayestar-cdn.yml`, sha256 in `sources.φ` nachtragen.

### sources.φ dr3_stars netloc-Drift
- **Status:** offen | **Bindung:** eigen
- **Trigger:** Register-Korrektur.
- **Lage:** `sources.φ:8327` registriert `dr3_stars.bin` unter Tag `gea.esac.esa.int` → HTTP 404; dieselbe Datei (75 001 828 B, sha256 `fb9a1408…`) liefert `ssd.jpl.nasa.gov/dr3_stars.bin` 200 (auch `frame_registry.φ:157`, `ztf-cdn.yml:22`) (gemessen 2026-09-24).
- **Blockade:** keine.
- **Braucht:** `url`-Zeile auf den tragenden Tag umstellen (Netloc-Konvention prüfen).

### Witnesses absent (4)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** Konsument-/Release-Entscheid.
- **Lage:** `witnesses.φ:7/13/37/91` `absent` (0 honored, regeneriert 2026-09-24): S²-Richtung/Energie-Feld real nicht geführt.
- **Blockade:** keine.
- **Braucht:** `declined`/`descoped`-Entscheid oder `absent` als Endzustand führen.

### dropped-gate-Baseline
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf.
- **Lage:** `register-dropped 35922188109` success (2026-09-24); lokal `register_lookup --dropped --count` = 3258; `docs/zustand/dropped-baseline.md` Baseline 678 @`48ae5e734`.
- **Blockade:** keine.
- **Braucht:** prüfen, ob der nächste `ci-check` `dropped-gate` grün ist; sonst Baseline im annehmenden Commit heben.

## Benchmark

- Dispatches dieser Session: 3× `grind-flash`-Klassen (2× flash, 1× general), 2× `grind-pro` (Bayestar19, DEMETER), 1× `council`, 1× `research-max`. Alle trugen.
- **Befund:** zwei Operator-Worte (DEMETER, SuperDARN) waren trotz Wort **maschinell nicht ausführbar** — fehlendes Binary/CI-Workflow bzw. fehlender Transfer-Token. Der Wort-Weg allein schließt einen Punkt nicht; die Infrastruktur ist der zweite Gatter. Beide als `post.md`/Offen-Punkt mit gemessenem Block benannt.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `phi/bindings/bayestar19.φ` (neu), `phi/canon.φ`, `phi/sources.φ`, `phi/witnesses.φ`, `phi/footprints.φ`, `phi/blocked_sources.φ`, `phi/pipeline/index.φ`, `phi/pipeline/catalog/archeology_gaps_index.φ`, `src/archivar/bayestar.rs`, `src/archivar/main_flow.rs`, `src/archivar/membrane.rs`, `src/archivar/spatial.rs`, `src/archivar/tests.rs`, `src/mathematikerin/tests.rs`, `src/mathematikerin/machines/tests.rs`, `docs/handover/post.md`, `docs/handover/handover-2026-09-24-mycelium-folge150.md`.
- **Move mit dem Commit:** `handover-2026-09-23-mycelium-folge149.md` → `archiv/`.
- **Fremd im geteilten Baum:** `.github/workflows/te-ncurve.yml` (staged, andere Linie) — nicht Teil dieses Commits.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
