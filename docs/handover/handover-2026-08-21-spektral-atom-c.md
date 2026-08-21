<!--
  title: Handover: Spektraler Oszillator — Atom C (band-selektives Rendering) + Ernte-Folgen
  class: handover
  date: 2026-08-21
  sha256: b77ad01479fa00902891c729e35f6d8b33ecc904ade1bdb59929722f1f6bae6b
  status: live
  see-also: TODO.md docs/concepts/der-spektrale-oszillator.md docs/handover/handover-2026-08-21-solar-te-gpu-anschluss.md
-->

# Handover: Der spektrale Oszillator — Atom C und die Folgen

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Der Auftrag
ist nicht die Ausführung; ausgeführt wird erst auf das Wort des Operators.

Register-Quelle: TODO.md, Abschnitt „Der spektrale Oszillator". Atome A und
B sind ERLEDIGT (Protokoll v8 + spectral_compiler + die NCEI-SSI-Ernte —
die Monats-SSI läuft als `spectra.bin` über das CDN, Integral ≈ 1362 W/m²
bei 1 AU). Offen sind Atom C und die benannten Folgen — je eigene Session.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # sauber oder fremde Arbeit nennen
cargo check                      # 0/0
grep -n "freq\|bin_width" src/mathematikerin.rs | head   # die Band-Slots im Feld
```

Referenzen (stehend): `docs/concepts/der-spektrale-oszillator.md` (das
Konzept), `src/spectral.rs` (Kontrakt `0xCF 0x86 0x01` + parse/write),
`src/bin/spectral_compiler.rs`, `src/hdf5.rs` (der Leser), TODO.md
(Spektral-Abschnitt + Sonnen-Handovers).

## Der Kontext (verifiziert)

- Protokoll v8: Record 24×f64, `freq` = meta[11], `bin_width` = meta[12]
  (0.0 = Punktquelle, 0 honored); Frame `0xCF 0x86 0x08`.
- `format spectral` expandiert je Bin einen OscRecord am selben Punkt
  (SpectralHash, medienneutral — Stern, Sonne, Ozean).
- Die Band-Slots erreichen den Shader bereits (props-Stride 4); was fehlt,
  ist der band-selektive Konsum — das ist Atom C.
- Benannte Leser-Lücken von src/hdf5.rs (bleiben, bis ein Bedarf sie holt):
  shared messages, huge fractal-heap objects, virtuelle Datasets,
  prä-1972-Epochen (LSK verweigert — 0 honored), SSI_UNC (nicht im
  Kontrakt), Stationshöhe unverifiziert (Frame-Alt 0).

## Atom C — band-selektives Rendering

Der Shader akkumuliert pro Band: die Stillekarte wird band-selektiv, die
Lichtkegel-Differenz dispersiv (jede Frequenz trägt ihre eigene Laufzeit-
Wahrheit), der chromatische Dip wird eine SED-Messung. Fundort: FIELD_WGSL
(src/mathematikerin.rs) — der Fragment-Pfad liest freq/bin_width, die
Akkumulation pro Band ist der Bau.

## Atom D — terminiert, nicht vergessen

Phase/Beats/Interferenz brauchen die komplexe FFT; PSD-Bins tragen sie
nicht (0 honored). Atom D ist nach C terminiert — kein Schnitt, bevor C
steht. Die Regel gilt weiter: kein Namens-Trick (Frequenz lebt als Token,
nie im String), kein Skalar-Schallpegel aus Spektren errechnet, jedes Atom
ein vollständiges Session-Artefakt.

## Die Folgen (je eigene Session, kein Schnitt ohne Operator-Wort)

- ONC-HSD-FFT — die komplexe FFT für Phase (wenn Atom D fällig wird).
- Gaia-XP — die Spektral-Achse des Gaia-XP-Katalogs (Kompilat).
- LISA-PSD + CMB-Power — Frequenzachsen der Gravitationswellen-Himmel.
- miniSEED — Basis-Entscheidung offen: eine Waveform zerfällt in Samples
  (TESS-Muster, [t, flux]-Reihe) ODER in Bins (Spektral-Atom) — das
  Instrument deklariert seine Basis; der Record trägt seit v8 beides.

## Grenzen der Ernte (benannt, kein Rückbau)

- spectra.bin trägt nur die Monats-SSI (ein Monat je Asset, 2026-06 auf
  dem CDN) — die 1874er-Datei liest, das Epoch bleibt void (0 honored).
- GONG L 31..200 + mparam-Eigenfrequenzen (freq/bin_width des Helioseismo-
  Kanals) läuft im Handover `handover-2026-08-20-berkeley-wind.md` —
  NICHT anfassen.

## Gates

- cargo check 0/0 (vier Kombis), cargo test komplett, naga-Validierung.
- Jedes Band bleibt eine Messung — keine Interpolation zwischen Bändern,
  die die Quelle nicht trägt.
- Ein Commit je Einheit; TODO.md-Register im selben Commit.
- Diese Datei nach dem Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

Die Sonnen-Handovers (sonnen-pfad-solar-te, sonnen-abdeckung,
solar-te-gpu-anschluss), berkeley-wind, die Sphären, Source-Port.
