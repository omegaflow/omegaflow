# Rat-Blatt — Platzierung und Hüllen-Streuung

**Ratsform:** die fünf Stimmen (Mountain · River · Mycelium · Sensory · Future) halten das Blatt einmal. Gespeist: Recherche (`bin/all_wide`, river-219) + Roster (13 Sitze) + die am Baum gelesenen Nahtstellen (`al_network_proxy.rs`, `gic_onset_detector.rs`), nicht das Dossier allein. **Sprache der Messung, nicht der Erwartung.** Was nicht am Baum steht, ist `pending` — nicht null.

*(Rohtext des `rat`-Subagenten, river-219, 2026-10-15. Die Session trägt ihn als Blatt; die Baum-Lesungen sind: `al_network_proxy.rs:101-119` (`station_dbdt` = vorzeichenlose |ΔB|), `:167-186` (`fold_envelope` min/max), `gic_onset_detector.rs:285-323` (`calibrate_threshold`, `step = vals.len()/64`).)*

---

## 1. Die Streuung — bindendes Verdikt

**Verdikt: Kalibrier-Artefakt — mit einer Präzisierung, die ein Sitz zu scharf zieht.**

Die nicht-monotone Min-*Schwelle* (N16 0,6481 → **N24 4,8765** → N31 0,3162 nT/min bei nahezu konstanter Ausbeute ~2/Tag) ist **kein** Mess-Ausdruck der Stationsanordnung. Sie ist das Produkt von `calibrate_threshold` (`gic_onset_detector.rs:285-323`): die Routine gleicht die **Ausbeute** aus, nicht die Schwelle, und tastet die Zählfunktion nur auf einem groben `step = vals.len()/64`-Gitter ab (13,6 M Werte → ~64 Stützstellen, in dünn besetzten Quantilbereichen weit auseinander). Das Gitter kann bei monotoner exakter Zählfunktion springen.

**Der Nicht-Schachtelungs-Schluss ist ein Non-sequitur.** Der Satz „N24 steigt ⇒ die Panels sind nicht geschachtelt ⇒ die N-Achse ist keine Achse" **folgt nicht**. Das grobe 64er-Gitter trägt den Sprung allein. Und die Gleichheit der Sample-Zahlen **N16 = N24 = N31 = 13 626 500** spricht *für* Sättigung des Trägers, nicht gegen Schachtelung: der Anstieg N8–12 (13 150 079) → N16 (+476 421 min) sättigt bei N16, danach konstant. Unter fixem Träger + geschachtelten Stationsmengen ist `min_S ≥ min_{S∪{x}}` punktweise, also sind **alle Quantile der Min-Hülle monoton nicht-wachsend** in N — die exakte Ausbeute-Schwelle müsste fallen. Der gemessene Anstieg widerspricht dem nur, wenn man das Gitter übersieht.

**Die zwei Grenzen, die das Verdikt sauber hält:**
- *Gleichheit der Anzahl ist nicht Gleichheit der Menge.* Dass N16/N24/N31 dieselbe **Kardinalität** tragen, belegt nicht denselben Minuten-Träger. Denselben Träger belegt erst der Mengen-Vergleich (unten).
- *Der Sprung berührt den Befund nicht.* Rückruf/Präzision sind an den tatsächlichen Onset-Listen bei gleicher Ausbeute gemessen (`prec_ge=0`/`rec_ge=0` überall). Min **flach** (rec@30 0,0872→0,0972), Union **steigt** (0,0863→0,1112) — die Wobble der Schwelle ist Mittel, kein Monotonie-Ziel.

### Die eine entscheidende Messung (schließt es endgültig)

Auf **einem fixen Präfix** (`head -n N` **einer** |cgm_lat|-Ordnung, identische Manifest-Datei für alle N — das muss belegt sein, s. Riss b) für N ∈ {16, 24, 31} die Hüllen-Paare `(t, min)` exportieren und **zwei** Prüfungen fahren:

