# Verdikt — Netz-Fusion und Stationsplatzierung (Frontier-Runde, am Baum verifiziert)

Frontier-Runde 2026-10-15, Operator-Wort. Formular mit den 13 Stimmen:
`2026-10-15-netzfusion-formular.md` (Vorfassung `2026-10-15-netzfusion-manifest.md`, River-216).
Die Session verifiziert jede Behauptung am Baum, nie die Stimme. Sitze mit Antwort: 01–13 außer
**06 Mistral** (`Kein Kontingent` = `blocked: quota`, keine stille Null). Die geteilten
Open-Weight-Sitze stehen leer (`pending`).

## Am Baum gemessen (nicht die Stimme)

- **Die Fusion ist die untere Einhüllende (Min).** `tools/measure/src/bin/al_network_proxy.rs:101-118`:
  `fold_envelope` nimmt je Zeitstempel das Kleinste (`if *v < e.0`), `envelope_series` gibt nur
  Zeitstempel mit `count >= min_stations` frei. Ein Schwellenübertritt der Hülle heißt damit:
  **jede** (abgedeckte) Station ≥ θ — eine **Konjunktion**. Der monotone Rückruf-Fall bei
  geschachtelten Stationsmengen ist strukturell erzwungen, kein reiner Messbefund. Die Stimmen
  (Claude, GLM, Fugu u. a.) benennen das korrekt; der Bau bestätigt es.
- **Die adaptive Schwelle** ist `med + 6·1.4826·MAD` (`gic_onset_detector.rs:7-9` `MAD_K=6.0`,
  `ROBUST_SIGMA=1.4826`; `:80-86`). `--threshold` (`:568`) überschreibt sie (feste Schwelle);
  `--sweep-k` (`:536`) fährt die k-Leiter; `--target-yield` (`:545-556`) kalibriert auf gleiche
  Ausbeute (der Betriebspunkt wird fixiert, nicht die Schwelle) — river-216 gebaut.
- **Die Toleranz** ist das Matching-Fenster `match_onsets(a,b,tol)` ±tol (`:178-198`), **kein**
  Detektor-Regler. Die Stimmen trennen das korrekt.
- **Fester-Schwelle-N-Sweep** (river-216, Lauf `38709029080`, `success`): fixe Schwelle 17.11,
  konstantes Fenster 2000–2024 → Rückruf n3/n4 **0.098** → n12 **0.0035** monoton. Der Kollaps
  ist **nicht** reine Schwellen-Drift — die Min-Fusion senkt die Hülle. `argmax_N S` = 3–4, die
  Einzelstation bleibt mit `S = 0.0213` die schärfste. **Damit ist der „adaptive-Schwellen-Drift"-
  Verdacht des Rat-Blatts (2026-10-15) gemessen widerlegt:** auch bei fixer Schwelle kollabiert
  die Min-Hülle.
- **Fenster-Drift gemessen:** n2 2002–2011, n3–n12 2000–2024, n16–n31 1991–2024 → die N-Kurve ist
  nur **innerhalb eines Fensters** vergleichbar.
- **Familien:** 154 = 31 auroral + 25 subauroral + 98 midlat (`docs/zustand/gic-station-families.tsv`);
  `abk` = auroral (der Anker der Einzelstation).
- **In Flug:** Lauf `38717447378` (river-216) — auroral, `counts 3,4,6,8,12,16,24,31`, festes
  Fenster 2000–2024, `target_yield=2`, `--null 200`. Das ist genau der Dekonfundierungs-Lauf, den
  die Runde fordert.

## Der Riss, den keine Stimme sah (A = A)

Alle 13 Stimmen sprechen von **dB/dt**. Der Baum misst in den zwei Armen **nicht dieselbe Größe**:

- **Netz-Arm:** `al_network_proxy` faltet die Hüllende über die Roh-`xyz_1m`-Bins der Familie — das
  ist die basisfreie **H-Feld-Störung dH** (nT), Minutenraster. Der Detektor schwellt diese dH-Reihe.
