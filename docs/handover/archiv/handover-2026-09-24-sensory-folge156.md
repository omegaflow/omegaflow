<!--
  title: Handover — Sensory-Folge 156 (Stand 2026-09-24)
  session: Sensory-Folge 156
  class: handover
  date: 2026-09-24
  sha256: 1ab1c28fa872786880ab5b346991d64df4ffcfc2f7b92f4d47ec7beb2311c9e5
  status: live
-->
# Handover — Sensory-Folge 156 (2026-09-24)

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
Punkt wird **aufgeschlüsselt** geführt — **Trigger** / **Lage** (gemessen, mit
Messstempel) / **Blockade** / **Braucht** (der wörtliche Schritt). Jeder Punkt
trägt seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` |
`termin` | `LOCK`); `operator-gebunden`, `blockiert` und `wartend` werden benannt,
nie dispatcht.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung.

## Stehender Pass (gemessen 2026-09-24, Sensory-Folge 156)

- **HEAD** `0c7682916` == `origin/main`; Arbeitsbaum == HEAD bis auf fremd
  staged, nicht angefasst: `.github/workflows/te-ncurve.yml`,
  `tools/measure/src/bin/pcmci_class_benchmark.rs`.
- **Postfach** — keine Mail an sensory (`state/mail/` leer, `mail_ledger.φ` absent;
  gemessen 2026-09-24 via `glob state/mail/*`).
- **CI** — `ci-check 35920939598` @`48ae5e734` **failure**: 10
  `archivar::ble`-Tests (1567 passed / 10 failed; gemessen 2026-09-23 via
  `ci_manage log`). `te-gate 35893882101` **in_progress** (gemessen 2026-09-24 via
  `ci_manage view`). **Eigener Push** `fdb4f9f27` triggert `ci-check 35971097226`
  **queued** + `tools-build 35971097502` (gemessen 2026-09-24 via `ci_manage
  view`). Fremd: viele CDN-Läufe.
- **`git_safety --snapshot`** → `refs/safety/1790203447` (fremd staged; nichts
  Eigenes zu sichern).

## Operator-Queue

Keine offene Frage. **Hardware-Bestellung ist LOCK** (Operator-Wort 2026-09-23,
für alle drei Beschaffungs-Punkte: HRV-Träger, Sensor-BOM, 945-ANT-Hardware) —
die Punkte werden nicht erneut vorgelegt.

## Offen (aufgeschlüsselt)

### GFDI-Protobuf-Decode (Folgeschritt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** erster gemessener GFDI-Capture (Live-Lauf FR945, `ble gfdi`-Hexdump)
- **Lage:** Transport gebaut; GFDI-Schema ungemessen — `sgrep -i gfdi` / `6a4e28`
  = 0 Treffer im Baum (gemessen 2026-09-23).
- **Blockade:** kein Capture, kein Spec.
- **Braucht:** `ble gfdi`-Hexdump aus einem Operator-Live-Lauf; daraus das
  hand-dekodierte Protobuf + Force-Gate-Klassifikation.

### FIT-Verifikation 945
- **Status:** wartend | **Bindung:** operator (Datei) / eigen (CI-Lauf)
- **Trigger:** echte 945-FIT-Aktivität + grüner `ci-check` am HEAD
- **Lage:** Parser gegen Garmin-Fixtures grün; der 10-Test-`ble`-Rot ist im
  Codec geheilt, `cargo check --tests` 0/0 (gemessen 2026-09-24); die
  Test-Verifikation hängt am push-getriggerten `ci-check 35971097226` (queued);
  945-spezifische Aktivität unverifiziert.
- **Blockade:** keine 945-Datei; ci-check-Ergebnis pending.
- **Braucht:** `ci_manage log 35971097226` (am Commit `fdb4f9f27`; Ergebnis aus
  dem Watchdog-Snapshot, nicht pollend); echte `GARMIN/Activity/*.FIT`
  (`post.md` → future/Operator).

### te-gate n=1000 — `fpr-ksg`/`issue`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des Laufs `35893882101`
- **Lage:** `flush`/`flare` success (Mountain folge143); `fpr-ksg`/`issue` offen;
  Lauf **in_progress** (gemessen 2026-09-24 via `ci_manage view`).
- **Blockade:** Lauf nicht abgeschlossen.
- **Braucht:** `ci_manage log 35893882101 --all` nach Abschluss.

### BLE-Live-Bring-up (FR945)
- **Status:** operator-gebunden | **Bindung:** operator + Hardware (LOCK)
- **Trigger:** Operator-Wort für einen Live-Lauf am gekoppelten Gerät
- **Lage:** Codec-Wire korrigiert (g-Signatur + 8-Alignment, RR-Bit; 2026-09-24);
  Session-Logik + GFDI-Transport frame-getestet; Funkstrecke ungemessen.
- **Blockade:** Hardware/Operator-Kontext.
- **Braucht:** Live-Lauf mit `OMEGAFLOW_BLE_HR=AA:BB:CC:DD:EE:FF` am gekoppelten 945.

### HRV/Puls→Strahlung (physischer Träger)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** Firmware-Kette inkl. BLE gebaut; physische Teile fehlen (gemessen 2026-09-23).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK (Hardware) | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut; GNSS fehlt im
  BOM (`docs/specs/mantis-shrimp-bom.md`) (gemessen 2026-09-23).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

### Flyby-Path-2-Kette
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28
- **Lage:** Retention gemessen — RTSW ~24 h (kritisch), ACE ~31 d, Kp ~7,4 d
  (→GFZ-Backfill), OMNI2 ~6 d Lag, Swarm frisch — `docs/auftrag/auftrag-flyby2-kette.md`
  §Retention (gemessen 2026-09-23).
- **Blockade:** externer Termin.
- **Braucht:** Fill-Run ≤ 24 h nach der ersten Perigäum-Zelle (RTSW-1m-Vorrat).

### NSE/Haug
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Dateieingang (Mail 2026-09-17)
- **Lage:** kein Dateieingang (gemessen 2026-09-24 via `state/mail/`).
- **Braucht:** eingehende Datei lesen.

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen 2026-09-23).
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

## Benchmark

- **ci-check-Rot `archivar::ble` (10 Tests) — neue Klasse (DBus-Wire-Atom):**
  `grind-flash` implementierte nach präzisem Wurzel-Brief drei+eine Korrektur,
  `cargo check --tests` 0/0. Test-Verifikation am CI-Lauf (flash-first).
  **Eskalationsregel:** bleibt der Lauf rot, ist die flash-Antwort unvollständig
  → `grind-max` für denselben Baum.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/ble.rs` (DBus-Wire + RR-Bit)
- `docs/handover/handover-2026-09-24-sensory-folge156.md` (neu)
- Move folge155 → `docs/handover/archiv/handover-2026-09-23-sensory-folge155.md`
  (eigene Linie, atomar, in `fdb4f9f27`)

Fremd staged, nicht angefasst: `.github/workflows/te-ncurve.yml`,
`tools/measure/src/bin/pcmci_class_benchmark.rs`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent, nie das Commit-Wort.
