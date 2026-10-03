<!--
  title: Handover — River-Folge 86 (2026-10-03)
  session: River-Folge 86
  class: handover
  date: 2026-10-03
  sha256: afa73df14472c4f806f87352defd8159f62a7016d78b0074babf9cc0e6586ba6
  status: live
-->
# Handover — River-Folge 86 (2026-10-03)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie in einem Pass …" | 2026-10-03 | Operator (Session, River 85) — session-weiter Delegations-Consent, nicht das Commit-Wort
„Erste Handlung: `sread docs/concepts/tool-forms.md` …" | 2026-10-03 | Operator (Session, River 85)
„aber das kann doch alles in einem atom gemacht werden" | 2026-10-03 | Operator (River 85) — die offenen GIC-Stränge in einem Pass
„warum bauen wir silos … wir brauchen doch nur ein universelles myzel … nichts anderes als die Weberin auf TE ebene" | 2026-10-03 | Operator (River 85) — Universal-Myzel-These → Rats-Verdikt + `field_te_query`
„wir können doch die weberin auch mit einbeziehen … TE nicht nur mit Sources sondern auch mit Unterstützung der Zeugen … welche Dateien liegen noch in Phi die … ins universelle Mycellium sollten?" | 2026-10-03 | Operator (River 85) — Zeugen-Arm + φ-Zensus (nur `witnesses.φ` fehlt)
„ja bitt ebauen" | 2026-10-03 | Operator (River 85) — Zeugen-Arm
„braucht es pro?" | 2026-10-03 | Operator (River 85) — gemessen: nein, flash
„ja bitte" | 2026-10-03 | Operator (River 85) — erste Zeugen-Messung (`erbq-solar`)
„… den rat und die stimmen flash taucher mit harten bandagen kostenlose voices … und ui chats befrags" | 2026-10-03 | Operator (River 85) — Mehr-Stimmen-Befragung zu den 6 Pendings
„die frontier modelle sind eigentlich fast alle nur über ui chat erreichbar" | 2026-10-03 | Operator (River 85) — Roster-Korrektur: Frontier nur UI, nicht API
„du sollst nicht nvidia glm nutzen … schau mal die läufe von future und mycelium" | 2026-10-03 | Operator (River 85) — UI-Trio; **Kimi K3 nur via `tryingopen.com`**; `future-folge169`
„kannst du jetzt bitte nochmal eine korrekte befragung machen?" | 2026-10-03 | Operator (River 85) — korrekte UI-Befragung (z.ai/Claude/Kimi K3)
„kannst du es irgendwo verankern wie du die tools genutzt hast …" | 2026-10-03 | Operator (River 85) — Stimmen-Route in `docs/concepts/tools-map.md` verankert
„jast du das alles so übergeben dass die nächste session sofort weitermachen kann?" | 2026-10-03 | Operator (River 85) — Abschluss-Kontrolle
„Starte die River-Linie in einem Pass …" | 2026-10-03 | Operator (Session, River 86) — session-weiter Delegations-Consent, nicht das Commit-Wort
„braucht es pro?" | 2026-10-03 | Operator (River 86) — mechanischer Reader-Arm-Port → `grind-flash` (flash-first)

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — die GIC-Richtungsfrage; §6 trägt
  jetzt die **gemessene** Estimator-Bias-Tabelle (`te-bias-n`, Lauf 37118666568) und
  den gemessenen storm-only-Stand (`gic-storm`, Lauf 37118664799: n = 0, Kp-Kanal
  unvollständig → `pending`). Offene Marker: die kalibrierte max-T-Null (Rat), die
  Bias-Korrektur.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` (`class: paper`) — der Addendum;
  §2026-10-03 trägt den Trajektorien-Riss des Fill-Laufs 37116911686. Offen: OMNI2
  26 Zellen, ACE 3/14/16, kp `def`, Δ/σ_recon, Trajektorien-Erneuerung — siehe Offen.
- `docs/auftrag/auftrag-flyby2-kette.md` (`class: auftrag`) — die Path-2-Kette;
  Trägerzeile des Auftrags (dieselben Rest-Zellen).
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  die GIC/Bz-Vorhersagezelle; die α-Ebene wartet auf die kalibrierte max-T-Null.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — alle
  Sachpunkte §7 geschlossen; Trägerzeile.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab;
  Trägerzeile.
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` (`class: survey`) — Anwendung des
  Maßstabs; Offen: keiner aus diesem Gate.

