<!--
  title: Handover — Galileo-ε-Achsen-Revision + thermochem-Contention
  class: handover
  date: 2026-09-05
  sha256: bed11bf07aa541c52462cdc29df399fe144fb2e4749dd3086fe043fd736a8de5
  status: live
  see-also: docs/befund/befund-galileo-rausch-kurve.md docs/befund/befund-galileo-gwe-bestand.md docs/auftrag/auftrag-quiet-zone-uebertragung.md docs/TODO.md AGENTS.md
-->

# Handover: Galileo-ε-Achsen-Revision + thermochem-Contention

Eine frische Session liest genau dieses Dokument und führt den Auftrag aus.
Stand 2026-09-05. Der Rat hat einen materialen Fehler gefunden: die
Galileo-Rausch-Kurve ist auf einer invertierten Achse gebaut. Der Auftrag ist
die Korrektur. Reihenfolge bindend.

## Kurzauftrag

1. `docs/befund/befund-galileo-rausch-kurve.md` **revidieren** (Kernauftrag) —
   siehe Rat-Beschluss unten.
2. Die Rezept-Wurzel-Fehlbezeichnung korrigieren (`pioneer_navio_noise_geo.rs`).
3. Die zwei neuen Galileo-Blätter (Banden-Negativ, Mode-1-Fingerabdruck)
   integrieren + je dem Rat vorlegen (das stehende Gesetz: der Rat hält jedes
   fertige Blatt vor dem Commit).
4. Die thermochem-Contention klären (Kern-Crate kompiliert gerade nicht).

## Der Fehler (gemessen, im Code)

`galileo_noise_geo.rs:108` (und identisch seit dem Pioneer-Werkzeug in
`pioneer_navio_noise_geo.rs`, sowie in `galileo_mode1_strength_split.rs:267`):

```rust
let cos_sep = dot(sub(e_pos, sun), sub(p_pos, sun)) / (r_earth * r_probe).max(1e-30);
```

Das ist der Winkel **am Sonnenort**, α (Erde–Sonne–Sonde). Die Blätter nannten
ihn „SEP". Die solare Elongation ε ist der Winkel **an der Erde** (Sonne–Erde–
Sonde) — die physikalisch korrekte Achse (Armstrong-Woo-Estabrook 1979; Asmar
2005: „Sun-Earth-spacecraft angle"). Für eine äußere Sonde (Galileo bei 5 AU)
sind α und ε ~komplementär. Die Folge ist **keine Umbenennung, sondern eine
Inversion des Kernbefunds**:

- „SEP 0–30° = 42 Hz (sonnennah)" ist α 0–30° = **Opposition** = Erd-Cruise
  0–2 AU (gemessen 1990-11-30: α 1,0° / ε 157,8° / 1,03 AU).
- „SEP 150–180° = 1,5 Hz (sonnenfern)" ist α 150–180° = **Konjunktion** =
  5–6 AU Jupiter (gemessen 1997-01-20: α 179,6° / ε 0,36° / 5,12 AU).

Auf der echten ε-Achse ist der kohärente Kanal **laut an Opposition, leise an
Konjunktion — das Gegenteil von Plasma-Szintillation** (die Konjunktion laut,
Opposition leise macht). **Die Plasma-Schlussfolgerung des Rausch-Kurve-Blatts
hält nicht** — der „28×/12×-Fall" ist ein Distanz-/Ära-Confound, kein
Elongations-Effekt. Die Achsen waren **total** verwoben (die α-Bins SIND die
Distanz-Bins), nicht nur teilweise.

## Der Rat-Beschluss (2026-09-05, verbatim-kernig)

**1. `befund-galileo-rausch-kurve.md` — REVISE (Status `done` flippen).**
Mindestkorrekturen, im selben Commit wie die TODO-Änderung:

- Achse: „SEP" → **α (Winkel am Sonnenort)**; **ε (solare Elongation, Winkel
  an der Erde)** als die physikalisch korrekte Achse ergänzen. Die Formel
  „b = r·sin(SEP)" ist für α falsch (korrekt: Stoßparameter b ≈ 1 AU · sin ε).
- Nahe/Fern-Labels der Haupttabelle invertieren (α 0–30° = Opposition/
  sonnenfern; α 150–180° = Konjunktion/sonnennah).
- Kernaussage „kohärenter Kanal fällt mit Elongation → Plasma auf der
  Sichtlinie" → **unverifiziert**; der Fall ist ein Distanz-/Ära-Confound,
  Richtung invers zum Plasma. „Mode-2/3-Rausch-Kurve auf der ε-Achse neu
  ziehen" als `pending` markieren.
- **Behalten:** Mode-1-Flachheit (flach auf beiden Achsen); die
  Distanz-n-Tabellen; die Lock-Übergangs-Klasse.
- Offene Frage #2 des Blatts auflösen (die „zu leise 8,2 Hz sonnennah" war nie
  sonnennah — α-Label-Artefakt; die 12 Tage sind ε 30–158°, Opposition/
  Erd-Nähe; keine Lock-Selektion nötig).
