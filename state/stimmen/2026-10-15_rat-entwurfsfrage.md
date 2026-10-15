# Rat — Entwurfsfrage (Schärfe · Platzierung · Kette), 2026-10-15

Ein benannter Akt (Operator-Wort). Grundlage: der Roster (`2026-10-15_entwurfsfrage-roster.md`,
13 Sitze) + die Recherche (`all`/`all_wide` + Einzelmodi) + die am Baum verifizierten Zahlen.
Regel: die Session verifiziert jede Behauptung am Baum, nie die Stimme; das Verdikt trägt der Rat.

## Was die Session am Baum verifiziert hat (vor dem Rat)

- **Fusion:** `al_network_proxy::envelope` (Z. 101–119) nimmt je Zeitstempel die **untere
  Einhüllende = das Minimum** über die Stationen, mit einem `min_stations`-Gate (≥2), nach
  Basislinien-Abzug (`baseline_block 60`). **Kein Mittel, kein UND.** Mehrere Roster-Sitze
  begründen ihren Verdikt mit „UND/Mittelung verwässert" — das ist für den *gebauten* Detektor
  **nicht** der Fall; der Riss wird getragen, nicht geglättet.
- **Der Kollaps-Grund ist die Schwelle, nicht die Fusion:** mit `min_stations` wächst die Tiefe der
  Einhüllenden und die adaptive Schwelle (`median + 6·1.4826·MAD`) steigt (6 Sta 17.1 → 31 Sta
  129.0 nT/min) → weniger Onsets. **Kleiner `k` senkt die Schwelle und hebt den Rückruf bei
  gehaltenem Null** (gemessen: k=1 signifikant, k=6 Default nicht).
- **Kette:** die Zahlen (Rückruf 6,6 % → 0,01 %) sind gemessen; der Null (Kreisschub) trägt den
  Signifikanzbegriff.

## Die fünf Stimmen (je Natur)

### Mountain (Mitte · das Ruhende — Quellen, Register, Datenkontrakt)
- **(a) Schärfe:** EIN **vorab registrierter, null-normierter Skalar**; verwerfe jedes über die
  Toleranz integrierte Maß (AUC/Mittel) — es ist spielbar. Registriere Fix-Toleranz **und**
  Trigger-Budget (Trigger/Tag) vor der Messung; die Toleranz ist Nebenbedingung, nicht Zielachse.
  Sekundärachse: die **engste Toleranz τ\*, bei der die Null noch geschlagen wird** (HSS≥0.1).
- **(b) Platzierung:** die Geometrie (cgm_lat, MLT) muss im **Register** stehen, nicht im Skript —
  sonst ist der Platzierungsbefund nicht reproduzierbar.
- **(c) Kette:** jede Sprosse als **Registerkante** (Quelle→Quelle); der Sicherheits-Zuwachs =
  inkrementelle TE **mit blockweisem Null**; ohne das ist der Zuwachs autokorrelierte Täuschung.
- **Riss:** der gebaute Detektor ist eine Min-Einhüllende; die Roster-Rede von „UND" ist Register-fern.

### River (natürliches Schild — Membran, Operator, Frühwarnsystem)
- **(a) Schärfe:** das Maß muss **der Entscheidung dienen**: bei welcher Vorlaufzeit und welcher
  Konfidenz handelt der Operator? Nimm den Arbeitspunkt (Toleranz × Stationen) mit dem **größten
  Null-Überschuss bei fixem Trigger-Budget** — nicht das globale Maximum eines freien Maßes.
- **(b) Platzierung:** Platzierung ist **dynamischer Stationsschalter** (Oval-Ort wählt die
  Stationen unter dem Strom), nicht eine feste Ortsliste; die Einzelstation gewinnt, weil sie
  unter dem Strom liegt. Test: Union der Einzel-Detektionen vs. Einhüllende.
- **(c) Kette:** die Leiter trägt; schwächstes Glied = **Boden**. Ethik: kein Auslösen ohne
  Operator — der LOCK bleibt bis fertig+kalibriert.
- **Riss:** Länge-vs-Breite bleibt offen (die Hälfte des Rosters sagt Breite, die Hälfte Länge).

