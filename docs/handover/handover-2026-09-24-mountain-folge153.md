<!--
  title: Handover — Mountain-Folge 153 (2026-09-24)
  session: Mountain-Folge 153
  class: handover
  date: 2026-09-24
  sha256: fcaaca870e80be6a7e7dffa6d451a1315dc545a67e0f7f06f61d20c29e787d73
  status: live
-->
# Handover — Mountain-Folge 153 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder offene Punkt wird **aufgeschlüsselt**
geführt — kein Register-Kürzel: **Trigger** (das Ereignis/Datum/Wort/der Lauf,
dessen Eintreffen den Punkt kippt — Status = f(Trigger)) / **Lage** (der Zustand,
gemessen, mit Messstempel) / **Blockade** (warum es hängt, oder „keine") /
**Braucht** (was es löst: der wörtliche, kopierbare Schritt).

**Vorbereitung ≠ Akt (Operator-Wort 2026-09-24).** Ein `operator-gebundener`
Punkt wird **immer** in zwei Zeilen getrennt geführt, nie in einer: die
**Vorbereitung** ist autonom (`eigen`), läuft bis zur Kante; `operator-gebunden`
ist **allein der Akt**. Es gibt keinen „kompletten Prozess operator-gebunden".

**Sortierung — von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit:** autonom
(`eigen`, dispatcht) → operator-gebunden → blockiert → wartend → termin → LOCK.
Innerhalb einer Stufe nach Trigger. Stufe 2–6 werden benannt, nie dispatcht —
ausgenommen die Vorbereitung eines operator-gebundenen Punktes.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung.

## Stehender Pass (gemessen 2026-09-24, Mountain-Folge 153)

- **HEAD** `f417cdd69` (*handover: abolish post.md (Aufenthalt = Eigentum) and
  reconcile the five live handovers against code/git/CI*); `origin/main` == HEAD.
  Arbeitsbaum: **fremde** uncommittete Änderungen vorhanden
  (`docs/specs/mantis-shrimp-bom.md`, `src/archivar/fit.rs`,
  `src/archivar/spectral.rs`, `src/gate/commit_gate.rs`,
  `tools/register/src/bin/open_points_check.rs`, `tools/service/src/bin/smail.rs`)
  — nicht berührt, gehören anderen Linien im geteilten Baum; eigener Pfad-Satz
  unten.
- **Postfach:** `state/mail/mail_ledger.φ` **fehlt** → `pending` (Mail-Ledger-Bau
  in CI, `tools-build`); kein Mountain-Eingang.
- **CI** (gemessen 2026-09-24 via `ci_manage list`): frische Läufe am HEAD
  `f417cdd69` in_progress — `ci-check 36059450659`, `tools-build 36059450770`,
  `register-dropped 36059450771`; `health-check 36059360178` pending. Letzter
  abgeschlossener `ci-check 36030755250` @`e05f8a419` **failure** (Test-Red
  `bayestar::tests::load_map_leaf_record_finds_the_pixel` +
  `ble::tests::managed_objects_reply_resolves_device_and_characteristic`;
  clippy `bayestar.rs:352` chunks_exact + `spectral.rs:752` needless_range_loop).
  Der `ble`-Test gehört **sensory** (ble.rs uncommittet, sensory folge158:152).
- **`register_lookup --open`:** 116 Docs, 582 offen — **keine
  `[mountain]`-Zustandszeile**; `ledger`/`witnesses` → mycelium.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

keiner (die zwei handlungsfähigen Punkte der Session sind abgearbeitet).

#### Stufe 2 — operator-gebunden

keiner.

#### Stufe 3 — blockiert

keiner.

#### Stufe 4 — wartend

### Bayestar-CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des `ci-check`-Laufs am Commit dieser Session (nach
  Push)
- **Lage:** (gemessen 2026-09-24 via `cargo check --tests` + `git diff`) der
  Clippy-Red `chunks_exact` @`bayestar.rs:352` ist auf
  `body.as_chunks::<REC_BYTES>().0.iter()` geheilt (Hausmuster `6318eea0`); die
  Leaf-Test-Erwartung @:494 war falsch — `bf[3]` trägt μ = `BE19_MU0 +
  3.0 * BE19_DMU` = 4.375, nicht 6.0 (6.0 traf Bin 16 = 0.0), Erwartung auf den
  Bin-3-Modulus korrigiert; `cargo check --tests` = 0 Fehler / 0 Warnungen. Das
  CI-Urteil steht aus (Testläufe sind lokal strukturell verboten).
- **Blockade:** keine.
- **Braucht:** nach dem Push `ci_manage view <run-id>` **einmal** lesen (oder den
  Watchdog-Snapshot `/tmp/opencode/ci_status.md` beim nächsten Pass); bei Rot
  `ci_manage log <run-id> --all`.

### dropped-Baseline 960 (Bump bei Drift)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Drift der dropped-Nettozahl gegen die Baseline (Delta > 0) im
  annehmenden Commit
- **Lage:** Baseline **960** @`5179b438b` (gemessen 2026-09-24 via
  `docs/zustand/dropped-baseline.md`); das dropped-Gate ist grün (kein Delta).
- **Blockade:** keine.
- **Braucht:** bei Drift (Delta > 0) im annehmenden Commit die Baseline in
  `docs/zustand/dropped-baseline.md` nachziehen.

#### Stufe 5 — termin

keiner.

#### Stufe 6 — LOCK

keiner.

## Benchmark

- **AFAD-Block-Verifikation** → `grind-flash` (Routine-Klasse; der flash-Sieger
  der Routine-Klasse ist bereits gemessen — kein Doppel). Ergebnis: der
  `health-check`-Ausfall war ein externer Runner-Shutdown (transient), AFAD
  `JSON ok` / live HTTP 200 — kein Quellenfehler.
- **Bayestar-Code-Rot** → `grind-pro` (parser-def-Urteil + Schreiben in einem
  Kontext; der Leaf-Test ist kein mechanischer Fix). Kein Doppel dieses Atom.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/bayestar.rs` (M)
- `docs/handover/handover-2026-09-24-mountain-folge153.md` (neu)
- Move folge152 nach
  `docs/handover/archiv/handover-2026-09-24-mountain-folge152.md` (eigene Linie,
  atomar, git trägt)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
