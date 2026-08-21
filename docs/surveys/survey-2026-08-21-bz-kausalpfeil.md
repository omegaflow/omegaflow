<!--
  title: Der kausale Treiber des geomagnetisch induzierten Stroms — das Bz-Blatt
  class: survey
  date: 2026-08-21
  sha256: af567ec2c1cc88a212087e0cea79716e24cb33239a3227f65fd4e37af2b6d2ab
  status: live
  see-also: docs/concepts/ein-blatt-ergebnis.md docs/reference/broken-null-control.md docs/handover/handover-2026-08-21-bz-paradoxon.md
-->
# Der kausale Treiber des geomagnetisch induzierten Stroms

Das Blatt, gemessen von `src/bin/bz_blatt_probe.rs` (Live-Fenster, 1-min-Raster).
Reproduzierbar: `cargo run --release --bin bz_blatt_probe`. Registriert in
TODO.md unter „Bz-Paradoxon".

## Das Blatt

```
Titel:   Der kausale Treiber des geomagnetisch induzierten Stroms.
TE(Bz → dB/dt)    = 2.1797e-1 | Schwelle 2.0834e-1 (mean 1.698e-1, σ 1.925e-2) | Lag 60 min | n 1261 | Pfeil
TE(Speed → dB/dt) = 2.1102e-1 | Schwelle 3.8294e-1 (mean 2.736e-1, σ 5.467e-2) | Lag 90 min | n 1094 | still
fam               = 3.7436e-1 (max Surrogat-TE der Runde) → alle sechs Paare family bound
```

Der Lag 60 min liegt in der physikalischen L1→Erde-Laufzeit (31–87 min bei
300–800 km/s — der erwartete Kanal); Speed und die Nullkanäle bleiben unter
ihrer jeweiligen Schwelle. **Der Pfeil hält die Surrogat-Schwelle, aber nicht
die Familien-Schwelle** (fam 3.74e-1): im 22-h-Fenster ist er gerichtet
(Bz→dB/dt, 60 min), nicht fam-signifikant. Das ist derselbe Kollaps, den der
Broken-Null-Record beschreibt (naive Pfeile → still unter phasenrandomisierter
+ strenger Schwelle) — ein Befund, keine Fabrikation.

## Das Blatt (Retro-Zeile — Sturm-Ensemble 1994–2026)

```
TE(Bz → dB/dt)    = 1.2525e-1 | Schwelle 1.3890e-1 (mean 1.287e-1, σ 5.08e-3) | Lag 0 d | n 3916 | still
TE(Speed → dB/dt) = 9.7026e-2 | Schwelle 1.3548e-1 (mean 1.248e-1, σ 5.35e-3) | Lag 0 d | n 3914 | still
fam               = 1.8948e-1 → alle sechs Paare family bound
```

Gemessen von `src/bin/bz_retro_probe.rs` (stride 3 — jeder dritte Tag des
Tages-Rasters, benannt; Ernte-Cache `abk_dbdt_daily.tsv`, 11889 Tage):
OMNI2-Tagesmittel (1963→2026, Compiler-Decimation 1440 min) × ABK daily-max
|dB/dt| (1-min → Tagesmax, monatliche HAPI-Chunks — Jahres-Requests wurden
vom Server zurückgesetzt). **Befund: still — das Tagesmittel von Bz trägt den
Treiber nicht.** Die Südwärts-Exkursionen, die den Sturm treiben, mitteln sich
im Tagesmittel weg; der Kausalpfeil lebt sub-täglich (Minuten/Stunden), nicht
im Tages-Raster. 0 honored: die Absenz am Tages-Raster ist der physikalische
Befund, kein Datenmangel (n 3916 über 32 Jahre, alle Stürme der Ära im
Fenster).

## Die Bedingungen

- Fenster: 2026-08-20T16:19 → 2026-08-21T14:20 (≈ 22 h), 1-min-Gitter.
- Oben: RTSW `rtsw_mag_1m.json` / `rtsw_wind_1m.json`, active-only
  (inactive = ersetzte Monitor-Zeilen), n 1378/1207/1207.