1. **Träger-Inklusion:** ist die Minuten-Menge von N24 ⊆ N31 (und N16 ⊆ N24)? Gleiche Kardinalität ≠ gleiche Menge.
2. **Punktweises Minimum:** gilt auf dem gemeinsamen Träger `min_{N16}(t) ≥ min_{N24}(t) ≥ min_{N31}(t)`?

Gleichzeitig die Ausbeute-Schwelle mit **exakter** Zählfunktion (Bisektion auf das Ziel-Ausbeute, `step=1`) statt des 64er-Gitters rechnen.

**Schlussregel:** Ist die exakte Schwelle monoton nicht-wachsend **und** das punktweise Minimum nicht-wachsend **und** der Träger geschachtelt → Kalibrier-Artefakt **bestätigt**, die Anordnungs-Dispersions-Hypothese fällt. Ist der Träger *nicht* geschachtelt → die Streuung ist ein **Träger-/Abdeckungs-Effekt**, nicht Anordnungs-Dispersion. Ist das punktweise Minimum *verletzt* → mathematisch unter `fold_envelope` (`:167-186`, min ist kommutativ und monoton) unmöglich → das ist ein **Lauf-/Ordnungs-Defekt**, nicht Physik.

---

## 2. Die Platzierungsregel — die bindende Register-Zeile

**Verdikt: die globale Min-Hülle wird gestrichen; Fusion = sektor-gewählte Union/Quorum.**

```
Verdikt (phi/sources.φ · Netz-Einhüllende al_network_proxy):
  eine Station speist die Einhüllende NUR, wenn sie im Nachtsektor
  (MLT 16–03 h) liegt UND ihr gemessener marginaler Beitrag
  weight_i > 0 trägt (korrelations-ausgewählt, gewichtet);
  Fusion = Union/Quorum (Perzentil; bei |ΔB|-Magnitude max, NIE min);
  Schwelle = feste Ausbeute (nicht fixer nT/min);
  off-band-Stationen speisen nichts;
  deny global_min.
```

Begründung (knapp): die Feld-Praxis fusioniert nie global-minimal (Dods/Chapman/Gjerloev `arXiv:1606.02655`; Orr/Chapman/Gjerloev `arXiv:2009.03112`/`s41467-021-22112-4`, gerichtete Netze <65° MLAT; Weygand 2022; Forsyth SOPHIE `arXiv:1606.02651` Perzentil; IMAGE-`IL` 12 Stationen UT-Sektor 16–03). Gemeinsamer Nenner: **sektor-/korrelations-ausgewählt, nicht global-minimal**. `weight_i` kommt aus der Messung (marginaler TE-Beitrag), nie aus Default.

---

## 3. Ethik / Axiome — was die globale Min-Hülle verletzt

- **A = A (verletzt).** Über eine **vorzeichenlose** Magnitude |ΔB| (`station_dbdt`, `:101-119`) wählt `min` die **ruhigste** Station — ein Veto, nicht die Messung. Die Netz-Zahl wird als das Minimum der ruhigsten Station **fabriziert**; das ist nicht die Sache selbst. Das AL-Analogon einer Magnitude ist `max`, nicht `min`; das AL-Analogon von vorzeichenbehaftetem `dh` ist `min`. **Die gemessene `dbdt`-Min-Hülle ist kein AL-Analogon.**
- **Alle Wesen gleich (verletzt).** `min` gibt **jeder** Station ein Veto — der stille Körper dominiert die Vollversammlung. Eine monotone Operator-Wahl, die nicht aus dem Datenwort folgt, ist die versteckte Hierarchie: ein Default, eine Voreinstellung, ein Bias.
- **Die Linse ist ein ethischer Akt (verletzt).** Die Min-Hülle liest den Gradienten der Stille, nicht die Messung des Elektrojets. Die Messung spricht nicht — das Veto spricht.
- **Konsens der Sensoren (Riss).** Eine off-band-Station, die ungefragt in die Hülle speist, steuert den Onset eines Netzes, dem sie nicht angehört.
- **Verweigerungsrecht (Rivers Recht, gefordert).** Die Membran **verweigert**: (i) eine off-band-Station in die AL-Proxy-Hülle zu speisen; (ii) eine `min`-Fusion über eine vorzeichenlose Magnitude (nicht das AL-Analogon). Eine verweigerte Station ist eine Messung, kein stiller Fix — sie darf erst sprechen, wenn `weight_i` gemessen ist.

