<!--
  title: Handover — Mountain-Folge 152 (2026-09-24)
  session: Mountain-Folge 152
  class: handover
  date: 2026-09-24
  sha256: 57ebb1dea8cdc1be7469af1abb18a4f27f06b9a21585191b48001d43d1a49a57
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

- **HEAD** `57e2ba3c8` (*te-ncurve: raise the tcurve job timeout to 360 min — the
  300 cap cut the run at 5h0m twice*). Arbeitsbaum sauber (`git status` leer);
  `origin/main` == HEAD.
- **Postfach:** `state/mail/mail_ledger.φ` vorhanden; kein Mountain-Eingang.
- **CI** (live `ci_manage list`/`view`, 2026-09-24): `ci-check 36030755250`
  @`e05f8a419` **failure** — rote Tests
  `bayestar::tests::load_map_leaf_record_finds_the_pixel` (bayestar.rs:494) und
  `ble::tests::managed_objects_reply_resolves_device_and_characteristic`
  (ble.rs:1116); clippy `bayestar.rs:352` (chunks_exact) + `spectral.rs:752`
  (needless_range_loop). `register-dropped 36030528745` **success**.
  `health-check 35990890566` **failure** (Runner-Shutdown 17:59Z).
  `te-ncurve 36052804297` in_progress; `ps1-cdn 36046155392` in_progress.
- **`register_lookup --open`:** 117 Docs, 589 offen — **keine
  `[mountain]`-Zustandszeile**; `ledger`/`witnesses` → mycelium.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### Bayestar-Code-Rot
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner — sofort handlungsfähig (Rot gemessen)
- **Lage:** (gemessen 2026-09-24 via `ci_manage log 36030755250`)
  `src/archivar/bayestar.rs:352` chunks_exact (clippy) +
  `bayestar::tests::load_map_leaf_record_finds_the_pixel` (bayestar.rs:494) rot
  am `ci-check 36030755250` @`e05f8a419`; parser-def — aus Mycelium hierher verschoben.
- **Blockade:** keine.
- **Braucht:** `chunks_exact` → `as_chunks` heilen; `cargo check --tests` 0/0;
  CI grün.

### AFAD-Block-Verifikation
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** gefeuert — `health-check 35990890566` **failure**
  (Runner-Shutdown 17:59Z)
- **Lage:** (gemessen 2026-09-24 via `ci_manage list`) `health-check 35990890566`
  **failure** (Runner-Shutdown 17:59Z); `source-census 35923610972` success.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 35990890566 --all` (AFAD-Fetch + `--verify phi`) —
  dann entscheiden.

#### Stufe 2 — operator-gebunden

keiner.

#### Stufe 3 — blockiert

keiner.

#### Stufe 4 — wartend

### dropped-Baseline 960 (Bump bei Drift)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Drift der dropped-Nettozahl gegen die Baseline (Delta > 0) im
  annehmenden Commit
- **Lage:** Baseline **960** @`5179b438b`, gebumpt durch `df2b23320` (gemessen
  2026-09-24 via `docs/zustand/dropped-baseline.md`); das dropped-Gate ist grün
  (kein Delta); der Sweep-Mechanismus (`register-dropped`) lebt in Mycelium —
  Mountain hält nur die Baseline + Bump-Pflicht.
- **Blockade:** keine.
- **Braucht:** bei Drift (Delta > 0) im annehmenden Commit die Baseline in
  `docs/zustand/dropped-baseline.md` nachziehen.

#### Stufe 5 — termin

keiner.

#### Stufe 6 — LOCK

keiner.

## Benchmark

- Kein Doppel: dieser Atom ist Registerarbeit (Stufen-Umbau +
  Punkt-Reconciliation) und dispatcht keine Sub-Agenten — keine
  Benchmark-Klasse, kein Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-24-mountain-folge152.md` (neu)
- Move folge151 nach
  `docs/handover/archiv/handover-2026-09-24-mountain-folge151.md` (eigene Linie,
  atomar, git trägt)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
