<!--
  title: Handover — Atom C geschlossen: die offenen Pflichten der Spektralen Achse
  class: handover
  date: 2026-09-08
  sha256: 91225f50275e932f4a8fbd0c80b145c2713754223df86994ddc70750b24cf2bb
  status: live
  see-also: docs/TODO.md docs/concepts/archivar-mathematikerin.md docs/specs/spectral-oscillator.md docs/befund/befund-todo-gegen-code-leichen.md
-->

# Handover — Atom C geschlossen: die offenen Pflichten der Spektralen Achse

Übergabe der Sitzung 2026-09-08: Atom C (band-selektives Rendering, Datenseite)
ist gebaut und committet (404cd78). Dieses Blatt trägt, was offen bleibt —
getrennt nach pending, descoped und tot.

## 1. Geschlossen (Commit 404cd78)

Commit-Lage (gemessen 2026-09-08): 404cd78 trägt den Atom-C-Code und die
Blattkorrektur von `archivar-mathematikerin.md` (cone mode descoped); das
Operator-Wort ist als Code (Band-Gate-Postfilter, `freq = 0` bleibt sichtbar)
in 404cd78 und als Wort in §1 unten registriert; dieses Blatt selbst liegt in
4741582, der Register-Feinschliff (TODO-Ein-Pending, `color_lut_rgb`-Doc-
Korrektur) in abf6d11. Puls (gemessen): `cargo check --workspace` 0/0, die
Atom-C-Suite grün (18 Tests inkl. des 781-Stützstellen- und der
`sed_to_bp_rp`-Tests).

- SED → BP−RP: `spectral::parse_passbands` (Gaia EDR3 BP/RP, 781 Stützstellen,
  Riello+ 2021, 7 Spalten, 99.99 = absent, als Kernel eingebettet wie
  `naif0012.tls` — `src/archivar/kernels/gaia_edr3_passbands.dat`, sha256
  `46160d3b84dc0b78530d1ef3d18c0cf632fdaeb27876c0773c7a62033127f301`) +
  `spectral::sed_to_bp_rp` (photon-counting); die
  Spektral-Emission (`membrane.rs`) trägt den gemessenen color_index statt hart 0.
- Band-Gate: `spectral::band_overlap` als Postfilter im Omega-Loop (filtert nur
  `freq>0`-Oszillatoren). Operator-Wort 2026-09-08 (Gaze): `freq = 0`
  (Punktquelle) bleibt sichtbar — der Band-Modus reichert an, er verdeckt nicht.
- Farbe-Verbraucher: `color_emission` (actuators.rs, Muster `force_ref_medians`)
  sampelt `color_lut_rgba` → `DiodeState.em_color` + HUD; die LUT ist kein
  Orphan mehr.

## 2. Offene Pflicht — pending

- **Dispersionsrelation** (`TODO.md` Spektrale Achse): die Laufzeit-Geschwindigkeit
  bleibt band-flach (v = PROPAGATION_SPEED[force]); eine echte Dispersionsrelation
  (Rayleigh-Oberflächenwelle) ist pending. Die Steckstelle v(freq) steht; kein
  erfundenes v0·(f/f0)^β (0 honored). Getrennt vom cone mode — zwei Zeilen,
  nicht eine.

## 3. Descoped — gemessen freigegeben, kein pending

Kanonsatz (AGENTS.md 0-Kanon, 2026-09-08): descoped ist der Befund als
Eintrag — „gemessen: nie gebaut, nicht gebraucht", kein später, keine Pflege;
ohne Messung wäre es gepudertes Deferred. Jede Zeile trägt ihren Befund:

- **cone mode** (Lichtkegel-Differenz dispersiv): Rendering-Konzept im toten
  Browser-Zweig, nie gebaut. `archivar-mathematikerin.md` führte ihn als „done";
  die Blattkorrektur (2026-09-08) stellt das richtig.
- **Browser-Textur-Pfad** (`color_lut_rgb`, Bindings 9+12): toter Zweig — kein
  Renderer trägt ihn. Die em-Farbe erreicht ihren Verbraucher jetzt über
  `color_emission` (CPU), nicht über die tote Browser-Textur.

## 4. Der tote Zweig selbst

Der Browser-Render-Zweig (index.html, constants.js, fieldShader) existierte in
keinem Commit — gemessen in `befund-todo-gegen-code-leichen.md`. Eine
Wiederbelebung wäre ein eigener Auftrag, keine offene Pflicht dieser Sitzung.

## 5. Register

Die eine offene Pflicht steht in `docs/TODO.md` (Spektrale Achse, Dispersionsrelation);
die Blattkorrekturen (cone mode descoped, em-Pfad `color_emission`/`color_lut_rgba`)
in `docs/concepts/archivar-mathematikerin.md`. TODO trägt nur pending; Geschlossenes
liegt im Commit.
