<!--
  title: Handover — Mycelium-Folge 152 (CDN-Familien-Tag-Rotation vollendet, HEAD-Red geheilt, DEMETER-Prämisse korrigiert, Beat-Paar descoped) (Stand 2026-09-24)
  session: Mycelium-Folge 152
  class: handover
  date: 2026-09-24
  sha256: 846cfd358f0f9ddecbb96998dc9c7985b05d10e8bd8f9a35c3164f085ad61725
  status: live
-->
# Handover — Mycelium-Folge 152 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Sortierung von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit: `autonom` →
`operator-gebunden` → `blockiert` → `wartend` → `termin` → `LOCK`.
**Vorbereitung ≠ Akt**: diese Linie führt derzeit keinen operator-gebundenen Punkt.

Diese Session hat `handover-2026-09-24-mycelium-folge151.md` konsumiert.

## Stehender Pass (gemessen 2026-09-24)

- **HEAD** `f417cdd69` beim Plan-Pass; während der Session rückte der geteilte
  Baum vor (fremde Commits `1c42ed09e`, `a1e35a19c`, `c015aa9b2`) — eigener
  Commit pfad-begrenzt, Fast-Forward geprüft.
- **Postfach** — `state/mail/mail_ledger.φ` (149 Zeilen); keine handlungsrelevante
  Neueingang für diese Linie. Jüngste: GitHub-Support-Antworten zum PII-Purge
  (future-Sache), `ivoa/uvor`-Invite abgelaufen.
- **CI am HEAD** — Watchdog-Snapshot 2026-09-24T22:54:01: aktiv `te-ncurve
  36052804297`, `health-check 36027534328`; `ci-check` mehrfach rot (attempt 1,
  jüngste `36030755250`). **Wurzel des Red gemessen:** `signal_cone_audit_probe.rs`
  rief `signal_reach` mit 3 statt 5 Argumenten (siehe unten, geheilt).
- **`open_points_check` folge151:** 9 Pfad-Refs, 0 absent, 0 guardians, 0 format-gaps.
- **`register_lookup --open`:** 116 Docs, 582 offen; pipeline: `ledger` 2,
  `index` 9, `witnesses` 4, `candidates` 1. `--dropped mycelium` lief in den
  120-s-Timeout (Netz/Scan) — Dropped-Audit bleibt offen (unten).

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### Bayestar19-Manifestation (Netloc geheilt, Lauf + sha offen)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** `bayestar-cdn.yml`-Lauf nach Push.
- **Lage:** (gemessen 2026-09-24 via Read/`cargo check`) `bayestar-cdn.yml:23`
  lud vom falschen Release `ssd.jpl.nasa.gov`; `bayestar_compiler.rs:312` uploadet
  auf Tag `dataverse.harvard.edu` → Zeile 23 umgestellt. `phi/sources.φ:10000`
  führt das Asset bereits unter `dataverse.harvard.edu`. Der be19-sha256 ist noch
  nicht gemessen (kein Upload) — `pending`, nach dem Lauf nachtragen.
- **Blockade:** keine.
- **Braucht:** nach Push `gh workflow run bayestar-cdn.yml`; danach be19-sha256 in
  `phi/sources.φ:10000` nachtragen.

### pre-cdn Stage-Regeneration (Merge-Atom)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Merge-Atom.
- **Lage:** (gemessen 2026-09-24 via Register) Queue gitignored → CI sieht sie
  nicht; `sources_potential_pre-cdn_9k_richest.φ` = 9045 Zeilen, 817 `url https`,
  151 `hapi`.
- **Blockade:** keine.
- **Braucht:** `--port` über `queue/sources_potential_pre-cdn_9k_richest.φ` +
  `…_params.φ`, gebunden im Merge-Atom.

### Dropped-Audit (Register-Sweep)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Sweep-Ergebnis gelesen.
- **Lage:** (gemessen 2026-09-24 via Watchdog) CI-Sweep `register-dropped
  36029814914` success; `register_lookup --dropped mycelium` lief in den
  120-s-Timeout. Kandidaten-Tabelle (folge151 §Dropped-Audit) unklassifiziert.
- **Blockade:** `--dropped`-Scan-Laufzeit.
- **Braucht:** `ci_manage log 36029814914` **einmal** lesen (kein Polling), je
  Kandidat resolve/rename/descope vs. echter Drop entscheiden.

