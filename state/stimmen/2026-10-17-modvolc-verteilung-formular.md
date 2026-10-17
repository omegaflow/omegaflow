<!--
  title: Rat-Formular — MODVOLC-Verteilung (Verteilungs-Ethik)
  class: concept
  date: 2026-10-17
  sha256: 34a5c16bf0109284a89aeefe381264e7162b6b1c3401c47982f85648dd4f7fe1
  status: live
-->
# Rat-Formular — MODVOLC-Verteilung (Verteilungs-Ethik), 2026-10-17

**Anlass (Operator-Wort):** „das ist eine **Ratsfrage**." Mountain ist dasselbe Modell mit demselben
Wissen und trägt zur Verteilungsfrage nichts Eigenes bei — die Frage gehört an den **Rat**.

**Gemessen (Belege):**
- **LANCE-Route** `nrt3.modaps.eosdis.nasa.gov/archive/allData/61/MODVOLC` — damals im Register `terms PD`
  (Block später gefallen, s. Nachtrag 2), belegt über die NASA-Datapolitik
  `earthdata.nasa.gov/engage/open-data-services-software-policies/data-information-guidance`
  („full and open sharing … all data … distributed … by NASA data systems"); Messung **mountain-340**
  (2026-10-13); Route gesetzt von **mycelium-343**.
- **HIGP-Route** `modis.higp.hawaii.edu` — **keine** offene Lizenz (© UH Mānoa, Data-Use-Agreement) →
  `local-only`; Host zudem tot (Timeout, DNS löst auf `128.171.157.89`).
- **Neu, widersprechend:** LAADS Support — „this product is **not approved for public distribution**."
- Das kompilierte `modvolc_thermal.bin` (**4 306 035 B**) lag auf dem **öffentlichen** CDN — inzwischen
  **zurückgezogen** (Asset **HTTP 404**, gemessen 2026-10-17).
- Gate-Lücke: `license_census` prüft die **Existenz** einer `terms`-Zeile, nicht ihre **Wahrheit**.

**Fragen an den Rat:**
1. Deckt `terms PD` die **LANCE**-Route — oder ist MODVOLC ein `decline-redistribution`-Fall?
2. Trägt die allgemeine **NASA-Politik** oder die **produktspezifische LAADS-Absage** (LANCE vs LAADS:
   derselbe Produzent, zwei Kanäle)?
3. Soll das veröffentlichte CDN-Asset **zurückgezogen** werden (Operator-Hand) — und wird bis zum
   Entscheid **nicht weiter** veröffentlicht?
4. Verdient die Gate-Lücke (Existenz vs. **Wahrheit** einer `terms`-Zeile) eine **Wand**?

**Nachtrag (2026-10-17) — entschärft durch die Rohdaten-Route.** Verifiziert: LANCE `allData/61` trägt
MODVOLC, **LAADS** ist das **L1/Atmosphere**-Archiv (`MOD021KM`/`MYD021KM`, 200); die Support-Mail ist
kein Lizenzinstrument; `terms PD` ist der **falsche Wert** (NASA = open/Attribution/kein Endorsement).
Der MODVOLC-Algorithmus ist **publiziert** (Wright et al., JVGR `10.1016/j.jvolgeores.2003.12.008`:
4 µm Band 21/22 + 12 µm Band 32) → das Haus kann sein **eigenes** Thermalanomalie-Feld aus der **offenen
L1** rechnen und MODVOLC nur zum **Abgleich** nutzen. **Damit bleibt die Ratsfrage nur für den einen
Fall: geben wir das MODVOLC-Derivat selbst weiter?** Für unser L1-Derivat genügt Mountains `terms`-Verdikt
(`free-open` + Attribution).

**Rat-Antwort (M1), `4f50553f5`:** **4:2 gespalten** — API-Sitze 4× `decline` (PD deckt LANCE nicht,
Asset zurückziehen); **UI-Sitze (Fugu Max, Lumo) überstimmen**: `PD` ist der falsche *Wert*, aber
`decline` der falsche *Schluss* — die **LANCE-Route ist offen**; Terme hängen an (Produkt, Kanal).
Das Haus liest es als UI-Überstimmen (`state/stimmen/2026-10-17-rat-modvolc-dropped-gate.md`).

**Operator-Wort (2026-10-17) — entschieden, konservativ:** „wir nehmens runter im Zweifel lieber nicht …
wir rechnen eh selber." → **MODVOLC-CDN-Asset zurückziehen** (Mycelium), Block `local-only` (kein
`terms PD`), lokal zum **Abgleich**; das **Feld** trägt der **L1-Block `MOD021KM`** (NASA-open +
Attribution). Der Spalt ist damit **durch das Operator-Wort** entschieden — kein neuer Rat nötig.

**Nachtrag 2 (gemessen, Sensory-Pass 2026-10-17):** Vollzogen: der Register-Block ist **gefallen**
(`phi/sources.φ` trägt **keine** MODVOLC-Zeile; Mycelium `796ae91c3`, „no fabricated PD line") und das
CDN-Asset ist **HTTP 404** (zurückgezogen). MODVOLC bleibt `local-only`; der Träger ist der **L1-Block
`MOD021KM`/`MYD021KM`** bei Mountain (`xfer:mycelium-361-7`; terms/field/ttl + Band-21/22/32-Reader).
Die vier API-Sitze (`decline-redistribution`) und die zwei UI-Sitze (`LANCE offen`) stehen **benannt**,
nicht geglättet; die `license_census`-Lücke (Existenz ≠ Wahrheit) bleibt **Rat-getragene Wand**.

**Speist:** die gemessene Recherche oben + der Roster `~/projects/omegaflow-work/ui-seats.md`.
