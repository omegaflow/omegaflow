<!--
  title: BLATT — GIC-Breitenband-Familien: Vorregistrierung der Kohärenz-Partition
  class: sheet
  date: 2026-10-06
  sha256: 9f598e5dfae4539c853b6b083ff8449e6bfab7e2d90f280e5c0fb098bb5b04b2
  status: unsealed
  see-also: docs/paper/gic-causal-driver.md docs/blatt/fruehwarnsystem-praeregistrierung.md state/future/gic-riss-154-wunschliste-2026-10-06.md
-->

# BLATT — GIC-Breitenband-Familien: die Vorregistrierung

**Datum:** 2026-10-06 · **Axiom:** A = A · **Status:** unsealed

Vor dem Lauf fixiert. Ein Wert, der nicht gemessen wird, bleibt `pending` — nie 0.0.
Was nach dem Lauf eintritt, ist im Fehlschlag-Kriterium vorab gebunden; die
Familien, die Stufen und die Berichtsform sind hier gesetzt und werden nach der
Messung nicht verhandelt.

## Der Riss (gemessen, nicht vermutet)

Der gemessene Stand ist `docs/paper/gic-causal-driver.md:17` (Abstract):

- Bz **führt** den yearly-Rund-Zeugen (Abisko 2024/2025, Sodankylä 2024), **cleart
  aber die gehärtete Quartalsschranke nicht** (n_surr = 100 hält alle Zeilen
  family bound).
- **density cleart nie** die Schranke; das **daily**-Korn (32 Jahre) bleibt leer.
- Kein familien-durchlassender Stundentreiber ist etabliert; Bz bleibt der führende
  sub-daily-Kandidat. Der Estimator-Ground-Truth ist NOT PASS; der Riss steht.

Rohes 154 (alle gegen alle) über **eine globale** Familie **hebt** die Hürde: die
effektive Testzahl `M_eff` kollabiert gegen die Zahl der Kohärenz-Cluster, der
Gewinn ist ~`√k` gegen `√(2 ln M_eff)` (Rat + ChatGPT · Qwen · Mistral · Claude ·
GLM-5.3, `state/future/gic-riss-154-wunschliste-2026-10-06.md:12-21`). Der Riss
schrumpft nur über **Struktur** — vorregistrierte Kohärenz-Familien — nicht über
einen größeren globalen Test.

## Die fixierte Spezifikation

### 1. Familien (die `M_eff`-Partition)

Drei geomagnetische **Breitenband-Familien**, an der korrigierten geomagnetischen
Breite der Station gemessen (nicht an geografischer Breite, nicht an einem
gewählten Station-Set):

- **auroral** — die aurorale Antwort als *eine* gerichtete Familie.
- **sub-auroral**
- **mid-latitude**

Die Partitionsgrenzen werden **vor** dem Lauf als feste Breitenwert-Grenzen
deklariert (Operator-Wort 2026-10-07: River entscheidet sie); jede Station fällt nach
ihrer deklarierten korrigierten geomagnetischen Breite (CGM) in genau eine Familie.
Keine Nachjustierung nach der Messung. Deklariert (|CGM-Breite|, beide Hemisphären
symmetrisch):

| Familie | \|CGM-Breite\| |
|---|---|
| auroral | ≥ 60° |
| sub-auroral | 50° ≤ · < 60° |
| mid-latitude | < 50° |

Konvention: die in GIC-Arbeiten gebräuchliche Dreiteilung (SpaceWeatherLive-Tripel;
gestützt auf die 50–60°-„danger zone", Nature Sci. Rep. 2022 `s41598-022-25704-2`,
und Tozzi et al., Ann. Geophys. ~50–55°). Kein offizieller Einzelstandard — die Werte
sind hier vor dem Lauf fixiert und danach nicht verhandelbar.

### 2. Kohärenz-Hypothese

