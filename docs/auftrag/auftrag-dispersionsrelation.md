<!--
  title: Auftrag — Atom E: die Dispersionsrelation gemessen, nicht erfunden
  class: auftrag
  date: 2026-09-08
  probe-commit: pending
  sha256: 6ac9e7c5eef1934e4fc12dad8aab65eeace596eeb6ea011201b5041c1cdc42e7
  status: pending
  see-also: docs/TODO.md docs/handover/handover-2026-09-08-atom-c-offene-pflichten.md docs/specs/spectral-oscillator.md docs/auftrag/auftrag-dispersions-ortungstest.md
-->

# Auftrag: die Dispersionsrelation

## Gegenstand

Die eine offene Pflicht aus Atom C (`docs/TODO.md` Spektrale Achse): die
Laufzeit-Geschwindigkeit bleibt heute band-flach
(v = PROPAGATION_SPEED[force]). Das ist keine Ignoranz, es ist eine Aussage —
und Behauptungen werden gemessen. Eine echte Dispersionsrelation v(freq)
wird nur gebaut, wenn die Messung sie trägt; ein erfundenes
v0·(f/f0)^β steht nicht an der Stelle der Sache (0 honored).

## Pflichtfeld (Bindung vor dem Sehen)

Der Header trägt `probe-commit`. Der Auftrag ist erst **scharf**, wenn dieses
Feld den Commit-Hash der gebauten Probe trägt — die Kybernautin kalibriert
sonst an einer wandernden Version. Pflicht, nicht Zier (Ephemeris-Lektion,
dritte Wiederholung).

## Sensitivität im Verdikt — die Zahl ist das Verdikt

explore B (2026-09-08, gemessen): die feinste auflösbare Latenz ist die
**24-s-Analyse-Zelle** (`DT = 24.0` in `corona_conditional_probe` und
`solar_seconds_matrix`); die 2-s-Rohdaten (GOES `xr_YYYYMMDD.nc`, 1069
Dateien) sind der Floor, nicht die Zelle. Damit:

- δτ_min = **24 s**;
- δv = C · δτ_min / τ = **1,44e7 m/s** (4,8 % von c bei 1 AU, τ = 499,0 s).

Beide Abschluss-Wortlaute tragen die Zahl **im Text**: „v band-flach gemessen
**bis herab zu 24 s Latenz** (24-s-Zellen, 2-s-Rohdaten als Floor)". Die zwei
Lesungen werden mitgetragen, nie geglättet — Fluss: „24 s ist die Kadenz,
nicht das Medium"; Berg: „das Flat-Verdikt trägt die benannte Grenze δv".

## DM-Regal — die Messung trägt ihren Fehlerbalken

ATNF misst DM je Pulsar **mit** Unsicherheit: die Messung ist „DM ± σ", nicht
„DM". Das Regal trägt daher **zwei Spalten** (`dm_pc_cm3, dm_unc_pc_cm3`),
Kernel-Format wie `gaia_edr3_passbands.dat` (eingebettet, sha256 über den
Datenkörper), erste kuratierte Einträge, `phi/sources.φ`-Url-Line für die
CDN-Manifestation. DM je Pulsar = Messung; daraus abgeleitete Karten =
Modelle — getrennt gehalten.

## Vorab genagelte Abschluss-Wortlaute (Konditionierung vor Verdikt)

- **flach:** „Alle Bandpaare liegen unter der konditionalen Null oder unter
  δτ_min = 24 s. Die band-flache Konstante v = PROPAGATION_SPEED[force] ist
  damit die gemessene Nullhypothese mit benannter Auflösungs-Grenze
  δv = 1,44e7 m/s bei 1 AU. Das v(f)-Regal trägt die flachen Messzeilen
  (v = c je Band, v_unc = δv); der Feld-Pfad bleibt unverdrahtet."
- **Schichten:** „Mindestens ein Bandpaar liegt über der konditionalen Null
  bei |δτ| ≥ 24 s. v(f) wird Messpflicht und ist dreischichtig verdrahtet in
  derselben Sitzung (Schreiben → pack_window → WGSL-Binding 5)."

Rayleigh (force 4, seismische Oberfläche) bleibt eine eigene
pending-Regalklasse — kein Abkürzen über sie.

## Mess-Design (Rat-Verdikt 2026-09-08)

- Probe: `tools/measure/src/bin/dispersion_solar_probe.rs`, Muster
  `corona_conditional_probe`; CLI `--year <aia_lines.bin> <goes-dir>`,
  `--max-lag` (Default 8), `--n-surr` (10), `--confound goes`,
  `--write-shelf` (schreibt nie ohne).
- Richtung je Bandpaar: `D_freq(ℓ) = TE(hi→lo|C) − TE(lo→hi|C)` über
  `transfer_entropy_conditional_h`; Null = `conditional_te_stats_lagged`
  (`te.rs:733`, Residuum-Surrogat, OLS auf eigenen Lags + Hüllen-Lags,
  mean + 2σ über 10 Surrogate); PE-Gate `permutation_entropy`
  (`te.rs:1630`), |pe − mean| > 2·sd → separat als `gated`.
- `δτ = sign · ℓ* · 24 s`; `freq = C_LIGHT / (λ · 1e-10)`.
- Die Sonne ist der freie Kalibrier-Sender (bekannter Ursprung aus
  Ephemeriden, 1 AU, Mehrband-Emission, **2514 Schüsse** — gemessener
  Ereignis-Bestand der `solar-seconds-matrix`, 2013–2015) — der Zirkel
  Ursprung ↔ Dispersionsparameter ist gebrochen.
- Neupert-Abzug **Quell-Seite** → das Residual ist die **Medium-Seite**
  (die Hüllen-Lektion der Korona-Kaskade).

## Regal-Mechanismus

`v_freq_shelf.dat` v1: Kopfzeile sha256 über den Datenkörper, Spalten
`force_type freq_hz bin_width_hz v_m_s v_unc_m_s epoch_tdb`; Lookup
`v_at(force, freq, bin_width)` über `band_overlap` — keine Interpolation,
keine β-Formel. Ein leeres Regal ist ein echtes Objekt (sha256 über den
leeren Körper).

## Verdrahtung beider Ausgänge (explore A, gemessen)

- **flach:** Feld-Pfad bleibt unverdrahtet; das Regal trägt die Messzeilen.
- **Schichten:** dreischichtig — CPU `signal_reach`/`propagation_speed`
  (`src/archivar/membrane.rs:320/362`) + Retardation
  (`src/archivar/spatial.rs:408-413`) → `pack_window` (`actuators.rs`,
  meta[m+11] = freq, meta[m+12] = bin_width) → WGSL `PROPAGATION_SPEED[ft]`
  liest `props` freq/bin_width (`shaders.rs:5-15`, `val_eff_at:105`,
  `val_eff_grad:117/121`, `osc_field:136`, `osc_flow:155`); f64/f32-Parität.

## Delegation

Die parallele Kybernautin führt den Ortungs-Test aus —
`docs/auftrag/auftrag-dispersions-ortungstest.md`. Sie bekommt **nur** ihren
Auftrag, kein Blatt außer dem Auftrag.

## Gates und Puls

Broken-null-Kontrolle (Muster `flare_envelope_*`); `cargo check --workspace`
0/0; die Kalibrier-Gates der TE-Maschine bleiben unberührt.

## Abschluss

Die TODO-Zeile „Spektrale Achse" wird durch einen der beiden vorab genagelten
Wortlaute ersetzt; Regal + Sonne-als-Kalibrier-Sender werden registriert;
`probe-commit` ist gefüllt. Bis dahin `pending`.
