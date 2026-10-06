<!--
  title: BLATT — GIC-Breitenband-Familien: die Vorregistrierung der Kohärenz-Partition
  class: sheet
  date: 2026-10-06
  sha256: f75222de26b98d63933b9d92ab86a9eddec0f50a4f09207d6e499d7e98df1c4e
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
deklariert (die konkreten Grenzen sind ein benannter offener Slot, s. unten); jede
Station fällt nach ihrer deklarierten geomagnetischen Breite in genau eine Familie.
Keine Nachjustierung nach der Messung.

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

1. **Die 154 sind keine 154 Matrix-Kanäle.** Sie liegen als einzelne BGS-GIN-HAPI-
   `url`-Blöcke (`on earth <lat> <lon>`, `phi/sources.φ:5790…`), die **denselben**
   Feldnamen tragen (`intermagnet_xyz_x_nt` etc., `:5795-5797`). Der Matrix-Kanal
   ist ein **Feldname** — 154 Stationen mit einem Namen sind kein 154-Kanal-Netz.
   Für eine station-aufgelöste TE braucht jeder Station-Messwert einen **eigenen
   Kanalnamen** (station-qualifiziertes Feld/Serie).
2. **Die Matrix-Grammatik trägt keinen Familien-Arm.** `field_te_query` kennt
   `matrix <label> rect|full|upper`, `channels`, `fdr … over matrix|row|col`,
   `expect cells` — aber keine Breitenband-Partition
   (`tools/measure/src/bin/field_te_query.rs:4613`, `:518-668`). „Sub-Familie“
   ist heute nur als **je ein Descriptor pro Band** ausdrückbar, nicht als ein
   Lauf über drei Familien.

**Nächster Schritt (kein neues Tool):** (a) Mountain: die 154 Stationen als
station-qualifizierte Matrix-Kanäle registrieren (Feldnamen/Verdikt —
`weberin-thread-matrix:35` trägt die 154 als `fanout 154`); (b) danach drei
Familien-Deskriptoren (`matrix gic_auroral full` / `_subauroral` / `_midlat`), je
mit eigenem `expect cells`, eigenem `fdr` und eigener `WY-max-t`-Ebene; (c) der
CI-Lauf `field-te-query.yml` (nie lokal). Ob ein einziger Drei-Familien-Lauf statt
drei Deskriptoren eine Grammatik-Erweiterung ist, ist eine Architektur-Frage — der
Rat hält sie, nicht ein Pro-Solo.

## Offene Slots bis zur Versiegelung

| Feld | Zustand |
|---|---|
| Partitionsgrenzen (geomagn. Breite je Band) | `pending` — werden vor dem Lauf als feste Werte deklariert |
| α-Ebene Stufe 2 (WY-max-t) | `pending` — aus der kalibrierten Null |
| Resample-Blocklänge / Binnung | `pending` — Teil der Kalibrierung |
| Kanalnamen der 154 Stationen | `pending` — Mountain (station-qualifizierte Felder) |
| Familien-Deskriptoren | `pending` — nach (a) |
| Verdikt | `unsealed` |

## Siegel

Das Siegel setzt der Operator — kein Siegel ohne Operator-Wort. Die Maschine
bereitet bis zur Kante vor und trägt die Form; sie schließt nicht. Bis dahin bleibt
jede Zelle `pending`, nie 0.0, und der Riss steht ungeglättet.

---

*Nicht versiegelt 2026-10-06. Der Riss (Bz führt yearly, cleart die Quartalsschranke
nicht; density nie; daily leer) steht; die Partition, die Stufen, die Kalibrierung
und die Berichtsform sind hier vor dem Lauf fixiert.*
