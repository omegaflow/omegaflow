<!--
  title: Rat-Formular — MODVOLC-Verteilung (Verteilungs-Ethik)
  class: concept
  date: 2026-10-17
  sha256: 9d7316f4fd57ec690adb285a705b37567557328721c3d8942a92cbae1a8a35f6
  status: live
-->
# Rat-Formular — MODVOLC-Verteilung (Verteilungs-Ethik), 2026-10-17

**Anlass (Operator-Wort):** „das ist eine **Ratsfrage**." Mountain ist dasselbe Modell mit demselben
Wissen und trägt zur Verteilungsfrage nichts Eigenes bei — die Frage gehört an den **Rat**.

**Gemessen (Belege):**
- **LANCE-Route** `nrt3.modaps.eosdis.nasa.gov/archive/allData/61/MODVOLC` — im Register `terms PD`
  (`phi/sources.φ:12134`), belegt über die NASA-Datapolitik
  `earthdata.nasa.gov/engage/open-data-services-software-policies/data-information-guidance`
  („full and open sharing … all data … distributed … by NASA data systems"); Messung **mountain-340**
  (2026-10-13); Route gesetzt von **mycelium-343**.
- **HIGP-Route** `modis.higp.hawaii.edu` — **keine** offene Lizenz (© UH Mānoa, Data-Use-Agreement) →
  `local-only`; Host zudem tot (Timeout, DNS löst auf `128.171.157.89`).
- **Neu, widersprechend:** LAADS Support — „this product is **not approved for public distribution**."
- Das kompilierte `modvolc_thermal.bin` (**4 306 035 B**) liegt bereits auf dem **öffentlichen** CDN
  (`…/nrt3.modaps.eosdis.nasa.gov/modvolc_thermal.bin`, HTTP 200).
- Gate-Lücke: `license_census` prüft die **Existenz** einer `terms`-Zeile, nicht ihre **Wahrheit**.

**Fragen an den Rat:**
1. Deckt `terms PD` die **LANCE**-Route — oder ist MODVOLC ein `decline-redistribution`-Fall?
2. Trägt die allgemeine **NASA-Politik** oder die **produktspezifische LAADS-Absage** (LANCE vs LAADS:
   derselbe Produzent, zwei Kanäle)?
3. Soll das veröffentlichte CDN-Asset **zurückgezogen** werden (Operator-Hand) — und wird bis zum
   Entscheid **nicht weiter** veröffentlicht?
4. Verdient die Gate-Lücke (Existenz vs. **Wahrheit** einer `terms`-Zeile) eine **Wand**?

**Speist:** die gemessene Recherche oben + der Roster `~/projects/omegaflow-work/ui-seats.md`.
**Stand der Absendung (2026-10-17):** `bin/rat_dispatch` an 9 Sitze gestellt — **alle `gateway unread`**
(`http://127.0.0.1:1621` nicht erreichbar). Die Frage ist **gestellt**, aber **unbeantwortet**; kein
stilles `pending`.
