<!--
  title: Handover — River-Linie folge13 (Stand 2026-09-23)
  session: river
  class: handover
  date: 2026-09-23
  sha256: ce1d2feb46600bd1571ad855e8e8b30d08fd5df8a4c7de9d31c08ac1fea1383c
  status: live
-->
# Handover — River-Linie folge13 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Anlass (Operator-Wort 2026-09-23): die Garmin-945-Live-Anbindung wurde gebaut —
nicht über ANT+ (Descope aus folge12), sondern über den BLE-Broadcast der Uhr in
den gebauten `nn=`-Socket der Membrane.

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — in diesem operator-geführten Atom nicht gelesen (kein Mailbezug);
  fällig 2⁶ min.
- **CI-Status am HEAD** (`/tmp/opencode/ci_status.md`, gelesen 2026-09-23 ~16:09):
  `tools-build 35871395171` in_progress, `ci-check 35868973875` in_progress;
  `ci-check 35861931781` @`145cbe612` **failed** — die Ursache
  (`clippy::too_many_arguments`, `shaders.rs:1006 beat_pair`) ist von mycelium
  folge147 geheilt, ein grüner Beleg-Lauf am neuen HEAD steht aus.
- **Post (fremde Linien):** `post.md` trägt uncommittete fremde Hunks; die zwei an
  river gerichteten Zeilen (CI-Info `59bd7ef`; FIT-Brücke erfüllt durch sensory
  folge152) sind logisch gefaltet, die physische Löschung steht aus (geteilter
  Baum — fremde Hunks werden nicht überschrieben).

## Offen (aufgeschlüsselt)

### Puls-Brücke — erster Lauf am Gerät (R-R-Nachweis)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** der Operator startet (a) an der Uhr „Herzfrequenz senden", (b) die
  Membrane mit `OMEGAFLOW_SERIAL_IN=/tmp/omegaflow-pulse`, (c) `pulse_bridge`.
- **Lage:** Brücke gebaut (`tools/service/src/bin/pulse_bridge.rs`, `cargo check`
  0/0 gemessen 2026-09-23); Uhr gekoppelt, Service `0x180D` beworben (gemessen
  2026-09-23 via `bluetoothctl info`, Operator-Terminal); **R-R im BLE-Broadcast
  ungemessen**.
- **Blockade:** der physische Lauf (Uhr + Laptop + Membrane).
- **Braucht:** der Dreiklang; danach die Zeile „R-R im Broadcast absent" ja/nein
  und `pulse: <n> bpm` melden.

### Puls-Brücke — CI + Release
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Commit + Push des Atoms.
- **Lage:** neues Bin in `tools/service`; `tools-build.yml` baut die Tools, neue
  Bins werden nicht automatisch in `tools-latest` manifestiert (gemessen 2026-09-23,
  Recherche).
- **Blockade:** kein Commit-Wort (`/commit`).
- **Braucht:** `gh workflow run ci-check` + `gh workflow run tools-build.yml`;
  danach `pulse_bridge` über `tools-latest`/PATH prüfen.

### LAN-Knoten (erstes Geräte-Atom, aus folge12)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** die zwei Operator-Worte — (a) LAN-Exposition (`0.0.0.0`-Bind),
  (c) Radiator/Vibration (per-Act, nie im `OMEGAFLOW_HIDDEN`-Lauf).
- **Lage:** `src/archivar/relay.rs:9` `PORT_CONST=1618`, Zeile 59 loopback-gebunden
  (gemessen 2026-09-23).
- **Blockade:** Consent (LAN-Exposition + Strahlung).
- **Braucht:** Operator-Wort a+c; dann Bind über Loopback hinaus, Sensor-Cluster
  (≥ 2 Oszillatoren je Gerät), WS-Rückkanal `frame_bytes` → Vibration.

### Beat-Paar (Atom-D-Fortsetzung, aus folge12)
- **Status:** wartend | **Bindung:** linie:mycelium
- **Trigger:** eine Zweistationen-Open-Loop-Aufnahme eines Trägers, oder ein
  gehaltenes Asset mit zwei kohärenten Tönen in einem Band.
- **Lage:** WGSL-Beat-Term gebaut und feuerfähig; kein Datensatz liefert das Paar
  (gemessen 2026-09-23).
- **Blockade:** keine Paarquelle; Dual-Comb-Klassifikation offen.
- **Braucht:** mycelium prüft die Dual-Comb-Kandidaten gegen Force-Gate/Registry
  (`post.md`-Zeile).

### AGENTS.md — Tool-Zählungen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein freier Pass (kein Gate hängt daran).
- **Lage:** Zeile 6 „161 tools … tools/service (5 services)" — die Zahlen sind
  **keine** Dateizählungen (gemessen 2026-09-23: harvest 227 / measure 243 /
  register 17 / service 9 / science 5 / gate 2 / utils 72 Bin-Dateien); die
  Zähl-Semantik ist ungemessen. `pulse_bridge` wurde namentlich ergänzt.
- **Blockade:** Zähl-Semantik ungemessen.
- **Braucht:** Semantik messen und die Zeile wahr machen.

## Benchmark

- **Bau Puls-Brücke (neues Bin, bluer/tokio):** `grind-pro`, **ein** Lauf, Ergebnis
  vollständig (`cargo check` Workspace 0/0, 4 Parser-Tests + 2 Ingress-Tests) — kein
  Doppel-Lauf: die Architektur-Klasse war vorab im Rat (pro/max, bluer + PTY +
  `OMEGAFLOW_SERIAL_IN`) entschieden. Die gemessenen Routine-Klassen (PII, Routine-
  Agent) bleiben geschlossen.

## Geteilter Baum — eigener Pfad-Satz

- `tools/service/src/bin/pulse_bridge.rs` (neu)
- `tools/service/Cargo.toml`
- `src/archivar/ingress.rs`
- `Cargo.lock`
- `AGENTS.md` (Zeile 6, `pulse_bridge` benannt)
- `docs/specs/radiators.md` (Drift-Fix `tone_scale`)
- `docs/handover/handover-2026-09-23-river-folge13.md` (neu; folge12 → `archiv/`)
- **Nicht angefasst (fremd):** `src/archivar/fit.rs`, `src/archivar/main_flow.rs`,
  `src/archivar/mod.rs`, `docs/handover/handover-2026-09-23-sensory-folge153.md`
  (+ sensory archiv-Move), `docs/handover/post.md`,
  `docs/auftrag/auftrag-flyby2-kette.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
