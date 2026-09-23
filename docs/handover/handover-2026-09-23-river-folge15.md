<!--
  title: Handover — River-Folge 15 (Stand 2026-09-23)
  session: river
  class: handover
  date: 2026-09-23
  sha256: c8191f100e3dcf9cc4f5dc192ce07018625c169d16ed65eddc09eeaa6510f15a
  status: live
-->
# Handover — River-Folge 15 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Anlass (Operator-Wort 2026-09-23): der BLE-Puls-Versuch der Garmin-945 (river
folge13, über die Crate `bluer`) wird zurückgenommen — `bluer` zieht die
C-Bibliothek `libdbus` (`tools-build` rot; die Doktrin ist `std + curl +
serialport`, vgl. `serialport` mit `default-features=false` gegen `libudev`), und
der BLE-Live-Reader der 945 ist sensorys offener Punkt. Er geht per `post.md` an
die sensory-Linie (std-only BlueZ-D-Bus). Diese Session führt **folge15**: eine
**parallele River-Session** (folge14) hatte ihre folge13 konsumiert und archiviert;
folge14 wird hierin gefaltet (→ `archiv/`), ihr wiederangelegtes folge13 entfällt.

## Stehender Pass (gemessen 2026-09-23)

- **Postfach** — in diesem operator-geführten Atom nicht gelesen (kein Mailbezug);
  fällig 2⁶ min.
- **CI** — `tools-build 35874560639` @`905c81276` **failure**: `libdbus-sys`
  braucht `dbus-1` (nicht am Runner) — Folge der `bluer`-Abhängigkeit; der Revert
  nimmt sie zurück. `ci-check 35874560727` @`905c81276` **cancelled** (baut
  `tools/service` nicht — `default-members = ["."]`). Der Push dieses Reverts
  startet `ci-check`/`tools-build` neu.
- **Post** — `post.md` trägt fremde uncommittete Hunks (mountain/sensory); die
  eigene Zeile „An sensory" (BLE-Reader-Route) steht im Baum; folge14 hatte zwei
  River-Zeilen physisch gefaltet (CI-Info, FIT-Brücke).
- **`git_safety --snapshot`** → `refs/safety/1790173796`.

## Offen (aufgeschlüsselt)

### CI-Verifikation am Revert-HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push dieses Reverts (`bluer` raus).
- **Lage:** `tools-build` @`905c81276` rot (`libdbus-sys`/`dbus-1`, gemessen
  2026-09-23 via `ci_manage view`/`log`); der Revert entfernt
  `bluer`/tokio/futures/uuid/nix + das Bin `pulse_bridge`.
- **Blockade:** kein Commit-Wort (`/commit`).
- **Braucht:** `ci_manage view <id>` am neuen HEAD → grün (der Push startet die
  Läufe selbst).

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

### Beat-Paar
- **Status:** wartend | **Bindung:** dritter / linie:mycelium
- **Trigger:** eine Zweistationen-Open-Loop-Aufnahme, oder ein gehaltenes Asset mit
  zwei kohärenten Tönen in einem Band.
- **Lage:** WGSL-Beat-Term feuerfähig; kein Datensatz liefert das Paar (gemessen
  2026-09-23, folge11).
- **Blockade:** keine Paarquelle; Dual-Comb-Klassifikation offen.
- **Braucht:** `post.md`-Zeile „An mycelium: …" gesetzt (folge14, 2026-09-23);
  mycelium prüft die Dual-Comb-Kandidaten gegen Force-Gate/Registry.

### AGENTS.md — Tool-Zählungen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein freier Pass (kein Gate hängt daran).
- **Lage:** Zeile 6 zählt „161 tools … tools/service (5 services)"; keine
  Dateizählung (gemessen 2026-09-23: harvest 227 / measure 243 / register 17 /
  service 8 / science 5 / gate 2 / utils 72 Bin-Dateien — `service` wieder 8 nach der
  `pulse_bridge`-Rücknahme); Zähl-Semantik ungemessen.
- **Blockade:** Zähl-Semantik ungemessen.
- **Braucht:** Semantik messen und die Zeile wahr machen.

## Geroutet / fremd (nicht river)

- **945-BLE-Live-Reader** — Operator-Wort 2026-09-23 an die **sensory**-Linie
  (`post.md`-Zeile); sensorys folge153-Punkt, std-only BlueZ-D-Bus (kein Crate).
  Transporthaken `OMEGAFLOW_SERIAL_IN` (`src/archivar/ingress.rs`) steht bereit.
  **Ein-Quellen-Regel:** nur eine Beat-Quelle gleichzeitig — sonst speisen ESP32-
  Firmware (`nn=`), FIT, Browser-Samples und ein BLE-Reader in denselben `nn_buf`
  (`main_flow.rs` `feed_beat_to_hrv`).

## Benchmark

- **LAN-Bind (folge14):** Routine-Atom (env-abhängige Bind-Adresse + zwei reine
  Tests) — kein Doppel-Lauf; die Architektur trägt der Rat (pro/max) aus folge12.
- **Bau Puls-Brücke (folge13):** `grind-pro`, ein Lauf, baulich vollständig
  (`cargo check` 0/0) — **durch Messung verworfen**: `bluer` zieht die C-Bibliothek
  `libdbus`, `tools-build` wird rot. Lehre: eine BLE-Dependency vor dem Bau gegen
  CI-/Doktrin-Kosten prüfen; der Rat hatte `bluer` ohne diese Messung gewählt.

## Geteilter Baum — eigener Pfad-Satz

- das Bin `pulse_bridge` in `tools/service` (gelöscht)
- `tools/service/Cargo.toml` (Abhängigkeiten zurückgenommen)
- `Cargo.lock` (`bluer`-Baum entfällt; `nix` bleibt via `serialport` 0.26.4)
- `AGENTS.md` (Zeile 6 zurückgesetzt)
- `docs/specs/radiators.md` (`tone_scale`-Drift-Fix, ohne Brücke; Route an sensory)
- `docs/handover/handover-2026-09-23-river-folge15.md` (neu; folge14 → `archiv/`)
- `docs/handover/post.md` (eine eigene Zeile an sensory; fremde Hunks bleiben unangetastet)
- **Nicht angefasst (fremd):** `src/archivar/fit.rs`, `src/archivar/main_flow.rs`,
  `src/archivar/mod.rs`, `docs/handover/handover-2026-09-23-sensory-folge153.md`
  (+ sensory archiv-Move), `docs/auftrag/auftrag-flyby2-kette.md`,
  `.github/workflows/te-gate.yml`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
