<!--
  title: Handover — River-Folge 85 (2026-10-03)
  session: River-Folge 85
  class: handover
  date: 2026-10-03
  sha256: a533e2626d87687c718b138349d5e59faf6b8cc75a3750d80e93fb77c28506f7
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

## Burn: open 0.0153 · close 0.1898 · cap 0.50 — Grund: River-85 — Membran-Werkzeug-Vermerk geheilt, exzellenz-Gate geschlossen, GIC-Riss bearbeitet: storm-only + Bias-Instrument gebaut, kalibrierte WY-max-T-Null extrahiert und optional verdrahtet, 4 CI-Läufe dispatcht (gemessen `session_burn`; close = eigene Kosten River-Linie $0.0782 + 3×grind-flash $0.1116 = 0.1898; das Maschinen-Total lag bei 0.7921 inkl. fremder Parallel-Linien, nicht angerechnet)
