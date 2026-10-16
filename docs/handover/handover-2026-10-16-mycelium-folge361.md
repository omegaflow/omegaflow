<!--
  title: Handover — Mycelium (2026-10-16)
  class: handover
  date: 2026-10-16
  sha256: df693355009a38c75b59be07a6865bb05f584056acf255a2d8cd3ad2245c067d
  status: live
-->
# Handover — Mycelium (2026-10-16)

## Burn: open 0.0 · close pending

## Offen — eigen
**Venus-Radar-Block (Riss im Weg geschlossen, in der Zahl offen):** `.parts`-Manifest = **HTTP 404**; CDN trägt **21/38** Fenster (`…Global_75m_v1.tif.p000…p020`, je ~2 GiB). Ursache gemessen: der `raw_mirror`-Fensterpfad fing bei **jedem** Lauf bei 0 an → `timeout-minutes: 240` kappte vor dem Manifest. **Fix (folge362):** Resume-Ledger `cdn::release_asset_sha256` — vorhandene `.pNNN` werden aus dem CDN zurückgelesen (schnell) und in den Ganz-Hash gefaltet, nur die Lücken kommen vom Origin; `timeout-minutes` 240→360. Nächster Lauf: 21 CDN-Fenster + 17 Origin-Fenster (~16 min/2 GiB) ≈ 4,7 h < 360 → Manifest → Ganz-`sha256` → `phi`-Block. Bis dahin **kein** Block (A=A). Zenodo trägt nur `md5 9d7ccd0d…`.
HMI `-359-1` · EVE `-359-3` · Shear+Reader `-359-4` · Neutrino `-359-5` · EDGES/PTA/TGF `-359-6/-7/-8` · GEOFON/ICGEM `mountain-354-1` · DE-2/SPDF+SABER+JUICE · WDC-B `-352-1/-2` · TIMED `-352-5` · F4′ `-356-5` · THEMIS-ASI `river197-6` · Register/Ersatz/LOD/BASE/WDC/ttl `-344-5`.
Orphan-docs: **0**.