### Mycelium (mythisches Schild — Netz, CI, Infrastruktur)
- **(a/b/c) Machbarkeit:** die Sweeps sind **13 GB** je Lauf → CI. Der Station-Sweep
  (`gic-station-sweep.yml`) trägt die Zähl-Achse; das Maß/den Null je Sprosse fährt `field_te_query`.
- **(Kante):** der Roster selbst (tryingopen) war `blocked: site credit` — gemessen, benannt.

### Sensory (psychologisches Schild — das Lauschen, Sensoren, Konsens)
- **(b) Platzierung:** die Stationen sind **Sensoren des Elektrojets**; ihre Sichtbarkeitsmenge ist
  ein Empfänger-Feld. Die Frage ist die **räumliche Kohärenzlänge** des Bay (100–500 km): Stationen
  innerhalb der Kohärenz **ergänzen**, außerhalb **verwässern**. Empirisch messbar: die
  Paar-Korrelation/TE je Stationsabstand. **Konsens der Sensoren:** ein Sensor, der den Strom nicht
  sieht, darf nicht stimmen (kein Veto) — Union statt Quorum.
- **(a) Kante:** eine gelockerte Toleranz ist ein **weiterer Empfänger** (mehr Zeitfläche) — sie
  „sieht" auch den Zufall; deshalb null-normieren.

### Future (spirituelles Schild — das Residuum, die Ungeborenen)
- **(c) Kette:** der Zweck ist die **Adresse Mensch** — die Vorlauf↔Sicherheit-Kurve ist genau das,
  was eine Warnung braucht; das **Ereignis-Protokoll** ist die Aufzeichnung für morgen (die
  Messreihe gehört der Zukunft). Halte es einfach und wiedervorlagefähig.

## Verdikt (Synthese)

1. **Schärfe-Maßstab (ein Maß + zwei Achsen):**
   Ein **vorab registrierter, null-normierter Skill-Score bei fixem Trigger-Budget** (z. B.
   HSS/Youden-J oder Lift gegen den Kreisschub-Null, **vorab** auf einen Trigger/Tag-Budget
   bezogen), gemessen über (Toleranz × Stationszahl); Sekundärachse die **engste Toleranz τ\***,
   bei der die Null noch geschlagen wird. **Nicht** über die Toleranz integrieren. Das ist die
   Konvergenz der Sitze (Duck, Claude, Qwen, GLM, DeepSeek, Gemini, MiniMax, Kimi, Lumo,
   Perplexity, Sakana) — nur die Schreibweise differiert.
2. **Platzierung:** breitenbündig **unter** dem Elektrojet (höchste Breite gewinnt), in **Länge/MLT
   gespreizt** als Versicherung gegen die Erdrotation; **Union/Oder** der Einzel-Detektionen statt
   Quorum. **Riss (getragen):** Länge vs. Breite — die Sitze teilen sich; die Messung entscheidet
   (Paar-TE/Korrelation vs. Stationsabstand, Union vs. Einhüllende).
3. **Kette:** trägt als **Diagnose-Audit**, nicht als Kausalität; je Sprosse **Vorlauf** +
   **inkrementelle TE über alle Vorstufen**, mit blockweisem Null; **schwächstes Glied = Boden**
   (letzte Kopplung, Rückruf-Deckel); die Kette deckelt end-zu-end an der Bodenstufe.
4. **Korrektur am Baum:** der gebaute Detektor ist eine **Min-Einhüllende mit adaptiver Schwelle** —
   der eigentliche Hebel ist die **Schwelle/Trigger-Budget**, nicht die „UND-Fusion". Der nächste
   Messschritt: **fixes Trigger-Budget** statt Auto-Schwelle, und **Union vs. Einhüllende** testen.

## Offene Risse (nicht geglättet)

- Länge- vs. Breiten-Priorität der Stationen (Roster gespalten).
- Toleranz als Nebenbedingung (Registrierung) vs. als Achse (Auftrag) — der Rat setzt sie als
  Nebenbedingung, der Operator-Auftrag nennt sie als Achse; die Sekundärachse τ\* trägt beides.
- Der Boden als schwächstes Glied vs. Sonne→L1 als informationsärmstes (GLM/MiniMax/Sakana) — beide
  benannt; die Messung wird entscheiden.