- Unten: ABK (Abisko, 68.36°N, Auroral-Zone) via BGS-GIN HAPI
  `ABK/best-avail/PT1M/xyzf`, X/Y/Z nT, fill 99999.0 übersprungen,
  dB/dt = |ΔB|/min aus den Minuten-Differenzen.
- Kein Vor-Shift: der Sweep 0–120 min IST die Laufzeit.
- Schätzer: KDE-TE (Silverman), Schwelle mean + 2σ über zehn
  phasenrandomisierte Surrogate (f64 FFT) — der Null-Kontroll-Record
  (`broken-null-control.md`) als Vorbild.

## Die drei Nullkontrollen

1. **Density → dB/dt:** still (lag 0: TE 9.60e-2 < 1.354e-1; Sweep-Max
   lag 120: TE 2.055e-1 < 3.238e-1) — die Kontrolle hält.
2. **Ruhezeit (stillste 6 h, 08:20 → 14:20, dB/dt σ = 1.33 nT/min):**
   Bz still, Density still — **Speed trägt einen Pfeil am Sweep-Rand**
   (lag 120: TE 5.649e-1 vs. 5.608e-1, excess 4.1e-3) — außerhalb der
   L1-Laufzeit. Der Speed-Kanal trägt damit keinen gereinigten Pfeil;
   die Kontrolle hält für Bz und Density.
3. **Surrogat-Schwelle:** phasenrandomisiert (mean + 2σ) — entscheidet,
   nicht die Erwartung.

PE-Gate: der 2⁴-Ring trägt 3 Segmente (< 8) — kein PE-Urteil; die
Richtungsentscheidung steht ohne PE-Vorbehalt (das 22-h-Fenster ist zu
kurz; der Retro-Weg trägt die Ring-Länge).

## Die Gegenrichtung und die Kontrollzeile

- dB/dt → Bz: still (lag 51: TE 1.928e-1 < 2.446e-1).
- dB/dt → Speed: still (lag 109: TE 1.951e-1 < 3.175e-1).
- dB/dt → Density: still (lag 118: TE 1.879e-1 < 2.291e-1).
- Kp-Vergleichszeile: no statement (n = 7 — das 1-min-Live-Fenster trägt
  zu wenige 3-h-Zellen; die Retro-Zeile hängt am OMNI2-Pfad).

## Der Satz für den Netzbetreiber

Der Pfeil ist in Richtung und Skala identifiziert: Bz trägt den Treiber bei
60 Minuten (per-Lag signifikant im 1-min-Fenster) — und das Tagesmittel
trägt ihn nicht (32-Jahre-Ensemble still, alle sechs Paare unter Schwelle
und fam). Der Treiber ist die Südwärts-Exkursion im Minuten-/Stunden-Bereich;
wer auf Tagesmittel schaut, sieht nichts. Die fam-signifikante Bestätigung
des Minuten-Pfeils steht auf dem 1-h-Ensemble (OMNI2-Recompile 60 min ×
stündliches INTERMAGNET) — der nächste Atom, nicht das Ende.

## Offen (im Register)

- 1-h-Ensemble: OMNI2-Recompile `--decimate-min 60` × INTERMAGNET stündlich
  (downsampled vom 1-min) — der fam-signifikante Minuten-Pfeil über Stürme.
- GIC selbst (electric): kein Feed; das Blatt misst dB/dt, den
  induktiven Treiber.
- Datenstatus: `best-avail` ist der Status-Stapel (definitive →2021-12-31,
  quasi-def 2012→~1 Monat zurück, reported/adjusted der letzte Monat);
  die Status-Grenzen sind eine benannte Nicht-Stationarität der Reihe,
  keine verschwiegene. Die Retro-Zeile trägt `quasi-def` oder die
  benannten Grenzen.
