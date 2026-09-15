<!--
  title: Handover — Bau & Code (Stand 2026-09-11, Bau7)
  session: Bau-Folge
  class: handover
  date: 2026-09-11
  sha256: 34b43a4adebcde15cf85fe0db3c18b23903102122c91b6af42476f6aaa33979f
  status: archived
-->
# Handover — Bau & Code (2026-09-11, Bau7)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Membran — die M-Punkte unter der Radiator-Gleichwertigkeit

Die Archäologie (2026-09-11, Taucher über Legacy-Git + `archive-root` +
Rust-Archiv-Werkzeug `tools/utils/archive_search`) maß: der Browser-Client
existiert in der Legacy-Historie (`static/index.html` blob 1a0da547 +
`constants.js` blob 95f9c949, letzter tragender Commit `01a22e84`, kanonisch
im Bundle `omegaflow-history.bundle` unter
`archive-root/omegaflow-legacy-backup-2026-09-02/`). Gremium-Beschluss: **kein
1:1-Backport** — die Gleichwertigkeit aller Radiatoren (Peer-Menge
window/audio/stderr/serial/USB/BT/HID, Σω kanonisch, Backport-Verbot, eine
Apertur-Regel) ist die Klammer aller M-Atome.

- **M-Festhalten (erster Bau-Schritt):** `docs/specs/radiators.md` erweitern um
  die vier verbindlichen Sätze des Gremiums + die Doktrin-Karte (Atom 8
  Sensorium, Atom 9 Geräte-Bias, agnostische Benennung, Stille-Doktrin,
  Relay-Floor-Präzedenz, offene Radiatoren Bluetooth/HID/ESP32/Gamepad):
  (i) Peer-Menge — flach und geschlossen, ein Dispatcher, ein `Arc<Buffer>`,
  ein `PresenceFrame { omega: [f32; 9] }`, kein Radiator privilegiert, keiner
  Zentrum; (ii) Kanal-Regel — jeder Radiator erhält alle neun Kräfte, die
  Übersetzungsregel ist seine Eigenschaft (`canRadiate`), Σω kanonisch für
  skalare Anregungen, die verlorene Kanal-Zuweisung (audio→2, haptics→4) kehrt
  nicht zurück (Geräte-Bias Atom 9); (iii) Backport-Verbot — aus `01a22e84`
  kehrt nur die Sache selbst zurück (flaches Peer-Set = der eine Dispatcher,
  Consent-Doppel-Frage, Navigation als Operator-Akt); Synthesizer,
  Exposure-Maschinerie (`get_expose`/`exposureBoost`), `window_median_extent`,
  Fenster-als-Zentrum bleiben tot; (iv) Apertur-Bindung —
  `target = inTE/(inTE + threshold + ε)`, `alpha = 1 − exp(−1/max(1, naturalLatencyTicks))`,
  eine Regel für alle; pending, bis die Permeabilitäts-Bindung ihr Atom baut.
- **M03 Audio-Gain ohne tanh** — nativ gebaut (`AcousticOscillator`/
  `SeismicOscillator` roh, `actuators.rs:31,71`; die zwei tanh-Stellen
  `omega.rs:1293`/`s2.rs:170` sind Permeabilitäts-Atem, kein Audio). Offen:
  Property-Test, der „kein tanh im Audio-Pfad" pinnt; ein Browser-Audio-
  Radiator folgt dem `AcousticOscillator`-Gesetz (Σω, ein Frame = ein Sample)
  oder wartet auf die Apertur.
- **M04 Navigation/Nebra-Kalibrierung** — Navigation ist der Operator-Akt
  (Pfeile = Schub, `s` hält, die Presence ruht): übertragbar, kein
  Radiator-Privileg. Die Nebra-Exposure-Maschinerie bleibt tot (`get_expose`
  starb am Float32-Subnormal 2^-64, `ce6b5a07`).
- **M05/M06 Stations-Sensoren als SI-4-Token** — Einheiten-Vertrag
  `recordSample(name, value, force, unit)` statt Namensraten: `BrowserSensor`
  erhält `unit` (`types.rs:262`), `sensor_config` deklariert sie
  (`membrane.rs:389`), die Read-Sites füllen `FieldConfig.unit`
  (`main_flow.rs:905`, `relay.rs:596/655`); `""` bleibt der ehrliche
  absent-Pfad. Dazu der gemessene `serial_ingress`-Bug (`ingress.rs:47-50`
  leert den Puffer → parst nichts; `:39` fabriziert τ=0.0): `Option<τ>`,
  benannter Skip. Der M06-Spiegel nur Runtime, nie in Tests (Tests bleiben
  still).
- **M07 ⌘K-Palette (fuzzy)** — Spec PLANNED (`docs/specs/search-command-palette.md`),
  nie gebaut; Substrat lebt (`load_sources`, 782 sources, SIMBAD-Probe
  `h0_gaia_crossmatch_probe.rs`). Doktrin-neutral.
- **M02 ESP32-S3-Radiatorium-Firmware (no_std)** — der ESP32 ist ein Peer
  unter sieben: rohe Intensität wie `SeismicOscillator`, kein
  flow-Textsonderweg (keine der beiden flow-Specs wurde je implementiert).
  Firmware bleibt pending (Hardware); ethischer Puls/HRV-Filter pending (nur
  Spec, `omegaflow-sense-hardware.yaml.md:128-137`).
- **M01 WebSerial-flow-Protokoll** — die zwei Specs widersprechen sich und
  wurden nie gebaut; die Wire-Wahrheit ist die rohe Intensität. Auflösung
  erst, wenn die Architekturfrage per-Pixel-Membran (Legacy) vs heutige
  Punktwolke gemessen werden kann.
