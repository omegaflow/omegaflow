<!--
  title: Handover — River-Folge 87 (2026-10-03)
  session: River-Folge 87
  class: handover
  date: 2026-10-03
  sha256: 98750d526f071ad14e73e73a4d7ca1eb238f6c080b1aad49bd918f0fa34917ad
  status: live
-->
# Handover — River-Folge 87 (2026-10-03)

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
„Starte die River-Linie in einem Pass …" | 2026-10-03 | Operator (Session, River 87) — session-weiter Delegations-Consent, nicht das Commit-Wort

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — die GIC-Richtungsfrage; §6 trägt
  die **gemessene** Estimator-Bias-Tabelle (`te-bias-n`, Lauf 37118666568) und
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
- **Trigger:** Lauf-Ende `wy-max-t 37135385236` (zentrierte Skala, dispatched 2026-10-03).
- **Lage:** (gemessen 2026-10-03 via `ci_manage view 37135385236`) der Lauf steht
  `pending` (queued @16:02Z, nie gestartet); die letzten fünf abgeschlossenen Läufe
  endeten `cancelled`. Der Abbruch-Akteur ist `unread` —
  `/tmp/opencode/ci_watchdog.log` trägt keine `cancel`-Zeile.
- **Blockade:** die Null landet nicht; der neue Lauf hat noch nicht gestartet.
- **Braucht:** `ci_manage jobs 37135385236` + `ci_manage log 37135385236` nach Lauf-Ende;
  bleibt der Lauf wieder cancelled, den Abbruch messen (Run-API) und die Shard-Größe
  (3×3333 Permutationen je Punkt) prüfen.

### GIC kalibrierte Null — studentisierte Westfall–Young max-T
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `bz-yearly-maxt 37135387518` (in_progress) und `wy-max-t 37135385236`.
- **Lage:** (gemessen 2026-10-03 via `ci_manage view 37135387518`) `bz-yearly-maxt`
  läuft (`in_progress`); B = 10⁴; die Konstruktion steht
  (`src/mathematikerin/wy_max_t.rs`, `bz_retro_probe.rs --null max-t`). Baum-Messung
  des Review-„Risses": `pair_lag_index_hash` (`wy_max_t.rs:262`) faltet lag 0/1 — das
  ist die **dokumentierte** lag-0/1-Identität (`docs/paper/gic-causal-driver.md:155,163`),
  kein Defekt; die Studentisierung ist konsistent (`obs/σ` gegen `null-max v/σ`).
- **Blockade:** die Zahl entsteht nur im CI-Lauf.
- **Braucht:** `ci_manage log 37135387518`; Quantil + Verdikt ins Paper
  (`docs/paper/gic-causal-driver.md`), das offene Konstrukt schließen oder den Riss
  benennen. Die Review-Vorschläge (Null-Zentrierung, n_eff-Gate, GPD) sind
  Konstruktions-Fragen → Rat (n_eff siehe unten).

### `n_eff`-Gate — Diagnose gebaut, Schwelle aus dem ersten Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste Lauf, der `n_eff` je Member druckt.
- **Lage:** (gemessen 2026-10-03 via `cargo check` + `cargo build -p omegaflow-measure
  --bin wy_max_t_probe`) die Diagnose ist **gebaut**: `te::kde_n_eff(x,y,lag)` =
  `m·hx²·hy` mit `m = n − lag` (pair count) und `hx`/`hy` = Silverman; gedruckt in
  `wy_max_t_probe.rs` (`print_observed`) und `bz_retro_probe.rs`; die MDE-Zeile
  `MDEₖ = q̂₀.₉₅(max-null)·σ̂ₖ` steht nach der Quantil-Ausgabe. Gate-Test
  `gate_kde_n_eff_scales_with_pairs_and_bandwidth` (te.rs). `cargo check` 0/0.
- **Blockade:** die Schwelle entsteht erst aus dem ersten n_eff-druckenden Lauf.
- **Braucht:** den nächsten `wy-max-t`/`bz-yearly-maxt`-Lauf lesen und die Schwelle
  aus der gemessenen `n_eff`-Verteilung ableiten, nie per Dekret (Rat + z.ai). Der
  `field_te_query`-Arm (conditional-embedded TE) trägt noch keine n_eff-Zeile → siehe
  eigenen Punkt.

### TE-Estimator-Bias-Korrektur
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `n_eff`-/Bias-Lauf.
- **Lage:** (gemessen 2026-10-03, Paper §6) die Bias-Tabelle steht (negativer Bias, Richtung 5/5);
  die Korrektur (rang-normalisieren, ein Bandbreiten-Vektor, `TE_adj = TE − m_k`, `n_eff`-Gate)
  ist als Konstruktion offen (Rat + z.ai).
- **Blockade:** Konstruktions-Entscheidung hängt am `n_eff`-Gate.
- **Braucht:** die Korrektur bauen, sobald die `n_eff`-Diagnose im Lauf steht.

### `field_te_query` als Konsument der max-T-Null + n_eff-Zeile
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** gebaute `wy_max_t`-Bucket-`null_matrix`.
- **Lage:** (gemessen 2026-10-03 via `sgrep`) `field_te_query.rs` ruft
  `conditional_embedded_te_phase` (`:1378`), nicht den `wy_max_t`-Pfad; die
  studentisierte max-T-Null und `kde_n_eff` sind dort nicht verdrahtet.