### Fremdmodell-Benchmark
- **Status:** autonom | **Bindung:** eigen (braucht `chrome-devtools`-MCP)
- **Trigger:** Post `An mycelium` 2026-09-24.
- **Lage:** (gemessen 2026-09-24 via Post/Survey) GLM-5.2/5.3 (z.ai) + Claude
  Sonnet 5 (claude.ai) finden das Verdikt von `flyby-path-2-preregistration.md`
  unabhängig als tautologisch; Claude ergänzt Common-cause-Confound + Fix
  (Residuenbudget, Entscheidungsregel, Pipeline hashen). Vollrekord
  `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`.
- **Blockade:** MCP/Browser-Profil nicht im Plan-/Sub-Profil.
- **Braucht:** kontrollierter Benchmark N ≥ 5 harte Artefakte, identischer Prompt,
  blind bewertet, €/Qualität je Modell. Der `An future`-Rest (Claude- vs.
  DeepSeek-Kosten) bleibt bei future.

#### Stufe 2 — operator-gebunden

keiner.

#### Stufe 3 — blockiert

### DEMETER Order 18387 (WAF, nicht Workflow)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** F5-ASM-WAF erholt ODER Order-Ablauf 2026-09-28.
- **Lage:** (gemessen 2026-09-24 via `demeter_harvest.rs`/`ci_manage log`
  35851193831/REGARDS-API) Die folge151-Prämisse „kein Binary, kein CI-Workflow"
  ist **falsch**: `demeter-cdn.yml` + `demeter-aggregate-cdn.yml` existieren,
  Secrets `cdpp_user`/`cdpp_pass`/`omegaflow_token` gesetzt. Katalog-Scan
  erfolgreich (57760 URNs), aber **jede** `rs-order`-Erzeugung scheitert an
  `F5 ASM: Request Rejected` → Exit 137. `blocked_sources.φ:49`-Notiz korrigiert.
  Dispatch wiederholte denselben gemessenen WAF-Block und erzeugt Orders bei
  CNES (Dritt-Schreibakt) — kein Dispatch.
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** Wiedervorlage; bei Erholung `gh workflow run demeter-cdn.yml`.

#### Stufe 4 — wartend

### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf nach dem Red-Heil-Commit.
- **Lage:** (gemessen 2026-09-24 via `cargo check -p omegaflow-measure`) Der
  `ci-check`-Red war der Compile-Fehler in `signal_cone_audit_probe.rs:255`.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`; Ergebnis ins nächste Handover.

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ps1-cdn`-Lauf bis `all_present`.
- **Lage:** (gemessen 2026-09-24 via footprints.φ/ps1-cdn) `ps1_dr2_coverage.fp01`
  HTTP 404; Band-Parts 637–671, `band_max 2643`; Note in `footprints.φ:19`.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** bei `all_present` PS1-Note finalisieren.

### src.pas TAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `/tap/tables` 200.
- **Lage:** (gemessen 2026-09-24 via archive_search --verdict) `ledger.φ:10`
  ausstehend; `/tap` 200, `/tap/tables` 500 (`http://pithia.cbk.waw.pl/tap`).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** (gemessen 2026-09-24 via blocked_sources.φ/archive_search)
  `blocked_sources.φ:3`; api 000 (direct) / 5xx (Proton), Frontend 200;
  Ein-Exit-Stichprobe 500 vs. notiertem 502.
- **Blockade:** Broker-Backend.
- **Braucht:** Multi-Exit-Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Antwort / Freigabe.
- **Lage:** (gemessen 2026-09-24 via blocked_sources.φ/archive_search)
  `blocked_sources.φ:21`; `release_date 2099-01-01`, `data?PRODUCT` 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### SuperDARN MAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Globus-Task-`af68c4f1`-Status / Task-Ende.
- **Lage:** (gemessen 2026-09-24 via blocked_sources.φ) `blocked_sources.φ:16`
  `pending`; Zugang gewährt (Mail `1790021001`/`1790020962`); MAP 6561 Dateien/
  21,93 GB → `data/superdarn/map`; FITACF `sources.φ:9660`, RAWACF `:8318`.
- **Blockade:** Globus-Task-Status ungemessen (kein CLI/Token am Host).
- **Braucht:** Task-Status messen; bei Abschluss die Note schließen.

### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** (gemessen 2026-09-24 via ledger.φ/archive_search) Operator-Wort **nein**
  (2026-09-23); `ledger.φ:14` „Permission Denied" (Konto `omegaflow`), Host 200.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

#### Stufe 5 — termin

### EMODNET HFRADAR NADR
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Trigger:** Datum 2026-10-19.
- **Lage:** (gemessen 2026-09-24 via sgrep) `docs/zustand/external-state.md` trägt
  **keine** EMODNET-Zeile.
