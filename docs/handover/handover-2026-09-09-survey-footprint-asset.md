<!--
  title: Übergabe — Survey-Footprint-Asset-Klasse (Weberin §9 Stufe 5), geschlossen
  class: handover
  date: 2026-09-09
  sha256: 845410ec99e1dc9e65c4b362fd97c52bc013584755763798138ea29aa4bc7268
  status: live
  see-also: docs/concepts/zeugnis.md docs/concepts/die-weberin.md phi/footprints.φ phi/blocked_sources.φ docs/TODO.md
-->
# Übergabe — Survey-Footprint-Asset-Klasse (Weberin §9 Stufe 5), geschlossen

Eine Folge-Session erbt die Survey-Footprint-Arbeit. Gemessener Stand der
geschlossenen Session; nichts geraten.

## 1. Was gebaut ist (alles committet)

- **Record `FP01`** (`src/archivar/footprint.rs`): HEALPix-Pixel (order + ipix, nest)
  + Coverage-Fraktion je Band, 12-Byte-Record, 13-Byte-Header. Band-Register
  U/G/R/I/Z/Y/J/H/Ks/W1/W2. `magic_identity(FP01) = FeldIdentitaet::Footprint`
  (`src/archivar/zeuge.rs`) — Geschwister der drei Zeugen-Arten, keine vierte Art.
- **Gate** (`footprint_gate`, `direction_gate`, `find_pixel_records`): beobachtet /
  nie-beobachtet / Band-nicht-abgedeckt / pending (zeugnis §7). Nside-bewusst:
  `direction_gate` liest die Auflösung aus dem `order`-Feld der Records (der DES-Pfad
  order 12 bleibt unberührt).
- **DES-Compiler** (`des_coverage_compiler.rs`): anonyme TAP-Route
  datalab.noirlab.edu, Pagination je hpix_4096-Bereich. Aggregation GELOEST:
  MAX je (pixel, band) — die Tafel-Zeilen sind ueberlappende Coverage-Aussagen
  derselben Zelle, SUM/union waere Doppelzaehlung (Fabrication). `collapse_max`.
- **Re-Grid-Kern** (`src/archivar/regrid.rs`): GnomonicRegrid, TAN/SIN-WCS,
  flaechengewichtete Ueberlagerung, 16×16-Subzellen-Quadratur an Grenzen, absent
  traegt nie bei.
- **PS1 fraktional** (`ps1_coverage_compiler.rs`): erntet stack.num je Skycell,
  re-gridded, count→Fraktion. Autoresume (unten).
- **PS1 binaer** (`ps1_binary_compiler.rs`): Rezept A — Archiv-Existenz der
  Skycell-Tiles (ps1grid.fits-Geometrie + ps1filenames.py-Existence, ~7900 Probes)
  → binaere Maske Nside 256 (order 8, ~34 MB). Kein voller Ebenen-Download.
- **Gate-Bindung** (`footprint_gate_probe.rs`): `--survey des-dr2` loest die
  DES-Tafeln (`II/357/des_dr1`, `des_dr2`) gegen `des_dr2_coverage.fp01` auf.
- **SDSS**: ehrlich `absent` (anonyme CAS ist SQL-lesbar, aber keine per-Pixel-
  Flaeche; Imaging liegt als Polygon-Geometrie).

## 2. Stand der Assets (CDN)

- **DES** `des_dr2_coverage.fp01`: fertig, auf der CDN, vertrauenswuerdig
  (MAX-Aggregation, Re-Ernte-Lauf 34222237312, 10.311.965 Records, 25.239.595
  Quell-Zeilen exakt).
- **PS1 binaer** `ps1_dr2_binary.fp01`: Compiler gebaut, CDN-Lauf 34298040472
  laeuft/stand offen — die Maske (~34 MB) ist der Gate-Fussabdruck.
- **PS1 fraktional** `ps1_dr2_coverage.fp01`: laeuft im Autoresume
  (`ps1-cdn.yml`, stuendlicher Cron, Band je Lauf, Combiner `ps1_coverage_combiner.rs`
  merged am Ende). Tiefen-Asset, KEIN Gate-Asset. Preis gemessen: ~60 TB /
  ~100 Tage (ein Band ~1,25 h; der Re-Grid voller Ebenen ist der Engpass).
- **SDSS**: absent. **2MASS**: pending (keine Coverage-Tabelle im TAP).
- **AllWISE**: Quelle gefunden, Compiler (`wise_coverage_compiler.rs`) von der
  parallelen Session gebaut — Ernte offen.

## 3. Die zwei Asset-Klassen (benannt, nicht versteckt)

- **DES fraktional** (aus der Coverage-Tafel, frac_det 0..1).
- **PS1 binaer** (aus Archiv-Existenz, frac=1.0 fuer beobachtet).
Beide genuegen §7. Das Caveat des binaeren Wegs ist benannt: der Fehler geht
Richtung "beobachtet" — der aeuszerste Suedsaum (≤1 Tile) und das ~1,6e-3 deg²
Polarloch koennen ueberzeichnet sein; Quellen dort koennten faelschlich der
ehrlichen Leere zugeordnet werden.

## 4. Was die Folge-Session erbt

- **AllWISE** ernten/registrieren (Route gemessen, W3/W4 im Band-Register noetig).
- **2MASS** entscheiden: pending lassen oder ehrlich absent (keine leichte
  Flaeche-Quelle gemessen).
- **PS1 fraktional** weiterlaufen lassen oder entschieden unterbrechen (Datum +
  Grund benennen, das Autoresume ist der Weckruf).
- **DES-Eintrag** im Register ist auf "GELOEST" gezogen.

## 5. Kosmetische Schuld (bewusst gelassen, Operator-Wort)

`fb3c2e7` traegt eine Footprint-Message auf Dateien der parallelen Session
(Corona-Probes, docs/TODO.md). Liegenlassen entschieden — ein History-Rewrite
(Force-Push) waere teurer als die Schramme.