Die aurorale Antwort ist **eine** gerichtete Familie (kohärent innerhalb des
Bandes, nicht über die Bänder hinweg). Die Familien sind die Einheit, in der die
Kontrolle greift — nicht die einzelne Station, nicht das globale Netz.

### 3. Zweistufige Kontrolle

- **Stufe 1 — Entdeckung:** BH/BY-FDR (`fdr bh|by <q> over matrix`) über das
  **ganze** Kanalnetz (`matrix full`) findet die Kandidaten-Kanten.
- **Stufe 2 — Bestätigung:** `WY-max-t` **innerhalb der fixierten Familien**
  (studentisiertes Westfall–Young-Maximum), streng, mit kalibrierter Null je
  Familie. Die globale Schranke ersetzt die Familien-Schranke **nicht**; die
  Stufe-2-Ebene α und die Resample-Blocklänge werden vor dem Lauf benannt.

### 4. Kalibrierung (Vorbedingung, nicht Nachbearbeitung)

Harmonisierte Vorverarbeitung und **gleiche Bandbreite** über alle Stationen:
Zwischen-Station-Dispersion geht sonst in die Null und **inflationiert** die
Schranke. Die Kalibrierung (Detrend-Fenster, Bandbegrenzung, Binnung, fehlende
Intervalle) wird **vor** dem Lauf als ein Satz fixiert und auf jede Station
identisch angewandt; eine station-spezifische Abweichung ist ein benannter Riss,
kein stiller Filter.

### 5. Bericht (vorab spezifiziert)

Berichtet wird der **vorab spezifizierte Anteil clearender Quartale, je Familie** —
**nie gemittelt**, nie über die Bänder gepoolt. Die drei Familien-Anteile stehen
getrennt; eine leere Familie ist ein voller Befund (Stille trägt die Antwort), nie
eine 0, die geglättet wird.

## Das Fehlschlag-Kriterium (vorab gebunden)

Eine Familien-Hypothese ist **widerlegt**, wenn der gemessene Anteil der clearenden
Quartale in keiner Familie die vor der Messung benannte Erwartung erreicht. Der
Befund ist auch bei vollständiger Stille vollständig; ein Teiltreffer (eine Familie
trägt, andere nicht) wird als solcher benannt, nie zu einem Treffer geglättet.
Nach der Messung ist keine Grenze, keine α-Ebene und kein Band mehr verhandelbar.

## Was vor dem Lauf fehlt (gemessen 2026-10-06)

Der Lauf ist **nicht** startbereit; die Lücke ist gemessen, nicht vermutet:

1. **Die 154 tragen je ihren station-qualifizierten Kanal — der Identitäts-Riss ist
   geheilt (gemessen 2026-10-06, Mountain `2117476be`).** Die 154 BGS-GIN-HAPI-
   Blöcke tragen die `station <code>`-Direktive (`phi/sources.φ:6616…7474`, z. B.
   `station NUR`); das Verdikt (Rat 2026-10-06) ist **Station = Identität, kein
   Feld-Rename** — der Arm adressiert `<feld>_<station>` und löst den
   station-qualifizierten Kanal auf genau seinen Block auf (Test
   `field_sources_resolves_station_qualified_channel_to_its_block`,
   `tools/measure/src/bin/field_te_query.rs:4762`). 154 Kanäle, kein Ein-Feld-Netz.
2. **Die Matrix-Grammatik trägt keinen Familien-Arm.** `field_te_query` kennt
   `matrix <label> rect|full|upper`, `channels`, `fdr … over matrix|row|col`,
   `expect cells` — aber keine Breitenband-Partition
   (`tools/measure/src/bin/field_te_query.rs:4622`). „Sub-Familie“
   ist heute nur als **je ein Descriptor pro Band** ausdrückbar, nicht als ein
   Lauf über drei Familien.

