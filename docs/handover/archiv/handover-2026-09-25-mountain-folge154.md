<!--
  title: Handover — Mountain-Folge 154 (2026-09-25)
  session: Mountain-Folge 154
  class: handover
  date: 2026-09-25
  sha256: d8dd3d609d1f7d2088d79d4a71d04784400ddf444650c6aeaf3b90007008664a
  status: live
-->
# Handover — Mountain-Folge 154 (2026-09-25)

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
geführt — kein Register-Kürzel: **Trigger** (Status = f(Trigger)) / **Lage** (der
Zustand, gemessen, mit Messstempel) / **Blockade** (oder „keine") / **Braucht** (der
wörtliche, kopierbare Schritt). Sortierung von Handlungsfähigkeit zu
Nicht-Handlungsfähigkeit: autonom → operator-gebunden → blockiert → wartend →
termin → LOCK.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung.

## Stehender Pass (gemessen 2026-09-25, Mountain-Folge 154)

- **HEAD** `de76fda6a`; `origin/main` == HEAD. Arbeitsbaum: **fremde** uncommittete
  Änderung `src/archivar/fit.rs` (andere Linie im geteilten Baum) — nicht berührt.
  Eigener Pfad-Satz unten.
- **Postfach:** `state/mail/mail_ledger.φ` vorhanden; 6 Eingänge (2026-09-24:
  GitHub-Support, Resend-Bounces, STScI News, Rubin-Forum) — **kein
  Mountain-Eingang**.
- **CI** (gemessen 2026-09-25 via `ci_manage view`/`list`/`log`): `ci-check
  36064053750` @`de76fda6a` **in_progress** (created 2026-09-24T21:51:21Z, updated
  21:58:07Z). Alle `ci-check`-Läufe zwischen `1c42ed09e`…`cfb8a1e6` sind
  **cancelled** — der jeweils nächste Push queued einen neuen Lauf in derselben
  Gruppe `ci-check-${{ github.ref }}` (`ci-check.yml:17`), der vorherige **pending**
  Lauf wird verworfen (`cancel-in-progress: false` schützt nur laufende, nicht
  wartende Läufe). Letzter abgeschlossener Test-Befund ist der **Vor-Heil**-Lauf
  `36059450659` @`f417cdd69` **failure** (1597 pass, 2 fail:
  `archivar::bayestar::tests::load_map_leaf_record_finds_the_pixel` @`bayestar.rs:494`,
  `archivar::ble::tests::managed_objects_reply_resolves_device_and_characteristic`
  @`ble.rs:1116`). Damit sind die Heilungen `1c42ed09e` (bayestar, mountain) und
  `c015aa9b2` (ble, sensory) **unverified**; Lauf `36064053750` trägt beide.
- **`register_lookup --open`:** keine `[mountain]`-Zustandszeile; `phi/` trägt kein
  Token `mountain`. `open_points_check` (folge153): 12 Pfade, 0 absent.
- **Angrenzend, nicht mountain-eigen:** `handover-2026-09-20-operator-entscheidungen.md:20`
  (force-gate B) ist im Code gebaut — `kernel_riss_thermal_diffusion_advective`
  (`src/mathematikerin/force.rs:258`) trägt den Riss **gemessen**, kein offener Bau;
  `:26` vC-Permeabilität `termin` (Sensoren) ist `future`/`operator`.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

keiner (beide Punkte hängen am selben laufenden CI-Lauf).

#### Stufe 2 — operator-gebunden

keiner.

#### Stufe 3 — blockiert

keiner.

#### Stufe 4 — wartend

### Bayestar-CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss von `ci-check 36064053750` @`de76fda6a` — der erste Lauf,
  der die bayestar-Heilung `1c42ed09e` trägt
- **Lage:** (gemessen 2026-09-25 via `ci_manage view`/`list`/`log`) kein
  abgeschlossener `ci-check` seit `1c42ed09e`; alle Zwischenläufe cancelled
  (Concurrency `ci-check-${{ github.ref }}`); letzter Test-Befund ist der
  Vor-Heil-failure @`f417cdd69` (2 rote Tests). `36064053750` in_progress.
- **Blockade:** keine — der Lauf läuft.
- **Braucht:** `ci_manage view 36064053750` **einmal** (bei Abschluss); bei Rot
  `ci_manage log 36064053750 --all`. Der `ble`-Test gehört **sensory** (ble.rs) —
  bei BLE-Rot dorthin tragen, nicht heilen.

### dropped-Baseline 960
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Drift des dropped-Nettos gegen die Baseline (Delta > 0) im
  annehmenden Commit
- **Lage:** (gemessen 2026-09-25 via `ci_manage view` und lokalem
  `register_lookup --dropped`, das >120s timeoutete — schwer, gehört ins CI-Gate,
  nicht auf die Maschine des Operators) Baseline **960** @`5179b438b`
  (`docs/zustand/dropped-baseline.md`, live); `register-dropped 36064053762`
  in_progress am HEAD; die Messung ist der `dropped-gate`-Job in `ci-check
  36064053750`.
- **Blockade:** keine.
- **Braucht:** `dropped-gate`-Job in `ci-check 36064053750` lesen
  (`ci_manage log 36064053750`); bei Delta > 0 die Baseline in
  `docs/zustand/dropped-baseline.md` nachziehen.

#### Stufe 5 — termin

keiner.

#### Stufe 6 — LOCK

keiner.

## Benchmark

- **CI-Archäologie** (Klasse: `ci-check`-Historie/`ci_manage` lesen) → `grind-flash`
  (Routine). Ergebnis vollständig und korrekt: Run-IDs, SHAs, der Concurrency-Grund
  mit `file:line`. Kein Doppel — die Routine-Klasse bleibt flash. Messung: der
  pending-Cancel (`ci-check-${{ github.ref }}`) ist das Kernergebnis.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-25-mountain-folge154.md` (neu)
- `docs/handover/archiv/handover-2026-09-24-mountain-folge153.md` (Move aus
  `docs/handover/`, eigene Linie, atomar)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
