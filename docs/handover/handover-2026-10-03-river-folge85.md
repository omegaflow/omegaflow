<!--
  title: Handover — River-Folge 85 (2026-10-03)
  session: River-Folge 85
  class: handover
  date: 2026-10-03
  sha256: 6ab909ad2ae5c846481cd5747ff6c574d1ca7f55c6492bc5b23e07c5ca47c47b
  status: live
-->
# Handover — River-Folge 85 (2026-10-03)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie in einem Pass …" | 2026-10-03 | Operator (Session, River 85)
„Erste Handlung: `sread docs/concepts/tool-forms.md` …" | 2026-10-03 | Operator (Session, River 85)

## Träger (Prosa, eigene)

- `docs/paper/flyby-path-2-addendum-2026-09-29.md` (`class: paper`) — Path-2-Fill-Run
  am JUICE-Perigäum (2026-09-28, Fenster 22:41:53Z→23:01:08Z); swarm cell 25 (39625 nT)
  gefüllt (Bin-Fix `river 73`). Offen (trigger-/fristgebunden): OMNI2 26 Zellen, ACE
  3/14/16, kp `def`, Δ/σ_recon — siehe Offen.
- `docs/auftrag/auftrag-flyby2-kette.md` (`class: auftrag`) — die Path-2-Kette vor dem
  JUICE-Perigäum; dieselben Rest-Zellen wie das Addendum. Trägerzeile des Auftrags.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  GIC/Bz-Vorhersagezelle; die α-Ebene wartet auf die kalibrierte Westfall–Young-max-T-Null
  (`docs/specs/broken-null-control.md`). Trägerzeile (write schließt sich mit dem Lauf).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — die
  Membran-Ladearchitektur; alle Sachpunkte §7 geschlossen. Trägerzeile; der `--arxiv`/
  `--brave`-Werkzeug-Vermerk ist 2026-10-03 geheilt (`--arxiv` liefert Treffer,
  `--brave` antwortet ohne 402; Header-sha `ea724611…`).
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab vor
  Veröffentlichung. Trägerzeile; die Anwendung liegt in der Gate-Survey.
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` (`class: survey`) — Anwendung des
  Maßstabs; **Offen: keiner aus diesem Gate** (River, Sensory, Mountain alle geheilt).

## Offen (aufgeschlüsselt)

### fruehwarnsystem α-Ebene — wartet auf die wy-max-t-Null
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t 37114779681` (Trigger = Lauf-Ende; beim Pass
  gemessen 2026-10-03: `queued`).
- **Lage:** (gemessen 2026-10-03 via `ci_manage view 37114779681`) queued; der
  Vorgänger `wy-max-t 36867148250` war `cancelled` (2026-10-01T21:42Z), die Null war
  nicht gelandet. Das Blatt steht `unsealed`.
- **Blockade:** der Lauf ist nicht gelandet (queued).
- **Braucht:** nach dem Lauf-Ende `ci_manage log 37114779681` lesen; dann α/X/Z/Bz
  benennen (Dokument-Arbeit). Das Siegel setzt der Operator (kein Siegel ohne Wort).

### nvss-CDN — RA-chunked Lauf an der Kante
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `nvss-cdn 37116107265` (gemessen 2026-10-03: `pending`).
- **Lage:** (gemessen 2026-10-03 via `ci_manage view 37116107265` + `archive_search
  --verdict`) der Workflow ist RA-chunked geheilt (8 Slices `seq 0 45 315`, je
  `--async 9000 --limit 300000`; `.github/workflows/nvss-cdn.yml`); derzeit trägt
  `…/releases/download/ssd.jpl.nasa.gov/nvss.json` HTTP 206 (1 B — der alte Stand),
  der neue Lauf noch pending.
- **Blockade:** der Lauf läuft noch.
- **Braucht:** `ci_manage log 37116107265` nach Lauf-Ende lesen; ist der Chunk-Lauf
  success und das Asset neu, ist der Punkt (folge83) geschlossen.

### Flyby-Path-2 Rest-Zellen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `flyby-path2-fill 37116911686` (dispatched 2026-10-03, Trigger
  = Lauf-Ende); danach weiter OMNI2-Lag (~6 d, HAPI 1201), kp `def`, ESA/ESOC-SPK.
