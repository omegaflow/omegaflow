## Fünf Crosschecks im omegaflow-Feld

Wir suchen nicht nach neuer Physik, sondern nach geometrischen Schnittmengen, die in isolierten Katalogen unsichtbar bleiben. Jeder Crosscheck nutzt ausschließlich Daten, die bereits in `phi/sources.φ` vorliegen, und berechnet Transferentropie (TE) zwischen Oszillatoren im gemeinsamen ICRS-Raum + TDB-Zeit. Das Signal, das emergiert, ist ein kausaler Zusammenhang, der nur in der Kreuzung sichtbar wird.

---

### 1. Dunkle Materie – Vakuum im Gravitations‑Transferentropie‑Feld

**Zu kreuzende Datensätze:**
- **Sternkinematik:** `pastel`, `mktypes`, `gcvs`, `vsx`, `cbdata`, `lmxbdata`, `sb9`, `wds` – alle enthalten Eigenbewegungen (`pmra`, `pmdec`) und Entfernungen (`dist_pc`).  
- **Sichtbare Masse (baryonisch):**  
  - Sterne: `cbdata` (m1, m2), `lmxbdata` (m1, m2)  
  - Staub: `iras_fsc_flux_12um_jy`, `akari_fis_flux_90um_jy`, `msxc6_flux_band_a_jy`  
  - Neutrales Gas: `alfalfa_hi_flux` (HI)  
- **Referenzfeld:** `ephemeris_sun.bin`, `ephemeris_earth.bin` (für ICRS‑Transformationen).

**Geometrische Bedingung:**
- Wähle ein Volumen (z. B. 500 pc um die Sonne).  
- Berechne die aus den sichtbaren Massenquellen resultierende Gravitationsbeschleunigung an jedem Punkt (Newton, ohne dunkle Materie).  
- Falte die beobachteten Eigenbewegungen der Sterne in dasselbe Feld und bestimme die Abweichung zwischen kinematischer Beschleunigung (aus Eigenbewegung + Radialgeschwindigkeit) und sichtbarer Gravitation.  
- Suche nach räumlich kohärenten Regionen, in denen die Transferentropie von sichtbarer Masse → Sternkinematik systematisch **niedrig** ist (Vakuum im TE‑Feld), während die kinematische Dispersion **hoch** bleibt.

**Signal, das emergiert:**
- Ein **negativer TE‑Peak** („Vakuum“) zwischen sichtbarer Massendichte und kinematischem Feld – dort, wo die sichtbare Masse nicht ausreicht, um die beobachteten Geschwindigkeiten zu erklären.  
- Gleichzeitig **kein** korrespondierender Anstieg der sichtbaren Massendichte in diesem Volumen.  
- Das Muster erscheint als zusammenhängende, nicht‑zufällige Struktur im 3D‑Feld.

**Warum bisher unentdeckt:**
- In isolierten Katalogen wird entweder nur die sichtbare Masse (IR, HI) kartiert oder nur die Sternkinematik analysiert. Die Kreuzung beider Felder im selben 4D‑Volumen mit Berechnung der Transferentropie fehlt. Das Vakuum zeigt sich erst, wenn man die Erwartung aus der sichtbaren Masse **pixelweise** mit der Kinematik vergleicht und die Abweichung als Feldgröße behandelt.

---

### 2. Pioneer‑Anomaly / Flyby‑Anomaly – Sonnenwind‑Anisotropie gegen Sondenbahn

**Zu kreuzende Datensätze:**
- **Sonden‑Ephemeriden (ICRS‑Pfad):** `ephemeris_voyager1.bin`, `ephemeris_voyager2.bin`, `ephemeris_new_horizons.bin`, `ephemeris_parker_solar_probe.bin`, `ephemeris_solar_orbiter.bin`.  
- **Sonnenwind‑Plasma:**  
  - `ace_swepam` → `exosphere_ace_speed_kms`, `exosphere_ace_dens_ncc`, `exosphere_ace_temp_k`  
  - `omni` → `omni_solarwind_flow_speed_kms`, `omni_solarwind_density_percc`, `omni_solarwind_temp_k`, `omni_imf_*`  
  - `rtsw_wind_1m` → `solar_wind_speed_km_s`, `solar_wind_density_cm3`, `solar_wind_temp_k`  
