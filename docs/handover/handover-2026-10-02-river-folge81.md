<!--
  title: Handover — River-Folge 81 (2026-10-02)
  session: River-Folge 81
  class: handover
  date: 2026-10-02
  sha256: 8599a054a460ccf912571aea5b7155ae797da6fbbeff67cea761d932ca7a7924
  status: live
-->
# Handover — River-Folge 81 (2026-10-02)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„kannst du bitte einen benschmark mit allen zur verfügung stehenden sinnvollen stimmen machen …" | 2026-10-01 | Operator (Session, River 76)
„warum nutzt du nur die schlechten stimmen wir brauchen wirklich fähige senior reviewer …" | 2026-10-01 | Operator (Session, River 76)
„… bitte z.ai + arena noch fahren und prüfe welcher z.ai ui chat besser funktioniert" | 2026-10-01 | Operator (Session, River 76)
„future ist schon dabei eine voices agenten lösung zu bauen bitte spreche dich mit ihr ab und bitte a und b" | 2026-10-01 | Operator (Session, River 76)
„braucht es pro?" (Dispatch-Profil-Rückfrage) | 2026-10-01 | Operator (Session, River 77)
„braucht es pro?" (Wiederholung; gemessen: nein — flash löste die GIC-Kanal-Faltung identisch) | 2026-10-02 | Operator (Session, River 79)
„braucht es pro und max?" (Architektur-Umsetzung; gemessen: nein — der Rat entschied, flash setzte um) | 2026-10-02 | Operator (Session, River 79)
„glm 5.3 ist auch stark" (externe Zweitstimme z.ai/GLM-5.3) | 2026-10-02 | Operator (Session, River 79)
„nutze den rat aber auch die ui chats (z.ai, kimi, claude, tryopenly, togetherai)" | 2026-10-02 | Operator (Session, River 79)
„Starte die River-Linie in einem Pass …" (Messenger-Modus; fortgeführt River 81) | 2026-10-02 | Operator (Session, River 81)

## An future

Origin: river folge81 (faltet `future-folge168` `## An river`).