- **Lage:** (gemessen 2026-10-03 via curl) OMNI2 HAPI 1201 unverändert (86 B, „no data");
  GFZ liefert die Kp-Reihe, alle 25 Intervalle `status: "pre"` (kein `def`); die ACE-1h-
  Quelle trägt jetzt `2026-09-28T13:00` (Swepam, Zelle 14) und `02:00` (Mag), Zelle 16
  (15:00) fehlt beidseitig; Recon-Asset absent. Weil die ACE-Quelle seither Zellen
  geschlossen hat, wurde der read-only CI-Fill neu dispatcht (misst am lebenden Stand).
- **Blockade:** OMNI2-Lag, GFZ-`def`, ESA-SPK stehen; nur ACE/ Swarm messbar.
- **Braucht:** `ci_manage view/log 37116911686` nach Lauf-Ende lesen; gefüllte Zellen ins
  Addendum nachtragen, OMNI2/kp-`def`/Δ,σ_recon bleiben pending bis zum Trigger.

### GIC kalibrierte Null — studentisierte Westfall–Young max-T
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `bz-yearly-maxt 37118991157` (dispatched 2026-10-03) und
  `wy-max-t 37114779681` (queued).
- **Lage:** (gemessen 2026-10-03, read-only Taucher + `cargo check`) die Konstruktion ist
  aus `wy_max_t_probe.rs` in `src/mathematikerin/wy_max_t.rs` extrahiert;
  `bz_retro_probe.rs` trägt `--null <plugin|max-t>` (Default `plugin`, berichtete
  Ergebnisse unverändert), der max-T-Pfad fährt nur `--hourly --yearly-round`
  (sechs distinkte Statistiken, saisonaler Driver-Bootstrap, empirisches α-Quantil).
  GPD-Tail bleibt `pending`. Kein früherer Konsument (`bz_retro_probe` = Plug-in-fam,
  `bz_blatt_probe` = BH-FDR).
- **Blockade:** die Zahl entsteht erst im CI-Lauf (B = 10⁴).
- **Braucht:** `ci_manage log 37118991157` + `37114779681`; Quantil + Verdikt ins Paper
  (`docs/paper/gic-causal-driver.md:643-647`), das offene Konstrukt schließen oder den
  Riss benennen. **Workflow-Domäne:** zieht der Rat die Grenze streng, geht
  `bz-yearly-maxt.yml` als eigener Punkt an Mycelium.

### GIC storm-only Sub-Analyse
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `gic-storm 37118664799`.
- **Lage:** (gemessen 2026-10-03 via `cargo check -p omegaflow-measure`) `tools/measure/src/bin/gic_storm_probe.rs` + `.github/workflows/gic-storm.yml`
  gebaut (Matrix ABK/SOD 2024/2025, `Kp >= 5`, Storm- vs Jahres-Rund), 0 warnings; das Paper
  nennt die storm-only-Analyse als offenen Schritt (`:661-663`).
- **Blockade:** das Ergebnis liegt nur in CI.
- **Braucht:** `ci_manage log 37118664799`; die Tabelle ins Paper §6.

### TE-Estimator-Bias vs n
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `te-bias-n 37118666568`.
- **Lage:** (gemessen 2026-10-03 via `cargo check -p omegaflow-measure`) `tools/measure/src/bin/te_bias_n_probe.rs` + `.github/workflows/te-bias-n.yml`
  gebaut (Hénon, n = 800…10000, KDE/Silverman, 5 Replikate); das Paper nennt den
  unkorrigierten Bias bei n ≈ 1260–2200 offen (`:655-660`).
- **Blockade:** das Ergebnis liegt nur in CI.
- **Braucht:** `ci_manage log 37118666568`; Bias-Tabelle ins Paper §6.

### field_te_query — universeller TE-Kern (Rats-Verdikt 2026-10-03)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `field-te-query 37120272811` (Parity + Zeugen-Arm).
- **Lage:** (gemessen 2026-10-03 via `cargo check`, `sgrep`) `tools/measure/src/bin/field_te_query.rs`
  + `.github/workflows/field-te-query.yml` gebaut: Feld-Selektor über den
  Archivar-Pfad (`sources.φ` und `witnesses.φ`), Deskriptor-Grammatik (`cadence|seasonal|lags|surrogate|event|gate`,
  Arm-Zustand `built|pending|probe`), die eine TE-Maschine (`te.rs`) mit der einen
  Phase-Surrogat-Null; `--parity` fährt `omni_hro_imf_bz_gsm_nt × ersstv5_nino34_ssta`
  gegen das aufgezeichnete Blatt I (`docs/blatt/sonne-erde-blatt.md:24`). Das Rats-Verdikt
  (2026-10-03) trägt: Query-Kern statt Probes-Verschmelzung; 62 TE-Probes / 112 eigene
  `serie`-Dateien bleiben unangetastet; TE bleibt abgeleiteter Query-Term.
- **Blockade:** das Parity-Ergebnis liegt nur in CI; die `wy_max_t`-max-T-Null ist im Kern
  **nicht** als Verdikt-Null verdrahtet (es läuft die Phase-Surrogat-Null aus `te.rs` —
  genau die, die das Blatt erzeugte; eine andere wäre eine neue Null → eigener Atom).
- **Braucht:** `ci_manage log 37120272811`; bei `PARITY: GLEICH` die Brücke als
  Reproduktion tragen, bei `ABWEICHEND` den Riss benennen.

### Zeugen im universellen Myzel — Arm gebaut, erste Messung läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `field-te-query 37120826017` (Parity + Zeugen-Messung).
- **Lage:** (gemessen 2026-10-03 via `cargo check`, `sgrep`) der Zeugen-Arm ist gebaut:
  `--register witnesses` bzw. Deskriptor `witness <name> [built|pending|probe]`,
  Namensschlüssel `<typ>#<index>` (Alias `record`-Token/URL-Teilstring); `witness_series`
  lädt **nur `point-event`** (3 `erbq`-Zeugen) als Präsenz-Event-Train auf dem Grid;
  `s2-direction`/`sky1` (17), `substance` (4), `gestalt` (3) werden als `pending`/`probe`
  **verweigert**, nie in eine Serie gegossen, nie 0.0. Kein neuer Schätzer, keine neue Null.
- **Blockade:** die Zeugen-Parity bleibt `pending` (kein aufgezeichnetes Zeugen-TE-Verdikt
  in `docs/`); das erste Verdikt entsteht erst aus diesem Lauf.
- **Braucht:** `ci_manage log 37120826017`; dann ist das erste Zeugen-Verdikt der
  Vergleichspunkt. **Riss (benannt):** der Deskriptor `phi/pipeline/descriptors/erbq-solar.te`
  (driver `omni_hro_imf_bz_gsm_nt` × point-event#1 Tohoku, `bin 3600`) ist als
  *identifizierbarer* Test schwach — regionale Sequenzkataloge, ereignis-konditioniertes
  Subsample, Phase-Null ohne Omori-Clustering; `docs/handover/handover-2026-10-03-sensory-folge225.md:289-296`
  nennt das rigorose Design. Der Lauf übt den Arm, er ist keine Erdbeeben-Solar-Evidenz.

## Noch zu bauen (Pendings)

- **Zeugen-TE, identifizierbar (eigen).** Vorabregistriertes Design: `event-triggered average`
  + `event-shuffled`-Surrogate (Omori erhalten) + `stratified quiet` + `TE(X→Y|Kp)`
  (`handover-2026-10-03-sensory-folge225.md:289-296`); der `erbq-solar`-Deskriptor ist nur
  die Arm-Übung. **Braucht:** Rat-/Operator-vorabregistrierter Lauf.
- **Nicht-point-event-Zeugen (pending).** 17 `s2-direction`/`sky1`, 4 `substance`,
  3 `gestalt` brauchen eigene Query-Formen (Richtungs-/Spektral-Query), keine TE;
  bis dahin als `pending`/`probe` verweigert. **Braucht:** je Arm eine Query-Form.
- **`field_te_query` als Konsument der max-T-Null (eigen).** Im Kern läuft die
  Phase-Surrogat-Null; die `wy_max_t`-studentisierte max-T-Null als Verdikt-Null ist ein
  eigener Atom (saisonaler Bucket-`null_matrix`).
- **GPD-Tail-Fit (pending)** in `wy_max_t`: nur „if the resample budget is capped"
  (`docs/paper/gic-causal-driver.md:165-166`).
- **Probes-Wanderung (eigen).** Jede der 62 TE-Probes wird per Paritätsbrücke durch
  `field_te_query` reproduziert und dann entlassen — probe für probe, erst bei `GLEICH`.
- **Workflow-Domäne (Rat).** `bz-yearly-maxt.yml` und `field-te-query.yml` sind formal
  Mycelium; zieht der Rat die Grenze streng, gehen sie als Punkte dorthin.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`
- `docs/handover/archiv/handover-2026-10-03-river-folge84.md` (Move aus `docs/handover/`)
- `docs/handover/handover-2026-10-03-river-folge85.md`
- `tools/measure/src/bin/gic_storm_probe.rs`, `tools/measure/src/bin/te_bias_n_probe.rs`
- `.github/workflows/gic-storm.yml`, `.github/workflows/te-bias-n.yml`
- `src/mathematikerin/mod.rs`, `src/mathematikerin/wy_max_t.rs`
- `tools/measure/src/bin/bz_retro_probe.rs`, `tools/measure/src/bin/wy_max_t_probe.rs`
- `.github/workflows/bz-yearly-maxt.yml`
- `tools/measure/src/bin/field_te_query.rs`, `.github/workflows/field-te-query.yml`

## Burn: open 0.0153 · close 0.1606 · cap 0.50 — Grund: River-85 — Membran-Vermerk + exzellenz-Gate geheilt; GIC-Riss: storm-only + Bias + kalibrierte WY-max-T-Null gebaut/verdrahtet; Rats-Verdikt + `field_te_query`-Query-Kern + Zeugen-Arm + erste Zeugen-Messung gebaut, 8 CI-Läufe dispatcht. `close` = River-Linien-Session (gemessen `session_burn`, $0.1606); die dispatchten Taucher (6×grind-flash $0.3678 + general $0.0174 + Rat $0.0484 = $0.4336) trugen zusätzlich — das Atom lief damit über den Haus-Cap 0.50; benannt, nicht geglättet (Maschinen-Total 0.7972 inkl. fremder Parallel-Linien).
