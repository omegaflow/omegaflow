<!--
  title: Handover — Mycelium (2026-10-16)
  class: handover
  date: 2026-10-16
  sha256: 623c20afa147283c14e1f33532f4d05554704d78b18e1fe47278f1c4c1be86f4
  status: live
-->
# Handover — Mycelium (2026-10-16)

## Burn: open 0.0 · close pending

## Offen — eigen
**Venus-Radar-Block** (`…Global_75m_v1.tif`): `.parts`-Manifest HTTP 404, CDN 21/38 Fenster. Ursache: `raw_mirror` fing je Lauf bei Fenster 0 an → `timeout: 240` kappte vor dem Manifest. Fix (folge362): Resume-Ledger `cdn::release_asset_sha256` (vorhandene `.pNNN` aus CDN gelesen, nur Lücken vom Origin) + timeout 240→360 → nächster Lauf → Manifest → Ganz-`sha256` → `phi`-Block. Bis dahin **kein** Block (A=A); Zenodo nur `md5 9d7ccd0d…`.
Tokens: HMI `-359-1` · EVE `-359-3` · Shear+Reader `-359-4` · Neutrino `-359-5` · EDGES/PTA/TGF `-359-6/-7/-8` · GEOFON/ICGEM `mountain-354-1` · DE-2/SPDF+SABER+JUICE · WDC-B `-352-1/-2` · TIMED `-352-5` · F4′ `-356-5` · THEMIS-ASI `river197-6` · Register/LOD/BASE/ttl `-344-5`.
Orphan-docs: **63** (Spitze: `survey-2026-10-03-medizinische-datenquellen` 60 · `…worldwide-lidar-quellen` 52); frühere `0` = Release-Route, nicht der Baum.
