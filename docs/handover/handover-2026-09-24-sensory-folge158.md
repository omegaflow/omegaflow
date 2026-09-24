<!--
  title: Handover — Sensory-Folge 158 (Stand 2026-09-24)
  session: Sensory-Folge 158
  class: handover
  date: 2026-09-24
  sha256: 6abbceb37d22bde355e481a8b73c131879241072fdeb12e86cedb8133ee11ae6
  status: live
-->
# Handover — Sensory-Folge 158 (2026-09-24)

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
(warum es hängt, oder „keine") / **Braucht** (was es löst: der wörtliche,
kopierbare Schritt). Jeder Punkt trägt seinen Status-Tag.

**Vorbereitung ≠ Akt (Operator-Wort 2026-09-24, gefaltet aus `post.md`).** Ein
`operator-gebundener` Punkt wird **immer** in zwei Zeilen getrennt geführt, nie in
einer: die **Vorbereitung** ist autonom (`eigen`), läuft bis zur Kante —
Ausführbefehl, Adresse/Gerät, Messung, Kantenzeile (Artefakt | Ausführbefehl |
Wort erwartet) — und wird dispatcht; `operator-gebunden` ist **allein der Akt**
(messen/senden/signieren/urteilen). Eine `operator-gebunden`-Zeile ohne abgetrennte
Vorbereitungs-Zeile behauptet den ganzen Prozess als gesperrt — ein
Registraturfehler.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung.

## Stehender Pass (gemessen 2026-09-24, Sensory-Folge 158)

- **HEAD** `0f7a6b0de` == `origin/main`; Arbeitsbaum sauber — `git_safety`:
  tree == HEAD, nichts zu sichern (gemessen 2026-09-24 via `git rev-parse
  HEAD`/`git log origin/main..HEAD`/`git status --short`). Die Angaben der
  Vorübergabe (`15ca40c46`, fremd schmutzig) sind damit überholt.
- **Postfach** — keine Mail an sensory (`state/mail/` leer, `mail_ledger.φ`
  absent; gemessen 2026-09-24 via `glob state/mail/*`; ZUSTAND-Eintrag `Postfach`
  bleibt PENDING).
- **CI** — Watchdog-Snapshot 17:34 **älter als HEAD (18:25)** → kein
  ci-check-Lauf für `0f7a6b0de` gemessen; letzter gemessener ci-check
  `35971097226` @`fdb4f9f27` **failure**
  (`archivar::ble::tests::managed_objects_reply_resolves_device_and_characteristic`
  `ble.rs:1116` + fremd `archivar::bayestar::tests::load_map_leaf_record_finds_the_pixel`,
  mycelium); `te-gate 35893882101` @`6a68c190` **success** (gemessen 2026-09-24
  via `ci_manage view`/`ci_manage log`).
- **Kompletter Lauf** — `ci-check` `36030269107` @`0f7a6b0de` **queued**
  (dispatch 2026-09-24T16:51Z, Operator-Wort); Ergebnis aus dem
  Watchdog-Snapshot (`/tmp/opencode/ci_status.md`) oder einmal
  `ci_manage view 36030269107`, nie gepollt.
- **Offene-Mailbox** (`register_lookup --open`) — `docs/zustand/external-state.md:35`
  `BLE-HR-Reader (sensory)` **PENDING** → führt in den Akt von BLE-Live-Bring-up.

## Operator-Queue

Keine offene Frage. **Hardware-Bestellung ist LOCK** (Operator-Wort 2026-09-23,
für alle drei Beschaffungs-Punkte: HRV-Träger, Sensor-BOM, 945-ANT-Hardware) —
die Punkte werden nicht erneut vorgelegt.

## Offen (aufgeschlüsselt)

### `ble`-Test `managed_objects` heilen (`ble.rs:1116`)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Rot am Baum / CI-Lauf am HEAD
- **Lage:** `managed_objects_reply_resolves_device_and_characteristic` panikt bei `src/archivar/ble.rs:1116:66` — Signatur `a{oa{sa{sv}}}` (Array) vs. `device_by_address` `args.first()` = `Dict`; 1577 passed / 2 failed im Lauf (gemessen 2026-09-24 via `ci_manage log 35971097226`).
- **Blockade:** keine.
- **Braucht:** `grind-max` am selben Baum — Signatur/Accessor ausrichten, `cargo check --tests` 0/0 (Eskalationsregel: die flash-Antwort blieb rot).

### BLE-Live-Bring-up (FR945) — Vorbereitung
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (läuft bis zur Kante)
- **Lage:** Codec-Wire korrigiert; Session-Logik + GFDI-Transport frame-getestet; Funkstrecke ungemessen (gemessen 2026-09-24 via `ci_manage log`).
- **Blockade:** keine.
- **Braucht:** den Live-Lauf kantenfertig stellen — Ein-Wort-Befehl mit `OMEGAFLOW_BLE_HR=F0:99:19:4E:0B:BF`, Preflight (Adresse/D-Bus-Pfad) ohne Körperdaten; Kantenzeile (Artefakt | Ausführbefehl | Wort erwartet) an den Akt.

