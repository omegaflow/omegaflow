<!--
  title: Handover — Uranus-Riss-Kontur: das Planetenzentrum (DE441 + ura111xl, mas-Niveau) und die korrigierte Diurnal-Dekomposition
  class: handover
  date: 2026-09-09
  sha256: 70c2c6e4bb6539a5c460f7643c8e4a4a83a352fa574e8a5222cb1535b471f565
  status: live
  see-also: docs/befund/befund-2026-09-08-uranus-diurnal-dekomposition.md docs/befund/befund-2026-09-08-uranus-riss-schiedsspruch.md docs/TODO.md
-->

# Handover — Uranus-Riss-Kontur

Übergabe für die nächste Sitzung. Die Nacht hat den Riss vom Schatten zur
Kontur gebracht: drei unabhängige Bezeugungen stehen, das Planetenzentrum
ist gebaut, und die drei Bau-Linien der Folge sind registriert.

## 0. Die abgebende Sitzung ist abgeschlossen

Commits der Nacht: `47a8d92` + `bbe60d2` (Diurnal-Dekomposition, erste
Version), `4774efc` (Korrektur + Ernten-Fixes + Manifestation),
`687496f` (inpop-epm-cdn-Workflow), `d5c8c72` (motion-Fixes + Lift),
`b6b2520` (Register), `9a59c91` (Planetenzentrum), `2fde843`
(Register-Schärfung). Offen bleiben nur der CI-Lauf (in flight) und die
registrierten Bau-Linien (§4).

## 1. Was gemessen steht — die drei Zeugen

| Zeuge | Messung | Anordnung |
|---|---|---|
| Weberin-Dreilinien | 1 000–8 000 km Divergenz | drei Bureau-Linien gegeneinander (indirekt) |
| Schiedsspruch-Anker | 32–47 mas | Beobachtungen gegen die Modelle (verdict ii) |
| Planetenzentrum | 0,36–1,57·10⁶ m (0,048″ median) | DE441 gegen DE442, direkt am Zentrum |

**Die Unterscheidung, die das Register trägt:** 1,57·10⁶ m (DE441↔DE442,
zwei Auflagen desselben Hauses) gegen 1,59·10⁶ m (Weberin-Riss, drei
Häuser) ist **Skalen-Konsistenz, nicht Identität** — zwei verschiedene
Meßpaare. Daß sogar zwei Auflagen desselben Hauses am Zentrum in
Riß-Skala divergieren, heißt: die Uranus-Position ist beobachtungsmäßig
unterbestimmt — keine zwei Umläufe seit der Entdeckung, ein einziger
Besucher (1986), ein Jahrhundert Bogenminuten-Astrometrie. Der Riss ist
kein Bureau-Streit.

## 2. Der Stand der Diurnal-Dekomposition (korrigiert)

Die erste Instrument-Version (Commit `47a8d92`) maß ein Artefakt-Null —
`body_fixed_to_icrs` nahm die nächste Rotationsmatrix (32-Tage-Raster der
32d-Chebyshev-Granulen) und fror die Station innerhalb einer Nacht ein
(par_ra konstant über 1,3 h). Das Kalibrier-Gate schützte nicht: es prüft
die Fit-Maschinerie gegen sich selbst, nicht die physikalische Form der
Signatur — der physikalische Prüfstein ist die Stundenwinkel-Variation
innerhalb einer Nacht. Die korrigierte Station ist probe-lokal: WGS84 +
IAU 1982 GMST + analytisch ω×r.

Korrigierte Messung: **c_par 0.94–0.95, c_aber ≈ 0** — die Camargo-Tabellen
tragen die topozentrische Parallaxe und keine Diurnal-Aberration:
topozentrisch astrometrisch, das Papier bestätigt. Reduktion: RMS
242.9/222.6/228.9 → 77.5/73.6/73.3 mas — **unter** ⟨σ⟩ = 87.8 mas, der
~170-mas-„Boden" war die Parallaxe selbst. Verdict (ii): die Beobachtungen
schlichten nicht. Der Riss-Anker steht (32.1/39.5/47.0 mas). Befund:
docs/befund/befund-2026-09-08-uranus-diurnal-dekomposition.md.

## 3. Der Infrastruktur-Stand (was die Folge-Sitzung wissen muß)

- `iau_rotate_to_icrs` ist behoben (korrekte IAU-Rotation
  Rz(90°+α)·Rx(90°−δ)·Rz(W)); Regressionstest `test_earth_zenith_geometry`
  (Zenith −22.40° geozentrisch).
