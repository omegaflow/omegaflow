<!--
  title: Handover — River-Folge 14 (Stand 2026-09-23)
  session: River-Folge 14
  class: handover
  date: 2026-09-23
  sha256: 5ae8f17c2d7f65b0cc2c73198aaf05a36ed8c6b253b50f8ab6155e2ebe51c99a
  status: live
-->
# Handover — River-Folge 14 (2026-09-23)

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
Punkt wird **aufgeschlüsselt** geführt: **Trigger** / **Lage** (mit Messstempel) /
**Blockade** / **Braucht**. `operator-gebunden`, `blockiert` und `wartend` werden
benannt, nie dispatcht.

## Stehender Pass (gemessen 2026-09-23, River-Folge 14)

- **HEAD** beim Start `29cc04213` (`mycelium 147b`). Während der Session landete
  `905c8127` (`river folge13`, BLE-Puls-Brücke) aus einer **parallelen River-Session**;
  HEAD == `origin/main` = `905c8127`.
- **Kollision im geteilten Baum:** mein erster Schreibversuch auf
  `handover-2026-09-23-river-folge13.md` überschrieb die parallele folge13; aus
  `git show HEAD:` wiederhergestellt, diese Session führt **folge14**.
- **`git_safety --snapshot`** — `refs/safety/1790173318`.
- **`register_lookup --open`** — kein `owner=river`.
- **Postfach `post.md`** — zwei River-Zeilen physisch gefaltet (CI-Nur-Info gelöscht;
  FIT-Brücke-Punkt gelöscht, erfüllt durch sensory folge152); neue Zeile an
  mycelium (Beat-Paar). **Nicht committet** — `post.md` trägt fremde uncommittete
  Hunks (HRV-future, flare-research); nur eigene Hunks, fremde bleiben unangetastet.
- **`open_points_check folge12`** — 14 Pfad-Refs, **0 absent**.
- **CI** (`ci_manage list`/`view` 2026-09-23 ~14:35Z) — `ci-check 35873370330`
  @`29cc04213` **in_progress**; Lauf davor `35868973875` @`e0a0e52a` **failure**
  (`dropped-gate` delta 262: baseline 2680 | current 2942 — mycelium-Linie; `test`
  grün 44m16s); `register-dropped 35871377787` success 14:19Z. Kein River-Red.
  Zustand in `docs/zustand/external-state.md`.

## Offen (aufgeschlüsselt)

### Gerätemessung LAN-Knoten (a+c freigegeben, gebaut)
- **Status:** operator-gebunden | **Bindung:** operator (Gerät)
- **Trigger:** der Operator startet die sichtbare Membran (`cargo run --features
  browser_relay`) und öffnet `http://<host>:1618` auf dem Pixel.
- **Lage:** (a) Bind über Loopback gebaut — `relay_bind_addr()`
  (`src/archivar/relay.rs`), `OMEGAFLOW_RELAY_BIND`, Default `0.0.0.0`, Tests
  `the_bind_reaches_the_ether_when_unset` / `the_bind_honours_the_operator_address`
  (gebaut folge14, 2026-09-23; Operator-Wort a+c erteilt); (b) `static/sensorium.js`
  (Generic Sensor API + Device-Motion/-Orientation) und (c) Vibrations-Peer
  `static/radiator.js` über `KINETIC_TAG` in `static/index.html` (gemessen 2026-09-23).
- **Blockade:** lokale Funktionsläufe strukturell verweigert; Messergebnis braucht
  Operator + Gerät im LAN.
- **Braucht:** Operator startet den `browser_relay`-Lauf, der Pixel öffnet die
  Seite, Consent-Gate bestätigt; sichtbar: Gerät vibriert mit Σω und der
  Sample-Strom trägt ≥ 2 Oszillatoren.

### Puls-Brücke — erster Lauf am Gerät (R-R-Nachweis, aus folge13)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** der Operator startet (a) an der Uhr „Herzfrequenz senden", (b) die
  Membrane mit `OMEGAFLOW_SERIAL_IN=/tmp/omegaflow-pulse`, (c) `pulse_bridge`.
