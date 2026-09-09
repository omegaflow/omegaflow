<!--
  title: The spectral oscillator — the frequency axis of the block
  class: concept
  date: 2026-09-08
  sha256: 1759ba39ddafdd7c73567511fe825d0062546e404336d5d0a3db4bf550120924
  status: live
-->
# The spectral oscillator — the frequency axis of the block

Self-carrying. This document is the plan of the quantum leap that the
operator pushed through on 19.8.2026 against the first council verdict.
The council had downgraded `format spectral` to a later step; the
operator showed that the wave is the other half of the oscillator —
and that the project's own registers carry the proof. This document
holds the truth, the diagnosis, and the atoms. The atoms now exist as
code — Git carries them; this document keeps the diagnosis (II) and the
law (III) and its state names the current register seats.

State 2026-09-08: Atoms A and B (`src/archivar/spectral.rs`,
`spectral_compiler`, `format spectral`, SpectralHash) and Atom C on the
data side (`band_overlap`, `sed_to_bp_rp`, `color_emission`) are code.
The NCEI-SSI harvest is done (`src/hdf5.rs` reads the container,
`spectral_compiler --input-nc` builds the bands, the CDN carries
spectra.bin, integral ≈ 1362 W/m²). Atom C's rendering claims split by
measurement (below): cone mode and the browser-texture path are
descoped; the silence map lives in LOST_CONCEPTS §14–17. The one open
code duty — the dispersion relation — sits in the thematic handover register
(Spektrale Achse) and the neighboring session.

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

Measured re-check 2026-09-08: the named IDs resolve as follows. LISA
Pathfinder `J/PhRvL/116/231101/table1` does not exist in any VizieR ID
(0 rows across VizieR) and the PRL renders the PSD only as a figure — the
"Freq/PSD_DA columns" were an unverified claim, now struck. The CMB power
spectra are openly downloadable (IRSA Planck release_3 cosmoparams,
`COM_PowerSpect_CMB-*.txt`, l-axis + D_ℓ in µK²) — now located, not yet
registered. ONC carries the HSD `.fft` product (deviceCategory
HYDROPHONE, 85 stations) but token-gated and archived as 5-minute `.mat`
spectra, not `.fft` ASCII — the "512 bins × 250 Hz" geometry is
unconfirmed. Gaia XP `gdr3spec.spectra` is reachable at
`dc.g-vo.org/tap/sync` (41 samples, 400–800 nm, not "~55 bins"). These
are register duties (locate the source or strike the claim), not
holdings.

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
its basis** — samples (a waveform) or bins (a spectrum). The Council
verdict C1 (2026-09-07) adds the third form: a spectrum held whole as a
series (NRS1, geo-series), no band chosen, no scalar invented — a held
spectral record is a fully realized property, never decomposed into
oscillators.

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

Since Atom 7 and Atom D the record is 26 × f64 (208 B), protocol v9,
`0xCF 0x86 0x09`: `phase` and `presence` ride the wire after
`bin_width` (the Atom D bit); the form slots `pole_x/y/z, j2, j4, r_eq`
are pad for gravity (the form belongs to the anchor). The two spectral
slots sit at index 23/24; the wire truth lives in
`docs/concepts/archivar-mathematikerin.md`.

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
3. **Gaia XP spectra** (`gdr3spec.spectra`, verified 2026-09-08 at
   `dc.g-vo.org/tap/sync`) — the big case: CDN compiler like
   dr3_stars.bin; measured 41 samples (400–800 nm, Δλ 10 nm,
   W·m⁻²·nm⁻¹) per star, not the "~55 bins" written here.
4. **LISA Pathfinder PSD + CMB power** — the Freq/l column instead of the
   scalar reduction; phase gets taken along where it exists. LISA
   Pathfinder carries no open tabular PSD (struck, see the re-check under
   I). The CMB power spectra are openly downloadable (IRSA Planck
   release_3, `COM_PowerSpect_CMB-*.txt`, l-axis + D_ℓ), but l is the
   multipole order — an angular axis, not Hz — so the freq/bin_width slot
   does not carry it; descoped from the freq axis (the S² power axis is
   its natural future home).
5. **GONG + miniSEED** — waveforms: own FFT (std-only: Goertzel
   per band or a small FFT atom); the instrument declares the
   basis — samples (TESS pattern) or bins (spectral atom).

State 2026-09-08: item 1 is code. Item 3 (Gaia XP) is built end-to-end:
`gaia_xp_compiler` + the `xp_spectra.bin` v2 format + the ω-loop consumer
(`format xp_spectra`, parallax seat); the CDN workflow is the one open
step. Item 4 is split by measurement: LISA Pathfinder descoped (no open
tabular PSD), CMB power descoped from the freq axis (l is angular, not
Hz; the S² power axis is its natural future home). Item 2 (ONC-HSD-FFT)
is corrected and filed as a `mat5` parser gap. Item 5: GONG modes are
harvested as scalar series (`gong_compiler`); the waveform/bins hold
form is open in its own line; miniSEED/FDSN is a Weberin-line gap. The
compiler also serves the RIXS charge/spin harvests (`rixs_charge.bin`,
158727 oscillators; Kuprat spin, 456 em) — the scattering photon is an
honest lab anchor, val is relative intensity on the loss axis (Council
2026-09-03, no field channel).

### Atom C — band-selective rendering

Built on the data side (2026-09-08): `spectral::band_overlap` is a
postfilter in the ω-loop — the operator's gaze band (lo, hi) filters
`freq > 0` oscillators; `freq = 0` (point source) stays visible — the
band mode enriches, it does not mask. `spectral::sed_to_bp_rp`
(photon-counting over the Gaia EDR3 BP/RP passbands) + `parse_passbands`
turn a spectral asset into a measured color_index; `color_emission`
samples `color_lut_rgba` (`color_for_ci`) into `DiodeState.em_color`.

The three rendering claims of the original atom split by measurement:

- **The silence map** — LOST_CONCEPTS §14–17 (outstanding, "await their
  return"), not an open spectral duty.
- **The dispersive light-cone difference** (cone mode) — descoped: a
  rendering concept in the dead browser branch, never built.
- **The chromatic dip of Nadel Ⅴ** — superseded: the Nadel measures the
  dip achromatically (two-band lightcurve, `lsst_anomaly_probe` +
  `nadel_gate`); a two-band SED would be interpolation. The built
  successor of the goal is the measured color_index from
  `sed_to_bp_rp`, which serves the spectral assets — not the Nadel dip.

### Atom D — the phase

Beats and interference — two stars with slightly different
redshifts whose spectra strike in the same pixel — need the
phase. PSD bins do not carry it. The `phase` and `presence` slots ride
the wire since protocol v9 (the Atom D bit), but nothing reads them into
a beat: Atom C now stands on the data side, Atom D is unbuilt. Nothing
gets claimed as oscillating before it is (0 honored).

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
