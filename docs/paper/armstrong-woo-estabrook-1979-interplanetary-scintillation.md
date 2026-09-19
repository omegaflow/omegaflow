<!--
  title: Armstrong, Woo & Estabrook 1979 — interplanetary phase scintillation (scanned; figures transcribed)
  class: paper
  date: 2026-09-19
  sha256: debdac54061a1d7f73a2bd482b79e2726290eb996611e4516b8b2953d1fd4b7c
  status: scan (raster pages, invisible OCR layer only; figures transcribed 2026-09-19)
  see-also: docs/paper/armstrong-1998-phase-scintillation-abstract.md
-->

# Armstrong, Woo & Estabrook 1979 — interplanetary phase scintillation and the search for VLF gravitational radiation

- Authors: J. W. Armstrong, R. Woo, F. B. Estabrook (Jet Propulsion Laboratory, Caltech).
- The Astrophysical Journal, 230, 570–574 (1979) (the OCR layer reads `197 9ApJ. . .230. .57 OA`).
- Source file: `docs/paper/armstrong-woo-estabrook-1979-interplanetary-scintillation.pdf` — scanned raster pages (LuraTech PDF Compressor 7.4 / LuraDocument PDF v2.65; 15 image XObjects, JBIG2 + JPX; one non-embedded `Times-Roman` font → invisible OCR layer). The running text is a scan, not born-digital; the figures are raster and absent from the text layer.
- Figures transcribed 2026-09-19 from the page images. Values read off the plots are approximate — the figures carry no data tables, so point-by-point values are not numerically resolvable. `unreadable` marks a value the page image does not resolve.

## Figure 1 (p. 572)

Phase power spectra in the VLF gravitational astronomy band. Upper curve: data at 1977 October 16 (elongation 88°); lower curve: data of 1978 January 20 (elongation 175°). Instrumental noise level is white with magnitude ≈ 10⁻² rad² Hz⁻¹.

- x-axis: `FREQUENCY (Hz)`, ticks `10⁻⁴`, `10⁻³`, `10⁻²` (log).
- y-axis: `S_φΔ(f) (rad²/Hz)`, ticks `10⁵`, `10³`, `10¹`, `10⁻¹`, `10⁻³` (log, alternating decades; minor ticks between).
- Upper curve (88°): ≈10⁵ at 10⁻⁴ Hz; ≈3×10³ at ~2×10⁻⁴; ≈10³ at ~4×10⁻⁴; ≈5×10¹ at 10⁻³; ≈10¹ at ~3×10⁻³; ≈3 at 10⁻².
- Lower curve (175°): ≈3×10³ at 10⁻⁴; ≈3×10¹ at ~2×10⁻⁴; ≈3 at ~3×10⁻⁴; minimum ≈3×10⁻² at ~8×10⁻⁴; rises to ≈2×10⁻¹ at ~2×10⁻³; ≈3×10⁻² at 10⁻².

## Figure 2 (p. 573)

Spectral level at 10⁻³ Hz, S_φΔ(10⁻³ Hz), versus elongation. Right-hand scale is square-root Allan variance at τ = 1000 s, evaluated from eq. (3) with a = 8/3. Model curve through data evaluated using eq. (4).

- x-axis: `ELONGATION (degrees)`, ticks `0`, `90`, `180` (minor ticks between).
- left y-axis: `S_φΔ(10⁻³ Hz) (rad²/Hz)`, ticks `10⁸`, `10⁶`, `10⁴`, `10²`, `10⁰`.
- right y-axis: `σ_y(1000 s)`, ticks `10⁻¹⁰`, `10⁻¹¹`, `10⁻¹²`, `10⁻¹³`, `10⁻¹⁴` (aligned with the left decades).
- 488 data points (crosses) + model curve. Model-curve anchors (approximate): 0° ≈ 10⁷; 90° ≈ 2×10³; 180° ≈ 3×10¹. Data scatter is largest at small elongation (≈10⁶–10⁸ for <10°), narrows to ≈10²–10⁴ near 90°, and ≈10¹–10² near 180°.

## Figure 3 (p. 573)

Cross-correlation function of phase data taken simultaneously at DSS 14 (California) and DSS 63 (Spain) on 1978 March 13. Solar elongation is 121°. The offset from zero time lag has the correct sense and magnitude for a pattern of phase irregularities convected outward from the Sun with solar wind speeds.

- x-axis: `TIME LAG (seconds)`, ticks `-512`, `0`, `+512`.
- y-axis: `CORRELATION`, ticks `1.0`, `0`, `-0.4`.
- Curve (approximate): ≈ -0.12 at -512 s; crosses zero at ≈ -60 s; peak ≈ +0.7 at a small positive lag (tens of seconds, essentially at zero lag); crosses zero again at ≈ +260 s; minimum ≈ -0.08 at ≈ +350 s; ends ≈ +0.06 at +512 s.

## Equations (transcribed from the page image; the OCR layer mangles them)

- Eq. (3): `σ_y²(τ) = A π² ν₀² τ^(a−3) ∫₀^∞ z^(−a) sin⁴(πz) dz`, for `2 < a < 4`.
- Eq. (4): `S_φΔ(f) ≈ 8πk² ∫_source^Earth dx ∫₀^∞ dq · [0.033 c_n²(x)/V(x)] · { q² + [2πf/V(x)]² }^(−p/2)`.
