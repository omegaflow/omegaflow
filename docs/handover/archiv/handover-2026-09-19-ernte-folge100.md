<!--
  title: Handover — Ernte-Folge 100 (Stand 2026-09-19)
  session: Ernte-Folge 100
  class: handover
  date: 2026-09-19
  sha256: 309a83875962c8b5e8b151c3609040ce395c25aa40f13bea4fd36099f5a8a734
  status: live
-->
# Handover — Ernte-Folge 100 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**
— das Register ist die Frage, der Baum die Messung.

## Stehender Pass (gemessen 2026-09-19, Folge 100)

- **HEAD** `7aa5c23e` bei Start (== `origin/main`, „bau folge94: fold the ernte98
  post into the handover"); während der Session auf `4638d7f3` gezogen (forschung
  folge98, `silence-map-probe`-Detach, gepusht); Safety-Net `refs/safety/1789851255`
  (Start).
- **Postfach** bei Start leer (`post.md` trägt nur die ernte→bau-Zeile aus Folge
  98/99); kein neuer Ledger-Eingang. `register_lookup --live`: 111 Docs, 545 offene
  Zeilen, 19 released.
- **CI** — `ci_manage list` (~21:00Z): `openneuro-cdn 35468606830` @`7aa5c23e`
  **failure** (ds007822, alle .set „carry no EEG contract", 0 honored → parser-gap,
  unten); `openneuro-cdn 35468312819` @`d1750fe0` success (ds005034-Idempotenz);
  `allwise-cdn 35464786486` success (Final fehlt, 127/304 Spans, unverändert);
  `harvest 35467467726` success; in_progress `ps1-cdn`, `placebo-ave-cdn`,
  `silence-map-probe`; pending `ci-check`, `hyperscanning-te`, `te-gate`.

## Offen

Kein abarbeitbarer undatierter Ernte-Punkt mehr — ds007822 ist als `parser-gap`
registriert und an bau übergeben; die übrigen Punkte sind `wartend`/`blockiert`
(extern)/`operator-gebunden`/`termin`.

- **OpenNeuro ds007822** `phi/pipeline/ledger.φ:122-124` — Probe `35468606830`
  @`7aa5c23e` **failure**: `extract_eeg` (`openneuro_compiler.rs:300`) lehnt jedes
  `.set` ab („carries no EEG contract", `:426/:483`), Compiler void (0 honored).
  Zustand jetzt `parser-gap`. **Schritt:** `--local`-Lauf auf einer .set in CI,
  fehlendes Feld (nbchan/pnts/chanlocs/data) benennen; dann Arm/Guard in bau. Post
  an bau gesetzt. `wartend` auf bau.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20` —
  `blockiert` (extern): sync-QUERY beider VOTable-`ERROR` localhost:5432 connection
  refused, Root je 200 (Re-Messung 2026-09-19). **Schritt:** Re-Check
  (`archive_search --verdict` + sync-QUERY), Trigger Backend-Erholung. `blockiert`.
- **Lasair-LSST API** `external-state.md:23` — Re-Messung 2026-09-19: direct absent
  (HTTP 0), Proton-Exit 502, Wayback 429 ohne Snapshot; Token present, unverified.
  **Schritt:** Exit-Rotation (Operator-Wort) + Query, Trigger Banner-Wechsel.
  `blockiert`.
- **ds007471 BrainVision-Arm + ds008192 SNIRF-Arm** `phi/pipeline/ledger.φ:126-132`
  — an bau übergeben (Code-Gap). `wartend` auf bau.

## Wartend (kein Auswahlpunkt)

- **allwise-cdn** — 127/304 Spans, Final `allwise_coverage.fp01` absent; stündlicher
  Schedule (`allwise-cdn.yml`). Bei Abschluss CDN-Pflicht (`sources.φ`). **Schritt:**
  `ci_manage view <allwise-run>`. `wartend`.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `phi/blocked_sources.φ:39-53` —
  Anfragen offen. `wartend`.
- **BepiColombo bc_mpo_more** `phi/blocked_sources.φ:26-29` — Freigabe ~April. `wartend`.
- **Babamul / IA2 TAP / GHRC** `phi/blocked_sources.φ` — kein gebauter Konsument →
  `pending`. `wartend`.

## Operator-gebunden

- **Limadou PI-Freigabe** `phi/pipeline/ledger.φ:26-28` — per-act consent.
  `operator-gebunden`.
- **Queue-Korpora** `phi/pipeline/ledger.φ:114-120` — `--port` braucht das
  Operator-Wort. `operator-gebunden`.

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — maxTime unverändert
  `2026-07-30T23:30:00Z`. Nächste Re-Messung **2026-10-19**. `termin`.

## Benchmark

- **Ernte-Folge 100** (build/flash, Hauptsession): die Lauf-Messung und die
  Log-Analyse (`ci_manage log 35468606830` → `openneuro_compiler.rs:426/483`) liefen
  in der Hauptsession; kein pro/max-Delegat nötig. Kein neuer Klassen-Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/pipeline/ledger.φ` (ds007822-Note → parser-gap),
  `docs/handover/post.md` (bau-Zeile um ds007822 erweitert),
  `docs/zustand/external-state.md` (CI-Status-Zeile),
  `docs/handover/handover-2026-09-19-ernte-folge100.md` (neu), Move
  `handover-2026-09-19-ernte-folge99.md` → `archiv/` (aus Folge 99 mitgeführt),
  `handover-2026-09-19-ernte-folge98.md` → `archiv/`.
- **Fremd (nicht anfassen):** `tools/utils/src/bin/archive_search/net.rs` (bau),
  `handover-2026-09-19-bau-folge95.md`, der bau94-Move,
  `.github/workflows/measure-gates.yml` (modifiziert),
  `.github/workflows/silence-map-probe.yml` (untracked), die drei
  `handover-2026-09-16-*`-Moves. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