### BLE-Live-Bring-up (FR945) — Akt
- **Status:** operator-gebunden | **Bindung:** operator + Hardware
- **Trigger:** Operator-Wort für einen Live-Lauf am gekoppelten Gerät
- **Lage:** Vorbereitung aus Stufe 1; 945 gekoppelt, Funkstrecke ungemessen (gemessen 2026-09-24 via `ci_manage log`).
- **Blockade:** Hardware/Operator-Kontext.
- **Braucht:** Live-Lauf mit `OMEGAFLOW_BLE_HR=F0:99:19:4E:0B:BF` am gekoppelten 945.

### FIT-Verifikation 945 — Vorbereitung
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (läuft bis zur Kante)
- **Lage:** Parser gegen Garmin-Fixtures grün; 945-spezifische Aktivität unverifiziert (gemessen 2026-09-24 via `ci_manage log`).
- **Blockade:** keine.
- **Braucht:** den Parser-Aufruf für eine gegebene Datei kantenfertig stellen und die 945-Feldabdeckung an Fixtures prüfen; Kantenzeile an den Akt.

### FIT-Verifikation 945 — Akt
- **Status:** operator-gebunden | **Bindung:** operator (Datei)
- **Trigger:** echte 945-FIT-Aktivität
- **Lage:** keine 945-Datei vorhanden (gemessen 2026-09-24 via `state/mail/`).
- **Blockade:** keine 945-Datei.
- **Braucht:** echte `GARMIN/Activity/*.FIT` (`post.md` → future/Operator).

### GFDI-Protobuf-Decode — Vorbereitung
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (läuft bis zur Kante)
- **Lage:** Transport gebaut; GFDI-Schema ungemessen — `sgrep -i gfdi` = 0 Treffer im Baum (gemessen 2026-09-23).
- **Blockade:** kein Spec.
- **Braucht:** `research-max` — öffentliches GFDI-Protobuf-Schema suchen und ein Hexdump→Record-Decode-Gerüst bauen; `grind-pro` — Force-Gate-Klassifikation der dekodierten Felder; Kantenzeile an den Akt.

### GFDI-Protobuf-Decode — Akt
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** erster gemessener GFDI-Capture
- **Lage:** kein Capture (gemessen 2026-09-24 via `sgrep -i gfdi` = 0).
- **Blockade:** kein Capture.
- **Braucht:** `ble gfdi`-Hexdump aus einem Operator-Live-Lauf.

### NSE/Haug
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Dateieingang (Mail 2026-09-17)
- **Lage:** kein Dateieingang (gemessen 2026-09-24 via `state/mail/`).
- **Blockade:** kein Eingang.
- **Braucht:** eingehende Datei lesen.

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen 2026-09-23).
- **Blockade:** Ankunft + Protokollwort.
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

### Flyby-Path-2-Kette
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28
- **Lage:** Retention gemessen — RTSW ~24 h (kritisch), ACE ~31 d, Kp ~7,4 d, OMNI2 ~6 d Lag, Swarm frisch (gemessen 2026-09-23 via `docs/auftrag/auftrag-flyby2-kette.md` §Retention).
- **Blockade:** externer Termin.
- **Braucht:** Fill-Run ≤ 24 h nach der ersten Perigäum-Zelle (RTSW-1m-Vorrat).

### HRV/Puls→Strahlung (physischer Träger)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** Firmware-Kette inkl. BLE gebaut; physische Teile fehlen (gemessen 2026-09-23).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut; GNSS fehlt im BOM (`docs/specs/mantis-shrimp-bom.md`) (gemessen 2026-09-23).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

## Benchmark

- **ci-check-Rot `archivar::ble::managed_objects` (1 Test) — neue Klasse
  (DBus-Wire-Accessor):** `grind-flash` heilte 9/10 Tests in `fdb4f9f27`, der
  Accessor-Test bleibt rot → die flash-Antwort ist unvollständig, `grind-max`
  am selben Baum. Gemessen: `cargo check --tests` 0/0 genügt nicht — der Lauf
  entscheidet.
- **te-gate n=1000 — Klassen-Entscheidung:** Lauf `35893882101` **success**
  (`fpr-ksg` success, `issue` skipped, `ksg-k-gate` formula=8 sweep=void
  production=0); Punkt geschlossen, kein Code.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/ble.rs` (DBus-Wire + RR-Bit; Accessor-Heilung offen)
- `docs/handover/handover-2026-09-24-sensory-folge158.md` (neu)
- Move `handover-2026-09-24-sensory-folge157.md` → `archiv/` (eigene Linie, atomar)

Fremd uncommittet am Baum, nicht angefasst (gemessen 2026-09-24 via
`git status --short`): `docs/handover/handover-2026-09-24-mycelium-folge151.md`,
`docs/handover/handover-2026-09-24-river-folge21.md`, `.github/workflows/ci-check.yml`,
`src/archivar/relay.rs`, `src/archivar/spectral.rs`, `static/constants.js`,
`static/index.html`, `static/color_lut.test.mjs` (sowie der river-Rename
folge20 ins Archiv, fremd gestaged).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