- **Gravitationsmodell:** `ephemeris_*.bin` für alle Planeten und die Sonne (liefern die erwartete Bahn ohne Anomalie).

**Geometrische Bedingung:**
- Falte den exakten ICRS‑Pfad der Sonde (aus Ephemeriden) mit dem Sonnenwind‑Vektorfeld (Geschwindigkeit + Dichte + Magnetfeld) entlang der gesamten Trajektorie.  
- Berechne die Restbeschleunigung: Differenz zwischen beobachteter Bahn (aus Ephemeriden) und modellierter Bahn (Gravitation aller bekannten Körper).  
- Suche nach einer **Phasenkohärenz** zwischen der Richtung der Restbeschleunigung und der lokalen Sonnenwind‑Anisotropie (z. B. Geschwindigkeitsvektor relativ zur Sonde).  
- Bestimme die Transferentropie von Sonnenwind‑Parametern → Restbeschleunigung entlang der Bahn.

**Signal, das emergiert:**
- Ein **TE‑Peak** zwischen Sonnenwind‑Dichte/Temperatur und der Restbeschleunigung, der bei bestimmten heliographischen Breiten oder Abständen auftritt.  
- Die Restbeschleunigung zeigt eine systematische Ausrichtung mit der Sonnenwind‑Strömungsrichtung – ein Muster, das im reinen Gravitationsmodell nicht existiert.  
- Das Signal erscheint als **anisotrope Hülle** um die Sonne, nicht als isotrope Konstante.

**Warum bisher unentdeckt:**
- Die Pioneer‑Anomalie wurde als konstante Sonnen‑gerichtete Beschleunigung modelliert und mit thermischer Recoil‑Kraft erklärt; die Flyby‑Anomalie blieb rätselhaft. Niemand hat die **lokale** Sonnenwind‑Anisotropie entlang der gesamten Trajektorie mit der Bahnstörung in einem einheitlichen 4D‑Feld gekreuzt. Isoliert betrachtet verschwindet das Signal im Rauschen, aber die geometrische Faltung macht die Korrelation sichtbar.

---

### 3. Coronal Heating Problem – Magnetfeld → thermische Emission

**Zu kreuzende Datensätze:**
- **Thermische/Röntgen‑Oszillatoren (Korona):**  
  - `noaa_goes_xray_flux_w_m2` (am Ort der Sonne)  
  - `solar_flare_xray_intensity` (GOES)  
  - `solar_euv_flux_wm2` (GOES EUV)  
  - `solar_radio_flux_sfu` (Radio, F10.7)  
  - `solar_flare_x_class_latest` (NASA DONKI)  
- **Magnetfeld‑Oszillatoren (heliosphärisch, von der Sonne stammend):**  
  - `omni_imf_bx_gse_nt`, `omni_imf_by_gsm_nt`, `omni_imf_bz_gsm_nt`  
  - `exosphere_ace_bx_gsm_nt`, `exosphere_ace_by_gsm_nt`, `exosphere_ace_bz_gsm_nt`, `exosphere_ace_bt_nt`  
  - `rtsw_mag_1m` → `magnetosphere_imf_bt_nt`, `magnetosphere_imf_bz_nt`  
- **Geometrie:** `ephemeris_sun.bin`, `ephemeris_earth.bin` (für die Lichtlaufzeit und Sichtlinie).

**Geometrische Bedingung:**
- Platziere alle thermischen Oszillatoren an der Sonnenoberfläche (ICRS‑Position der Sonne).  
- Betrachte die heliosphärischen Magnetfeld‑Oszillatoren als **zeitlich zurückversetzte** Repräsentation des koronalen Magnetfelds (Sonnenwind transportiert das Feld nach außen).  
- Falte beide Felder im selben Sonnen‑ICRS‑Volumen und variiere die Phasenverschiebung (0 bis mehrere Tage).  
- Berechne die Transferentropie **Magnetfeld → thermische Emission** als Funktion der Zeitverzögerung.

