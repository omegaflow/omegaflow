<!--
  title: Der kausale Treiber des geomagnetisch induzierten Stroms — das Bz-Blatt
  class: survey
  date: 2026-08-21
  sha256: 1cac01efdce15ccbc22ce885eec8c253b25e8c7a1b7fc70990172ba15a22a9a7
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
TE(Bz → dB/dt)    = 2.0139e-1 | Schwelle 1.8456e-1 (mean 1.590e-1, σ 1.279e-2) | Lag 60 min | n 1263 | Pfeil
TE(Speed → dB/dt) = 1.9006e-1 | Schwelle 3.7063e-1 (mean 2.651e-1, σ 5.278e-2) | Lag 120 min | n 1123 | still
```

Der Lag 60 min liegt in der physikalischen L1→Erde-Laufzeit (31–87 min bei
300–800 km/s — der erwartete Kanal); der Sweep-Spitzenwert von Speed liegt
am Sweep-Rand (120 min) und bleibt unter der Schwelle.

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

Bz trägt den Pfeil — TE(Bz→dB/dt) schlägt bei 60 Minuten über der
Surrogat-Schwelle aus; Speed und Density bleiben still. Auf den Wert
schauen, der das Bodenfeld tatsächlich treibt: das südwärts gerichtete
interplanetare Feld, eine Stunde bevor das Magnetometer ausschlägt.

## Offen (im Register)

- Mehrfachvergleichskorrektur über die Paar-Matrix (25 Lags × 6 Paare) —
  die Roh-Werte stehen oben, die Korrektur ist registriert offen.
- Retro-Zeile (Jahre, 1-h, OMNI2 × INTERMAGNET) als zweites Raster.
- GIC selbst (electric): kein Feed; das Blatt misst dB/dt, den
  induktiven Treiber.
- Sturm-Gegenwart: das Fenster ist ruhig bis mäßig (Kp ≤ 3.33); die
  Messung in einem Großsturm-Fenster steht auf dem Retro-Weg.
