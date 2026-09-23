<!--
  title: Handover — River-Folge 16 (Stand 2026-09-23)
  session: river
  class: handover
  date: 2026-09-23
  sha256: bf83a3c1a9b1fe51261977fa5d1336ec0aaa10fc3eb174d27dcd83c63af0e97e
  status: live
-->
# Handover — River-Folge 16 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Anlass: folge15 (Revert der `bluer`-Pulsbrücke — `libdbus` zurückgenommen, der
945-BLE-Live-Reader an die sensory-Linie geroutet) wird gefaltet; diese Session
misst CI am Revert-HEAD, korrigiert die AGENTS.md-Tool-Zählung und schreibt die
`external-state.md`-CI-Zeile fort.

## Stehender Pass (gemessen 2026-09-23)

- **Postfach** — kein Mail-Werkzeug (`mail_digest` absent); kein Mailbezug
  (operator-geführtes Atom). Die Postfach-Zeile in `external-state.md` bleibt
  unverändert (letzter Eingang `1790116573`).
- **CI** — HEAD `2af5b5b5`: `tools-build 35877164366` **success** (der
  `bluer`-Revert nimmt `libdbus` zurück → der Red gelöst); `ci-check 35877164288`
  **pending** (gemessen 2026-09-23 via `ci_manage view`).
- **`git_safety --snapshot`** → `refs/safety/1790175256`.

## Offen (aufgeschlüsselt)

### CI-Verifikation am HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 35877164288` endet.
- **Lage:** HEAD `2af5b5b5`; `tools-build` success (Revert gelöst), `ci-check`
  pending (gemessen 2026-09-23 via `ci_manage view`).
- **Blockade:** Lauf noch offen (Session pollt nicht).
- **Braucht:** `ci_manage view 35877164288` → grün; dann die
  `external-state.md`-CI-Zeile fortschreiben.

### LAN-Sensorik: adb-reverse-Route (Rat-Verdikt 2026-09-23)
- **Status:** operator-gebunden | **Bindung:** operator (Pixel: USB-Debugging + Kabel)
- **Trigger:** der Operator aktiviert am Pixel Entwickleroptionen/USB-Debugging
  (einmalig) und steckt das USB-Kabel.
- **Lage (gemessen 2026-09-23):** Host-Relay erreichbar (`0.0.0.0:1618`, HTTP 200,
  `/field` 92 060 Samples / 56 Felder); Pixel über `http://192.168.178.26:1618`
  verbunden, `/consent`=1, 2 Sensoren / 6 Radiadoren — aber **`sensor: 0 samples`**
  und keine Vibration; `http://<LAN-IP>` ist kein secure context (Generic Sensor API,
  Geolocation, `navigator.vibrate` sind secure-context-gated — genau die zwei
  nicht-gated Sensoren registrieren). **Rat-Verdikt:** `adb reverse`, weil `localhost`
  per Secure-Contexts-Spec eine potentiell vertrauenswürdige Origin ist — secure
  context ohne TLS, ohne neue Dependency, ohne Dritten; verworfen: Cloudflare-Tunnel
  (Dritter sieht den Bewegungs-Strom), Chrome-Flag (entfernt die Schutz-Membran),
  TLS jetzt (C-Dependency im Kern + ungemessener Warnungs-Durchklick).
- **Blockade:** `adb devices` listet kein Gerät (gemessen 2026-09-23) — USB-Debugging
  + Kabel fehlen.
- **Braucht:** Operator schaltet USB-Debugging am Pixel ein und steckt das Kabel; dann
  `adb reverse tcp:1618 tcp:1618` (Host), Pixel öffnet `http://127.0.0.1:1618`,
  `/consent`=1; messen: `sensor: N samples` mit N>0 und Vibration.

### TLS im Relay für kabellose Sensorik
- **Status:** pending (Registraturpflicht, kein stiller Nullpunkt) | **Bindung:** eigen (Rat/Bau)
- **Trigger:** ein kabelloser Sensor wird gemessen nötig.
- **Lage:** für jetzt verworfen (Rat 2026-09-23): `rustls`/`native-tls` = neue
  C-Dependency im Kern (folge13-Lehre); der Durchklick einer Zertifikat-Warnung ergibt
  am Pixel keinen gemessenen vertrauenswürdigen Kontext.
- **Blockade:** keine — bewusst `pending`.
- **Braucht:** Rat/Bau, falls kabellos nötig; dann privates CA-Zertifikat am Gerät.

### Beat-Paar
- **Status:** wartend | **Bindung:** linie:mycelium
- **Trigger:** eine Zweistationen-Open-Loop-Aufnahme oder ein gehaltenes Asset mit
  zwei kohärenten Tönen in einem Band.
- **Lage:** WGSL-Beat-Term feuerfähig; kein Datensatz liefert das Paar (gemessen
  2026-09-23, folge11).
- **Blockade:** keine Paarquelle; Dual-Comb-Klassifikation offen.
- **Braucht:** `post.md`-Zeile „An mycelium" gesetzt; mycelium prüft die
  Dual-Comb-Kandidaten gegen Force-Gate/Registry.

## Geroutet / fremd (nicht river)

- **945-BLE-Live-Reader** — Operator-Wort 2026-09-23 an die **sensory**-Linie
  (`post.md:21`); sensorys folge153-Punkt, std-only BlueZ-D-Bus. Transporthaken
  `OMEGAFLOW_SERIAL_IN` (`src/archivar/ingress.rs`) steht. Ein-Quellen-Regel: nur
  eine Beat-Quelle gleichzeitig.

## Benchmark

- **AGENTS.md-Tool-Zählung:** Routine-Messung (Bin-Targets je Crate) —
  `grind-flash`, ein Lauf; die Routine-Klasse ist seit 2026-09-16 geschlossen, kein
  Doppel-Lauf. Ergebnis: top-level Bin-Targets je Crate = harvest 227 / measure 243 /
  register 17 / service 8 / science 5 / gate 2 / utils 27 = **529**; die frühere
  utils-Zahl 72 war ein Pathspec-Recursionsartefakt (`git ls-files` mit `*` matcht
  Unterordner), verifiziert mit `:(glob)`.

## Geteilter Baum — eigener Pfad-Satz

- `AGENTS.md` (Zeile 6: Zählung 161 Tools → 529 Bin-Targets)
- `docs/handover/post.md` (eigene Hunks: „An mycelium", „An sensory", Faltung der
  „An river"-Info-Zeile)
- `docs/handover/handover-2026-09-23-river-folge16.md` (neu; folge15 → `archiv/`)
- **lokal, gitignored (nicht im Commit):** `docs/zustand/external-state.md`
  (.gitignore:134) — CI-Zeile auf HEAD `2af5b5b5` fortgeschrieben
- **Nicht angefasst (fremd):** `src/archivar/fit.rs`,
  `src/archivar/main_flow.rs`, `src/archivar/mod.rs`,
  `docs/handover/handover-2026-09-23-sensory-folge153.md`, der gestagte Rename
  `handover-2026-09-23-sensory-folge152.md` → `archiv/`,
  `docs/auftrag/auftrag-flyby2-kette.md`, die fremden `post.md`-Zeilen
  (An future×2, An research).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
