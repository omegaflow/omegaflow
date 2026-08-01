## Plan: 8-Kräfte-Punktwolken-Verdichtung

### Priorität 1 — Gravity (sofort, null Aufwand)

**Was:** 63 Gaia-Quellen in `sources.φ` haben `force em`. Ändern auf `force em gravity`.  
**Effekt:** 1.8 Mrd. Oszillatoren bekommen Gravitation. Die Gravity-Punktwolke springt von ~78 Körpern auf das komplette Sternenfeld.  
**Code:** Keiner. Nur `sources.φ`.

### Priorität 2 — Thermal (mittel)

**Was:** CMEMS alle 186+ Thermal-Produkte (SST, Heat Content, Mixed Layer). AERONET alle 500 Stationen mit vollem AOD-Profil. FIRMS auf globale Abdeckung.  
**Effekt:** Thermal wird von ~50 auf >100k Oszillatoren verdichtet.

### Priorität 3 — Advective (mittel)

**Was:** Pegelonline von 2 auf alle 10.938 Stationen. CMEMS Strömungsprodukte (Oberflächenströmung, Drift).  
**Effekt:** Advective bekommt erstmals eine sichtbare Punktwolke.

### Priorität 4 — Diffusion via GBIF (hoch)

**Was:** GBIF Occurrence API — 2.2 Mrd. biologische Fundpunkte. Batch-Download → räumliches Sharding → Spatial Cache.  
**Effekt:** Diffusion wird zur dichtesten Punktwolke aller Kräfte.

### Priorität 5 — Seismic-Surface (mittel)

**Was:** RaspberryShake (2k+ Bürgerseismometer). Smithsonian GVP (1.500 Vulkane).  
**Effekt:** Seismic-Surface von ~20 auf mehrere tausend Oszillatoren.

### Priorität 6 — Acoustic (gering)

**Was:** NDBC von 36 auf alle 1.351 Bojen.  
**Effekt:** Acoustic verdichtet sich auf den Ozeanen.

### Priorität 7 — Seismic-Body (mittel)

**Was:** IRIS DMC + ORFEUS Stationen.  
**Effekt:** Seismic-Body bekommt globale Abdeckung.

### Priorität 8 — EM TOP-Cap (mittel)

**Was:** Gaia TOP 500 → Vollabfrage oder Paginierung.  
**Effekt:** EM von ~31k auf potenziell Milliarden (sukzessive).

---