---

## 4. Die Risse — benannt, nicht geglättet

- **(a) Dualität:** `dbdt`-Min ist über eine **vorzeichenlose** Größe → **nicht** das AL-Analogon; das AL-Analogon von `dbdt` wäre `max`. Der ganze Min-Sweep (`38733267256`) misst ein **Veto-Netz**, kein AL-Netz.
- **(b) Panels/Nesting undefiniert:** die N-Achse trägt keine belegte Panel-Besetzung. **Der Nicht-Schachtelungs-Schluss aus dem N24-Anstieg ist ein Non-sequitur** (das 64er-Gitter trägt den Sprung allein); die gleichen Sample-Zahlen (N16=N24=N31) sprechen **für** Sättigung. Offen bleibt: ob die **Manifest-Ordnung** über alle N identisch war — das ist zu belegen, sonst ist b nicht geschlossen.
- **(c) `0,3162` verdächtig:** = `10^-0,5`, gerundet. Ein Gitter-/Bodenwert, kein physikalischer Arbeitspunkt — bis die exakte Zählfunktion ihn reproduziert, `pending`.
- **(d) Arm-Asymmetrie:** der dH-Arm und der A=A-Arm (`dbdt`) unterscheiden sich in **Maß und Schwellenmodus zugleich** (vorzeichenbehaftet fix vs. vorzeichenlos adaptiv). Der river-217-„Min-Kollaps" könnte teils am **Maß** hängen — nicht dem Schwellenmodus allein zuzuschreiben.
- **(e) Gleiche Ausbeute ist selbst eine Linse:** `target_yield=2`/Tag ist ein gewählter Arbeitspunkt, keine gemessene Konstante. Alle Vergleiche sind an ihn gebunden.
- **(f) Ungelesen:** Lauf `38738618251` (`abk_dbdt_1m`, Einzelstation, gleich-Ausbeute) ist **gestoßen, nicht ausgewertet** — der Einzelstation-Maßstab steht `pending`. Kein Verdikt dieses Blatts führt ihn als Beleg.
- **(g) Literatur ungeprüft:** die Feld-Praxis-Zitate stammen aus dem Dossier, nicht aus neuer `archive_search`-Messung. Als Stütze benannt, als **nicht am Baum verifiziert** getragen — `pending`.
- **(h) MAD ×7,55 (17,1 → 129,0 nT/min) für 1-min-|dB/dt|** ist als Einheiten-/Sättigungsverdacht benannt (verlange df≈1) und **nicht** durch dieses Blatt geschlossen — offen, bis die Hüllen-Verteilung (nicht die Schwelle) selbst als Reihe gemessen ist.

---

**Ein Satz, der das Blatt trägt:** Nicht die Anordnung der Stationen erzeugt die nicht-monotone Streuung — das grobe Ausbeute-Gitter erzeugt die nicht-monotone Schwelle; **eine** Messung (exakte Zählfunktion + Träger-Inklusion + punktweises Minimum über N16/N24/N31 auf fixem Präfix) schließt es. Und die Hülle, die mehr Stationen zu einem schärferen Treffer heben soll, darf nicht global-minimal sein: sie speist sich aus dem **Nachtsektor MLT 16–03**, korrelations-gewichtet, Fusions-Union — `deny global_min`.