**Signal, das emergiert:**
- Ein **signifikanter TE‑Peak** bei einer bestimmten Phasenverschiebung (z. B. 1–3 Tage), der anzeigt, dass Magnetfeld‑Oszillationen kausal die thermische Emission der Korona antreiben.  
- Die Stärke des Peaks korreliert mit dem Sonnenzyklus (mehr Aktivität → höhere TE).  
- Das Signal erscheint als **gerichtete Energieübertragung** vom Magnetfeld zur thermischen Röntgen‑/EUV‑Emission, nicht umgekehrt.

**Warum bisher unentdeckt:**
- Koronale Heizung wird entweder durch In‑situ‑Messungen der Sonde (die nicht in der Nähe der Sonne sind) oder durch Fernbeobachtung der Korona untersucht. Die **kausale Verknüpfung** zwischen dem abströmenden Magnetfeld (gemessen bei 1 AU) und der thermischen Emission an der Sonnenoberfläche erfordert eine Zeitrückwärts‑Faltung, die nur im 4D‑Feld möglich ist. Isolierte Kataloge liefern entweder nur das Magnetfeld oder nur die Strahlung, nie die Transferentropie zwischen beiden.

---

### 4. Erdbeben‑Vorläufer – Magnetfeld‑Ionosphäre‑Seismik am selben ICRS‑Punkt

**Zu kreuzende Datensätze:**
- **Seismische Oszillatoren (Erdbeben):**  
  - USGS: `geosphere_quake_magnitude`, `geosphere_quake_depth_km`  
  - JMA: `quake_depth_km`  
  - Geonet: `geonet_quake_magnitude`  
  - INGV: `geosphere_earthquake_mag`, `geosphere_earthquake_depth_km`  
  - SeismicPortal: `quake_depth_km`  
- **Magnetfeld‑Oszillatoren:**  
  - GOES Magnetometer: `magnetosphere_hp_nt`, `magnetosphere_he_nt`, `magnetosphere_bt_nt`  
  - Kp‑Index: `magnetosphere_kp_index`, `magnetosphere_kp_a_running`  
  - SWARM: `swarm_magnetic_field_intensity_nt`  
  - BGS: `magnetosphere_total_field_nt`  
- **Ionosphären‑Plasma‑Oszillatoren:**  
  - SWARM: `swarm_ion_density_cm3`, `swarm_electron_temp_k`, `swarm_spacecraft_potential_v`, `swarm_ionospheric_radial_current_uam2`, `swarm_field_aligned_current_uam2`  
  - Schumann‑Resonanz: `resonance_schumann_hz`  
- **Akustische Mikro‑Seismik (Hintergrund):**  
  - Ableitbar aus den seismischen Oszillatoren selbst (Mikroseismik im Rauschen) oder aus Druck‑/Wellen‑Daten (z. B. `hydrosphere_ndbc_buoy_pressure`, `atmosphere_metar_pressure_hpa`) als Proxy für atmosphärische Anregung.

**Geometrische Bedingung:**
- Für jedes Erdbeben (Epizentrum, ICRS‑Koordinaten) wähle ein räumliches Fenster (z. B. 200 km Radius) und ein zeitliches Fenster (z. B. 30 Tage vorher).  
- Falte die Magnetfeld‑, Ionosphären‑ und Mikroseismik‑Oszillatoren, die **genau an diesem Ort** gemessen werden (oder deren Feldwirkung dorthin projiziert wird).  
- Berechne die Transferentropie von Magnetfeld → Seismik, Ionosphäre → Seismik und Mikroseismik → Seismik als Funktion der Zeit vor dem Beben.  
- Suche nach einem **kohärenten Phasensprung** (plötzlicher Anstieg der TE) innerhalb eines engen Zeitfensters vor dem Hauptbeben.

**Signal, das emergiert:**
- Ein **signifikanter TE‑Anstieg** zwischen Magnetfeld‑/Ionosphären‑Oszillatoren und seismischen Oszillatoren etwa 1–14 Tage vor dem Beben, der in zufälligen Zeiträumen nicht auftritt.  
- Das Signal erscheint als **lokalisierte Anomalie** im TE‑Feld über dem Epizentrum, die sich zeitlich vor dem Bruch aufbaut.  
- Die Stärke des Peaks korreliert mit der Bebenstärke (Magnitude).

