<!--
  title: Handover — Ernte-Folge IX (Stand 2026-09-12)
  session: Ernte-Folge IX
  class: handover
  date: 2026-09-12
  sha256: 4cf9314b767eb569d9397a4df5e0d4939612959c90d4e5df8dace2046294e2c1
  status: live
-->
# Handover — Ernte-Folge IX (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty) — Push-gated

- igets.bin — Wächter rot. Fix gebaut (Station-Shards + k-way merge `igets_merge` +
  gestreamtes `--list-stations`; Workflow gate→stations→compile→merge), committet
  lokal. Push wartet (Bau-Session bau12 aktiv, Baum nicht ruhig). Nach Push: dispatch
  `igets-cdn`; Wächter igets.bin auf release igetsftp.gfz.de (HTTP 200).
- eso-harps — Dispatch nach Push; Wächter harps_rvcat.json auf ssd.jpl.nasa.gov
  (HTTP 200).

## Ernte

- Hi-net — HINET_PASS weiter absent (.secrets.local gemessen) — Operator.

## Abschluss

- Nur eigene Hunks committet (lokal); fremde uncommittete Arbeit (bau12: src/archivar,
  src/mathematikerin/omega+tests, static/, Handover bau11→bau12) blieb unberührt.
  Push wartet, bis der Baum ruhig ist.