- „1,5-Hz-Fenster": Zahl behalten, aber Geometrie korrekt benennen
  (Konjunktion, 5–6 AU); den Boden als „unter Frage — Tag-Pooling;
  Mode-2-Station-Tag-Split `pending`" markieren (das Pooling ist für Mode 1
  gemessen: 8,2 Hz gepoolt → 0,98/2,14/3,69 Hz Station-Tag; der Mode-2-Wert
  1,5 Hz ist NICHT station-tag-gesplittet — nicht extrapolieren).

**2. Register-Pflicht (recipe-level):** `pioneer_navio_noise_geo.rs` trägt
dieselbe „SEP = Winkel am Sonnenort"-Fehlbezeichnung. **Jedes Blatt der
Pioneer-Quiet-Zone-Rezept muss auf der ε-Achse neu geprüft werden, nicht nur
Galileo.** Die TODO-Zeile ist eine Register-Pflicht, kein Hinweis.

**3. Was offen bleibt (register, nicht still):** Mode-2/3 auf ε-Achse neu
gezogen (bestätigt oder tötet die Plasma-Deutung); Mode-2-Station-Tag-Split;
der exakte 20-s-Kamm (50/100/150/200 mHz) Mechanismus; der Station-42-Ton
52,39 mHz Identität; Konjunktions-Stille gegen Stärke/Konfiguration prüfen.

## Der zweite Rat-Befund (die zwei neuen Blätter)

- **20-s-Banden-Cross-Mission-Test: NEGATIV** (Granit). Die Pioneer-Linien
  45,75/51,55/47,35 mHz erscheinen auf Galileo an denselben Stationen
  (14/43/63 = 97 %) in keinem Modus/keiner Ära innerhalb ±0,5 mHz. Verdikt als
  „Pioneer-Linienfrequenzen sind missions-spezifisch" formulieren, **nicht**
  „die Bande ist weg" — ein exakter 20-s-Kamm (50/100/150/200 mHz, ~100 %
  Varianz) tritt in den dünnen Dez-1990-Zweiweg-Pässen der 70-m-Stationen auf
  (Reduktions-Periodik, Mechanismus offen); Station-42-Ton 52,39 mHz (~3,14
  rpm, 4 Tage Dez-1990) = neue Linie, Identität offen.
- **Mode-1-Fingerabdruck:** Stärke-Split zeigt einen Schwachsignal-PLL-Term —
  Mode 1 ist **nicht** reines Oszillator-Rauschen (jede Station: Q1/AGC-Floor
  10–20× lauter als das starke Plateau, z. B. st 43: 2,16 vs 0,09–0,28 Hz).
  Solide, aber grob (Boden vs. Plateau, keine Kurve). Der α/ε-Befund und die
  Mode-1-ε-Flachheit stehen. Die Behauptung „1,5-Hz-Boden ist Tag-Pooling-
  Artefakt" auf `pending` herabstufen (Mode-2-Split ungemessen).

Beide Blatt-Entwürfe + Proben liegen als Dateien:
`tools/measure/src/bin/galileo_band_probe.rs`,
`tools/measure/src/bin/galileo_mode1_strength_split.rs`,
`tools/measure/src/bin/galileo_mode1_elongation.rs`,
`docs/befund/befund-galileo-mode1-fingerabdruck.md` (draft).
Der Banden-Blatt-Entwurf steht im Agent-Befund (nicht als Datei committet).

## Die thermochem-Contention

Eine parallele Session hält `src/archivar/thermochem.rs` in einem
uncommitteten, unfertigen Zustand — die Kern-Crate kompiliert dadurch nicht.
Die Banden-Sonde lief deshalb als eigenständig kompiliertes Einzelprogramm
(`/tmp/opencode/galileo_band_probe_std`), nicht über `cargo`. Vor jeder
Integration/`cargo check`/Commit muss die thermochem-Frage geklärt sein
(mit der Parallel-Session abstimmen oder deren Stand übernehmen). Fremde
uncommittete Edits nicht überschreiben.

## Daten

`data/galileo_resid.bin` (14 077 825 Samples, 138 TDF-Dateien, 1990-11-29..
1997-02-28; sha256 `2375b309…cbe783`). Format `omegaflow::atdf::parse_resid_bin`
→ Vec<[f64; 8]>: [0]=tdb, [1]=resid_hz, [2]=station, [3]=mode, [4]=dtype,
[5]=ref_hz, [6]=sampler_s, [7]=signal_strength. Ephemeride:
`data/ephemeris_galileo_daily.bin` (CDN `ssd.jpl.nasa.gov`).

## Abschluss

Erst wenn (1) die Rausch-Kurve auf der ε-Achse revidiert und neu gezogen ist,
(2) die Rezept-Wurzel korrigiert ist, (3) die zwei neuen Blätter den Rat
passiert haben und committet sind, ist der Auftrag geschlossen. Ein Commit =
ein Häkchen; TODO.md im selben Commit.