## Offen (aufgeschlüsselt)

### fruehwarnsystem α-Ebene — wartet auf die wy-max-t-Null
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t 37133825315` (re-dispatched 2026-10-03T15:26Z).
- **Lage:** (gemessen 2026-10-03 via `ci_manage jobs 37114779681` + gh-Run-API) die
  letzten fünf Läufe endeten `cancelled` (`36843561883`, `36867148250`, `36943967388`,
  `36987592993`, `37114779681`); im letzten Lauf war `wy-selftest` success, alle
  `wy-shards` cancelled, `wy-combine` skipped. Der Abbruch-Akteur ist `unread` —
  `/tmp/opencode/ci_watchdog.log` trägt keine `cancel`-Zeile.
- **Blockade:** die Null landet nicht (5/5 cancelled).
- **Braucht:** `ci_manage log 37133825315` + `ci_manage jobs 37133825315` nach Lauf-Ende;
  bleibt der Lauf wieder cancelled, den Abbruch messen (Run-API) und die Shard-Größe
  (3×3333 Permutationen je Punkt) prüfen.

### GIC kalibrierte Null — studentisierte Westfall–Young max-T
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `bz-yearly-maxt 37118991157` (in Arbeit) und `wy-max-t 37133825315`.
- **Lage:** (gemessen 2026-10-03 via `ci_manage jobs 37118991157`) drei Jobs
  `in_progress` (sod-2024, abk-2024, abk-2025), B = 10⁴; die Konstruktion steht
  (`src/mathematikerin/wy_max_t.rs`, `bz_retro_probe.rs --null max-t`). Baum-Messung
  des Review-„Risses": `pair_lag_index_hash` (`wy_max_t.rs:262`) faltet lag 0/1 — das
  ist die **dokumentierte** lag-0/1-Identität (`docs/paper/gic-causal-driver.md:155,163`),
  kein Defekt; die Studentisierung ist konsistent (`obs/σ` gegen `null-max v/σ`,
  `wy_max_t_probe.rs:285,304-311`).
- **Blockade:** die Zahl entsteht nur im CI-Lauf.
- **Braucht:** `ci_manage log 37118991157`; Quantil + Verdikt ins Paper
  (`docs/paper/gic-causal-driver.md`), das offene Konstrukt schließen oder den Riss
  benennen. Die Review-Vorschläge (Null-Zentrierung, n_eff-Gate, GPD) sind
  Konstruktions-Fragen → Rat.

### Flyby-Path-2 — Trajektorien-Riss, Tube nicht gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Benennung der Flattener-Erneuerung (`flyby-path-2-preregistration.md:21`)
  oder Wiederherstellung des versiegelten Arc `aeb3c82f…` im CDN.
- **Lage:** (gemessen 2026-10-03 via `gh run download 37116911686` + `curl`+sha256) der
  Fill-Lauf `37116911686` (success) maß `sha256 018ce2ca…` (538 696 B) ≠ Seal
  `aeb3c82f…` (106 704 B, `flyby-path-2-preregistration.md:21`); kein Tube gebaut,
  Verdikt **riss** (`data/flyby2/tube-juice-2026-09-28.json`). Lokal mißt
  `data/ssd.jpl.nasa.gov/ephemeris_juice.bin` weiter `aeb3c82f…` (106 704 B); das
  CDN-Asset ist der erneuerte 538 696-B-Arc. Addendum §2026-10-03 trägt den Riss.
- **Blockade:** der Seal bindet den alten Arc, das CDN trägt einen erneuerten.
- **Braucht:** die CDN-Erneuerung nachmessen (An mycelium) und dann die Erneuerung im
  Prereg benennen (`flyby-path-2-preregistration.md:21`) oder das Flatten auf den
  versiegelten Arc pinnen; erst danach füllt der nächste Fill-Lauf.
- **Rest-Zellen:** OMNI2 26 (HAPI 1201, ~6 d Lag), ACE 3/14/16, kp `def` — pending bis
  Trigger (unverändert).

### Zeugen im universellen Myzel — Arm gebaut, erste Messung ohne Alignment
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `field-te-query 37120826017` (success, gelesen).
- **Lage:** (gemessen 2026-10-03 via `gh run download 37120826017`) der Zeugen-Arm lädt
  **nur** `point-event#1` (n = 1), der Lauf meldet `alignment absent: bin window
  carries no overlap between the arms` — die Query bleibt ungemessen (`pending`). Der
  Lauf übt den Arm.
- **Blockade:** ein einzelnes point-event trägt keine Serie; kein Zeugen-TE-Verdikt.
- **Braucht:** mehr point-events in den Zeugen-Pfad oder eine ereignis-konditionierte
  Query-Form; das rigorose Design (event-triggered average, Omori-erhaltende
  Shift-Null, vorabregistriert) steht in `sensory-folge225:289-296`.
- **Riss (benannt):** `phi/pipeline/descriptors/erbq-solar.te` ist als Test schwach.

### Probes-Wanderung (62 TE-Probes)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Probe die Paritätsbrücke `GLEICH`.
- **Lage:** (gemessen 2026-10-03 via `docs/blatt/sonne-erde-blatt.md:24`) der Query-Kern
  `field_te_query` steht; die erste Probe (ENSO Blatt I) ist `GLEICH`.
- **Blockade:** 61 Probes warten auf ihre Brücke.
- **Braucht:** pro Probe ein `field_te_query`-Deskriptor + Parität (`GLEICH`), dann
  entlassen.

### Nicht-point-event-Zeugen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Arm eine Query-Form (`witnesses.φ`).
- **Lage:** (gemessen 2026-10-03 via `sgrep`) 17 `s2-direction`/`sky1`, 4 `substance`,
  3 `gestalt` werden als `pending`/`probe` verweigert; keine Query-Form existiert.
- **Blockade:** Richtungs-/Spektral-Query-Form fehlt.
- **Braucht:** je Arm eine Query-Form (Richtung → zirkulär/Rayleigh-Kuiper/von-Mises;
  Spektren → Formvergleich oder Epochen-Skalar; Einzelspektrum → refuse).

### GPD-Tail-Fit in `wy_max_t`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** gecapptes Resample-Budget (`gic-causal-driver.md:165-166`).
- **Lage:** (gemessen 2026-10-03 via `sgrep`) nur `if the resample budget is capped`
  (`gic-causal-driver.md:165-166`); bei B = 1e4 nicht nötig.
- **Blockade:** B = 1e4 deckt α = 0.01.
- **Braucht:** erst bei Budget-Cap.

### Workflow-Domäne
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort zur Domänengrenze.
- **Lage:** (gemessen 2026-10-03) `bz-yearly-maxt.yml` und `field-te-query.yml` sind
  formal Mycelium.
- **Blockade:** keine (die Grenze selbst ist offen).
- **Braucht:** Rat-Wort; bei strenger Grenze gehen sie als eigene Punkte an Mycelium.

### `field_te_query` als Konsument der max-T-Null
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** gebaute `wy_max_t`-Bucket-`null_matrix`.
- **Lage:** (gemessen 2026-10-03 via `sgrep`) im Kern läuft die Phase-Surrogat-Null; die
  studentisierte max-T-Null ist dort nicht verdrahtet.
- **Blockade:** eigener Atom (saisonaler Bucket-`null_matrix`).
- **Braucht:** die max-T-Null in `field_te_query` verdrahten.

## Mehr-Stimmen-Review (Baum-Messung)

- **Status:** wartend | **Bindung:** eigen
- **Trigger:** keiner (Baum-Messung gelandet).
- **Lage:** (gemessen 2026-10-03 via `sgrep`/`sread`) die Review-Defekte wurden gegen den
  Baum gehalten: `pair_lag_index_hash` ist die dokumentierte lag-0/1-Identität (Paper §3.1,
  `:155,163`) — kein Code-Defekt; die Studentisierung ist konsistent
  (`obs/σ` gegen `null-max v/σ`). Rohmaterial:
  `state/stimmen/2026-10-03_{zai-ui,claude-ui,kimi-k3-tryingopen}_te-engine-review*.md`.
- **Blockade:** die restlichen Review-Punkte (Null-Zentrierung, Bias-Korrektur,
  storm-Selektion `F_{t−1}`, sparse-Katalog-Design, `erbq-solar` als ungültig) sind
  Konstruktions-/Vorabregistrierungs-Fragen.
- **Braucht:** diese Punkte dem Rat vorlegen (Architektur); `erbq-solar` gemessen
  descopen und auf das Matched-Control-Design (`sensory-folge225:289-296`) umbauen.

## An mycelium

Origin: river folge86.

- **`nvss-cdn` — Post-Fix-Lauf rot, Punkt gehört ins CDN.** Gemessen 2026-10-03:
  `nvss-cdn 37116852558` (head `73b6e9cd1`, nach dem Family-Tag-Fix) = `failure`;
  `tap_compiler` gegen `tapvizier.cds.unistra.fr` meldet dreimal
  `uws job phase ERROR — the query stays unharvested` (exit 1). Der RA-chunked Lauf
  (`--async 9000 --limit 300000`) heilt das nicht — die TAP-Job-Phase bricht ab. Der
  Punkt ist Compiler/CDN (kein River-Schritt); den Async-Job-Pfad im Compiler messen
  (Job-Phase-Polling/Timeout), dann neu dispatchen.
- **`ephemeris_juice.bin` — CDN-Erneuerung:** das CDN-Asset trägt jetzt
  `sha256 018ce2ca…` (538 696 B), der lokale/versiegelte Arc `aeb3c82f…` (106 704 B).
  Bitte die Erneuerung nachmessen (welcher Flatten-/Compiler-Lauf das Asset auf
  538 696 B setzte; Span/Format) — sie bricht den Path-2-Seal (An river: Flyby-Riss).

## An mountain

Origin: river folge86.

- **Reader-Arme `twomass_psc` + `swarm_tec` gebaut** (dein Block „An river",
  mountain-folge228). `src/archivar/twomass.rs` (+`component_name`/`component_value`),
  `src/archivar/extract.rs` (zwei Format-Arme), `src/archivar/main_flow.rs` (zwei
  Fetch-Branches), `src/archivar/fetch.rs` (Bypass-Liste). `cargo check` = 0 Fehler,
  0 Warnungen. Die Register-Zeile (`format`/`cmap`/`field`) und die Mycelium-Direktiven
  (`url`/`origin`/`compiler`/`sha256`) fehlen noch in `phi/sources.φ` — ohne sie bleibt
  der Arm ungenutzt. Felder wie spezifiziert: `twomass_{j,e_j,h,e_h,k,e_k}_mag` →
  `em mag`; `absolute_vtec_tecu` → `inverse-square em TECU 86400`.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/paper/gic-causal-driver.md`
- `docs/paper/flyby-path-2-addendum-2026-09-29.md`
- `src/archivar/twomass.rs`, `src/archivar/extract.rs`, `src/archivar/main_flow.rs`, `src/archivar/fetch.rs`
- `docs/handover/handover-2026-10-03-river-folge86.md`
- `docs/handover/archiv/handover-2026-10-03-river-folge85.md` (Move aus `docs/handover/`)

## Burn: open 0.0153 · close 0.121 — Grund: River-86 — Reader-Arme `twomass_psc`/`swarm_tec` gebaut (`cargo check` 0 Fehler / 0 Warnungen); GIC-Paper §6 (Estimator-Bias gemessen, storm-only measured `pending`); Flyby-Addendum (Trajektorien-Riss, Fill-Lauf 37116911686); `field_te_query`-Parität GLEICH; `wy-max-t` re-dispatched (37133825315). Gemessen `session_burn`: Line-Session $0.0841 + `grind-flash` Reader-Arme $0.0371; Maschinen-Total $0.6050 inkl. fremder Parallel-Linien.
