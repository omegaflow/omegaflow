<!--
  title: Handover — Mountain-Folge 141 (Stand 2026-09-23)
  session: Mountain-Folge 141
  class: handover
  date: 2026-09-23
  sha256: 0f1b7c6f0560d1bb3b14c9712a98702d3f372da0a69ec9646c78e4c4594c4bc7
  status: live
-->
# Handover — Mountain-Folge 141 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Trigger / Lage / Blockade / Braucht** und
seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`) —
Status = f(Trigger), `Lage` mit Messstempel, `Braucht` der wörtliche, kopierbare
Schritt (Tafel-Verschärfung, Operator-Wort 2026-09-23, `AGENTS.md:191`).

## Stehender Pass (gemessen 2026-09-23 10:01Z)

- **HEAD** `1d23d3522` (`main`) == `origin/main` — drei Linien seit dem eigenen
  `fca319727` vorgerückt (sensory folge152 ×2, mycelium 144, river folge10).
  Eigener uncommitteter Satz: folge141 (neu) + Rename folge140 → `archiv/`.
  `git_safety --snapshot` → Ausführungs-Beginn `refs/safety/1790154086`,
  Session-Ende `refs/safety/1790154835`.
- **Postfach** — `state/mail/mail_ledger.φ` 135 Zeilen, jüngster Eingang
  `1790116573` (Rubin Observatory LSST Community forum — Summary, Maschinendigest,
  keine Aktion); keine Mountain-Zeile. `post.md`: die beiden eigenen
  mycelium-Routings (register_sort/dropped-gate; path_reference_scan) sind von
  mycelium 144 gefaltet und gelöscht; jetzt 7 `An future:`-Zeilen (fremd,
  unberührt). **Neu geroutet:** tools-build-Red (unten).
- **CI** (gemessen 10:01Z via `ci_manage list`/`view`/`log`): `te-gate
  35834155918` @`7ed618d5a` **in_progress** seit 07:54Z; `ci-check 35838658864`
  failure 09:47Z (die gerouteten Reds, mycelium trägt sie); `ci-check
  35844564961` **in_progress**; **`tools-build 35844365704` @`250f07963`
  failure 09:44Z** — Kompilierung grün (`RUSTFLAGS: -D warnings`), rot ist der
  Upload-Step `gh release upload tools-latest … --clobber --repo
  omegaflow/omegaflow` (`.github/workflows/tools-build.yml:41–64`):
  `HTTP 404: Not Found
  (https://api.github.com/repos/omegaflow/omegaflow/releases/assets/582700040)`
  — der Clobber trifft ein bereits gelöschtes Asset. → `post.md` `An mycelium:`.
- **register_lookup --open** (09:13Z) — keine mountain-eigenen
  `DISPOSITION`-Zeilen; `INDEX 11 offen`, `CANDIDATES 1 → mycelium`.
- **open_points_check** — folge141: 0 absent (der stale `verdict.rs`-Pfad aus
  folge140 ist korrigiert geführt: real
  `src/mathematikerin/machines/verdict.rs:12`).

## Offen (aufgeschlüsselt)

### 1. KSG K-Regel — `TE_KSG_K_PROD` flippen
- **Status:** termin (CI) | **Bindung:** eigen
- **Trigger:** Ende des Laufs `te-gate 35834155918`
- **Lage:** Lauf **in_progress** seit 07:54Z (gemessen 2026-09-23 10:01Z via
  `ci_manage view 35834155918`); `TE_KSG_K_PROD` = **0**
  (`src/mathematikerin/machines/verdict.rs:12`); Apparat
  `src/mathematikerin/ksg_k.rs`, Step `ksg-sweep` in
  `.github/workflows/te-gate.yml`.
- **Blockade:** Messung liegt nicht vor (Lauf läuft).
- **Braucht:** `ci_manage log 35834155918` — Formel und Sweep übereinstimmend →
  Operator setzt das gemessene k in `src/mathematikerin/machines/verdict.rs:12`;
  bei `riss`/`void` bleibt K=0, der Log ist der Befund. Nie pollen. Derselbe
  Log bedient die zustand-Zeile „TE-Gate n=1000 FPR-Boden".

### 2. opencode-Browser Pfad 1 — Kaltstart (gemeldet „verbindet sich nicht")
- **Status:** operator-gebunden | **Bindung:** dritter
- **Trigger:** Operator-Wort — Kaltstart akzeptieren **oder**
  Extension-Änderung anstoßen
- **Lage:** gemessen **kein harter Ausfall** (gemessen 2026-09-23 via
  Session-Start-`browser_targets` + Messnachtrag
  `docs/surveys/survey-2026-09-20-browser-anbindung.md`): die Store-Extension
  (`cabnfapnafjlijmbpmgjkgobhdkbmpci` 0.16.1) verbindet sich erst beim nächsten
  Service-Worker-Wachruf; Abstand Broker→Executor bimodal (4–8 s warm,
  67–2276 s kalt); Reconnect ist ein `setTimeout`-Backoff im MV3-Worker (kein
  `chrome.alarms`). Offene Versionslücke: Plugin 0.17.0 vs Extension 0.16.1
  (drop-in, Protokoll v1).
- **Blockade:** Heilung braucht eine Extension-Änderung (dritter, Store) — nicht
  aus `bridge.json`/`opencode.jsonc` lösbar.
- **Braucht:** Operator-Wort auf die vorgelegte Frage im Survey-Nachtrag.

### 3. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Geräteankunft (Tracking `LZ473049629CN`)
- **Lage:** PINE64 hat zwei Ox64 SBCs versandt, physisch nicht angekommen
  (gemessen 2026-09-23 via `state/mail/mail_ledger.φ` `1790046330`).
- **Blockade:** physische Ankunft.
- **Braucht:** nach Ankunft Bring-up + Kopplung messen.

## Benchmark

- Punkt „ci-check `test`-Job-Heil-Confirm" (aus folge140) für Mountain
  **geschlossen** — per `grind-flash` gemessen: Run `35831089754` @`629486b77`,
  Kern-Suite `1510 passed; 0 failed`; die fünf Heilungen @`634437817` halten.
  Routine-Klasse (CI-Artefakt-Extraktion), kein Doppel-Lauf (Routine-Klasse seit
  2026-09-16 geschlossen, flash-Sieger registriert).
- Tafel-Abklopfen gegen die sensory-folge152-Verschärfung (Trigger-Spalte,
  Lage-Messstempel, wörtlicher Schritt): eigene Umformung der drei offenen
  Punkte, kein Sub-Agenten-Lauf (Struktur, kein Messatom).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-23-mountain-folge141.md` (neu)
- Move `handover-2026-09-23-mountain-folge140.md` → `archiv/` (eigene Linie,
  atomar; Rename staged)
- `docs/handover/post.md` — neue mycelium-Zeile (tools-build-404-Red)
- Zustand-Fortschreibung CI-Status lokal (`docs/zustand/external-state.md`,
  gitignored laut `.gitignore:134` — nicht committbar)
- **Fremd:** die 7 `An future:`-Zeilen in `post.md` unberührt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
