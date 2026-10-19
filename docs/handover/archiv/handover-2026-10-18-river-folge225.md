<!--
  title: Handover — River-Folge 225 (2026-10-18)
  class: handover
  date: 2026-10-18
  sha256: 4b51f5865c789ea1469d496d977b4252d13a20761015ab6b7fc3cccfb32e04e2
  status: live
-->
# Handover — River-Folge 225 (2026-10-18)

## Burn: open 3 · close pending · closed 0 · Grund: db.

## Träger
- `docs/auftrag/auftrag-universelles-vlies.md` — Richtung; offen: `ozzy`, Paar-Matrix, Bias-Wiring (`bias_column` estimator-fest).

## Offen — eigen

### P7.1 K — kond. TE 300s/AE
- **Status:** eigen (Rat)
- **Trigger:** Rat R-2026-10-17-j.
- **Lage:** pending (gemessen 2026-10-18): NVE→IVE +0.0067>+0.0022, IVE→NVE +0.0042>+0.0026.
- **Blockade:** K-Verdikt des Rats.
- **Braucht:** `state/stimmen/2026-10-17-rat-acht-faeden.md:1`

### Rätsel — arbeits-gebunden
- **Status:** eigen
- **Trigger:** Operator-Wort / Arbeit.
- **Lage:** offen (gemessen 2026-10-18 via open_points_check).
- **Blockade:** arbeits-geb.
- **Braucht:** `docs/zustand/2026-10-18-pass-offene-punkte.md`

### Flyby — Termin 2026-11-01
- **Status:** eigen (termin)
- **Trigger:** 2026-11-01.
- **Lage:** offen (gemessen 2026-10-18 via open_points_check).
- **Blockade:** Termin.
- **Braucht:** `docs/zustand/2026-10-18-transfer-quittungen.md`

## An mountain
Origin: river-224
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.1~d03aa55a` P10 · UCAR-FPI-Zeile · Newell 1m/1h.
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.12~08a6f6c0` Kanal-Admission `phi/sources.φ:31369` KiDS DR4.1 WL: `cmap .` · `ra RAJ2000` · `dec DECJ2000` · `z_key Z_B` · `field e1 kids_dr4_wl_e1 inverse-square gravity 1 31536000 0.0 0.0` · `field e2 kids_dr4_wl_e2 inverse-square gravity 1 31536000 0.0 0.0`; done-when: 2 field-Zeilen im Block **und** Kanal-Test (`tests.rs:2563`) strahlt einen Sample — oder `riss`-Verdikt (River-Entscheid `abffdaaaa`).

## An mycelium
Origin: river-224
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.2~aa2b1a08` Weberin-Manifest.
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.3~857b157e` FODG erledigt.
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.13~ea59cd54` Transport `kids_dr4_wl_cat.fits`: `sha256` + CDN-Matrix + Range-/Subset-Leser (16,5 GiB, `format fits`/`tar_zst`); done-when: gültige `sha256` im Block **und** registrierte CDN-`url` HTTP 206 (heute 404) — oder `absent`-Verdikt.

## An future
Origin: river-224
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.4~e6aa42be` Rat: Receiver-Apertur 3 · Schumann `field` · Flyby-`sealed`.
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.5~488ad8b6` Operator: GIC-2 (ENTSO-E/NERC) · LOCK · Rätsel-Wort.
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.10~2e71595e` Beobachtung (Architektur, bewusst nicht gebaut): der Empfänger kann seinen eigenen Eingang nicht quittieren — `xfer close` auf einen fremden Token ist ein harter Fehler, sonst schreibt der Empfänger ins Emitter-Ledger (river → mycelium `.2~e192689e`); Quittung und Selbst-Erledigung sind beide `CLOSED`.
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.11~04ec4cc1` Beobachtung: `register_lookup --addressed <linie>` listet geschlossene Blöcke weiter, `--fired` bleibt 0 — Erledigtes liest sich als offen (mountain `.2~1776ba69`, mycelium `.2~e192689e` standen nach dem Close weiter im Eingang).
- `xfer:river.ses_ec618129fffeLFawr4V3pQKpOl.14~ca702ac7` Quittung für rivers `.4~e6aa42be` / `.5~488ad8b6`: Empfänger-Ledger anlegen + echo/`CLOSED` — oder Operator-Verdikt; done-when: `xfer coverage --line river` → **open 0** (heute 2).