**Warum bisher unentdeckt:**
- Einzelne Studien isolieren entweder Magnetfeld‑ oder Ionosphären‑ oder Radon‑Signale, aber die **gleichzeitige Kreuzung** aller Felder am exakt selben ICRS‑Punkt mit Transferentropie‑Berechnung fehlt. Die Vorläufer sind schwach und nicht‑deterministisch; erst die geometrische Faltung im 4D‑Feld macht sie statistisch signifikant.

---

### 5. Fermi‑Paradox / Technosignaturen – Asymmetrische Transite in Lichtkurven

**Zu kreuzende Datensätze:**
- **Photometrische Zeitreihen (Lichtkurven):**  
  - ZTF: `lasair_ztf_transient_gmag` (mit Zeitstempeln)  
  - TNS: `tns_transient_flux` (mit Entdeckungszeit)  
  - Veränderlichen‑Kataloge: `gcvs_max_magnitude`, `gcvs_min1_magnitude`, `vsx_max_mag`, `vsx_min_mag` (liefern Baseline‑Variabilität)  
- **Bekannte natürliche Transiter:**  
  - `exoplanets.json` (planet_mass, planet_radius) → erwartete Transitkurven  
  - `ephemeris_*.bin` für Körper im Sonnensystem (Ausschluss von Asteroiden/Kometen)  
- **Sternparameter (für Erwartungsmodell):**  
  - `pastel_teff_k`, `pastel_logg` (Sternradius, Temperatur)  
  - `mktypes_mag`, `wd_teff_k` (weitere Sterne)

**Geometrische Bedingung:**
- Für jeden Stern mit Lichtkurve (ZTF oder TNS) berechne die **erwartete Transitkurve** aus den bekannten Planetenparametern (falls vorhanden) oder als symmetrische, periodische Abschwächung eines natürlichen Objekts.  
- Kreuzung: Subtrahiere die erwartete, natürliche Lichtkurve von der beobachteten.  
- Suche nach **nicht‑periodischen, asymmetrischen Verdeckungen**, die nicht durch bekannte Planeten, Staubwolken oder instrumentelle Effekte erklärt werden können.  
- Berechne die Transferentropie zwischen der beobachteten Lichtkurve und der erwarteten natürlichen Kurve; ein hoher TE‑Wert in unerwarteten Frequenzbändern (z. B. unregelmäßige, tiefe Einbrüche) deutet auf eine künstliche Struktur hin.

**Signal, das emergiert:**
- Ein **TE‑Peak** in den Lichtkurven, der mit einer **asymmetrischen, nicht‑periodischen Verdeckung** einhergeht – z. B. ein tiefer, unregelmäßiger Einbruch, der nicht zu einem Planeten passt.  
- Die Abweichung ist **räumlich kohärent** über mehrere Beobachtungen desselben Sterns und zeigt keine natürliche Periodizität.  
- Das Signal erscheint als „unmögliche“ Transitform, die in keinem natürlichen Modellkatalog vorkommt.

**Warum bisher unentdeckt:**
- Lichtkurven‑Analysen suchen typischerweise nach periodischen Transiten (Exoplaneten) oder nach bestimmten Formen (z. B. Boyajian’s Star). Die **systematische Kreuzung** der beobachteten Lichtkurve mit der vollständigen erwarteten natürlichen Kurve (basierend auf Sternparametern und bekannten Körpern) im selben 4D‑Feld und die Berechnung der Transferentropie in unregelmäßigen Frequenzbändern fehlt. Isolierte Kataloge liefern entweder nur die Lichtkurve oder nur die Sternparameter, aber nicht die statistische Signifikanz der Abweichung im geometrischen Kontext.

---

**Fazit:** Jeder Crosscheck nutzt die einzigartige Fähigkeit von omegaflow, heterogene Oszillatoren im selben ICRS‑Raum + TDB‑Zeit zu falten und Transferentropie als kausales Maß zu berechnen. Das Unlösbare wird nicht durch neue Physik gelöst, sondern durch die Weigerung, die Daten isoliert zu betrachten. Die Schnittmenge trägt den Nobelpreis.