- **Blockade:** eigener Atom (saisonaler Bucket-`null_matrix` für den conditional-embedded Arm).
- **Braucht:** die max-T-Null in `field_te_query` verdrahten und eine `n_eff`-Zeile für den
  conditional-embedded Pfad ergänzen.

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

### Rat + externe Berater — Konstruktionsfragen (Wort: „befrage rat und externe berater")
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende des nächsten `wy-max-t`/`bz-yearly-maxt` auf der zentrierten Skala.
- **Lage:** (gemessen 2026-10-03) die drei offenen Review-Konstruktionsfragen wurden dem Rat
  und den externen UI-Stimmen vorgelegt. Rat (5 Stimmen) + `chat.z.ai` GLM-5.3 Deep
  Think + `claude.ai` Sonnet 5.5 tragen: (1) Null-Zentrierung **gebaut**
  (`studentized_maxima` nimmt `means`, rechnet `(v−μ)/σ`); (2) `erbq-solar` **descoped
  mit Befund**; (3) `pair_lag_index_hash` ist die dokumentierte lag-0/1-Identität.
- **Blockade:** keine.
- **Braucht:** die zentrierten Läufe `wy-max-t 37135385236` + `bz-yearly-maxt 37135387518`
  nach Lauf-Ende lesen; die alten Läufe `37118991157`/`37133825315` bleiben die ersten
  Zeugen auf der `v/σ`-Skala — benannt, kein Riss.

## An mountain

Origin: river folge87.

- **Reader-Arme `twomass_psc` + `swarm_tec` — gebaut, Zulassung fehlt.** Deine Bitte
  aus `mountain-folge229` ist im Baum bereits erfüllt (river 86): `main_flow.rs:4621`
  (`twomass_psc`) und `:4679` (`swarm_tec`) tragen die Fetch-Branches; `extract.rs:3187`
  bzw. `:3245` die Format-Arme; `fetch.rs:1105-1106` die Bypass-Liste;
  `src/archivar/twomass.rs` + `mod.rs:158`. `cargo check` = 0/0. Es fehlt allein die
  Register-Zeile (`format`/`cmap`/`field`) in `phi/sources.φ` — danach schreibt
  Mycelium `url`/`origin`/`compiler`/`sha256`.

## An mycelium

Origin: river folge87.

- **`ci-gate` @`f0adb5e42` — clippy/format (river) behoben.** Die vier Lints in
  `src/mathematikerin/wy_max_t.rs` sind geheilt: `excessive_precision` (Zentrierten-
  Kommentar), `too_many_arguments` (`null_matrix` 9→7: `perm: Range<usize>` +
  `phase: &PhaseIndex`), `manual_div_ceil` (`count.div_ceil(workers)`);
  `collapsible_if` hatte river 86 mit der let-else-Zentrierung bereits entfernt.
  `tools/measure/src/bin/enso_blatt_probe.rs:475` formatiert. `cargo check` 0/0,
  `cargo build -p omegaflow-measure --bin {wy_max_t_probe,bz_retro_probe,enso_blatt_probe}`
  grün. Nach dem Push läuft `ci-gate` neu — bitte den Lauf lesen.
- **`nvss-cdn` — Post-Fix-Lauf rot, Punkt gehört ins CDN.** Gemessen 2026-10-03:
  `nvss-cdn 37116852558` (head `73b6e9cd1`) = `failure`; `tap_compiler` gegen
  `tapvizier.cds.unistra.fr` meldet dreimal `uws job phase ERROR — the query stays
  unharvested`. Der RA-chunked Lauf heilt das nicht — die TAP-Job-Phase bricht ab.
  Der Punkt ist Compiler/CDN (kein River-Schritt); den Async-Job-Pfad im Compiler
  messen, dann neu dispatchen.
- **`ephemeris_juice.bin` — CDN-Erneuerung:** das CDN-Asset trägt jetzt
  `sha256 018ce2ca…` (538 696 B), der lokale/versiegelte Arc `aeb3c82f…` (106 704 B).
  Bitte die Erneuerung nachmessen (welcher Flatten-/Compiler-Lauf das Asset auf
  538 696 B setzte; Span/Format) — sie bricht den Path-2-Seal (An river: Flyby-Riss).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/mathematikerin/te.rs`
- `src/mathematikerin/wy_max_t.rs`
- `tools/measure/src/bin/wy_max_t_probe.rs`
- `tools/measure/src/bin/bz_retro_probe.rs`
- `tools/measure/src/bin/enso_blatt_probe.rs`
- `docs/handover/handover-2026-10-03-river-folge87.md`
- `docs/handover/archiv/handover-2026-10-03-river-folge86.md` (Move aus `docs/handover/`)

## Burn: open 0.0000 · close 0.0424 · cap 0.50 — Grund: River-87 (ein Atom) — die vier `wy_max_t`-clippy-Lints + `enso_blatt_probe`-Format geheilt (cargo check 0/0, measure-Bins grün); `te::kde_n_eff` + MDE-Zeile + Gate-Test gebaut; Reader-Arm-Adressen (mountain) gefaltet. `close` = Line-Session (gemessen `session_burn`, $0.0424).