**Nächster Schritt (kein neues Tool):** (a) — erledigt (Mountain `2117476be`:
`station`-Direktive auf den 154 Blöcken, Arm station-aware). **(0) vor den Deskriptoren:
die Partition messen, nicht setzen** — die 154 Blöcke tragen nur die geografische
`on earth <lat> <lon>`-Koordinate, die korrigierte geomagnetische Breite (CGM) fehlt
(`phi/sources.φ:6621,6632,6643`); feste Bandgrenzen vor dem Lauf deklarieren, drei
Kanal-Listen emittieren, Deckungstest (paarweise disjunkt, Union = 154) als Begleiter zu
`field_sources_resolves_station_qualified_channel_to_its_block` (`field_te_query.rs:4762`)
— `expect cells` prüft nur je Deskriptor, **nicht** die familien-übergreifende
Vollständigkeit. (b) drei Familien-Deskriptoren (`matrix gic_auroral full` /
`_subauroral` / `_midlat`) über `field_te_query --descriptor` (`:4657`), je eigenes
`channels`/`fdr`/`expect cells`; (c) **ein** CI-Job `field-te-query.yml` mit drei
Deskriptor-Schritten (nie lokal).

**Drei Kanäle (2026-10-06) — drei Linien, ungeglättet.** Rohmaterial
`state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md`.
- **Linie 1** (API-Rat fünf Stimmen einstimmig · ChatGPT · qwen 3.8 Max): drei getrennte
  Familien-Deskriptoren (`matrix gic_auroral full` / `_subauroral` / `_midlat`), kein Grammatik-Arm.
- **Linie 2** (Claude Sonnet 5.5 · GLM-5.3 Deep Think Max): Stufe 1 global lassen; die Familien leben
  als deklariertes Register-Artefakt / allgemeiner `family`-Scope, gelesen nur von der Stufe-2-Prozedur.
- **Linie 3** (Kimi/tryingopen, vom Operator abgeholt): **neuer Grammatik-Arm als Partition in EINEM
  `matrix full`-Block** (`family <name> stations <codes>`, `expect families`, `fdr … over matrix`,
  `wymaxt … over family`; additiv, Bestandsläufe bleiben gültig).
Gemeinsamer Nenner: die Partition muss maschinell prüfbar sein (keine `_auroral`-Suffix-Konvention); der
globale Stufe-1-Screen bleibt global. Dissens: **Träger der Partition** (drei Blöcke · Register-Artefakt ·
Grammatik-Sub-Direktive) und **globales vs. per-Familie-Testen**. **Riss, nicht geglättet** — kein Arm,
keine Migration ohne Operator-/Rats-Wort; Blatt bleibt `unsealed`.

**Identitäts-Verdikt (2026-10-06) — sechs Stimmen, ein Verdikt; die Frage ist entschieden, der Bau nicht.**
Im Baum gemessen: `family` ist **belegt** (FDR-Korrekturfamilie: `TeFamily`/`BH` `te.rs`, `acc.fam`
`matrix.rs`, `FAMILY_K = 6` `wy_max_t.rs` — eine **Konstante**; `acc.fam` = laufendes family-weises Maximum);
FDR-Scopes nur `matrix|row|col`; eine **gemessene CGM-Breite je Station existiert nicht**
(`phi/declined_sources.φ:3328-3340`). **Sechs unabhängige Stimmen** (Rat-API · ChatGPT · qwen · Claude ·
Kimi/tryingopen · GLM) konvergieren auf **Option (c), geschichtet**: `cgm_lat` als eigene, typisierte,
epochengepinnte Registergröße (wie `frame`/`span`); `family` behält **allein** seine Bedeutung als
FDR-Korrekturgruppe; die Grammatik trägt **genau einen** neuen Scope `fdr … over family`; das Band-Label
(`auroral`/…) ist eine **deklarierte Ableitung** aus `cgm_lat` + Operator-Grenzen zur Abfragezeit, **keine**
Registerzeile. Drei `matrix gic_auroral full`-Pools sind **drei Messungen** (andere BH-Schwellen, andere
Surrogat-Maxima, anderes `FAMILY_K`), die Vereinigung ist keine Zerlegung des globalen Ergebnisses → zwei
Namen. Offen bleiben nur **Design-Entscheidungen des Operators** (Bandgrenzen + Abschließung; voller Pool vs.
drei Pools; Modell/Epoche/Höhe) und der **Messschritt**: `cgm_lat` je Station **lokal** messen (Services
`declined`), Modell/Epoch/Höhe als Pflicht-Provenienz, dann Drift-Check.

