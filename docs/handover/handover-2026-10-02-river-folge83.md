<!--
  title: Handover — River-Folge 83 (2026-10-02)
  session: River-Folge 83
  class: handover
  date: 2026-10-02
  sha256: 0418bc00032aa5a25e16f47a475349ad71e47fa1fca837d60f016859687e8733
  status: live
-->
# Handover — River-Folge 83 (2026-10-02)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„kannst du bitte einen benschmark mit allen zur verfügung stehenden sinnvollen stimmen machen …" | 2026-10-01 | Operator (Session, River 76)
„warum nutzt du nur die schlechten stimmen wir brauchen wirklich fähige senior reviewer …" | 2026-10-01 | Operator (Session, River 76)
„… bitte z.ai + arena noch fahren und prüfe welcher z.ai ui chat besser funktioniert" | 2026-10-01 | Operator (Session, River 76)
„future ist schon dabei eine voices agenten lösung zu bauen bitte spreche dich mit ihr ab und bitte a und b" | 2026-10-01 | Operator (Session, River 76)
„braucht es pro?" | 2026-10-01 | Operator (Session, River 77)
„braucht es pro?" (Wiederholung; gemessen: nein) | 2026-10-02 | Operator (Session, River 79)
„braucht es pro und max?" (gemessen: nein) | 2026-10-02 | Operator (Session, River 79)
„glm 5.3 ist auch stark" | 2026-10-02 | Operator (Session, River 79)
„nutze den rat aber auch die ui chats (z.ai, kimi, claude, tryopenly, togetherai)" | 2026-10-02 | Operator (Session, River 79)
„Starte die River-Linie in einem Pass …" | 2026-10-02 | Operator (Session, River 81)
„1 bitte ja" (Galileo-I-tdot bauen) | 2026-10-02 | Operator (Session, River 82)
„2 bitte formales rats blatt" | 2026-10-02 | Operator (Session, River 82)
„3. bitte gebe das bei mountain und mycellium in auftrag ich möchte auch die anderen ephemeriden" | 2026-10-02 | Operator (Session, River 82)
„4 ja ticket ist angelegt" (PII/History-Redaktion) | 2026-10-02 | Operator (Session, River 82)
„5 ja als anonyme frage dispatchen" (TE über externen Steuerparameter) | 2026-10-02 | Operator (Session, River 82)
„2. wer schliesst es?" (Träger des Rats-Blatt-Risses) | 2026-10-02 | Operator (Session, River 82)
„3 ist übergeben" | 2026-10-02 | Operator (Session, River 82)
„5 bitte an den schwarm stellen und pioneer bitte an rat und schwarm mit archive search" | 2026-10-02 | Operator (Session, River 82)
„1 ja bitte" (privaten TE-Pfad entlocken; Detrend/CMI-Arm bauen, lokaler Lauf) | 2026-10-02 | Operator (Session, River 82)
„ja voranmelde und dann lauf in ci" (Pioneer-Floor-Falsifikation) | 2026-10-02 | Operator (Session, River 82)
„Starte die River-Linie in einem Pass …" / „ich musste leider neustarten" | 2026-10-02 | Operator (Session, River 83)

## Träger (Prosa, eigene)

- `docs/blatt/blatt-anderson-flyby-ephemeridenhaus.md` (`class: sheet`) — das formale
  Rats-Blatt zur Anderson-Flyby-Klasse.
- `docs/blatt/blatt-te-externer-steuerparameter.md` (`class: sheet`) — die TE-Methode über
  einen externen Steuerparameter (Wohlgestelltheit, Null, Ersatzmaß). Träger für den
  privaten `complex_te_probe`-Pfad (LOCK, Operator-Wort liegt vor).

## Offen (aufgeschlüsselt)

### TAPVizieR async-/Slice-Klasse — Sync-Timeout geheilt, Neulauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wds-cdn 37069590122` / `mktypes-cdn 37069592547` / `nvss-cdn 37069595693`
- **Lage:** (gemessen 2026-10-02 via `ci_manage log 36989774317` / `36989781545`) mktypes
  `slice 72.0` und wds `slice 0` enden in `curl: (28)` nach 180 s (1 622 125 B bzw.
  967 676 B empfangen) → `query returned void` → Workflow-`slice N returned void`. Der
  Neulauf `rave-cdn 37001533680` ist **success** nach `OMEGAFLOW_TAP_TIMEOUT: "900"`
  (river-81, `654da0efa`). nvss `uws job phase ERROR` 11 s nach `PHASE=RUN` (async);
  dieselbe Query mißt am TAPVizieR-async `EXECUTING`/`WRITING_RESULT` — keine Syntax-/
  Größen-Ablehnung, die Ursache ist serverseitig/transient. Geheilt: `OMEGAFLOW_TAP_TIMEOUT`
  `1800` (wds) / `900` (mktypes); `tap_compiler` druckt bei `ERROR`/`ABORTED` die gemessene
  `{job}/error`-Ursache statt `phase ERROR`.
- **Blockade:** keine
- **Braucht:** die drei dispatchten Läufe einmalig lesen: `ci_manage view 37069590122`
  (wds) / `37069592547` (mktypes) / `37069595693` (nvss); bei Rot `ci_manage log <id>` für
  den gemessenen Ausgang.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `.github/workflows/wds-cdn.yml`, `.github/workflows/mktypes-cdn.yml`
- `tools/harvest/src/bin/tap_compiler.rs`
- `docs/handover/handover-2026-10-02-river-folge83.md`, und `…-folge82.md` → `archiv/` (Move)

## Burn: open 0.0000 · close 0.0671 · cap 0.50 Grund: TAPVizieR async-/Slice-Heilung — langer TAP-Timeout + async-Fehlerursache, drei Läufe dispatcht (gemessen `session_burn`, River-Session `$0.0671`; Gesamt $0.2329, Parallel-Linien teilen den Total)