- **Blockade:** Termin.
- **Braucht:** Eintrag in `external-state.md` nachtragen (Re-Messung fällig
  2026-10-19).

#### Stufe 6 — LOCK

keiner.

## In diesem Atom geschlossen (Register)

- **GitHub-Release-Asset-Cap** — Council-Verdikt umgesetzt: geteiltes
  `omegaflow::cdn::CDN_TAG` entfernt (`EPHEMERIS_TAG`), 8 Aufrufer auf
  per-caller Familien-Tags rotiert (`-ephemeris`/`-dastcom`/`-dcom5`/`-laic`/
  `-ps1`/`-weberin`; `footprint_gate_probe` auf `binding.source_host`); lokale
  Harvest-`CDN_TAG` waren bereits pro-Site. `cargo check` (core/harvest/measure/
  service) 0/0.
- **dr3_stars-Netloc-Drift** — `phi/sources.φ:8327` auf tragenden Tag
  `ssd.jpl.nasa.gov` umgestellt (200 gemessen; `gea.esac.esa.int` 404).
- **smail Cc** — `--cc <addr>` (wiederholbar) gebaut, Payload-Feld nur nicht-leer
  (0 honored), Tests im File; `cargo check -p omegaflow-service` 0/0.
- **Witnesses absent (4)** — Endzustand `absent` (0 honored) bestätigt: die
  Kanäle sind offen, das S²-Richtung-/Energie-Feld wird physisch nicht geführt;
  `witnesses.φ:7/13/37/91` sind terminal, kein descope nötig.
- **Beat-Paar** — `descoped mit Befund`: kein offener Zwei-Ton-Rohkanal über die
  Forschungs-Kaskade (38+ Modi, siehe `research-max`-Matrix); zudem tragen 0
  Register-Oszillatoren eine Phase (`presence = 0`).
- **HEAD-Red (fremd gemeldet, eigene Wurzel)** — `signal_cone_audit_probe.rs:255`
  rief `signal_reach` mit 3 statt 5 Argumenten (eingeführt in `50f2bdeec`
  mycelium folge150). Fix: `SPECTRAL_NO_BAND` explizit (kein Band → flaches
  Gesetz); `cargo check -p omegaflow-measure` 0/0.

## Katalog-Pool (Register, nicht handlungsfähig)

- `phi/pipeline/ledger.φ` 2 offen = src.pas TAP + SSDC Limadou (beide oben,
  wartend).
- `phi/pipeline/index.φ` 9 offen = Tür-Kataloge (Adapter-Route, keine Quelle);
  Zähllinie, kein eigener Schritt.
- `candidates` 1 = `pipeline/catalog/archeology_gaps_index.φ` (ledger `verifiziert`,
  54 Kandidaten → 35 live/18 dead/1 blocked key).

## Benchmark

- Planungs-Pass ohne Dispatch. Ausführung: parallel — 3× `grind-flash`
  (Bayestar/Netloc, smail-Cc), 1× `grind-pro` (Asset-Cap), 1× `research-max`
  (Beat-Paar), 1× `grind-pro` (DEMETER), Haupt-Linie (Red-Heil + Register/Fold).
- **Befund:** die DEMETER-Prämisse „kein Workflow" war unbemerkt falsch — die
  folge151-Lage wurde nicht gegen den Baum gemessen. Lehre: jeder `Braucht`-Schritt
  wird vor dem Bau gegen `git ls-files`/Read gehalten.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien:** `src/archivar/cdn.rs`,
  `tools/harvest/src/bin/ps1_coverage_compiler.rs`,
  `tools/measure/src/bin/` (footprint_gate_probe, laic_probe, silence_map_probe,
  topocentric_coupling_probe, weberin_body_verdict, weberin_verdicts_compiler,
  signal_cone_audit_probe),
  `tools/service/src/bin/smail.rs`, `.github/workflows/bayestar-cdn.yml`,
  `phi/sources.φ` (Zeile 8327), `phi/blocked_sources.φ` (Zeile 49),
  `docs/handover/handover-2026-09-24-mycelium-folge152.md`.
- **Move mit dem Commit:** `handover-2026-09-24-mycelium-folge151.md` → `archiv/`.
- **Fremd im geteilten Baum (uncommittet, andere Linien):** `src/archivar/ble.rs`,
  `src/archivar/fit.rs`, `src/gate/commit_gate.rs`,
  `tools/register/src/bin/open_points_check.rs`, die Sensory-Übergabe-Moves
  (folge158/159) — nicht Teil dieses Commits.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, nie das Commit-Wort).
