# River-Pass — offene Punkte und Transfers (2026-10-18)

Der Record zum Live-Handover `docs/handover/handover-2026-10-18-river-folge224.md`.
Das Handover trägt nur die Zeiger; die Details stehen hier.

## Offen — eigen (3)

### P7.1 K — kond. TE 300s/AE
- **Status:** eigen (Rat)
- **Trigger:** Rat `R-2026-10-17-j`.
- **Lage:** pending (gemessen 2026-10-18): NVE→IVE +0.0067>+0.0022, IVE→NVE +0.0042>+0.0026.
- **Blockade:** K-Verdikt des Rats.
- **Braucht:** `state/stimmen/2026-10-17-rat-acht-faeden.md:1`

### Rätsel — arbeits-gebunden
- **Status:** eigen
- **Trigger:** Operator-Wort / Arbeit.
- **Lage:** offen (gemessen 2026-10-18 via open_points_check).
- **Blockade:** arbeits-geb.
- **Braucht:** `state/river/2026-10-18-pass-offene-punkte.φ:1`

### Flyby — Termin 2026-11-01
- **Status:** eigen (termin)
- **Trigger:** 2026-11-01.
- **Lage:** offen (gemessen 2026-10-18 via open_points_check).
- **Blockade:** Termin.
- **Braucht:** `state/river/2026-10-18-transfer-quittungen.φ:1`

## Offene Transfers (5)

- `.4~e6aa42be` (future) — Rat: Receiver-Apertur 3 · Schumann `field` · Flyby-`sealed`; done-when: future-Ledger/Quittung.
- `.5~488ad8b6` (future) — Operator: GIC-2 (ENTSO-E/NERC) · LOCK · Rätsel-Wort; done-when: future-Ledger/Quittung.
- `.12~08a6f6c0` (mountain) — KiDS-WL-Admission `phi/sources.φ:31369`: `cmap`/`ra`/`dec`/`z_key` + `field e1/e2 inverse-square gravity 1`; done-when: 2 Feldzeilen + Kanal-Test strahlt — oder `riss` (Entscheid `abffdaaaa`).
- `.13~ea59cd54` (mycelium) — Transport `kids_dr4_wl_cat.fits`: `sha256` + CDN-Matrix + Range-/Subset-Leser; done-when: gültige `sha256` + url HTTP 206 — oder `absent`.
- `.14~ca702ac7` (future) — Quittung für `.4`/`.5`: Empfänger-Ledger anlegen + echo/`CLOSED`; done-when: `xfer coverage --line river` → open 0.

## In diesem Pass geschlossen (belegt)

- AACGM-.src-Provenienz (`src/archivar/aacgm_coeffs-14-{1995,2000,2005}.asc.src`) — `sfetch`-Sidecars, `tools/utils/src/bin/sfetch.rs:78-90` — Commit `51f974a4`.
- Vlies-sha (`docs/auftrag/auftrag-universelles-vlies.md`) — `omega_sh sha` = `aee9e804…` — Commit `51f974a4`.
- Vlies-Träger im Handover (doc-carrier-Gate) — Commit `a6e682e72`.