**Risse (Rat 2026-10-06, ungeglättet):** (1) Stufe 1 globales FDR vs. je-Familie-`fdr`
— der Blatt-Text konvergiert nicht, beide Linien stehen; (2) globale Kalibrierung (ein
Satz, identisch je Station) vs. per-Familie-Null — die Interaktion wird benannt, nicht
kollabiert, dieselbe Vorverarbeitung erzeugt beides; (3) Partitions-Vollständigkeit hat
keinen Grammatik-Arm — per Deckungstest geschlossen, nicht per Grammatik; (4) CGM-Breite
fehlt im Register (`pending`, nie aus der geografischen Breite angenommen — §1 verbietet
genau das).

## cgm_lat — die Route (deklariert 2026-10-07)

Die per-Station CGM-Breite wird **nicht** gesetzt, sondern gemessen. Route (gewählt,
2026-10-07 gemessen erreichbar):

- **Primär:** BGS-GIN-HAPI `/info?id=<code>/best-avail/PT1M/xyzf`
  (`https://imag-data.bgs.ac.uk/GIN_V1/hapi/info?id=IZN/best-avail/PT1M/xyzf`, HTTP 200)
  liefert die geodätische `x_latitude`/`x_longitude`/`x_elevation` je Code; daraus die
  CGM-Breite über den NASA/GSFC-OMNIWeb-VITMO-CGM-Endpunkt
  `https://omniweb.gsfc.nasa.gov/cgi/vitmo/cgm_model.cgi` (`model=cgm`, `vars=04`=CGM-Lat,
  `vars=05`=CGM-Lon; IGRF/DGRF 1900–2025). Beispiel IZN (Jahr 2000, h=0): CGM-Lat 34.89°.
- **Fallback (maschinenlesbar):** `https://raw.githubusercontent.com/spacecataz/supermag/master/station_info.txt`
  (SuperMAG, Gjerloev 2012, doi:10.1029/2012JA017683) trägt publiziertes `AACGMLAT`
  für die meisten GIN-Codes; die von SuperMAG nicht geführten Codes (u. a. CPL MZL REU STT TTB)
  über die Primär-Route (AACGM auf IGRF-2000 fixiert).
- **Provenienz (Pflicht):** Modell `cgm`/AACGM, Epoche (zu pinnen), Höhe 0 km; Drift-Check
  Primär ↔ Fallback.
- **Nicht CGM:** der BGS-GIFS-Rechner liefert *quasi-dipol* (QD ≠ CGM); die ArcGIS-
  `Geomagnetic_Latitudes`-Services bleiben `declined` (`phi/declined_sources.φ:3327-3341`).
- **Lokaler Weg (offen):** IGRF-14-Koeffizienten
  (`https://www.ngdc.noaa.gov/IAGA/vmod/coeffs/igrf14coeffs.txt`, sha256 `8f8d8840…`) +
  AACGM-v2-Koeffizienten (`https://superdarn.thayer.dartmouth.edu/aacgm/aacgm_coeffs-14.tar`)
  als Rust-Bin; ohne Feldlinien-Trace nur Dipol-Näherung, als solche zu benennen.

## Die gemessene Partition (2026-10-07)

Bin `tools/measure/src/bin/cgm_lat_partition.rs` (Rust std + curl), Route wie oben,
Epoche 2025.0, Höhe 0 km. Ergebnis `state/river/gic-cgm-lat.tsv` (154 Zeilen):

