# Der spektrale Oszillator — die Frequenzachse des Blocks

Selbsttragend. Dieses Dokument ist der Plan des Quantensprungs, den der
Operator am 19.8.2026 gegen den ersten Rats-Befund durchgesetzt hat.
Der Rat hatte `format spectral` als späteren Schritt zurückgestuft; der
Operator zeigte, dass die Welle die andere Hälfte des Oszillators ist —
und dass die eigenen Register den Beweis tragen. Dieses Dokument fasst
die Wahrheit, die Diagnose und die Atome. Es gilt, bis die Atome als
Code existieren; danach trägt Git sie.

## I. Der Einspruch

Ein Oszillator IST eine Frequenz. Das System nannte seine Atome
„Oszillatoren" und führte sie als Skalare — die Teilchen-Projektion
ohne die Wellen-Projektion. Das war pragmatisch und war nur die halbe
Wahrheit. Licht ist ein Spektrum. Schall ist ein Spektrum. Seismik ist
ein Spektrum.

Die eigenen Register beweisen, was reduziert oder verworfen wurde:

- **NCEI Solar Spectral Irradiance** (`ncei.noaa.gov/.../ssi_{year}{month}.txt`,
  im Bestand): die Datei ist ein Spektrum — die Einheit heißt W/m²/nm —
  und wurde als Skalar-Feld `spectral_irradiance_W_m2_nm` geführt: eine
  Wellenlänge im Namen der Einheit, die Achse selbst verworfen.
- **LISA Pathfinder** (VizieR `J/PhRvL/116/231101/table1`, im Bestand):
  die Tabelle trägt die Spalten **Freq, PSD_DA, PSD_noise_floor,
  Phase** — extrahiert wurde `PSD_DA` als Skalar; die Freq-Spalte fiel.
- **CMB-Power-Spektren** (BB_power, EE_power, Cl_kk, Δ²_mK²): die
  l-Achse — die Frequenz der Kosmologie — zu Skalaren reduziert.
- **Seismische Spektral-Antwort** (`spectral_acc_0_2s_g`, `SA1`): die
  Beschleunigung bei einer Periode, als gäbe es nur eine.
- **Spektrale Dichten** (`electric_spectral_density_mW`,
  `magnetic_spectral_density_mW`): Dichten ohne Achse.
- **Offene GAVO-Bulks**: `gdr3spec.spectra` (Gaia DR3),
  `mlqso.slitspectra`, `califadr3.spectra`, `lotsspol.spectra`,
  `dfbsspec.raw_spectra` — Spektren-Kataloge liegen im Katalog-Bestand
  und sind ungehoben.
- **ONC-HSD-FFT** (verifiziert 2026-08-19): 512 Frequenzbins × 250 Hz,
  dB re 1 µPa² — ASCII-Datei mit impliziter Achse (Bin i = i×250 Hz),
  85 Hydrophon-Stationen; als parser-gap abgelegt.
- **GONG** (FITS-Gap), **miniSEED/FDSN** (hinter dem Gate),
  **lidar waveform**, **wave spectra**, **hyperspectral** — als Gaps
  registriert, nie befreit.

## II. Die Diagnose

Die Frequenz ist im System überall **implizit** — und nirgends
querybar. Wir missbrauchen bereits Tokens als Frequenz:

- `tau` — τ⁻¹ IST die Bandbreite (die Kohärenz ist ein Band).
- `kernel_id` — der Kernel IST ein Frequenzgang
  (exponential-decay = Lorentzian, erfc = Absorptionskante,
  das τ-Gate = Tiefpass).
- `extent` — eine räumliche Skala, eine Wellenlänge.
- `color_index` — eine Zwei-Bin-SED.
- `pole_x` bei em — z, eine Frequenzverschiebung.

Keine Stelle erlaubt die Frage: *welche Oszillatoren schwingen zwischen
30 und 50 Hz?* Der Namens-Trick (`star_42_freq_450nm`) ist UNWAHR:
Name = Implementation — der Name IST das Ding; eine Frequenz im String
ist nicht filterbar, nicht in der Enclosure adressierbar, nicht im
Shader lesbar. Die Frequenz gehört als Token in den Record.

## III. Das Gesetz der Symmetrie

Teilchen und Welle sind zwei Darstellungen einer Identität. Der
Punktquellen-Skalar ist der Ein-Bin-Grenzfall eines Spektrums; die
Spektral-Familie ist die volle Basis. Beide laufen durch denselben
Record, dasselbe Gesetz, denselben Shader. **Die Quelle deklariert
ihre Basis.**

Ein Pflicht-Frequenzfeld für jeden Oszillator wäre Fabrikation:
gravity, thermal und diffusion besitzen keine Frequenz. Deshalb gilt:
`freq = 0.0` ist der ehrliche Zustand „Punktquelle" — 0 honored, wie
j2/j4 = 0 bei Nicht-Planeten heute. Abwesende Frequenz ist eine
vollständig realisierte Eigenschaft, kein Default.

## IV. Die Atome

