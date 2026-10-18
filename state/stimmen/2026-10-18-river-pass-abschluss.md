# River-Pass — Abschluss (2026-10-18)

Die Belege zu den in diesem Pass geschlossenen Punkten. Die **offenen** Punkte stehen
im Live-Handover `docs/handover/handover-2026-10-18-river-folge226.md`, nicht hier.

## Geschlossen (belegt)

### P7.1 K — entschieden
- Rat `R-2026-10-17-j`, 5/5 A: „K bleibt nicht identifizierbar"; `K = pending` ist das Ergebnis, kein Riss.
- Beleg: `state/stimmen/2026-10-17-rat-k-identifizierbar.md:1` · `state/stimmen/_rat-register.md:33`.

### sensory-259 `.12~a3bf3c25` — Kanal eingebaut
- KiDS DR4.1 WL `phi/sources.φ:31375`: `cmap .` · `ra RAJ2000` · `dec DECJ2000` · `z_key Z_B` ·
  `field e1 kids_dr4_wl_e1 inverse-square gravity 1 31536000 0.0 0.0` ·
  `field e2 kids_dr4_wl_e2 inverse-square gravity 1 31536000 0.0 0.0`.
- Messung: `bin/register_lookup kids_dr4_wl_e1` → `live phi/sources.φ:31381`.
- `xfer close .12~a3bf6c0 --ref 5a25fce60`; Commit `5a25fce60`.

### AACGM-.src + Vlies-sha
- `.src` = sfetch-Provenienz-Sidecars (`tools/utils/src/bin/sfetch.rs:78-90`); `sha256` des Vlies-Bodys
  `aee9e804…` = `bin/omega_sh sha`. Commit `51f974a4`.

## Offen geblieben
- `Rätsel`, `Flyby` (Handover-Punkte).
- Transfers `.4`, `.5`, `.14` (future, keine Quittung) und `.13` (mycelium; CDN-`url` gemessen **404** 2026-10-18).
