<!--
  title: Handover — Sensory-Folge 155 (Stand 2026-09-23)
  session: Sensory-Folge 155
  class: handover
  date: 2026-09-23
  sha256: 943f0f2b2ea61568cd18667d1f0e1ccf01ae64c72f079786cfeab92bd6a5c9f1
  status: live
-->
# Handover — Sensory-Folge 155 (2026-09-23)

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
( woran es hängt, oder „keine") / **Braucht** (was es löst: der wörtliche,
kopierbare Schritt — Werkzeug/Datei/URL/Befehl/Operator-Wort). `operator-gebunden`,
`blockiert` und `wartend` werden benannt, nie dispatcht. Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin` | `LOCK`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung.

## Stehender Pass (gemessen 2026-09-23, Sensory-Folge 155)

- **HEAD** `486b8e6c1` == `origin/main`; eigener Satz uncommittet bis `/commit`:
  `src/archivar/fit.rs` (clippy/fmt), `src/archivar/ble.rs` (clippy/fmt + GFDI-
  Transport), `docs/handover/post.md` (sensory-Zeile gelöscht), neues Handover
  folge155 + Move folge154→`archiv/`. Fremd, nicht angefasst:
  `phi/harvest.φ`, `phi/sources.φ`, `tools/register/src/bin/register_lookup.rs`.
- **Postfach** — keine Mail an sensory (`state/mail/mail_ledger.φ`, letzte Eingänge
  fremd/alt); `post.md` sensory-Zeile gefaltet und gelöscht (war ci-check-Red).
- **CI** — `ci-check 35906172370` @`746452f50` **failure**: clippy `ble.rs:136`,
  `:302` (`manual_is_multiple_of`), `fit.rs:224`,`:261` (`chunks_exact_to_as_chunks`),
  `:234` (`collapsible_match`), `:299` (`collapsible_if`), `:345`
  (`vec_init_then_push`) (gemessen 2026-09-23 via `ci_manage log`). `te-gate
  35893882101` **in_progress** (via `ci_manage list`/`view`). HEAD noch nicht
  ci-geprüft. Fremd: viele `ci-check`/`tools-build`/CDN-Läufe.
- **`git_safety --snapshot`** → Arbeitsbaum == HEAD am Session-Beginn, nichts zu
  sichern.

## Operator-Queue

Keine offene Frage. **Hardware-Bestellung ist LOCK** (Operator-Wort 2026-09-23,
für alle drei Beschaffungs-Punkte: HRV-Träger, Sensor-BOM, 945-ANT-Hardware) —
die Punkte werden nicht erneut vorgelegt.

## Geschlossen in dieser Session (git trägt sie)

- **ci-check-Rot fit.rs+ble.rs geheilt** (`grind-flash`): die sieben gemessenen
  clippy-Lints in `src/archivar/fit.rs` (`as_chunks::<4>/<3>`, zwei Match-Guards,
  eine `let`-Kette, `vec![]`) und `src/archivar/ble.rs` (`is_multiple_of` ×2)
  beseitigt; `cargo fmt --` auf beide. `cargo check` 0/0. `ingress.rs:17` ist
  Rivers und in `7a269ef1a` geheilt — nicht angefasst.
- **GFDI-Transport gebaut** (`grind-flash`, Rat-Verdikt (a) strikt):
  `src/archivar/ble.rs` — `characteristic_matches`/`characteristic_paths`
  (UUID-Fragment-Matcher, kein Basis-UUID-Gerate), `StartNotify` je `6a4e28`-Char,
  `gfdi_line(uuid,payload)` als **Roh-Hex-Dump** (`ble gfdi <uuid> <hex>`), **kein**
  Decode, keine Feldnamen, kein Wert an `tx`; 2 Fixture-Tests (Matcher + Hex-Golden).
  `cargo check` 0/0.

## Offen (aufgeschlüsselt)

### GFDI-Protobuf-Decode (Folgeschritt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** erster gemessener GFDI-Capture (Live-Lauf FR945, `ble gfdi`-Hexdump)
- **Lage:** Transport gebaut (s.o.); GFDI-Schema ungemessen — `sgrep -i gfdi` /
  `6a4e28` = 0 Treffer im Baum (gemessen 2026-09-23).
- **Blockade:** kein Capture, kein Spec.
- **Braucht:** `ble gfdi`-Hexdump aus einem Operator-Live-Lauf; daraus das
  hand-dekodierte Protobuf + Force-Gate-Klassifikation.

### FIT-Verifikation 945
- **Status:** wartend | **Bindung:** operator (Datei) / eigen (CI-Lauf)
- **Trigger:** echte 945-FIT-Aktivität + grüner `ci-check` am HEAD
- **Lage:** Parser inkl. `event_timestamp_12` gegen Garmin-Fixtures grün; clippy
  geheilt; 945-spezifische Aktivität unverifiziert (gemessen 2026-09-23).
- **Blockade:** keine 945-Datei; HEAD noch nicht ci-geprüft.
- **Braucht:** `gh workflow run ci-check` am HEAD (nach `/commit`); echte
  `GARMIN/Activity/*.FIT` (`post.md` → future/Operator).

### te-gate n=1000 — `fpr-ksg`/`issue`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des Laufs `35893882101`
- **Lage:** `flush`/`flare` success (Mountain folge143); `fpr-ksg`/`issue` offen;
  Lauf **in_progress** (gemessen 2026-09-23 20:03 via `ci_manage view`).
- **Blockade:** Lauf nicht abgeschlossen.
- **Braucht:** `ci_manage log 35893882101 --all` nach Abschluss.

### BLE-Live-Bring-up (FR945)
- **Status:** operator-gebunden | **Bindung:** operator + Hardware (LOCK)
- **Trigger:** Operator-Wort für einen Live-Lauf am gekoppelten Gerät
- **Lage:** Codec + Session-Logik + GFDI-Transport frame-getestet; Funkstrecke
  ungemessen (gemessen 2026-09-23).
- **Blockade:** Hardware/Operator-Kontext.
- **Braucht:** Live-Lauf mit `OMEGAFLOW_BLE_HR=F0:99:19:4E:0B:BF` am gekoppelten 945.

### HRV/Puls→Strahlung (physischer Träger)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** Firmware-Kette inkl. BLE gebaut; physische Teile fehlen
  (gemessen 2026-09-23).
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
- **Lage:** kein Dateieingang (gemessen 2026-09-23).
- **Braucht:** eingehende Datei lesen.

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen 2026-09-23).
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

## Benchmark

- **ci-check-Rot fit.rs+ble.rs:** `grind-flash` — geliefert, `cargo check` 0/0,
  alle sieben Lints geheilt. Routine-Klasse (flash-first), kein Gegenlauf.
- **GFDI-Transport:** `grind-flash` — geliefert, `cargo check` 0/0, 2 Fixture-Tests.
  Rat entschied **vorab** auf (a) strikt (Transport, kein Decoder) — der Grund,
  warum das harte Atom (Protobuf) gar nicht erst gebaut wurde.
- **Rat (GFDI):** Verdikt (a) strikt — 5/5 Stimmen, ein Decoder ohne Capture wäre
  Fabrikation; der Punkt wird gesplittet (Transport gebaut, Decode `pending`).

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/ble.rs` (clippy/fmt + GFDI-Transport)
- `src/archivar/fit.rs` (clippy/fmt)
- `docs/handover/post.md` (sensory-Zeile entfernt)
- `docs/handover/handover-2026-09-23-sensory-folge155.md` (neu)
- Move `handover-2026-09-23-sensory-folge154.md` → `archiv/` (eigene Linie, atomar)

Fremd uncommittet: `phi/harvest.φ`, `phi/sources.φ`,
`tools/register/src/bin/register_lookup.rs` — nicht angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent, nie das Commit-Wort.