- `body_fixed_to_icrs` setzt die nächste Rotationsmatrix mit der PM-Rate
  um die Polachse fort — keine Snap-Einfrierung mehr; `icrs_to_body_surface`
  trägt die konsistente Inverse.
- `light_time_worldline` lebt in `src/archivar/motion.rs`; alle acht
  Proben (zwei Uranus-, vier Pioneer-, Topocentric-, zwei neue) tragen
  keinen probe-lokalen Fold mehr — Reports byte-identisch, Kalibrierung
  1.00/1.00.
- INPOP/EPM-Earth-Bins tragen body-fixed Orientierung (Compiler-Fix
  `--pck pck00010/00011`, audit 120/120); CI `inpop-epm-cdn.yml` hat die
  kanonischen Bins re-manifestiert.
- Camargo-TSVs + ura111 + ura184_part-3 liegen auf dem CDN
  (`camargo-uranus-cdn.yml` mit Release-Anlage).

## 4. Das Planetenzentrum — die technischen Fakten

`ephemeris_uranus_c.bin` = DE441-Baryzentrum (`ephemeris_uranus.bin`) +
ura111xl-799 (799−7-Offset), 2-d-Granulen, 0.1-d-Abtastung, Fenster
1980–2040 — reproduziert seine Quelle auf **RMS 104 m / max 208 m**
(0.007 mas bei Uranus). **Die Nyquist-Lektion:** das erste 32-d-Raster maß
10 km RMS — taub für die 1,4-d-Mond-Wobble des Baryzentrums; erst die
feine Abtastung sieht die Schwingung statt ihres Echos. Gemessen →
Ursache benannt → korrigiert → reproduziert.

Gegen Horizons 799 (Quelle `ura184_merged`, DE442-basiert): 0,36–1,57e6 m
— die Versionen-Differenz, an jedem Punkt der Bahn ablesbar. Der
Horizons-Raster-Pfad (1-d) ist gemessen freigegeben (descoped: ~1 mas
Mittel, 0,36″ Rand-Ausreißer). Ernte registriert in `phi/sources.φ`
(ura111xl-799.bsp + ephemeris_uranus_c.bin); der kernel-flatten-Job trägt
den Kompositions-Schritt (`horizons_compiler --uranus-c-spk`).
**Folge-Sitzung: den CI-Lauf `34287663767` verifizieren —**
`ephemeris_uranus_c.bin` muß auf dem CDN 200 liefern; bei rotem Lauf den
Schritt lesen.

## 5. Die drei Bau-Linien (pending, je Linie)

- (a) **Topozentrische Kopplung gegen das Zentrum** — die Uranus-Astrometrie
  bekommt ihren richtigen Zielpunkt; die Wobble wird Information statt
  Störung (sie trägt die Mond-Massen).
- (b) **Die Versionen-Differenz als registrierbare Größe** — der Riss als
  per-Punkt-Meßwert statt globaler Divergenz.
- (c) **Neptun als zweiter Planet desselben Baus** — Komposition im Stil
  von `--uranus-c-spk`, Registrierung in `phi/sources.φ`, Schritt im
  kernel-flatten-Job.

## 6. Arbeitsregeln für die bauende Sitzung

- **Geteiltes Repo:** die Parallel-Session committet weiter — nur eigene
  Dateien stagen, ihre Hunks nie committen, `.git/index.lock` kurz
  abwarten. `src/` war heute Nacht zwischenzeitlich frei; der Zustand
  wechselt.
- **Commit-Gate:** `cargo check` null Fehler/null Warnungen; `commit_check`
  blockt `unwrap_or(0.0)`, `unwrap_or(0)`, `unwrap_or_default(`, `unwrap_or_else(`, `#[derive(Default)]`, Deutsch-in-Code, verbotene Wörter.
- **Kalibrier-Gate:** eine Signatur gilt erst als gemessen, wenn injizierte
  Fälle exakt zurückgewonnen werden — und die physikalische Form der
  Signatur (Stundenwinkel, Wobble-Frequenz) separat geprüft ist.
- **0-Kanon:** absent bleibt absent; ein gemessen freigegebener Pfad ist
  `descoped` mit Befund, kein Parkplatz.
- **A = A:** Konsistenz ist ein Zwirn, keine Bestätigung — die
  Skalen-Unterscheidung aus §1 gilt auch für die Folgemessungen.
