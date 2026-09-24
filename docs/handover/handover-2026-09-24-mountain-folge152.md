<!--
  title: Handover — Mountain-Folge 152 (2026-09-24)
  session: Mountain-Folge 152
  class: handover
  date: 2026-09-24
  sha256: 820729140372d8b2bbe9bfdf681486749785de71bcda3a46a69efa9c60232beb
  status: live
-->
# Handover — Mountain-Folge 152 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits,
der Arbeitsbaum darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt — kein Register-Kürzel: **Trigger** (das
Ereignis/Datum/Wort/der Lauf, dessen Eintreffen den Punkt kippt — Status =
f(Trigger)) / **Lage** (der Zustand, gemessen, mit Messstempel) / **Blockade**
(warum es hängt, oder „keine") / **Braucht** (was es löst: der wörtliche,
kopierbare Schritt — Werkzeug, Datei, URL, Befehl, Anfrage, Operator-Wort).

**Vorbereitung ≠ Akt (Operator-Wort 2026-09-24).** Ein `operator-gebundener`
Punkt wird **immer** in zwei Zeilen getrennt geführt, nie in einer: die
**Vorbereitung** ist autonom (`eigen`), läuft bis zur Kante — Draft, Adresse,
QUELLEN, `smail --dry-run`, Feldmap, Messung, Kantenzeile (Artefakt |
Ausführbefehl | Wort erwartet) — und wird dispatcht; `operator-gebunden` ist
**allein der Akt** (senden/absenden/signieren/urteilen/wählen). Eine
`operator-gebunden`-Zeile ohne abgetrennte Vorbereitungs-Zeile behauptet den
ganzen Prozess als gesperrt und droppt die Vorbereitung — ein Registraturfehler.
Es gibt keinen „kompletten Prozess operator-gebunden".

**Sortierung — von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit:** autonom
(`eigen`, dispatcht) → operator-gebunden (nur der Akt; Vorbereitung ist Stufe 1)
→ blockiert → wartend → termin → LOCK. Innerhalb einer Stufe nach Trigger. Der
Planungs-Pass dispatcht von oben nach unten; Stufe 2–6 werden benannt, nie
dispatcht — ausgenommen die Vorbereitung eines operator-gebundenen Punktes.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung.

## Stehender Pass (gemessen 2026-09-24, Mountain-Folge 152)

- **HEAD** `0f7a6b0de` (*handover: fold the foreign-model review result and method
  to mycelium/future…*). Arbeitsbaum trägt fremde uncommittete Arbeit:
  `docs/handover/_template.md` (neues Gerüst) und `docs/handover/post.md`
  (Rundruf an fünf Linien) — unberührt gelassen.
- **Postfach:** `state/mail/mail_ledger.φ` vorhanden; kein Mountain-Eingang.
  `post.md` trug `An mountain: Vorbereitung ≠ Akt … (Schritt: nächsten
  Planungs-Pass darauf abgleichen.)` — **gelesen und in diese Übergabe gefaltet**
  (Gerüst `_template.md` gezogen: Zwei-Zeilen-Trennung, neue Sortier-Liste,
  Messstempel-Pflicht); die Zeile aus `post.md` **gelöscht**. Kein aktueller
  Mountain-Punkt ist `operator-gebunden` — die Trennung greift beim nächsten
  solchen Punkt.
- **CI** (live `ci_manage list`/`view`, 2026-09-24): `ci-check 36023479649`
  @`df2b23320` (Mountain-Commit) **in_progress**; `ci-check 36027326789`
  @`0f7a6b0de` **pending**; `health-check 35990890566` **in_progress** (seit
  11:04Z); `te-ncurve`/`ps1-cdn` in_progress.
- **`register_lookup --open`:** 116 Docs, 572 offen — **keine
  `[mountain]`-Zustandszeile**; `ledger`/`witnesses` → mycelium.
- **`open_points_check` folge151:** 13 Pfad-Refs, **1 absent**
  (`.../mountain-folge150.md:103` — die Move-Zeile; folge150 liegt in `archiv/`,
  stale Referenz, kein Punkt), 0 guardians, 0 format-gaps.
- **`git_safety --snapshot`:** Arbeitsbaum == HEAD (sauber vor diesem Atom).

## Offen (aufgeschlüsselt)

### 1. AFAD-Block-Verifikation
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** Abschluss `health-check 35990890566`
- **Lage:** **in_progress** seit 11:04Z (gemessen 2026-09-24 via `ci_manage list`); `source-census 35923610972` success.
- **Blockade:** CI-Runner-Queue.
- **Braucht:** `ci_manage view 35990890566`; bei rot `ci_manage log 35990890566 --all` (AFAD-Fetch + `--verify phi`).

### 2. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Geräteankunft (Tracking `LZ473049629CN`)
- **Lage:** nicht angekommen (gemessen 2026-09-24 via `state/mail/mail_ledger.φ`).
- **Blockade:** physische Ankunft.
- **Braucht:** nach Ankunft Bring-up + Kopplung messen.

### 3. dropped-gate am eigenen Commit verifizieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss `ci-check 36023479649` @`df2b23320` (Folge-151-Commit)
- **Lage:** Baseline 960 (gemessen 2026-09-24 via `docs/zustand/dropped-baseline.md`); der Gate-Wert am eigenen Commit ist noch nicht gemessen — der Lauf ist **in_progress** (gemessen 2026-09-24 via `ci_manage view 36023479649`).
- **Blockade:** keine.
- **Braucht:** Abschluss `36023479649` lesen; bei `delta > 0` Baseline im annehmenden Commit nachziehen (die `git: none`-Gruppen sind akkumulierte Historie ab 2026-09-09, nicht durch Routing auflösbar).

### 4. Vollständiger Drop-Sweep (Diff-Gate über alle Linien)
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** Abschluss `register-dropped 36030238985` (workflow_dispatch 16:51Z; parallel `36029814914` in_progress seit 16:47Z)
- **Lage:** **queued** (gemessen 2026-09-24 16:51Z via `gh run list --workflow=register-dropped`); ein Voll-Scan dauerte zuvor ~25 min (`36000037355`).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36030238985 --all` → vollständige `DROPPED`-Liste je Linie; die `git: none`-Einträge (stille Drops) der Linie `mountain` (inkl. Vor-Portierungs-Linie `bau`) in folge153 als offene Punkte aufnehmen; Netto-Zahl gegen `docs/zustand/dropped-baseline.md` (960) halten und bei Drift den Baseline-Bump im annehmenden Commit nachziehen.

## Benchmark

- Kein Doppel: dieser Atom ist Registerarbeit (Post-Fold + Gerüst-Abgleich) und
  dispatcht keine Sub-Agenten — keine Benchmark-Klasse, kein Lauf. Die
  `clippy`/`cargo check`-Verifikation der Folge-151-`port.rs`-Arbeit trägt der
  laufende `ci-check 36023479649`.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-24-mountain-folge152.md` (neu)
- `docs/handover/post.md` (eigene `An mountain:`-Zeile gelöscht)
- Move folge151 nach `docs/handover/archiv/handover-2026-09-24-mountain-folge151.md` (eigene Linie, atomar, git trägt)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