- **Lage:** Brücke gebaut (`tools/service/src/bin/pulse_bridge.rs`); Uhr gekoppelt,
  Service `0x180D` beworben (gemessen folge13); **R-R im BLE-Broadcast ungemessen**.
- **Blockade:** der physische Lauf (Uhr + Laptop + Membrane).
- **Braucht:** der Dreiklang; danach die Zeile „R-R im Broadcast absent" ja/nein und
  `pulse: <n> bpm` melden.

### Puls-Brücke — CI + Release (aus folge13)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Commit + Push des folge13-Atoms (mit `/commit`).
- **Lage:** neues Bin in `tools/service`; `tools-build.yml` manifestiert neue Bins
  nicht automatisch (gemessen folge13).
- **Blockade:** kein Commit-Wort (`/commit`).
- **Braucht:** `gh workflow run ci-check` + `gh workflow run tools-build.yml`;
  danach `pulse_bridge` über `tools-latest`/PATH prüfen.

### Beat-Paar (aus folge12)
- **Status:** wartend | **Bindung:** dritter / linie:mycelium
- **Trigger:** eine Zweistationen-Open-Loop-Aufnahme, oder ein gehaltenes Asset mit
  zwei kohärenten Tönen in einem Band.
- **Lage:** WGSL-Beat-Term feuerfähig; kein Datensatz liefert das Paar (gemessen
  2026-09-23, folge11).
- **Blockade:** keine Paarquelle; Dual-Comb-Klassifikation offen.
- **Braucht:** `post.md`-Zeile „An mycelium: …" gesetzt (folge14, 2026-09-23);
  mycelium prüft die Dual-Comb-Kandidaten gegen Force-Gate/Registry.

### AGENTS.md — Tool-Zählungen (aus folge13)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein freier Pass (kein Gate hängt daran).
- **Lage:** Zeile 6 zählt „161 tools … tools/service (5 services)"; keine
  Dateizählung (gemessen folge13: harvest 227 / measure 243 / register 17 / service 9
  / science 5 / gate 2 / utils 72 Bin-Dateien); Zähl-Semantik ungemessen.
- **Blockade:** Zähl-Semantik ungemessen.
- **Braucht:** Semantik messen und die Zeile wahr machen.

### Atom-D CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 35873370330` @`29cc04213` wird fertig.
- **Lage:** der River-rote Lauf auf `59bd7ef` (`beat_pair`-clippy) ist von mycelium
  folge147 geheilt; der HEAD-Lauf läuft (gemessen 2026-09-23 via `ci_manage list`).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 35873370330` (bei rot `ci_manage log <id>`).

## Benchmark

- **LAN-Bind:** Routine-Atom (eine env-abhängige Bind-Adresse + zwei reine Tests) —
  kein Doppel-Lauf; die Architektur (Cluster-Rollen/Transport) trägt der Rat
  (pro/max) aus folge12.
- **Bau Puls-Brücke (aus folge13):** `grind-pro`, ein Lauf, Ergebnis vollständig —
  kein Doppel-Lauf (Architektur vorab im Rat).

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/relay.rs` (Bind `relay_bind_addr` + zwei Tests)
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (Bind-gebaut-Notiz)
- `docs/handover/handover-2026-09-23-river-folge14.md` (neu; folge13 → `archiv/`)
- `docs/zustand/external-state.md` (nur die eigene CI-Zeile; gitignored)
- **Nicht committet (fremde uncommittete Hunks):** `docs/handover/post.md`.
- **Nicht angefasst (fremd):** `.github/workflows/te-gate.yml`,
  `docs/auftrag/auftrag-flyby2-kette.md`, `src/archivar/main_flow.rs`,
  `src/archivar/mod.rs`, `docs/handover/handover-2026-09-23-sensory-folge153.md`
  (+ sensory archiv-Move).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
