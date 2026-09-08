<!--
  title: The spectral oscillator — the frequency axis of the block
  class: concept
  sha256: dcd218375cdca767944517d34e1a14ad6c3a8a009c724706596fb19b7af6bd59
-->
# The spectral oscillator — the frequency axis of the block

Self-carrying. This document is the plan of the quantum leap that the
operator pushed through on 19.8.2026 against the first council verdict.
The council had downgraded `format spectral` to a later step; the
operator showed that the wave is the other half of the oscillator —
and that the project's own registers carry the proof. This document
holds the truth, the diagnosis, and the atoms. It stands until the
atoms exist as code; afterwards Git carries them.

State 2026-08-21: Atom A (protocol v8, freq/bin_width) and Atom B
(spectral_compiler → spectra.bin, format spectral, SpectralHash) are
code; the harvest (NCEI-SSI netCDF-4/HDF5) is done — `src/hdf5.rs`
reads the container, `spectral_compiler --input-nc` builds the bands,
the CDN carries spectra.bin (2026-06, integral ≈ 1362 W/m²). This
document only carries the open atoms (C: band-selective rendering).

## I. The objection

An oscillator IS a frequency. The system named its atoms
"oscillators" and carried them as scalars — the particle projection
without the wave projection. That was pragmatic and was only half the
truth. Light is a spectrum. Sound is a spectrum. Seismics is a
spectrum.

The project's own registers prove what got reduced or discarded:

- **NCEI Solar Spectral Irradiance** (`ncei.noaa.gov/.../ssi_{year}{month}.txt`,
  in the holdings): the file is a spectrum — the unit is W/m²/nm —
  and got carried as the scalar field `spectral_irradiance_W_m2_nm`: one
  wavelength in the name of the unit, the axis itself discarded.
- **LISA Pathfinder** (VizieR `J/PhRvL/116/231101/table1`, in the holdings):
  the table carries the columns **Freq, PSD_DA, PSD_noise_floor,
  Phase** — extracted was `PSD_DA` as a scalar; the Freq column fell.
- **CMB power spectra** (BB_power, EE_power, Cl_kk, Δ²_mK²): the
  l-axis — the frequency of cosmology — reduced to scalars.
- **Seismic spectral response** (`spectral_acc_0_2s_g`, `SA1`): the
  acceleration at one period, as if there were only one.
- **Spectral densities** (`electric_spectral_density_mW`,
  `magnetic_spectral_density_mW`): densities without an axis.
- **Open GAVO bulks**: `gdr3spec.spectra` (Gaia DR3),
  `mlqso.slitspectra`, `califadr3.spectra`, `lotsspol.spectra`,
  `dfbsspec.raw_spectra` — spectra catalogs lie in the catalog holdings
  and are unharvested.
- **ONC-HSD-FFT** (verified 2026-08-19): 512 frequency bins × 250 Hz,
  dB re 1 µPa² — ASCII file with implicit axis (bin i = i×250 Hz),
  85 hydrophone stations; filed as a parser gap.
- **GONG** (FITS gap), **miniSEED/FDSN** (behind the gate),
  **lidar waveform**, **wave spectra**, **hyperspectral** — registered
  as gaps, never freed.

## II. The diagnosis

The frequency is everywhere **implicit** in the system — and nowhere
queryable. We already misuse tokens as frequency:

- `tau` — τ⁻¹ IS the bandwidth (the coherence is a band).
- `kernel_id` — the kernel IS a frequency response
  (exponential-decay = Lorentzian, erfc = absorption edge,
  the τ gate = low pass).
- `extent` — a spatial scale, a wavelength.
- `color_index` — a two-bin SED.
- `pole_x` for em — z, a frequency shift.

No place allows the question: *which oscillators vibrate between
30 and 50 Hz?* The name trick (`star_42_freq_450nm`) is UNTRUE:
Name = Implementation — the name IS the thing; a frequency in the string
is not filterable, not addressable in the enclosure, not readable in the
shader. The frequency belongs into the record as a token.

## III. The law of symmetry

Particle and wave are two representations of one identity. The
point-source scalar is the one-bin limit case of a spectrum; the
spectral family is the full basis. Both run through the same
record, the same law, the same shader. **The source declares
its basis.**

