<!--
  title: Handover — River-Folge 84 (2026-10-03)
  session: River-Folge 84
  class: handover
  date: 2026-10-03
  sha256: 24ae3191207001c75c99564505333f0553630ea33d0f9a99f96f3257eed3584e
  status: live
-->
# Handover — River-Folge 84 (2026-10-03)

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
„Starte die River-Linie in einem Pass …" | 2026-10-03 | Operator (Session, River 84)

## Träger (Prosa, eigene)

- `docs/blatt/blatt-anderson-flyby-ephemeridenhaus.md` (`class: sheet`) — das formale
  Rats-Blatt zur Anderson-Flyby-Klasse.
- `docs/blatt/blatt-te-externer-steuerparameter.md` (`class: sheet`) — die TE-Methode über
  einen externen Steuerparameter (Wohlgestelltheit, Null, Ersatzmaß). Träger für den
  privaten `complex_te_probe`-Pfad (LOCK, Operator-Wort liegt vor).

## Offen (aufgeschlüsselt)

### nvss async — Server-Phase ERROR nur auf dem Runner-Pfad, Ursache offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `nvss-cdn`-Lauf oder eine gemessene Server-Ursache am TAPVizieR-async
- **Lage:** (gemessen 2026-10-02 23:12 via `ci_manage view 37069595693` + `log`) `nvss-cdn
  37069595693` failure: async job `1790982742122` → `phase ERROR` 11 s nach `PHASE=RUN`;
  der neue `{job}/error`-Arm liefert leer. Zweiter Lauf `36989806823` ebenso ERROR. Dieselbe
  Query läuft von der Operator-Maschine als `EXECUTING`/`WRITING_RESULT` — der Fehler tritt
  nur auf dem Runner-Pfad auf. **Die Sync-Timeout-Klasse ist geschlossen:** wds
  `37069590122` und mktypes `37069592547` sind nach `OMEGAFLOW_TAP_TIMEOUT: "1800"` / `"900"`
  **success**.
- **Blockade:** die Server-Ursache ist nicht gemessen (`{job}/error` leer)
- **Braucht:** den nvss-Workflow RA-chunked wie wds/mktypes fahren (je Slice `--async`);
  Litmus: `--limit` senken und den eigenen Runner-Job `<errorSummary>` lesen.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/mathematikerin/machines/tests.rs`
- `docs/blatt/blatt-anderson-flyby-ephemeridenhaus.md`, `docs/blatt/blatt-te-externer-steuerparameter.md`
- `docs/handover/handover-2026-10-02-river-folge83.md` → `archiv/` (Move)
- `docs/handover/handover-2026-10-03-river-folge84.md`

## Burn: open 0.0027 · close 0.0154 · cap 0.50 — Grund: River-84 — OMX3-Testmagie geheilt, Blatt-Titel ≤75, matrix-rotor-Ursache als Runner-Shutdown gemessen (gemessen `session_burn`, River-Session)
