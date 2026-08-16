# Auftrag: Crossmatch-Welle — 12 distanzlose Kataloge mit Gaia-Distanz nachkompilieren

## Ziel

Die distanzlosen Sternkataloge tragen nur `ra`/`dec` und fallen pauschal auf die
Referenz-Sphäre (1 kpc). Mit `tap_compiler --crossmatch` bekommen sie echte
Gaia-Distanzen (`dist_pc`), wo Gaia gemessen hat.

Muster (fertig vorhanden): Katalog **lmxbdata** (`phi/sources.φ` mit
`dist dist_pc` + `dist_scale 3.085677581e16`) und CI-Schritt „Compile LMXB" in
`.github/workflows/kernel_flatten.yml` mit `--crossmatch 'I/355/paramp:RA_ICRS:DE_ICRS:Dist'`.

## Ablauf — sequenziell, ein Katalog nach dem anderen

Pro Katalog:
- (a) bestehenden CI-Schritt kopieren, `--crossmatch 'I/355/paramp:RA_ICRS:DE_ICRS:Dist'` vor `--out` einfügen.
- (b) im `phi/sources.φ`-Block `dist dist_pc` + `dist_scale 3.085677581e16` nach `dec` einfügen.
- (c) lokal kompilieren (ohne `--ci-mode`), verifizieren, dann mit `--ci-mode` auf den CDN hochladen.
- (d) Ledger-Eintrag in `phi/port/ledger.φ`.

## Kataloge

| Katalog | Tabelle | Datei |
|---|---|---|
| gcvs | `B/gcvs/gcvs_cat` | gcvs_cat.json |
| cbdata | `B/cb/cbdata` | cbdata.json |
| denis | `B/denis/denis` | denis.json |
| merlin | `B/merlin/merlin` | merlin.json |
| pastel | `B/pastel/pastel` | pastel.json |
| polarbase | `B/polarbase/polarbase` | polarbase.json |
| psr | `B/psr/psr` | psr.json |
| sb9 | `B/sb9/main` | sb9.json |
| vsx | `B/vsx/vsx` | vsx.json |
| wds | `B/wds/wds` | wds.json |
| CoRoT | `B/corot/Bright_star` | corot.json (neu) |
| mktypes | `B/mk/mktypes` | mktypes.json (neu) |

Spalten (aus `kernel_flatten.yml`, TAP-Schema verifiziert 2026-08-16):
- gcvs `ra:RAJ2000;dec:DEJ2000;mag:magMax;period:Period` (skip-null mag)
- cbdata `ra:RAJ2000;dec:DEJ2000;mag1:mag1;m1:M1;m2:M2` (skip-null mag1)
- denis `ra:RAJ2000;dec:DEJ2000;imag:Imag;jmag:Jmag;kmag:Kmag` (skip-null jmag)
- merlin `ra:RAJ2000;dec:DEJ2000;freq:Freq;ampflux:AmpFlux` (skip-null freq)
- pastel `ra:RAdeg;dec:DEdeg;teff:Teff;logg:logg;mag:Vmag` (skip-null teff)
- polarbase `ra:RA_ICRS;dec:DE_ICRS;teff:Teff;logg:logg;mag:Vmag` (skip-null teff)
- psr `ra:RAJ2000;dec:DEJ2000;p0:P0;dm:DM;s1400:S1400` (skip-null dm)
- sb9 `ra:RAJ2000;dec:DEJ2000;mag1:mag1;mag2:mag2` (skip-null mag1)
- vsx `ra:RAJ2000;dec:DEJ2000;max:max;min:min;period:Period` (skip-null max)
- wds `ra:RAJ2000;dec:DEJ2000;mag1:mag1;mag2:mag2;sep:sep1` (skip-null mag1)
- CoRoT `ra:RAJ2000;dec:DEJ2000;mag:Vmag;teff:Teff;logg:logg` (skip-null mag)
- mktypes `ra:RAJ2000;dec:DEJ2000;mag:Mag` (skip-null mag; 1,06 Mio Zeilen — bei Zeitlimit `--limit 20000` / `--xmatch-radius 1.0`)

## Sonderfälle

- **wd** (Weiße Zwerge): eigene Parallaxe, kein Crossmatch. Block um `plx plx`
  ergänzen; Distanz-Ableitung in `src/main.rs` (`Some(plx) if plx > 0.0 => PARSEC_M * 1000.0 / plx`).
- **evs_cat**: B1950-Koordinaten (Epoche, nicht J2000) — Crossmatch würde
  fehlmatchen. NICHT kompilieren, als Gap im Ledger registrieren.

## Regeln

- `cargo check` 0 Fehler UND 0 Warnungen.
- Beobachtende Sprache, kein `failed`/`error`/`fallback`/`default`.
- Keine Kommentare im Code, Name = Implementation.
- Jeder Commit schließt/öffnet einen TODO-Punkt; TODO im selben Commit.
- Vor `--ci-mode` lokal gegenprüfen: JSON valide, `dist_pc`-Spalte vorhanden.

## Verifikation

1. `cargo test --bin omegaflow test_extract_cmap` (4 Tests) grün.
2. Pro Katalog `grep -c dist_pc <name>.json` > 0, Block zeigt `dist dist_pc` + `dist_scale`.
3. `cargo check` 0/0, Commit schließt TODO + Ledger gemeinsam.
