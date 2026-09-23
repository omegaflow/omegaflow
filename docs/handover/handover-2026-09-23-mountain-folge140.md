<!--
  title: Handover — Mountain-Folge 140 (Stand 2026-09-23)
  session: Mountain-Folge 140
  class: handover
  date: 2026-09-23
  sha256: 1294f3650f5f269d4aa3142f4b1c27f442e3182e6fbc7098b0d22927b8eda4c5
  status: live
-->
# Handover — Mountain-Folge 140 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-23, Ausführungs-Beginn)

- **HEAD** `2339533c2` (`main`), Arbeitsbaum sauber; `git_safety --snapshot` beim
  Pass. (folge139-Commit war `634437817`; HEAD ist seither von anderen Linien
  vorgerückt.)
- **Postfach** — `post.md` trug eine Mountain-Zeile (sfetch-Body-Riss); eingefaltet
  und gelöscht. Kein `state/mail/mail_ledger.φ`, kein offener Eingang.
- **CI** (Watchdog `/tmp/opencode/ci_status.md` 10:49 + `ci_manage list`):
  **in_progress** `te-gate 35834155918` @`7ed618d5` (KSG-k-Messung), `ci-check
  35831089754` @`629486b77`, `health-check 35813434762`; **rot (attempt 1)**
  `ci-check 35806971846`/`35805438445`/`35796139491`/`35794686502`/`35789108042`.
  Geteilter Zustand: `docs/zustand/external-state.md` CI-Zeile (nennt `35834155918`).

## Offen (aufgeschlüsselt)

### 1. sfetch-Body-Riss — geheilt, Fix im Baum
- **Status:** wartend (Commit) | **Bindung:** eigen
- **Lage:** `sfetch` lief den Tag-Strip unbedingt, auch auf Text: `rprm.c` 15940 B
  (curl/`--sniff`) → 15659 B, Differenz 275 B in `<…>`-Spannen + 6 darin liegende
  Zeilenumbrüche. Geheilt: nur echtes Markup (führendes `<`) wird gestrippt, sonst
  fließen die Rohbytes (`tools/utils/src/bin/sfetch.rs`, Gate `is_markup`, Test
  `markup_gate`); `cargo check -p omegaflow-utils --bin sfetch` und `--tests` 0/0.
- **Blockade:** Commit/Push stehen aus (Operator-Wort `/commit`).
- **Braucht:** `/commit` (pfad-begrenzt: `tools/utils/src/bin/sfetch.rs`, dieses
  Handover, `post.md`, Survey).

### 2. KSG K-Regel — `TE_KSG_K_PROD` flippen
- **Status:** termin (CI) | **Bindung:** eigen
- **Lage:** Lauf `te-gate 35834155918` @`7ed618d5` in_progress (die KSG-Messung);
  `TE_KSG_K_PROD` bleibt **0** (`src/mathematikerin/verdict.rs:12`); Apparat
  `src/mathematikerin/ksg_k.rs`, Step in `.github/workflows/te-gate.yml:40`.
- **Blockade:** Messung liegt noch nicht vor (Lauf läuft).
- **Braucht:** `ci_manage log 35834155918` **einmal**, sobald beendet → stimmen
  Formel und Sweep überein, flippt der **Operator** `TE_KSG_K_PROD` auf das
  gemessene k; bei `riss`/`void` bleibt K=0, der Log ist der Befund. Nie pollen.

### 3. ci-check `test`-Job — archive_search-Heil-Confirm
- **Status:** termin (CI) | **Bindung:** eigen
- **Lage:** die fünf Logik-Heilungen liegen (`634437817`); `ci-check 35831089754`
  @`629486b77` in_progress. Mitgetragen (post an mycelium, nicht Mountain):
  `register phi/sources.φ` ttl-/url-order-Verstöße + `dropped-gate`-Baseline-Delta.
- **Blockade:** Testlauf nur CI (lokaler Lauf strukturell verweigert).
- **Braucht:** `ci_manage log <ci-check@HEAD>` einmal nach Ende — grün schließt den
  Mountain-Teil; rot heißt neuen Test-Ausgang lesen.

### 4. opencode-Browser Pfad 1 — Kaltstart (gemeldet „verbindet sich nicht")
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** gemessen **kein harter Ausfall**: `browser_targets` im Session-Start
  leer, die Store-Extension (`cabnfapnafjlijmbpmgjkgobhdkbmpci` 0.16.1) verbindet
  sich erst beim nächsten Service-Worker-Wachruf; Abstand Broker→Executor bimodal
  (4–8 s warm, 67–2276 s kalt). Reconnect ist ein `setTimeout`-Backoff im MV3-Worker
  (kein `chrome.alarms`). Token/Port stimmen, ein LISTEN-Socket auf 4517. Messnachtrag
  in `docs/surveys/survey-2026-09-20-browser-anbindung.md`.
- **Blockade:** Heilung braucht eine Extension-Änderung (dritter, Store) — nicht aus
  `bridge.json`/`opencode.jsonc` lösbar.
- **Braucht:** Operator-Wort: Kaltstart akzeptieren (Werkzeuge erst nach
  Executor-Connect rufen) **oder** Extension-Änderung anstoßen. Offene
  Versionslücke: Plugin 0.17.0 vs Extension 0.16.1 (drop-in, Protokoll v1).

### 5. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat zwei Ox64 SBCs versandt (Tracking `LZ473049629CN`); physisch
  nicht angekommen.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten → Bring-up + Kopplung messen.

## Benchmark

- Punkt 1 (sfetch-Riss) ist Routine-Klasse (Logik-Defekt in `tools/utils`), per
  `grind-flash` gemessen + geheilt — kein Doppel-Lauf (Routine-Klasse seit
  2026-09-16 geschlossen, flash-Sieger registriert).
- Punkt 4 (Browser): `general`/flash, Diagnose vollständig, kein pro/max-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-23-mountain-folge140.md` (neu)
- Move `handover-2026-09-23-mountain-folge139.md` → `archiv/` (eigene Linie, atomar)
- `tools/utils/src/bin/sfetch.rs` — Markup-Gate statt unbedingtem Tag-Strip
- `docs/handover/post.md` — Mountain-Zeile eingefaltet und gelöscht
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` — Messnachtrag Pfad-1-Kaltstart
- **Fremd:** fremde uncommittete Arbeit wird nicht berührt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
