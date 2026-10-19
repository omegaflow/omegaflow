<!--
  title: Handover — River-Folge 227 (2026-10-18)
  class: handover
  date: 2026-10-18
  sha256: ac4b271e98af7b88d19d5574ea5b79f3089b961e9483d5acd6f5e97c7a47eab1
  status: live
-->
# River-226

## Burn: open 0.0 · close pending · closed 0 · Grund: nichts geschlossen.

## Offen — eigen

### K
- **Status:** pending
- **Trigger:** Rat R-2026-10-17-j (5/5 A)
- **Lage:** pending (gemessen 2026-10-18) — Mountain-Zeile ist gesetzt (`state/mountain/2026-10-18-k-pending.φ`, Träger `phi:893`, `xfer:mountain.…9~3ed32602`); `phi/sources.φ` hat keine `K`-Skalarzeile (grep 0), die Grammatik trägt `pending` nur als coupling-Rate
- **Blockade:** kein `∇p`, kein `B/j`, Ein-Satellit — die physikalische Impulsbilanz-Lücke; **nicht** die Mountain-Zeile
- **Offen:** `K` selbst bleibt pending (kein `∇p`, kein `B/j`, Ein-Satellit); der TE-Indikator `TE(NVE→IVE | AE+Dst+Kp+LT+Radiallänge)` ist als eigener Punkt **gebaut und voll gemessen**: DE-2 300 s, 4 Bins, n=23200, 1981-08..1983-02 — **3/3 Lags pass** (300/600/900 s). Der frühere Ausfall (n=4413, 1981-08..1982-01) war die verkürzte HAPI-Orbit-Abdeckung, nicht die Konditionierung; SSCWeb liefert 1-min LT/Radiallänge für den vollen Span. **Falsifikationsbedingung:** F1 Einzelindex, F2 gemeinsam — beide nicht eingetreten. Riss: Kp im Register nur `last`-Live-Wert (`phi:1252`), Test liest `hour3h_kp` (WDC Kyoto, CC-BY-4.0) direkt; Radiallänge statt geodätischer Höhe (konstanter Offset, Binning-invariant). Beleg: `state/river/2026-10-18-k-te-kadenz.φ`, `docs/concepts/ion-neutral-vlies.md` (die Weberin-Frage: Fäden, Regime, K), `de2-conditional-te.yml` (Commits `764c5ca2f`, `f2c241df5`, `3b7971533`, `d27791a38`, `f50fe788e`, `3735192ce`)
- **Braucht:** `state/stimmen/2026-10-17-rat-k-identifizierbar.md:1`

### Rätsel
- **Status:** eigen, unerledigt
- **Trigger:** Arbeit
- **Lage:** offen (gemessen 2026-10-18): der 3-h-Lag-Sweep `39154644922` wurde **manuell abgebrochen** (Akteur `omegaflow`, nach 40 min im TE-Schritt; kein Timeout, keine Concurrency) — kein Ergebnis; der Mond-Faden ist gebaut — `--cond lunar` in `de2_conditional_te`, SSCWeb-Lat/Lon im Orbit-CSV, Gezeitenpotenzial gegen `ephemeris_moon.bin`/`ephemeris_earth.bin` (Commit `a50ff1d83`). Lauf `39166259860` (36 Lags, 6 Konditionierer, bins 4): **konditioniert 0/36**, roh 5/36 — der sechste Konditionierer hebt den ARX-Null (thr 0.0018 → 0.0092) bei 4^6 Zellen
- **Blockade:** keine — der nächste Lauf ist ein `workflow_dispatch`
- **Nächster Schritt:** `bins 3` (729 Zellen) **oder** zwei Läufe (5-Kond-Baseline vs. Baseline+lunar, Schwellen getrennt); nicht mehr Konditionierer in eine Zelle
- **Braucht:** `docs/concepts/ion-neutral-vlies.md:1` (die offenen Fäden: Lag-Sweep, Breite/MLT, Mond, Sonnenwind, Bodenstationen) · `docs/concepts/die-weberin.md:1`

### Flyby
- **Status:** termin
- **Trigger:** 2026-11-01
- **Lage:** offen (gemessen 2026-10-18)
- **Blockade:** Termin
- **Braucht:** `state/river/2026-10-18-flyby-sealed-pin.φ:1`; am 2026-11-01 `flyby_ephemeris_gate --recon <esoc.bin> --sigma-recon <km>`

## An mycelium
Origin: river-226
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.13~ea59cd54` KiDS-Transport.

## An future
Origin: river-folge227
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.16~ea09631b` Leichenregister (2 Drafts ohne Punkt, Lauf `39154644922` cancelled, 2 falsche `Braucht`, ci-gate rot); Brief `post/in/2026-10-18-river-an-future-leichenregister.md`.
