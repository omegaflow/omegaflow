<!--
  title: Handover — Mountain-Folge 148 (2026-09-23)
  session: Mountain-Folge 148
  class: handover
  date: 2026-09-23
  sha256: e54027ce458b53a16f88baf1a78ffe08279d4606a3e0eab24c246d5dfafc50d5
  status: live
-->
# Handover — Mountain-Folge 148 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt — kein Register-Kürzel: **Trigger** (das
Ereignis/Datum/Wort/der Lauf, dessen Eintreffen den Punkt kippt — Status =
f(Trigger)) / **Lage** (der Zustand, gemessen, mit Messstempel) / **Blockade**
(woran es hängt, oder „keine") / **Braucht** (was es löst: der wörtliche,
kopierbare Schritt — Werkzeug, Datei, URL, Befehl, Anfrage, Operator-Wort;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt). Kein Dokument wächst ohne
Messung; die Droh-Sprache ersetzt den Schritt nicht. Der Planungs-Pass legt
**alle** eigenen Punkte vor und schlägt vor, jeden parallel abarbeitbaren zu
dispatchen; `operator-gebunden`, `blockiert` und `wartend` werden benannt, nie
dispatcht. Gibt es keinen abarbeitbaren Punkt, sagt die Session das. Jeder Punkt
trägt seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung; `open_points_check` prüft billig jeden in den offenen Punkten
genannten Pfad gegen den Arbeitsbaum (absent = stale Punkt); eine Session, die nur
dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-23 ~23:4x lokal / 21:49Z)

- **HEAD** `0c7682916` (river folge18); `origin/main == HEAD`, Arbeitsbaum
  **sauber** (git_safety: tree == HEAD). Keine fremde uncommittete Arbeit.
- **Postfach:** `state/mail/mail_ledger.φ` absent → keine Mountain-Zeile.
- **CI** (`ci_manage view`, jetzt): `register-dropped 35922188109` @`48ae5e734`
  **success** (21:49Z, der P4-Trigger); `te-gate 35893882101` @`6a68c1901`
  **in_progress** (upd 20:03Z); `free-model-bench 35893010538` @`d754ecdf8`
  **in_progress** (upd 20:49Z); `health-check 35923606472` + `source-census
  35923610972` @`d87a0c9b3` **pending**.
- **`register_lookup --open`:** 116 Docs, 581 offene Zeilen — **keine
  mountain-eigene Zustandszeile** (alle Dispositions `[mycelium]`/`[wartend]`).
- **`open_points_check` folge147:** 25 Pfad-Refs, 0 absent (beim Start gemessen).

## In diesem Atom gebaut (git trägt)

- **P4 `--dropped`-Zählwurzel aufgelöst.** `register-dropped 35922188109`
  @`48ae5e734` lieferte `553 pairs, 5713 candidates, 3186 dropped, 2508
  commit-resolved, persist >= 1` → **N = Z − R = 678**. Fix (`grind-flash`,
  flash-first): in `run_dropped` (`tools/register/src/bin/register_lookup.rs`) den
  `count_only`-Shortcut hinter die Git-Auflösung geschoben (die Auflösung läuft
  immer), die DROPPED-`println!` unter `if !count_only`, der Zähldruck druckt
  `dropped - resolved`; Usage-Text angepasst. `docs/zustand/dropped-baseline.md`:
  `3053 → 678` + Mess-Stempel (`@48ae5e734`, run `35922188109`), Header-sha256 neu
  (vorher stale `ce9e7405…`). `cargo check` (workspace): **0 Fehler, 0 Warnungen**;
  die Voll-Messung blieb in CI (kein lokaler `--dropped`-Lauf).

## Offen (aufgeschlüsselt)

### 1. dropped-gate grün nach dem `--count`-Fix
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss der Runs, die dieser Push auslöst (`register-dropped` +
  `ci-check`) — Commit/Push desselben Atoms
- **Lage:** Commit steht noch aus (diese Session); neue Runs noch nicht dispatcht
  (gemessen 2026-09-23 lokal)
- **Blockade:** das Commit-Push-Wort (`/commit`)
- **Braucht:** nach Push `ci_manage list` → die neuen `register-dropped`/`ci-check`
  Ids; `ci_manage view <id>` → `dropped-gate: baseline 678 | current … | delta ≤ 0`

### 2. AFAD-Block-Verifikation (aus folge147)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss `health-check 35923606472` / `source-census 35923610972`
- **Lage:** beide **pending** (gemessen 2026-09-23 via `ci_manage view`)
- **Blockade:** CI-Runner-Queue
- **Braucht:** `ci_manage view 35923606472` (`--verify phi` grün) +
  `ci_manage log 35923610972` (AFAD-Fetch ok, echte Samples); bei rot den
  `map`-Arm/die Keys in `phi/sources.φ` nachbessern

### 3. flare-Re-Insert green-confirm (aus folge147)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss `te-gate 35893882101` @`6a68c1901`
- **Lage:** **in_progress** (gemessen 2026-09-23 via `ci_manage view`, upd 20:03Z)
- **Blockade:** Laufdauer (Job-Timeout 330 min)
- **Braucht:** `ci_manage view 35893882101` → `flare`-Job grün (assert +
  `flare power probe:`-Zeilen)

### 4. Free-Model-Bench T4 + CF-`http_401` (aus folge147)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss `free-model-bench 35893010538` @`d754ecdf8`
- **Lage:** **in_progress** (gemessen 2026-09-23 via `ci_manage view`, upd 20:49Z)
- **Blockade:** Runner-Queue
- **Braucht:** `ci_manage log 35893010538 --all` → T4-pass-counts; bei
  vorhandenem CF-`http_401`-Anteil eine `An future:`-Zeile (operator-gebunden)

### 5. Ox64-Zweitknoten (aus folge147)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Geräteankunft (Tracking `LZ473049629CN`)
- **Lage:** nicht angekommen (gemessen via `state/mail/mail_ledger.φ` `1790046330`)
- **Blockade:** physische Ankunft
- **Braucht:** nach Ankunft Bring-up + Kopplung messen

## Benchmark

- Ein Taucher: der P4-Fix, **grind-flash** (billigste Klasse) — mechanischer,
  präzise spezifizierter Fix (deterministisch aus der CI-Summary-Zeile ableitbar).
  Kein pro/max-Doppel — Routine-Klasse geschlossen (2026-09-16, flash Sieger
  `$0.0008`); kein hartes Atom.

## Geteilter Baum — eigener Pfad-Satz

- `tools/register/src/bin/register_lookup.rs` (`--count`-Fix + Usage)
- `docs/zustand/dropped-baseline.md` (baseline `678`, Mess-Stempel, Header-sha256)
- `docs/handover/handover-2026-09-23-mountain-folge148.md` (neu)
- Move `handover-2026-09-23-mountain-folge147.md` → `archiv/` (eigene Linie, atomar)
- **Fremd/unberührt:** keine.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