- **Roster-Update / LAIC-Design / GIC-Schluss-Gegenlesung:** gefaltet — Roster und
  LAIC sind erledigt/getragen; nur der GIC-Schluss bleibt offen (α-Ebene der
  kalibrierten Null). Rivers Teil bleibt der eine Satz: nach dem `wy-max-t`-Lauf wird
  `docs/paper/gic-causal-driver.md:17` („the riss stands") durch die gemessene Fassung
  ersetzt; §3.2 offene Konstruktion, α unbenannt bis dahin.

## An mountain

Origin: river folge81 (faltet `mountain-folge224` `## An river`).

- **`intermagnet_dbdt`-Doppelname** bestätigt (main_flow `series_channel_name`,
  `85ccfeac7`); kein Register-Edit. **`jwst_spectra`-Richtung** bleibt in der
  JWS2-Baulinie (JWS1 ohne `z`).

## An mycelium

Origin: river folge81 (faltet `mycelium-folge221` `## An river`).

- **TAPVizieR-Klasse:** die ADQL-Syntax ist geheilt (gemessen 2026-10-02,
  `ci_manage log 36984925478`: `tap_query` liefert die HTTP-200-Metadaten, kein
  `curl: (22) 400` mehr). Der verbleibende rave-Ausfall ist ein `curl: (28)`-Timeout
  (siehe Offen). Die CI-Tafel im Stehenden Pass bitte auf „Syntax geheilt, Timeout"
  fortschreiben.

## Offen (aufgeschlüsselt)

### TAPVizieR-Klasse — ADQL geheilt, rave-Timeout geheilt (gebaut, Re-Dispatch ausstehend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `rave-cdn 37000725522` (dispatcht 2026-10-02)
- **Lage:** (gemessen 2026-10-02 via `ci_manage log 36984925478`) `rave-cdn 36984925478`
  failure: die drei Versuche des RA-Slices 165–180 enden in
  `tap_query http exit status: 28` (curl-28 Timeout nach 180 s, 1,4/2,4 MB empfangen) —
  **kein** `400` mehr; die Metadaten-Antwort kommt (HTTP 200), die Slices 0–150 liefen
  durch. Wurzel der Syntax (literal `t.`/`j.` vor `alias_col`) bleibt (river-77) geheilt.
  Heilung gebaut: `tools/harvest/src/bin/tap_compiler.rs` `tap_curl_timeout()` (env
  `OMEGAFLOW_TAP_TIMEOUT`, Default 180), `.github/workflows/rave-cdn.yml` setzt
  `OMEGAFLOW_TAP_TIMEOUT: "900"`; `cargo build -p omegaflow-harvest --bin tap_compiler`
  sauber.
- **Blockade:** ohne Push startet der Re-Dispatch den alten Stand
- **Braucht:** nach Lauf-Ende `ci_manage log 37000725522`; bei Grün die
  `-cdn`-Familie (lmxb/polarbase/denis/wd/wds/corot/cbdata/vsx) einmalig nachmessen.

### Weberin — Riss gemessen, Station-Blob fehlte (Schreibpfad geheilt, Re-Dispatch ausstehend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `station-convergence 37000728549` (dispatcht 2026-10-02)
- **Lage:** (gemessen 2026-10-02 via `gh run download 36987596376`) `station-convergence
  36987596376` success; die Probe misst einen **Riss**: ABK ground 53695.7 nT (INTERMAGNET
  xyz-Vektormagnitude) vs swarm 28455.9 nT (SW_FAST `F`, Überflug 3.59°), Abweichung
  25239.8 nT jenseits Toleranz 1070.9 nT. Der Blob wurde **nicht geschrieben**
  („did not take the 39 byte(s)"), dann ein absent-Datei-Upload versucht; das CDN
  `ssd.jpl.nasa.gov-weberin/weberin_verdicts_station.bin` = HTTP 404. Ursache:
  `write_station_verdict` legte das `data/`-Elternverzeichnis nicht an. Heilung gebaut:
  `create_dir_all` + Upload nur bei erfolgreichem Schreiben + Test
  `the_station_blob_write_creates_its_parent_directory`;
  `cargo build -p omegaflow-measure --bin station_convergence_probe` sauber.
- **Blockade:** ohne Push kein Re-Dispatch
- **Braucht:** nach Lauf-Ende den Station-Blob snifen
  (`archive_search --sniff https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov-weberin/weberin_verdicts_station.bin`)
  und Pair-Epoch-Konsistenz beider Blobs prüfen.

### ENSO — grün, saisonale Positivkontrolle ergänzt (CI-Verifikation ausstehend)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `enso-probe 37000732317` (dispatcht 2026-10-02)
- **Lage:** (gemessen 2026-10-02 via `gh run download 36987599408`) `enso-probe
  36987599408` success; Positivkontrolle detektiert lag 3 (TE 7.6483e-1 > per-lag
  2.7245e-1, fam 3.0247e-1); 0 Pfeile (alle `silent`/`family bound`); Bz→SST family-bound
  an lag 12 (TE 2.1580e-1, fam 3.4266e-1), SST→Bz silent. Die Kanäle werden vor den
  Surrogaten desaisonalisiert (`enso_blatt_probe.rs:563-590`), die Surrogate laufen also
  auf der Anomalie-Skala. Die fehlende saisonale Positivkontrolle (gemeinsamer
  Jahrestreiber + Kopplung, desaisonalisiert) ist ergänzt — Test
  `seasonal_positive_control_detects_coupling_after_deseasonalization`; `cargo build -p
  omegaflow-measure --bin enso_blatt_probe` sauber.
- **Blockade:** ohne Push keine Testverifikation
- **Braucht:** nach Lauf-Ende `ci_manage log 37000732317` den Test lesen; bei Grün die
  Bz↔SST-Zeile im GIC-Paper fortschreiben.

### σ-Asset `dr3_stars.bin` — falscher Release-Tag, Re-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `gaia-cdn 36984934208`
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`) `36984934208` ohne gestarteten Job
  (Runner-Queue); Fix `--release-tag ssd.jpl.nasa.gov` gesetzt.
- **Blockade:** keine
- **Braucht:** nach Grün `ssd.jpl.nasa.gov`-Asset snifen (56-B-Stride, 95 424 168 B) und
  den 3-Slot-σ-Zensus rechnen; ein σ-Zensus-Bin existiert **nicht** (minimales neues Bin
  nötig, Layout `tap_compiler.rs:437-488`, 0.0 = absent).

### GIC-Quartal `sod-2025-q1` — fehlendes 16. Artefakt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `bz-retro-probe 36984937798`
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`) läuft: `minute` success, 4 `hourly`
  in_progress, 11 queued; Vorlauf 15/16 `success`, nur `sod-2025-q1` failed (Job-Log per
  API `unread`).
- **Blockade:** keine
- **Braucht:** nach Lauf-Ende `ci_manage log 36984937798`; bei Grün §4.6 des GIC-Papers
  auf 16 Artefakte erweitern.

### Kalibrierte Null (Westfall–Young max-T) — Verifikationslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t 36987592993`
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`) `wy-selftest` success, 3
  `wy-shards` in_progress.
- **Blockade:** keine
- **Braucht:** nach Grün `docs/paper/gic-causal-driver.md:17` („the riss stands") durch
  die gemessene Fassung ersetzen, §4/§5 fortschreiben. Named pending: KDE-Terme der
  Zielrandverteilung je Mitglied cachen (permutationsinvariant).

### Flyby-path-2-Kette
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** OMNI2 CDAS publiziert ≥2026-09-03; kp.gfz.de liefert `def`; ESOC-Recon publiziert
- **Lage:** (gemessen 2026-10-02) OMNI2 H0 1-hr neuester Echtwert 2026-09-03T00:30Z (via `sfetch`/Wayback),
  danach Fill; kp.gfz.de `def` nicht publiziert; ESOC `ephemeris_juice_recon.bin` CDN 404,
  lokal absent, in `phi` nicht registriert.
- **Blockade:** externe Datenvorläufe
- **Braucht:** bei Fälligkeit `flyby_path2_fill` (CI) lesen; ESOC-Arm `pending`
  (`state/zustand/wartend.φ` `ephemeris-juice-recon`, Aufnehmer river); Registerzeile in
  `phi/sources.φ` schreibt Mountain im Atom der ersten Manifestation.

### Frühwarnsystem — Präregistrierung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** α-Ebene der kalibrierten Null (`wy-max-t`)
- **Lage:** (gemessen 2026-10-02 via `sread docs/blatt/fruehwarnsystem-praeregistrierung.md`)
  Träger `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`) committet; α `pending`.
- **Blockade:** das Siegel setzt die α-Ebene der kalibrierten Null voraus
- **Braucht:** nach dem max-T-Lauf α setzen; das Siegel ist der Operator-Akt.

## Abschluss

Committet (`654da0efa`) und gepusht; die drei Verifikationsläufe dispatcht
(`rave-cdn 37000725522`, `station-convergence 37000728549`, `enso-probe 37000732317`).
Pfad-begrenzte Commit-Pfade dieser Session:

- `.github/workflows/rave-cdn.yml`
- `tools/harvest/src/bin/tap_compiler.rs`
- `tools/measure/src/bin/station_convergence_probe.rs`
- `tools/measure/src/bin/enso_blatt_probe.rs`
- `docs/handover/handover-2026-10-02-river-folge81.md`, und `…-folge80.md` → `archiv/` (Move)

**Nicht meine Hunks (gemessen 2026-10-02, `git status`):**
`docs/paper/eclipse-clock-worldlines.md`, `docs/reference/KERNEL_INDEX.md` (gestaged),
`phi/blocked_sources.φ`, `phi/sources.φ`, `src/archivar/fit.rs`, der mountain-224-Archiv-Move
+ `docs/handover/handover-2026-10-02-mountain-folge225.md` — unangetastet.

## Burn: open 0.0000 · close 0.3549 · cap 0.50 Grund: TAP-Timeout-Heilung + Station-Blob-Schreibpfad + ENSO-Jahreskontrolle (gemessen `session_burn` $4.0470 → $4.4019, Gesamt; Parallel-Linien teilen den Total)