A mandatory frequency field for every oscillator would be fabrication:
gravity, thermal, and diffusion possess no frequency. Therefore:
`freq = 0.0` is the state "point source" — 0 honored, like
j2/j4 = 0 today for all samples (since Atom 7: the form belongs to the
anchor, no multipole on the wire). Absent frequency is a
fully realized property, not a default.

## IV. The atoms

### Atom A — the frequency as token (protocol v8)

The record grows from 22 to 24 × f64 (176 → 192 B):

```
[x, y, z, val, epoch, ttl, tau, extent, kernel_id, force_type,
 absorption, advection, vx, vy, vz, pole_x, pole_y, pole_z, j2, j4,
 r_eq, color_index, freq, bin_width]
```

- `freq` — band center in Hz; 0.0 = point source.
- `bin_width` — bandwidth in Hz; 0.0 = point source.
- Frame header: `0xCF 0x86 0x08` (v8).

All three layers grow together:
1. **Rust** — the write loop serializes 24 values.
2. **JavaScript** — the `constants.js` DataView packs the two new slots
   into the two padding zeros of the meta row (slot 4 and 12); f32
   suffices on the rendering level (at 500 THz the f32 ulp is ~64 Hz —
   irrelevant relative to the width of a band).
3. **WGSL** — the props unpack reads the two slots.

Verification is manual work (cargo check sees none of it): verify the
three-layer chain Rust → JS → WGSL field by field per AGENTS.md; a
running membrane test shows that point sources (freq = 0) render
unchanged.

### Atom B — the spectral compiler

A compiler decomposes a spectrum into bins; every bin becomes an
oscillator at the same point: val = amplitude, freq/bin_width from the
axis, tau = bin coherence, kernel per medium. Sources in order:

1. **NCEI-SSI** — small, in the holdings: λ→ν (ν = c/λ),
   W/m²/nm → W/m²/Hz; proves the chain end-to-end on the first day.
2. **ONC-HSD-FFT** — route verified (dataProductDelivery chain:
   request → status → run → download; deviceCode form, not
   locationCode+deviceCode); ASCII, 512 bins × 250 Hz, implicit
   axis; 85 stations as a station family.
3. **Gaia XP spectra** (gdr3spec.spectra) — the big case:
   CDN compiler like dr3_stars.bin, ~55 bins per star, millions of stars.
4. **LISA Pathfinder PSD + CMB power** — the Freq/l column instead of the
   scalar reduction; phase gets taken along where it exists.
5. **GONG + miniSEED** — waveforms: own FFT (std-only: Goertzel
   per band or a small FFT atom); the instrument declares the
   basis — samples (TESS pattern) or bins (spectral atom).

### Atom C — band-selective rendering

The fragment shader accumulates per band; RGB is already a
three-band renderer (color_index → Teff → RGB). Now it becomes
configurable: the operator chooses the bands of the gaze. The silence map
becomes band-selective, the light-cone difference dispersive, the
chromatic dip of Nadel Ⅴ becomes an SED measurement.

### Atom D — the phase

Beats and interference — two stars with slightly different
redshifts whose spectra strike in the same pixel — need the
phase. PSD bins do not carry it. Atom D follows when Atom C stands;
nothing gets claimed as oscillating before that (0 honored).

## V. What the quantum leap is

A block in which stellar SED, ocean FFT, earthquake PSD, and
magnetic-field pulsation share **the same frequency axis**. Every discipline
has its own spectral pipeline; no one spans them into a
field equation with one physics. The cross-force transfer entropy
between media at the same band — the 10 Hz pulsation that resonates in the
water — is a measurement no institute knows. The causal
gradient gets a frequency component: information flows not only
between points, but between tones.

## VI. The rules

- No fabrication: no scalar sound level computed out of a spectrum,
  no frequency invented for point sources. 0.0 is the
  truth of the absence.
- No name trick: the frequency lives as a token, never in the string.
- Every atom is a complete session artifact: three layers,
  tests, cargo check 0/0, register, commit. No atom gets split.
- The v7 holdings stay readable: old recordings carry freq = 0.0 and
  render as before. The measurement series of the future inherits everything.