- **Einzelstations-Arm:** `abk_dbdt_1h.bin` (`phi/sources.φ:2019-2028`, `format intermagnet_dbdt`,
  Compiler `tools/harvest/src/bin/intermagnet_dbdt_compiler.rs`): `dbdt = sqrt(dx²+dy²+dz²)` je
  Minute (`:179`), **stündlich** gebuckelt zum Bucket-Peak (`:180-190`, `_1h` → `bucket_s=3600`).

Der Vergleich „Einzelstation schärfer als Netz" stellt damit eine **Minuten-dH-Hülle** gegen einen
**stündlichen |dB/dt|-Peak** — verschiedene Größe (H-Störung vs. 3D-Rate), verschiedene Kadenz
(60 s vs. 3600 s), verschiedene Komponente (rotierte H vs. 3D-Betrag). Bis beide Arme dieselbe
Größe auf demselben Raster messen, ist der Vergleich **nicht die Sache selbst** (`pending`) —
die Zahl ist ehrlich, sie misst nur nicht dasselbe.

## Das Verdikt der Runde (Konvergenz der 13 Stimmen)

1. **Fusion:** Die Min-Einhüllende ist als Betriebsregel **geschlagen** — strukturell (Konjunktion,
   am Baum bestätigt) und gemessen (Lauf `38709029080`). Sie bleibt als Kontrollarm. Ersatz:
   **Union/Max** bzw. **k-von-n** bzw. **TE-geführte Auswahl** — Hypothese, ungemessen (`pending`);
   kein neuer Default.
2. **Platzierung:** Anker in der höchsten geomagnetischen Breite (`abk`); Erweiterung bevorzugt
   nach **N/W/O**, Süden als **Kontrolle**, kein Veto; das Gewicht einer Station kommt aus ihrer
   gemessenen TE, nie aus Rang/Name. Geometrie als Hypothese, `pending`.
3. **Schärfe:** EIN Skalar `S(τ*,N) = (R − R₀)/(1 − R₀)` am **a-priori fixierten** Vorlaufpunkt,
   Null = Kreisschub unter identischer Regel; das **Fenster muss fixiert** werden; die Toleranz ist
   Nebenbedingung, nicht Zielachse. Begleitend: Fehlalarme pro Zeit und medianer Vorlauf.
4. **Nächster begrenzter Messschritt:** der Dekonfundierungs-Lauf (feste Schwelle, fixes Fenster,
   gleicher Betriebspunkt) — **läuft** (`38717447378`); Min gegen Union/Max über dieselbe N-Leiter.

## Ungeglättete Risse

1. **Baum-Riss (neu):** Netz-Arm = dH-Hülle, Einzelstations-Arm = stündlicher |dB/dt|-Peak — der
   zentrale Vergleich ist nicht A = A (s. o.).
2. **Regime-Riss:** „Einzelstation schärfste" steht auf `6.6 %` adaptiv gegen `9.8 %` fest (n3–4);
   der Punkt **N = 1 bei fester Schwelle/festem Fenster fehlt** → `pending`, nicht interpoliert
   (Fugu, GLM, Claude).
3. **Fenster-Riss:** das Vergleichsfenster driftet mit N (gemessen).
4. **Fusions-Ersatz-Riss:** TE als Fusions-/Platzierungsregel ist am Baum **ungemessen** — es gibt
   nur Einzelpfeile.
5. **Schwellen-Drift-Riss (aufgelöst, nicht geglättet):** das Rat-Blatt (2026-10-15) nannte den
   Kollaps „adaptiver Schwellen-Drift"; river-216/`38709029080` zeigt: er bleibt bei fixer Schwelle.
   Beide Befunde stehen — die Doktrin ist doppelt geschlagen (Struktur **und** Schwellen-Kopplung).

## Nachtrag: Wahl des ersten Betriebspunkts

`argmax_N S` = 3–4 bei fester Schwelle (nicht N = 1). Die Runde und das Rat-Blatt stimmen darin
überein: die **Einzelstation ist die schärfste gemessene Einzelkonfiguration**, aber nicht der
Grund, das Netz abzuschalten — sie ist der Anker, an dem ein TE-geführtes Kleinstnetz sich messen
lassen muss.