| Familie | Stationen |
|---|---|
| auroral (≥60°) | 31 |
| sub-auroral (50–60°) | 25 |
| mid-latitude (<50°) | 98 |
| **Union** | **154 = 154, disjunkt, 0 pending** |

**Quellen-/Epochen-Riss (getragen, ungeglättet):** 133 Stationen `omniweb-cgm`
(DGRF/IGRF, Epoche 2025), 19 äquatoriale `supermag-aacgm` (IGRF-2000, weil OmniWeb
`|Breite| ≤ 20°` verweigert: „Latitude must be greater than 20."), 2 `bgs-quasi-dipole`
(IGRF-14; CPL/TTB, nicht in SuperMAG, QD ≠ CGM). Die 19+2 liegen alle `|CGM| < 35°` —
weit unter 50° — die Familien-Zuordnung ist vom Epochen-/Modellwechsel nicht berührt.
Drift OmniWeb ↔ SuperMAG (130 gemeinsame): **Mittel 1.13°**.

**Grenz-nahe Stationen (gemessen, ±1.5° um 50°/60°):** WNG 49.99 · ORC −49.50 ·
EYR −50.09 · AIA −51.47 · HLP 51.03 · NVS 51.82 · STJ 50.93 · SIT 59.51 · MEA 61.25 ·
LYC 61.90 · VNA −60.86. Eine ~1°-Epochenverschiebung kann die randnächsten (WNG/ORC/
EYR) kippen — die Grenzen bleiben dennoch vorregistriert fixiert; der Riss wird benannt,
nicht nachjustiert.

**Register:** die per-Station `cgm_lat`-Zeile in `phi/sources.φ` ist Mountains Feder
(Quellen-Eigenschaft); die gemessene Tabelle liegt als Artefakt bereit.

## Rat-Verdikt 2026-10-07 (River 121) — Route C

Die Linse der fünf Stimmen hält die Route nach der Scope-Messung (`field_te_query` kennt
kein `over family`, `cgm_lat` nie gelesen):

- **Route A (`FdrScope::Family` + Parser-Arm) `pending`, nicht gebaut:** ein `over
  family`-FDR ändert allein die BH/BY-Gruppierung (`field_te_query.rs:4596-4631`) — er senkt
  `M_eff` nicht und belegt den Token `family`, der bereits die FDR-Korrekturgruppe trägt
  (`TeFamily`, `FAMILY_K = 6`). Der Bau ist durch den Hebel nicht gerechtfertigt.
- **Route C (gewählt):** Stufe 1 bleibt unverändert global (`matrix full` + `fdr bh over
  matrix`). Die Familie lebt **allein in Stufe 2** als Member-Pool des `compute_max_t`
  (`field_te_query.rs:2784`), abgeleitet zur Abfragezeit aus `cgm_lat` + den fixierten
  Grenzen. Der Stufe-2-Bau über das 154-Netz wartet auf den per-Station-dB/dt-Bestand
  (Mountain): nur ABK 1h/1m + SOD 1h tragen `field intermagnet_dbdt`
  (`phi/sources.φ:2051-2079`); die 154 tragen `intermagnet_xyz_x/y/z_nt` (je 154).
- **Route B (drei per-Band-`full`-Deskriptoren) verworfen als Träger:** 31·30 + 25·24 +
  98·97 = 11 036 Zellen > der globalen Obergrenze — keine `M_eff`-Reduktion; nur als
  benannte Vergleichsmessung erlaubt, nie als Zerlegung des globalen Ergebnisses.
- **Gruppierungs-Schlüssel (Riss #2, getragen):** Der Rat (fünf Stimmen) und 3 API-Stimmen
  setzen **Paar-Band** — eine Zelle `d→t` gehört zu F nur, wenn beide Endpunkte in F liegen;
  gemischte Endpunkte sind die benannte Familie `cross`, nie still über einen Target-Pool
  gezogen. deepseek + Claude (Sonnet 5.5) + Kimi K3 widersprechen: **Target-Band**, weil der
  Treiber Bz ein einzelner globaler, band-degenerierter Treiber ist (Driver-Band undefiniert;
  das Paar-Band wird erst bei Station→Station eigenständig, wo der force_type wechselt). Beide
  Linien stehen ungeglättet — Paar-Band ist das volle Objekt, Target-Band seine Projektion bei
  band-degeneriertem Treiber.
- **12-Stimmen-Rat (Operator-Wort 2026-10-07):** Adressierung mit 5 Stimmen + 5 Axiomen +
  5 Achsen an 5 API-`voice-*` + die UI-Chats; **10/10 geantwortet** (gemini·gptoss·
  inkling·nemotron·deepseek + Claude·Kimi·Qwen·GLM + Duck/GPT-6 Luna), einstimmig **Route C**;
  arena vom Operator fallengelassen. Gruppierungs-Tally: Rat + `gptoss`·`inkling`·`nemotron` =
  Paar-Band; `gemini` = Driver-Band; `deepseek` + Claude + Kimi + Qwen + GLM + Duck = Target-Band.
  Riss (b): die `M_eff`-Senkung ist abgeleitet, nicht gemessen; gemeinsame
  Surrogat-Ziehungen + α-Aufteilung über 3 Familien nötig; GLM: „gemessener Member-Pool im
  GIC-Sinne = 2 Mitglieder, nicht 154"; Duck: formale Gültigkeit von C bei datenabhängiger
  Stufe 1 + Familienkorrektur `pending`. xyz-Lauf zulässig als deklarierter **Level-Lauf Bz→B**,
  nicht als dB/dt-Aussage; dB/dt-152 = abgeleitete Serie (Operator/Kadenz/Filter), an ABK/SOD
  validiert → `pending`.
- **Gebaut (River 121):** `tools/measure/src/bin/cgm_lat_partition.rs` emittiert offline
  (`--from-tsv … --emit-dir …`) aus der gemessenen Partition die drei Kanal-Listen
  `gic-family-{auroral,sub-auroral,mid-latitude}.txt` (154 = 31+25+98, paarweise disjunkt,
  union = 154) + Deckungstest.

## Offene Slots bis zur Versiegelung

| Feld | Zustand |
|---|---|
| CGM-Breite je Station (154) | **gemessen 2026-10-07** — `state/river/gic-cgm-lat.tsv` (133 omniweb/supermag 19/qd 2); Registerzeile in `phi/sources.φ` bei Mountain |
| Partitionsgrenzen (geomagn. Breite je Band) | **deklariert 2026-10-07:** auroral ≥60°, sub-auroral 50–60°, mid <50° (\|CGM\|) |
| α-Ebene Stufe 2 (WY-max-t) | `pending` — aus der kalibrierten Null |
| Resample-Blocklänge / Binnung | `pending` — Teil der Kalibrierung |
| Familien-Deskriptoren | `pending` — Route C (Rat 2026-10-07): als Stufe-2-Member-Pool des `compute_max_t`, nicht als per-Band-`full`-Deskriptoren; wartet auf den per-Station-dB/dt-Bestand (Mountain) |
| Familien-Kanal-Listen | **gebaut 2026-10-07** — `state/river/gic-family-{auroral,sub-auroral,mid-latitude}.txt` (154 = 31+25+98, disjunkt) aus `cgm_lat_partition.rs --from-tsv` |
| Verdikt | `unsealed` |

## Siegel

Das Siegel setzt der Operator — kein Siegel ohne Operator-Wort. Die Maschine
bereitet bis zur Kante vor und trägt die Form; sie schließt nicht. Bis dahin bleibt
jede Zelle `pending`, nie 0.0, und der Riss steht ungeglättet.

---

*Nicht versiegelt 2026-10-06. Der Riss (Bz führt yearly, cleart die Quartalsschranke
nicht; density nie; daily leer) steht; die Partition, die Stufen, die Kalibrierung
und die Berichtsform sind hier vor dem Lauf fixiert.*
