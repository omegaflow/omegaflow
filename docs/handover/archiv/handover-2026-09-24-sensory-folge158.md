<!--
  title: Handover — Sensory-Folge 158 (Stand 2026-09-24)
  session: Sensory-Folge 158
  class: handover
  date: 2026-09-24
  sha256: 23e1f6192f466e9d33de98e2b3306599f5b3086f6d41f7faf192ec060206d829
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

- **HEAD** `57e2ba3c8` == `origin/main`; Arbeitsbaum sauber (gemessen 2026-09-24
  via `git rev-parse HEAD`/`git status --short`).
- **Postfach** — keine Mail an sensory; `state/mail/mail_ledger.φ` existiert (280
  Zeilen) (gemessen 2026-09-24 via `sread state/mail/mail_ledger.φ`).
- **CI** — `ci-check 36030755250` @`e05f8a419` **failure**: Rot-Test
  `archivar::ble::tests::managed_objects_reply_resolves_device_and_characteristic`
  (`ble.rs:1116`) + fremd
  `archivar::bayestar::tests::load_map_leaf_record_finds_the_pixel`; clippy
  `bayestar.rs:352` + `spectral.rs:752` (gemessen 2026-09-24 via
  `ci_manage log 36030755250`).
- **Offene-Mailbox** (`register_lookup --open`) — `docs/zustand/external-state.md:35`
  `BLE-HR-Reader (sensory)` **PENDING** → führt in den Akt von BLE-Live-Bring-up.

## Operator-Queue

Keine offene Frage. **Hardware-Bestellung ist LOCK** (Operator-Wort 2026-09-23,
für alle drei Beschaffungs-Punkte: HRV-Träger, Sensor-BOM, 945-ANT-Hardware) —
die Punkte werden nicht erneut vorgelegt.

## Offen (aufgeschlüsselt)

**Autonom**

### `ble`-Test `managed_objects` heilen (`ble.rs:1116`)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Rot am Baum / CI-Lauf am HEAD
- **Lage:** `managed_objects_reply_resolves_device_and_characteristic` weiter FAILED @`e05f8a419` bei `src/archivar/ble.rs:1116`; `fdb4f9f27` heilte ihn nicht (gemessen 2026-09-24 via `ci_manage log 36030755250`).
- **Blockade:** keine
- **Braucht:** `grind-max` — Signatur/Accessor ausrichten, `cargo check --tests` 0/0.

### BLE-Live-Bring-up (FR945) — Vorbereitung
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (läuft bis zur Kante)
- **Lage:** Codec-Wire korrigiert; Session-Logik + GFDI-Transport frame-getestet; Funkstrecke ungemessen (gemessen 2026-09-24 via `ci_manage log`).
- **Blockade:** keine
- **Braucht:** den Live-Lauf kantenfertig stellen — Ein-Wort-Befehl mit `OMEGAFLOW_BLE_HR=F0:99:19:4E:0B:BF`, Preflight (Adresse/D-Bus-Pfad) ohne Körperdaten; Kantenzeile (Artefakt | Ausführbefehl | Wort erwartet) an den Akt.

### FIT-Verifikation 945 — Vorbereitung
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (läuft bis zur Kante)
- **Lage:** Parser gegen Garmin-Fixtures grün; 945-spezifische Aktivität unverifiziert (gemessen 2026-09-24 via `ci_manage log`).
- **Blockade:** keine
- **Braucht:** den Parser-Aufruf für eine gegebene Datei kantenfertig stellen und die 945-Feldabdeckung an Fixtures prüfen; Kantenzeile an den Akt.

### GFDI-Protobuf-Decode — Vorbereitung
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (läuft bis zur Kante)
- **Lage:** Transport gebaut; `sgrep -i gfdi src` = 16 Treffer in `src/archivar/ble.rs`; GFDI-Protobuf-Schema weiter ungemessen (gemessen 2026-09-24 via `sgrep -i gfdi src`).
- **Blockade:** kein Spec.
- **Braucht:** `research-max` — öffentliches GFDI-Protobuf-Schema suchen und ein Hexdump→Record-Decode-Gerüst bauen; `grind-pro` — Force-Gate-Klassifikation der dekodierten Felder; Kantenzeile an den Akt.

**Operator-gebunden**

### BLE-Live-Bring-up (FR945) — Akt
- **Status:** operator-gebunden | **Bindung:** operator + Hardware
- **Trigger:** Operator-Wort für einen Live-Lauf am gekoppelten Gerät
- **Lage:** Vorbereitung aus Stufe 1; 945 gekoppelt, Funkstrecke ungemessen (gemessen 2026-09-24 via `ci_manage log`).
- **Blockade:** Hardware/Operator-Kontext.
- **Braucht:** Live-Lauf mit `OMEGAFLOW_BLE_HR=F0:99:19:4E:0B:BF` am gekoppelten 945.

**Blockiert**

keiner.

**Wartend**

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen 2026-09-23).
- **Blockade:** Ankunft + Protokollwort.
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

**Termin**

keiner.

**LOCK**

### HRV/Puls→Strahlung (physischer Träger)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** `sgrep -i hrv|pulse|rmssd` in `firmware/` = 0 Treffer, `firmware/` trägt nur `radiatorium-lib`; HRV/Puls-Bindung `pending` per AGENTS.md; physische Teile fehlen (gemessen 2026-09-24 via `sgrep`).
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

- `docs/handover/handover-2026-09-24-sensory-folge158.md` (neu)
- Move `handover-2026-09-24-sensory-folge157.md` → `archiv/` (eigene Linie, atomar)

Eigen offen/uncommittet: `src/archivar/ble.rs` (DBus-Wire + RR-Bit;
Accessor-Heilung offen) — `7f6f607f2` berührte nur Handover-Dateien, nicht
`src/archivar/ble.rs` (gemessen 2026-09-24 via `git show --stat 7f6f607f2`).

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