### Atom A — Die Frequenz als Token (Protokoll v8)

Der Record wächst von 22 auf 24 × f64 (176 → 192 B):

```
[x, y, z, val, epoch, ttl, tau, extent, kernel_id, force_type,
 absorption, advection, vx, vy, vz, pole_x, pole_y, pole_z, j2, j4,
 r_eq, color_index, freq, bin_width]
```

- `freq` — Band-Zentrum in Hz; 0.0 = Punktquelle.
- `bin_width` — Bandbreite in Hz; 0.0 = Punktquelle.
- Frame-Header: `0xCF 0x86 0x08` (v8).

Alle drei Schichten wachsen gemeinsam:
1. **Rust** — der Write-Loop serialisiert 24 Werte.
2. **JavaScript** — `constants.js` DataView packt die zwei neuen Slots
   in die zwei Padding-Nullen der meta-Reihe (Slot 4 und 12); f32
   genügt auf der Rendering-Ebene (bei 500 THz ist die f32-ulp ~64 Hz —
   irrelevant relativ zur Breite eines Bands).
3. **WGSL** — der props-Unpack liest die zwei Slots.

Verifikation ist Handarbeit (cargo check sieht nichts davon): die
Drei-Schichten-Kette Rust → JS → WGSL Feld für Feld nach AGENTS.md
verifizieren; ein laufender Membran-Test zeigt, dass Punktquellen
(freq = 0) unverändert rendern.

### Atom B — Der Spectral-Compiler

Ein Compiler zerlegt ein Spektrum in Bins; jeder Bin wird ein
Oszillator am selben Punkt: val = Amplitude, freq/bin_width aus der
Achse, tau = Bin-Kohärenz, Kernel nach Medium. Quellen in Reihenfolge:

1. **NCEI-SSI** — klein, liegt im Bestand: λ→ν (ν = c/λ),
   W/m²/nm → W/m²/Hz; beweist die Kette end-to-end am ersten Tag.
2. **ONC-HSD-FFT** — Route verifiziert (dataProductDelivery-Kette:
   request → status → run → download; deviceCode-Form, nicht
   locationCode+deviceCode); ASCII, 512 Bins × 250 Hz, implizite
   Achse; 85 Stationen als stations-Familie.
3. **Gaia-XP-Spektren** (gdr3spec.spectra) — der große Fall:
   CDN-Compiler wie dr3_stars.bin, ~55 Bins je Stern, Millionen Sterne.
4. **LISA-Pathfinder-PSD + CMB-Power** — die Freq-/l-Spalte statt der
   Skalar-Reduktion; Phase wird mitgenommen, wo sie existiert.
5. **GONG + miniSEED** — Waveforms: eigene FFT (std-only: Goertzel
   pro Band oder ein kleines FFT-Atom); das Instrument deklariert die
   Basis — Samples (TESS-Muster) oder Bins (Spektral-Atom).

### Atom C — Band-selektives Rendering

Der Fragment-Shader akkumuliert pro Band; RGB ist bereits ein
Drei-Band-Renderer (color_index → Teff → RGB). Jetzt wird es
konfigurierbar: der Operator wählt die Bänder der Gaze. Die Stillekarte
wird band-selektiv, die Lichtkegel-Differenz dispersiv, der
chromatische Dip der Nadel Ⅴ wird eine SED-Messung.

### Atom D — Die Phase (ehrlich terminiert)

Beats und Interferenz — zwei Sterne mit leicht verschiedener
Rotverschiebung, deren Spektren im selben Pixel schlagen — brauchen die
Phase. PSD-Bins tragen sie nicht. Atom D folgt, wenn Atom C steht;
nichts wird vorher als schwingend behauptet (0 honored).

## V. Was der Quantensprung ist

Ein Block, in dem Stern-SED, Ozean-FFT, Erdbeben-PSD und
Magnetfeld-Pulsation **dieselbe Frequenzachse** teilen. Jede Disziplin
hat ihre eigene Spektral-Pipeline; niemand spannt sie in eine
Feldgleichung mit einer Physik. Die Kreuz-Kraft-Transferentropie
zwischen Medien bei gleichem Band — die 10-Hz-Pulsation, die im Wasser
resoniert — ist eine Messung, die kein Institut kennt. Der kausale
Gradient bekommt eine Frequenzkomponente: Information fließt nicht nur
zwischen Punkten, sondern zwischen Tönen.

## VI. Die Regeln

- Keine Fabrikation: kein Skalar-Schallpegel aus einem Spektrum
  errechnet, keine Frequenz an Punktquellen erfunden. 0.0 ist die
  Wahrheit der Abwesenheit.
- Kein Namens-Trick: die Frequenz lebt als Token, nie im String.
- Jedes Atom ist ein vollständiges Session-Artefakt: drei Schichten,
  Tests, cargo check 0/0, Register, Commit. Kein Atom wird geteilt.
- Der v7-Bestand bleibt lesbar: alte Aufnahmen tragen freq = 0.0 und
  rendern wie zuvor. Die Messreihe der Zukunft erbt alles.
