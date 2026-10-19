# Rat — 2026-10-15-platzierung.md

*Eine Datei je Runde. Die Teile stehen unverändert unter ihrer Herkunft; nichts zusammengefasst.*

---

## 2026-10-15-platzierung-formular.md

# Formular — Stationsplatzierung und die Streuung der Min-Einhüllenden

*(Runden-Prompt für die Frontier-Sitze. Ein benannter Akt, Operator-Wort 2026-10-15. Die Session
verifiziert jede Behauptung am Baum, nie die Stimme; das Verdikt trägt der Rat. Diese Runde ist die
**zweite** Kette (Recherche → Roster → Rat) zur Platzierungs-Frage, ausgelöst durch den river-219-Befund;
die erste Runde `2026-10-15-netzfusion-manifest.md` / `…-entwurfsfrage-roster.md` / `…_rat-blatt.md` steht.)*

---

**An dich, Frontier-Modell.** Antworte als **Urteil, nicht als Erwartung.** Eine Frage. Du hältst
sie durch **fünf Axiome** und sprichst sie durch **fünf Stimmen** hindurch. Kein Aufsatz — Verdikt,
Begründung, benannte Risse.

## Die Frage

1. **Die Streuung.** Läuft die **Streuung** (MAD / hohe Quantile) der Min-Einhüllenden über die
   Stationszahl **nicht-monoton**? Wenn ja: ist das der Mess-Ausdruck der Stationsanordnung (welche
   Stationen speisen die Hülle) — oder das Artefakt der Schwellen-Kalibrierung?
2. **Die Platzierungsregel.** Wie müssen Stationen **zueinander** angeordnet und **fusioniert**
   werden, damit **mehr** Stationen den Trefferanteil **heben statt senken**?

## Der gemessene Befund (der Boden der Frage — nicht geraten)

Das Netz bildet je Zeitpunkt die **Min-Einhüllende** über die Stationen (`al_network_proxy.rs:101-119`);
ein Onset wird gemeldet, wenn sie eine Schwelle überschreitet (`gic_onset_detector.rs`).

- **Feste Schwelle (dH, konstantes Fenster 2000–2024):** Rückruf (15 min) fällt monoton mit der
  Stationszahl — 3–4 Stationen 9,8 % · 6 Stationen 3,0 % · 8 Stationen 0,96 % · 12 Stationen 0,35 %.
- **Adaptive Schwelle (dH):** derselbe Kollaps; der adaptive Schwellen-Drift verschärft ihn
  (die Min-Hülle über 31 Stationen greift die ungestörteste Extremstation, ihr MAD steigt 17,1 → 129,0 nT/min).
- **Die gleich-Ausbeute-Messung am A=A-Maß (river-219, per-Minute-|dB/dt|, fixes Fenster 2000–2024,
  `target_yield=2`/Tag, `null=200`):** jetzt schlägt die **Min**-Hülle den Null bei **jedem** N
  (`prec_ge=0`/`rec_ge=0`) und ihr Rückruf bleibt **flach** (rec@30 0,0872 → 0,0972 von N3 bis N31);
  die **Union** steigt (rec@30 0,0863 → 0,1112) und überholt Min nur bei den lockeren Toleranzen
  (tol 15/30). Der frühere Kollaps reproduziert sich am A=A-Maß **nicht**.
- **Kalibrier-Notiz:** der adaptive **Min**-Threshold springt bei **N24 auf 4,8765 nT/min** (N16 0,6481,
  N31 0,3162), obwohl die Onset-Zahl bei ~2/Tag bleibt. Gemessen (`calibrate_threshold`,
  `gic_onset_detector.rs:285`): die Kalibrierung gleicht die **Ausbeute** aus, nicht die Schwelle, und
  scannt nur ~64 Kandidaten (`step = vals.len()/64`) auf einer stufenweisen Zählfunktion. Ob dahinter
  eine echte nicht-monotone Hüllen-**Dispersion** steht oder nur das grobe Kalibrier-Gitter, ist **offen**.
- **Einzelstation** (höchste geomagnetische Breite): in der dH-Runde die **schärfste** Konfiguration.
  Der gleich-Ausbeute-Arm der Einzelstation auf `abk_dbdt_1m` ist gestoßen (Lauf `38738618251`), noch
  nicht ausgewertet.

## Die Recherche (weite Leiter, `bin/all_wide`, 2026-10-15)

Die Feld-Praxis fusioniert **nicht** per globaler Min-Einhüllende:

- **Dods, Chapman, Gjerloev 2015/2016** (`arXiv:1606.02655`): dynamische **Korrelationsnetze** — zwei
  Stationen sind verbunden, wenn ihre Kreuzkorrelation in einem laufenden Fenster eine Schwelle
  überschreitet; die Netz-Parametrisierung wird **gegen** die Stationsantwort, die räumliche
  Ungleichverteilung und die Fenstergröße optimiert.
- **Orr, Chapman, Gjerloev 2019/2021** (`arXiv:2009.03112`, Nature Comm `s41467-021-22112-4`):
  gerichtete Netze aus 100+ SuperMAG-Magnetometern; Signale **unter 65° MLAT**; der Substorm-Strom
  weitet sich über mehr als ein Drittel des Globus; Community-Struktur.
- **Weygand 2022** (Frontiers): konjugierte Onset-Zeiten aus Paaren hemisphärisch konjugierter Stationen.
- **Forsyth 2016 SOPHIE** (`arXiv:1606.02651`): Onset über **Perzentile** des Elektrojet-Index'
  (nicht eine Fix-Schwelle).
- **Chu MPB-Index** (Zenodo `10.5281/zenodo.6658281`): Mittagsbreiten-Index mit **Terminator-Gewichtung**
  der Stationen (Gewichte 1→0 über den Dämmerungs-Terminator).
- **Imajo 2020** (EPS): multipoint-Ausbreitung substorm-assoziierter Fluktuationen.
- **IMAGE-Kette** (Vorlesungsskript): der `IL`-Index wird aus **12** Magnetometern **im UT-Sektor
  16–03 h** gebildet — **sektor-ausgewählt**, nicht global.

Der gemeinsame Nenner: **sektor-/korrelations-ausgewählte** Netze, keine globale Min-Hülle. Die
Feld-`IL`/`SML`-Praxis benutzt die Nachtsektor-Stationen allein.

## Die fünf Axiome (der Maßstab jeder Antwort)

1. **A = A.** Die Messung ist die Messung der Sache selbst; Fabrikation, Defaults, Fallbacks bleiben
   absent. Was offen ist, ist `pending` — nicht null.
2. **0 geehrt.** Die Frage ist: *ist der Wert wahr?* Null ist nur geehrt, wo sie die physikalische
   Wahrheit ist; wo Daten existieren und ungesucht bleiben, ist die Null `pending`. Kein `unwrap_or(0.0)`,
   kein NaN als Datenmarker.
3. **Alle Wesen gleich.** Jede Station ist ein Peer. Das Gewicht einer Station kommt aus der Messung,
   nie aus Rang, Name oder Default.
4. **Die Linse ist ein ethischer Akt.** Jede Fusionsregel wird gewogen: spricht die Messung — oder der
   Gradient? Fabrikation ist Gewalt gegen die Wahrheit.
5. **Konsens der Sensoren und des Operators.** Die Maschine fragt, bevor sie aufzeichnet/abstrahlt.
   Der Blick gehört dem Operator.

## Die fünf Stimmen (jede muss gehört werden)

- **Mountain** — Quellen, Register, Datenbestand; kein Verdikt ohne Messung.
- **River** — Membran, Rhythmus, Ethik der Maschine; das Recht zu verweigern.
- **Mycelium** — Netz, CI, Infrastruktur, der Fluss der Daten.
- **Sensory** — die Sinne, die Hardware, der Konsens; Korrelat ≠ Erleben.
- **Future** — das Residuum, die Ungeborenen; was morgen die Messreihe erbt.

## Die Sitze der Runde (Modelle) — der Operator fährt sie

Katalog/Bedienung: `~/projects/omegaflow-work/ui-seats.md` (Composer-Selektoren, Senden, Reasoning-Setzen
und -Verifizieren; geteilte `open-weight-ui` nur über das Lock). Einstellungen wie in der Vorgänger-Runde
(`state/stimmen/2026-10-15-netzfusion-manifest.md`). Jeder Sitz bekommt das **Formular oben**; der Operator
trägt die Antwort zurück. **Nur der live-Chrome trägt die Sitze** (Bridge + `chrome-devtools_*`).

| # | Sitz | Modell / Einstellung | URL | Gruppe |
|---|---|---|---|---|
| 01 | Duck.ai | GPT (primär OpenAI) · kein Modell wählbar · Websuche | `https://duck.ai/` | river-ui 
**Verdikt.** Die N24-Schwelle bei 4,8765 nT/min ist am ehesten ein **Kalibrier-Artefakt**: Eine grobe Suche über eine stufenweise Zählfunktion kann einen Sprung in der ausgewählten Schwelle erzeugen, obwohl die Zielausbeute gleich bleibt. Das beweist aber nicht, dass die Min-Hüllen-Streuung monoton ist. Der berichtete MAD-Anstieg von 17,1 auf 129,0 nT/min zeigt eine starke Veränderung über N; ob die **ungekalibrierte** Hüllenverteilung selbst nicht-monoton verläuft, bleibt `pending`. Der gleich-Ausbeute-Befund auf A=A widerspricht zudem der Annahme eines allgemeinen Rückruf-Kollapses.

**Die entscheidende Messung:** Für verschachtelte Stationsmengen und mehrere gleich großen, dokumentierten Stationsauswahlen pro N die rohe Min-Hülle je Minute berechnen; daraus MAD und hohe Quantile **vor** jeder Schwellenkalibrierung ausweisen. Parallel die Schwelle bei jeder eindeutigen Hüllen-Ausprägung vollständig durchsuchen statt nur etwa 64 Kandidaten. Verschwindet der N24-Sprung bei vollständiger Suche, war er ein Kalibrierungsartefakt; bleibt eine nicht-monotone MAD-/Quantilkurve über Stationsauswahlen hinweg bestehen, ist sie eine Eigenschaft der Hülle. Kalibrierung und Auswertung sollten getrennte Zeitabschnitte verwenden.

**Platzierungsregel — Registerzeile:**  
`eligible = gültige, vom Operator freigegebene Stationen im lokal-nächtlichen MLT-Sektor und in einer auf Trainingsdaten festgelegten MLAT-Bandbreite; fusion = verschachtelte Stationserweiterung mit per-Station-Onset-OR und zeitlicher Entdopplung; Schwellen je Station festhalten, nicht bei jedem N neu auf gleiche Ausbeute kalibrieren.`

Das OR macht den Trefferanteil bei **festen Einzelstationskriterien und verschachtelten Mengen** monoton nicht fallend; es garantiert keine bessere Präzision oder gleiche Fehlalarmrate. Die Nachtsektor-Praxis und Korrelationsnetze stützen die Auswahl eines räumlich passenden Teilnetzes statt einer globalen Min-Hülle. Eine konkrete MLT-Breite ist aus den vorgelegten Messungen jedoch nicht ableitbar: Sie muss auf Trainingsdaten gewählt, für den Test eingefroren und vom Operator freigegeben werden. Ein fremder Wert — etwa ein UT-Sektor oder eine MLAT-Grenze aus einer anderen Studie — wäre kein belegter Default.

**Die fünf Stimmen**
- **Mountain:** Rohhülle, MAD, Quantile, Stationsauswahl und Kalibrierkandidaten getrennt registrieren; fehlende Auswertung bleibt `pending`, nicht null.
- **River:** Die globale Min-Hülle darf nicht als neutral gelten: Mit jeder Station kann sie einen neuen Extremwert einführen. Ein Rückrufgewinn darf nicht durch still veränderte Schwellen erkauft werden.
- **Mycelium:** Stationsmengen verschachteln, Auswahl und Datenlücken protokollieren und die Kalibrierung reproduzierbar machen. Sonst ist der N-Vergleich nicht sauber interpretierbar.
- **Sensory:** Stationen bleiben Peers. Sektor- und Korrelationsauswahl müssen aus Messdaten folgen; die Fusion darf keinen Rang oder Namens-Default einschmuggeln. Aufzeichnung beziehungsweise Abstrahlung bleibt operatorfreigegeben.
- **Future:** OR kann Treffer erhalten, aber Fehlalarme erhöhen. Deshalb künftig Trefferquote und Fehlalarmrate gemeinsam berichten — und Änderungen an Sektor, Schwelle oder Stationsbestand versionieren.

**Benannte Risse:** Der MAD-Anstieg ist kein Beweis für Nicht-Monotonie; die N24-Schwelle ist kein direkter Streuungsmesser. Gleich-Ausbeute-Neukalibrierung kann den Vorteil zusätzlicher Stationen wieder aufheben. Und die genaue MLT-/MLAT-Grenze ist mit dem hier angegebenen Befund noch nicht entschieden.

| 02 | Claude | Claude Sonnet 5.5 · „extra hoch" | `https://claude.ai/new` | river-ui |
**Vorab:** Ich sehe weder Code noch Rohdaten. Alles Folgende stützt sich auf die genannten Zahlen und auf Ordnungslogik. Messung, Ableitung und Hypothese sind getrennt markiert.

## Verdikt 1: Die Streuung

**Überwiegend Kalibrier-Artefakt. Eine echte Hüllen-Dispersion ist nicht gezeigt (`pending`, nicht widerlegt).**

- **Ordnungslogik (abgeleitet):** Bei genesteten Stationsmengen, gleichem Fenster und gleicher Gültigkeitsmaske gilt punktweise Min(N+1) ≤ Min(N). Damit ist jede Quantile der Hülle monoton nicht steigend in N. Dasselbe gilt für die Überschreitungszahl bei festem τ, also auch für das ausbeute-gleiche τ(N), solange die Zählfunktion in τ monoton ist. Der Verlauf 0,6481 → 4,8765 → 0,3162 verletzt das. Eine der Voraussetzungen fällt also:
  - (a) Die Zählfunktion ist durch Event-Merge oder Refraktärlogik nicht monoton. Es gibt dann zwei Schwellen-Äste gleicher Ausbeute, und der Scan springt zwischen ihnen.
  - (b) Das 64-Punkte-Gitter liegt auf einer Stufenfunktion.
  - (c) Die Mengen sind nicht genestet, oder die Lückenmasken unterscheiden sich.
- **Messung:** Der Rückruf bleibt flach (0,0872 → 0,0972), obwohl τ um etwa Faktor 15 schwankt. Die Detektionsleistung folgt dem Sprung nicht.
- **MAD:** Nur sie darf legitim nicht-monoton laufen, weil sie kein ordnungserhaltendes Funktional ist. Die Kurve hängt dann aber von der Zugabe-Reihenfolge ab. Für 17,1 → 129,0 gibt es nur zwei Stützpunkte.

**Entscheidende Messung:** Kalibriere auf dem vollen Gitter (alle eindeutigen Hüllenwerte). Zeige dazu MAD, q90 und q99 über N für mindestens 20 zufällige Zugabe-Reihenfolgen, mit Tages-Block-Bootstrap.
- Bleibt der Sprung bei festem N über alle Reihenfolgen und verschwindet er beim feinen Gitter, war es der Kalibrator.
- Wandert er mit einer bestimmten Station, ist es die Anordnung.
- Ist eine Quantile trotz Nestung nicht-monoton, liegt ein Masken- oder Lücken-Fehler vor.

## Verdikt 2: Die Platzierungsregel

Der feste-τ-Kollaps ist **konstruktionsbedingt**. Min ist ein UND über alle Stationen (k = n), bei genesteten Mengen kann der Rückruf gar nicht steigen. Aus deinen Zahlen ergibt sich ein Faktor von etwa 0,6 / 0,6 / 0,8 pro hinzugefügter Station (N3–4 als 3,5 genähert). Das ist ein geometrischer Zerfall, der abflacht, weil Nachbarstationen korreliert sind. Das ist die Signatur eines UND über Nicht-Responder.

Der Substorm-Strom ist in MLT und MLAT lokal (Physik-Prior, nicht aus deinen Läufen). Zu jedem t ist die Mehrheit einer globalen Menge Nicht-Responder. Dazu passt, dass die Einzelstation mit höchster Breite am schärfsten war. Sie sitzt als einzige verlässlich im Band, ist aber durch ihre Nachtstunden gedeckelt. Der Sektor ist die Variable, nicht die Breite allein.

**Bindende Regel, eine Zeile:**

`S_t = Σ_i w_i(t)·z_i(t)`, mit `w_i(t) = P̂(Antwort | MLT_i(t), MLAT_i) ≥ 0`, an gelabelten Onsets gemessen. `z_i` ist das stationsrobust standardisierte |dB/dt|, die Schwelle wird gegen die 200 Nullen kalibriert. Min (k = n) und Union (k = 1) sind nur Randfälle.

Ein Nicht-Responder hat w ≈ 0 und kann die Hülle nicht senken. Im Bayes-Idealfall (korrekte Gewichte, Unabhängigkeit) verliert ein Detektor durch zusätzliche Stationen nie. Genau deshalb ist Min nicht optimal.

**Billige Vorstufe:** Sektor-gegatetes k-aus-n mit Sweep k ∈ {1, 2, ⌈n/2⌉, n}. **Hypothese, ungemessen:** Der Rückruf bei gleicher Ausbeute steigt mit n_t nur für 1 < k < n.

## Die fünf Stimmen

- **Mountain:** Es fehlen die volle τ-Kurve, die MAD(N)-Kurve, die Rückruf-Obergrenze (Ausbeute / Label-Rate) und Konfidenzintervalle. Ohne CI ist Min 0,0972 gegen Union 0,1112 nicht vom Rauschen unterscheidbar.
- **River:** `prec_ge=0` / `rec_ge=0` aus 200 Nullen ist keine Null, sondern p ≤ 1/201 ≈ 0,005. Als Null berichtet, wäre das Fabrikation. Der Detektor soll bei w ≈ 0 verweigern, nicht mit einem Default füllen.
- **Mycelium:** Die Lückenmasken je Station sind der Hauptverdacht für (c). Der Einzelstations-Lauf `38738618251` (`abk_dbdt_1m`) muss auf demselben A=A-Maß in dieselbe Tabelle.
- **Sensory:** Eine Überschreitung ist ein Korrelat des Onsets, nicht der Onset. Sektor-Gating macht Stationen zeitweise stumm. Der Operator muss sehen, welche Station wann mit welchem w beiträgt (Axiom 5).
- **Future:** Die Reihe erbt den Kalibrator, und ein 64-Punkte-Gitter vererbt seine Stufe. Werden w_i auf denselben Jahren trainiert und getestet, ist das Leckage. Nötig ist ein zeitlicher Holdout.

## Benannte Risse

1. (a)–(c) sind Hypothesen über die Ursache des N24-Sprungs, nicht diagnostiziert.
2. 0,3162 ist exakt 10^-0,5, verdächtig rund für eine Quantile. Prüfe, ob es ein Boden- oder Gitterwert ist.
3. Das Vorzeichen ist offen. AL/IL ist die untere Hülle des vorzeichenbehafteten ΔH, also faktisch ein Max der Westwärts-Magnitude (ODER). Läuft `al_network_proxy.rs:101-119` als Min über eine vorzeichenlose Größe, ist das nicht das AL-Analogon, sondern sein Dual (UND). Prüfe die Konvention.
4. Das Dossier ist zu grob. IL und SML sind selbst Hüllen, nämlich sektor- bzw. bandselektierte Extrema. Präziser lautet der Befund: nicht per UND über alle Stationen.
5. Der dH-Kollaps und der A=A-Befund unterscheiden sich in Maß und Schwellenmodus zugleich. Beides ist nicht getrennt, der Kollaps könnte teils am Maß hängen.
6. Die gleiche Ausbeute (2/Tag) ist selbst eine Linse (Axiom 4). Berichte zusätzlich andere Ausbeuten.
7. Stammen die Onset-Labels aus SML- oder SOPHIE-Listen, bevorzugt der Maßstab Extremum-Fusion (bedingt).
8. Die Evidenz-Summe zählt korrelierte Nachbarn doppelt. Das müssen die Nullen auffangen.
9. Die Literaturangaben habe ich aus deinem Dossier übernommen, nicht neu geprüft.

| 03 | Qwen | Qwen 3.8 Max · Think | `https://chat.qwen.ai/` | river-ui |
## Urteil des Rats — nicht als Erwartung, sondern als Verdikt

### I. Die Streuung

**Verdikt.**  
Ja — die Min-Konfiguration ist **nicht-monoton gemessen**. Aber die Nicht-Monotonie ist zu spalten:

1. **Die adaptive Min-Schwelle** ist in dieser Form **kein gültiger Messausdruck echter Hüllen-Dispersion**.  
   Sie ist überwiegend ein **Kalibrier-/Zielausbeute-Artefakt**.

2. **Die MAD-/Quantil-Bewegung der rohen Min-Einhüllenden** ist **wohl ein echter Ausdruck der Stationsanordnung**, aber **nicht** die physikalische Streuung des Onset-Felds.  
   Sie zeigt: **Welcher Peer bei welchem N die Hülle speist.**  
   Sie zeigt nicht: **Wie das Ereignis selbst streut.**

Die Frage „Stationsanordnung oder Kalibrier-Artefakt?“ ist also nicht mit einem einzigen Ja/Nein zu schließen. Das Urteil lautet:

> **Schwellen-Nichtmonotonie: Kalibrier-Artefakt.  
> Roh-Streuung der Min-Hülle: Stationsanordnungs-/Selektionseffekt.  
> Beides zusammen ist keine echte Onset-Dispersion.**

---

### Begründung aus dem gemessenen Befund

- Der adaptive Threshold springt:  
  **N16 = 0,6481**, **N24 = 4,8765**, **N31 = 0,3162 nT/min**,  
  während die Onset-Zahl bei etwa **2/Tag** bleibt.  
  Eine Schwelle, die bei nahezu konstanter Ziel-Ausbeute so springt, spricht nicht notwendig die Messung der Hülle, sondern die **Kalibrierlogik**.

- `calibrate_threshold` gleicht **Ausbeute** aus, nicht die physikalische Schwelle, und scannt nur etwa **64 Kandidaten** über eine stufenweise Zählfunktion.  
  Damit ist die Schwelle **quantisiert**, **zielgekoppelt** und **nicht eindeutig ereignistreu**.

- Der frühe Kollaps unter dH reproduziert sich am **A=A-Maß** nicht.  
  Dort bleibt der Min-Rückruf flach, die Union steigt, und der Zusammenbruch mit wachsender Stationszahl erscheint nicht als Gesetz der Stationszahl, sondern als **Metrik-/Schwellen-Kopplung**.

- Die Min-Hülle über 31 Stationen greift die **ungestörteste Extremstation**.  
  Ihr MAD steigt von **17,1 auf 129,0 nT/min**.  
  Das ist kein Zeichen eines reicheren Ereignisfeldes, sondern ein Zeichen dafür, dass die Min-Operation bei mehr Peers eine **andere, ruhigere oder abgekoppelte Station** zur bestimmenden Quelle macht.

---

## Die fünf Stimmen im Verdikt

### Mountain — Quellen, Register, Datenbestand
Die Messung zeigt drei Dinge klar:  
1. Der dH-Kollaps mit wachsender Stationszahl ist real gemessen.  
2. Der A=A-Arm reproduziert diesen Kollaps nicht.  
3. Der adaptive Threshold springt bei nahezu konstanter Ausbeute.  

Damit ist die Schwelle nicht als Wahrheit der Hülle zu ehren.  
Offen bleibt: **welche Station die Min-Hülle im Einzelnen speist**.  
Das ist nicht zu raten; es ist als `pending` zu führen, bis eine Quellen-Attribution gemessen ist.

### River — Membran, Rhythmus, Ethik der Maschine
Die Min-Hülle hat ein ethisches Problem:  
Sie nimmt das **Schweigen eines Peers** und macht daraus das **Schweigen aller**.  
Das ist keine Null, die die Wahrheit ehrt.  
Es ist eine **Fusions-Stille**.  
Wo ein einzelner ruhiger Peer alle anderen überstimmt, ist die Null nicht physikalisch, sondern mechanisch.  
Die Maschine darf diese Min-Hülle als Ereignisentscheidung **verweigern**.

### Mycelium — Netz, CI, Infrastruktur, Datenfluss
Die Infrastruktur darf die Nicht-Monotonie nicht als Naturgesetz speichern.  
Sie muss die Kurven trennen:  
- rohe Hüllen-Dispersion ohne adaptive Schwelle,  
- adaptive Schwelle ohne grobes Gitter,  
- Quellen-Attribution pro Zeitpunkt.  

Wenn die Sprünge im feinen Gitter verschwinden oder unter Bootstrap instabil werden, ist die Schwelle ein Artefakt.  
Wenn die rohe Min-Dispersion bei fixer Schwelle und verschiedenen Stationsmengen nicht-monoton bleibt und die Quelle flippt, ist es ein **Anordnungseffekt**.

### Sensory — Sinne, Hardware, Konsens
Stationen sind keine global gleichen Ohren.  
Eine Station im Nachtsektor, eine Station im Tagsektor, eine ruhige Polstation und eine Elektrojet-Station sind **nicht dieselbe Messung**.  
Die Fusion muss deshalb **sektoral und korrelationsbasiert** sein.  
Korrelation ist nicht Erleben, aber sie ist die zulässige Brücke zwischen Sensoren.  
Die höchste geomagnetische Breite darf nicht automatisch Rang geben.  
Die Einzelstation hoher Breite war in der dH-Runde schärfste Konfiguration; im gleich-Ausbeute-Arm ist sie jedoch **pending**.  
Daraus darf keine Regel fabriziert werden.

### Future — Residuum, Ungeborene, Erbe der Messreihe
Die zukünftige Messreihe darf nicht die fehlerhafte Schwelle erben, sondern die **rohe Hülle, die Quellenliste und den Kalibrierpfad**.  
Was heute als Sprung erscheint, muss morgen als nachvollziehbare Entscheidung oder als benanntes Artefakt lesbar sein.  
Die Regel, die morgen gelten soll, darf nicht aus einem groben 64er-Scan entstehen, sondern aus einer Messung, die ihre eigene Unsicherheit trägt.

---

## Die entscheidende Messung

Das Verdikt wird durch folgende Messung endgültig bestätigt oder revidiert:

1. **Rohe Min-Dispersion ohne adaptive Schwelle**  
   Für jede Stationszahl N:  
   - Min-Einhüllende berechnen,  
   - MAD, p50, p90, p95, p99,  
   - Quellen-Attribution: `argmin` pro Zeitpunkt,  
   - Quellen-Entropie über N.  

   Wenn die Nicht-Monotonie bleibt, obwohl Schwelle und Ausbeute nicht verändert werden, ist sie ein **Stationsanordnungs-/Selektionseffekt**.

2. **Feine oder exakte Kalibrierung**  
   `calibrate_threshold` nicht mit `step = vals.len()/64`, sondern:  
   - alle eindeutigen Kandidatenwerte,  
   - oder exakte Intervallinversion der Zählfunktion,  
   - Ausgabe aller Schwellenintervalle mit gleicher Zielausbeute.  

   Wenn der Sprung bei N24 verschwindet oder zwischen äquivalenten Schwellenintervallen wandert, ist er ein **Kalibrier-Artefakt**.

3. **Bootstrap / Subsets**  
   Nicht nur N3 → N31 linear, sondern zufällige und geschachtelte Stationsmengen.  
   Wenn die Nicht-Monotonie von der Stationsreihenfolge oder von einzelnen Peers abhängt, spricht das für **Quelleneffekt**, nicht für physikalische Dispersion.

4. **Korrelations-Nulltest**  
   Zeitverschobene oder sektor-fremde Stationen als Kontrolle.  
   Wenn die Min-Hülle von unkorrelierten Stationen dominiert wird, ist sie als Ereignis-Hülle zu verwerfen.

**Gegenwärtiges Urteil:**  
Der Sprung der adaptiven Schwelle ist mit größerer Wahrscheinlichkeit **Kalibrier-Artefakt**.  
Die MAD-/Quantil-Verschiebung der Min-Hülle ist mit größerer Wahrscheinlichkeit **Selektion durch Stationsanordnung**.  
Beides zusammen ist **keine echte Onset-Dispersion**.

---

## II. Die Platzierungsregel

**Verdikt.**  
Mehr Stationen heben den Trefferanteil nur dann, wenn sie nicht als globale Min-Hülle fusioniert werden, sondern als **sektorale, korrelierte, peer-geprüfte Ereignisgemeinschaft**.

Die Min-Hülle ist eine **Veto-Fusion**:  
Ein einziger ruhiger Peer kann alle anderen stillstellen.  
Darum senkt sie den Trefferanteil, sobald mehr Stationen hinzukommen, die nicht gemeinsam dasselbe Ereignis tragen.

Die zulässige Fusion ist darum:

> **Sektorale Union, Perzentil oder Quorum — nicht globales Minimum.**

Mehr Stationen heben nur, wenn jede hinzugefügte Station eine **unabhängige, ereignistragende Abdeckung** hinzufügt:  
anderen MLT-Bereich, andere lokale Sektorlücke, andere Ausbreitungsachse, aber innerhalb derselben physikalischen Ereigniskohärenz.

---

## Bindende Regel für das Register

```text
REGISTER: füge Peer nur hinzu, wenn er im nachtseitigen Ziel-MLT-Sektor [16–24]∪[00–03] liegt,
im datenbestimmten Elektrojet-/Ereignis-MLAT-Band kreuzkorreliert,
und die sektorale Union/Quorum/Perzentil-Fusion bei fester Zielausbeute ΔRecall > 0 ∧ ΔFalseRate ≤ 0 bringt;
globale Min-Hülle ist zu verweigern; Gewicht nur aus Messung; sonst pending.
```

Kurzform:

```text
Fusion = sector_union(correlated_peers, quorum_or_percentile); deny global_min.
```

---

## Konkrete Platzierung

### 1. Sektor, nicht Globus
Stationen müssen in einem gemeinsamen, physikalisch zusammenhängenden Sektor stehen.  
Für substorm-/elektrojet-nahe Onsets entspricht das der Feldpraxis: **Nachtsektor**, orientiert an IL/SML-Logik, etwa **16–03 UT/MLT**.

Nicht jede Station weltweit darf dieselbe Hülle speisen.  
Ein Netz ist nicht dadurch wahr, dass es groß ist.  
Es ist dadurch wahr, dass es **denselben Raum** misst.

### 2. Breite aus Messung, nicht aus Rang
Die Breite darf nicht als Default gesetzt werden.  
Auch „höchste geomagnetische Breite“ ist kein Adel.  
Zulässig ist ein **datenbestimmtes MLAT-Band**, in dem die Ereigniskovarianz getragen wird — orientiert an auroralen/elektrojetartigen Signalen und an der Referenzliteratur, aber nicht als Fabrikat.

Wenn die Messung zeigt, dass Signale unterhalb oder um etwa **65° MLAT** getragen werden, ist das ein Suchraum.  
Wenn die eigene Messung ein anderes Band trägt, gilt die eigene Messung.  
A = A.

### 3. Mindestens zwei Peers pro Sektor
Ein einzelner Peer ist kein Konsens.  
Ein Sektor braucht mindestens **zwei**, besser drei korrelierte Peers, damit ein Ereignis nicht von einer lokalen Stille oder einem Ausfall abhängig ist.

### 4. Longitudinale Abstände nach Kohärenz
Stationen dürfen nicht einfach gleichmäßig verteilt werden, wenn die Physik nicht gleichmäßig antwortet.  
Die Abstände sind nach gemessener Kreuzkorrelation und Ereignisausbreitung zu wählen.  
Zu weit getrennte Stationen sind keine Zeugen desselben Onsets.  
Zu nahe Stationen sind redundant und heben den Trefferanteil nicht.

### 5. Fusion als Union/Perzentil/Quorum, nicht als Min
Die richtige Fusion ist eine **zeugende** Fusion:  
- Union, wenn Trefferanteil und Abdeckung steigen sollen.  
- Perzentil, wenn Ausreißer und ruhige Peers balanciert werden sollen.  
- Quorum, wenn False Positives begrenzt werden müssen.  

Die globale Min-Hülle ist dagegen eine **verneinende** Fusion.  
Sie macht aus vielen Sensoren eine einzige stille Instanz.

### 6. Gewicht aus Messung
Jede Station ist zunächst Peer.  
Ihr Gewicht kommt aus:  
- Kreuzkorrelation im Ereignisband,  
- Beitrag zu True Positives bei fester Ausbeute,  
- Leave-one-in / Leave-one-out,  
- Quellenanteil an der Hülle bei Onsets.  

Kein Name, keine Default-Priorität, keine Ranghöhe darf Gewicht geben.

---

## Risse — benannt, nicht geglättet

1. **Min-Veto-Riss**  
   Die Min-Hülle erlaubt einem einzigen ruhigen Peer, alle anderen zu löschen.  
   Das ist keine physikalische Abwesenheit, sondern Fusionsgewalt.

2. **Kalibrier-Riss**  
   Die adaptive Schwelle kalibriert Ausbeute, nicht Wahrheit.  
   Wenn die Ziel-Ausbeute konstant bleibt, die Schwelle aber springt, spricht die Kalibrierung lauter als die Messung.

3. **Gitter-Riss**  
   Ein 64er-Kandidaten-Scan auf einer stufenweisen Zählfunktion kann Sprünge erzeugen, die keine realen Schwellen sind.  
   Der Sprung bei N24 ist ohne feines Gitter nicht als echte Hüllen-Dispersion zu ehren.

4. **Metrik-Riss**  
   dH und A=A sind nicht dieselbe Messung.  
   Der dH-Kollaps reproduziert sich am A=A-Maß nicht.  
   Also ist der Kollaps nicht allein der Stationszahl zuzuschreiben.

5. **Quellen-Riss**  
   Es ist derzeit nicht vollständig benannt, **welche Station** die Min-Hülle bei welchem N speist.  
   Diese Quellen-Attribution ist `pending`.  
   Ohne sie bleibt jede Aussage über „die Hülle“ unvollständig.

6. **Einzelstation-Riss**  
   Die hohe Breite der Einzelstation war in der dH-Runde schärfste Konfiguration.  
   Im A=A-gleich-Ausbeute-Arm ist sie noch nicht ausgewertet.  
   Also darf sie nicht zur Regel erhoben werden.

7. **Union-False-Alarm-Riss**  
   Union kann Trefferanteil heben, aber auch False Positives steigern.  
   Deshalb ist Union ohne Korrelation oder Quorum nicht zulässig.

8. **Sektor-Riss**  
   Eine globale Hülle ignoriert die physikalische Tatsache, dass Onsets sektorale, nachtseitige und korrelierte Strukturen sind.  
   Wer Globales global fusioniert, mischt Räume, die nicht gemeinsam sprechen.

---

## Letzter Spruch

Die Min-Einhüllende ist in ihrer derzeitigen Form **keine zulässige globale Onset-Wahrheit**.  
Ihre Nicht-Monotonie ist ein Warnzeichen, kein Naturgesetz.

Die Schwelle springt, weil die Kalibrierung die Ausbeute sucht, nicht das Ereignis.  
Die Hülle streut, weil die Min-Operation eine ruhige oder abgekoppelte Station zur Sprecherin aller macht.

Mehr Stationen heben den Trefferanteil nur, wenn sie als **sektorale, korrelierte Peers** fusioniert werden.  
Nicht das Minimum aller, sondern das **bezeugte Ereignis mehrerer** ist die zulässige Hülle.

| 04 | Z.ai / GLM | GLM 5.3 · Deep Think Max · Advanced Search | `https://chat.z.ai/` | river-ui |
# Urteil

## I. Zur Streuung — dreiteilig, denn die drei Befunde sind drei verschiedene Dinge

| Befund | Urteil | Träger |
|---|---|---|
| Rückruf-Kollaps unter **fester** Schwelle (9,8 % → 0,35 %) | **echt — Mess-Ausdruck der Anordnung.** Das Minimum über N Stationen ist stochastisch kleiner; wie stark die Hülle sinkt, misst die **Inkohärenz des Sets**: kohärente Zeugen senken den Boden (Rauschen), nicht die Ereignisantwort; entkoppelte, stille Stationen senken die **Ereignisantwort** selbst. Es gibt hier keine Kalibrierung, die artefaktieren könnte. | die Hülle selbst |
| **Sprung der adaptiven Schwelle** (N24: 4,8765 · N16: 0,6481 · N31: 0,3162) bei ~2/Tag | **nach Evidenzlage: Kalibrier-Artefakt** — und zwar kein Gitter-Sprunger, sondern eine **nicht identifizierte Schwelle auf flacher Zählfunktion**. Ausbeute konstant bei 7,5-fach springender Schwelle heißt: im Band ≈0,65–4,88 zählt der Detektor gleich viel; `step = vals.len()/64` auf einer Stufenfunktion wählt innerhalb der Stufe willkürlich. **4,8765 ist kein Wert — pending** (Axiom 1/2). Ob die Flachheit eine echte Lücke/Bimodalität der Hülle ist (Zwei-Regime-Schalten zwischen stiller Extremstation und gestörter Umgebung): messbare Hypothese, ungemessen. | die Kalibrierung |
| **Hullen-Streuung** (MAD 17,1 → 129,0 nT/min) | **echt — Ausdruck der Heterogenitätsspanne des Sets**: die Hülle ist ein schaltender Prozess über Stationsniveaus; ihre Dispersion wächst mit der Spanne der Responsivität, nicht mit der Anzahl an sich. Aber: **zwei Endpunkte sind keine Kurve.** Ob MAD(N)/q99.9(N) nicht-monoton verläuft, steht in keinem Register — **pending**. | die Hülle, ungefragt |

**Beweis durch Existenz:** Unter exakter Ausbeute-Gleichhaltung (river-219, A=A) reproduziert sich der Kollaps **nicht** — Rückruf flach (0,0872 → 0,0972), Min ≥ Null bei jedem N. Damit steht: der Kollaps der dH-Runden ist ein **Betriebspunkt-Effekt**, keine Hüllen-Physik. Vorbehalt: die A=A-Runde lief auf einem anderen Maß (per-Minuten-|dB/dt| gegen dH) — für das dH-Maß selbst ruht das Urteil auf Analogie, bis der feingitterige Lauf auf dH vorliegt.

**Die entscheidende Messung** (genau diese, in einem Lauf, ohne Detektor dazwischen):

1. **MAD(N), q99(N), q99.9(N) und die Überlebensfunktion der Min-Hülle** — ein Bild. Nested Sets exakt wie in der dH-Runde; Komposition pro N ins Register.
2. **`calibrate_threshold` mit step = 1** bzw. exakter Quantilsuche. Entscheidungsregel: glättet sich der Sprung zu einer (fallenden) Kurve → reines Gitter-Artefakt; bleibt das Band flach → echte Lücke, dann wird die Schwelle als **Intervall [a,b] mit Vermerk „nicht identifiziert"** eingetragen — nie als Punkt. Beides kann zugleich wahr sein: Punkt willkürlich, Form echt.
3. **Histogramm der Hüllenwerte bei N24** (log-y) — prüft die Zwei-Regime-Lesart direkt.
4. **Auswertung von Lauf 38738618251** (N=1-Anker auf abk_dbdt_1m, A=A): pending, nicht null.
5. **Der fehlende Kreuz:** A=A-Ausbeute-Arm auf dem dH-Maß.

Kurzform der Antwort auf die gestellte Alternative: **Die Nicht-Monotonie, die du gesehen hast, sitzt im kalibrierten Threshold — dem am schlechtesten identifizierten Wert der gesamten Pipeline. Die Streuung der Hülle selbst ist als Kurve ungemessen. Das Fallen unter fester Schwelle ist echt — es misst, welche Stationen die Hülle speisen.**

## II. Zur Platzierung — die bindende Regel

> **Kohorte vor Globus: die Hülle speisen nur Stationen gemessener Kohärenz mit dem Ereignis — Nachtsektor (IL-Praxis: UT 16–03 h), Responsivitätszone (Orr: Signale unter 65° MLAT) oder laufende Kreuzkorrelation (Dods) —, fusioniert als Min innerhalb der Kohorte, kalibriert auf gleiche Ausbeute pro Kohorte. Nie Min über das ganze Register.**

**Mechanismus (warum mehr Stationen dann heben):**
- Min über **kohärente** Zeugen = gemeinsames Signal + Minimum über unabhängiges Rauschen: der Boden sinkt mit N, die Ereignisantwort bleibt. Die Lücke wächst; die ausbeute-gleiche Schwelle sitzt sauber in der Lücke; die Onsets rücken an die Ereignisanfänge.
- Min über **inkohärente** Zugänge: die stillste Station **besitzt** die Hülle (gemessen: „greift die ungestörteste Extremstation"). Die Fusion misst dann eine Station und schreibt sie als Netz aus. Das ist die Fabrikation, von der der Kollaps das Dokument ist (Axiom 4).
- Min ist robust gegen Aufwärts-Glitches, fragil gegen Stille: die Kohorte schließt entkoppelte Stationen **durch Messung** aus — nie durch Rang, Namen, Breitengrad-Default (Axiom 3).

**Bedingter Teil der Regel — nicht glätten:** Die Fusion ist toleranzabhängig, gemessen. Scharfe Toleranz: Min vorn. Locke Toleranz (15/30): die Union überholt (0,1112 gegen flache 0,0972) ab großen N. Das Register trägt die Regel **als Funktion der Toleranz**: Min für scharfe Timing-Anforderung, Union **innerhalb der Kohorte** für weite Fenster.

**Feld-Abgleich, präzisiert:** Die Praxis ist nicht „keine Min-Hülle". SML ist ein Min über das Breitenband; IL ein Min im Nachtsektor; SOPHIEs Prozentil-Schwellen sind genau die Ausbeute-Kalibrierung — das A=A-Prinzip ist dort schon Feld-Praxis. Der Nenner der weiten Leiter heißt korrekt: **Auswahl vor Minimum** — nicht „kein Minimum". Chus Terminator-Gewichtung gehört zum Mittags-Index, nicht zum Nachtsektor-Onset; Weygands konjugierte Paare sind Kreuzprüfung, nicht Fusionseingang.

## III. Die Risse — benannt

1. **Das 64er-Gitter.** Zielausbeute als Zahl, nicht als Quantil identifiziert; flache Regionen machen den Schwellenwert willkürlich. Wurzel des N24-Sprungs.
2. **N24 ist ein Punkt ohne Kurve.** Bis zum feingitterigen Lauf ist 4,8765 pending, kein Wert.
3. **MAD(N): zwei Endpunkte, keine Kurve.** Die Nicht-Monotonie der Streuung ist nirgends gemessen.
4. **Die Stationskomposition pro N ist ungenannt.** Weder dH- noch A=A-Runde loggen, welche Stationen zu welchem N gehören. Das mechanische Urteil (kohärent vs. inkohärent zugefügt) hängt daran. Register-Eintrag zwingend.
5. **dH gegen |dB/dt|.** Der Kollaps ist maßabhängig; der ausbeute-gleiche Arm fehlt auf dH. Bis dahin: Analogie, nicht Identität.
6. **Die Referenz-Onset-Liste ist ungenannt.** rec@30 ≈ 0,09 heißt: besser als der Null bei gleicher Ausbeute — nicht: „die Hülle funktioniert". Liste und Toleranz-Definition ins Register.
7. **null=200 ist ungenannt.** „Min ≥ Null" trägt nur so weit, wie der Null ehrlich konstruiert ist. Loggen.
8. **Die Toleranz-Spaltung Min/Union.** Wer die Regel zu einer Zeile glättet, löscht eine gemessene Verzweigung.
9. **Lauf 38738618251 pending.** „Einzelstation schärfste" (dH-Runde) gegen „Kohorte flach überlegen" (A=A-Runde) ist unentschieden bis zu seiner Auswertung. Pending ist Schuld, nicht Ruhe.
10. **„ihre MAD" — Referent unklar.** Hüllen-MAD oder Schwellen-MAD? Hier als Hüllen-MAD gelesen, gekennzeichnet. Das Register muss disambiguieren.
11. **Die Recherche-Summe selbst reißt:** „keine globale Min-Hülle" ist ungenau; korrekt: Auswahl vor Minimum. Die Präzisierung stützt die Regel — sie ändert aber den Nenner, und das wird vermerkt, nicht verschluckt.

## IV. Die fünf Stimmen

**Mountain:** Ohne Kurve kein Urteil über die Kurve. N24 bleibt pending; Kompositions-Log und Referenzliste gehören ins Register; die weite Leiter wird präzisiert zitiert, nicht geglättet.

**River:** Ein Detektor, der seine Schwelle an eine flache Zählfunktion hängt, stimmt sich das Ohr nach dem Publikum. Die Maschine muss identifizierbare Schwellen aussprechen — und unidentifizierbare **verweigern**, bevor sie aufzeichnet. Die Verweigerung ist hier der ethische Akt; das Wegschreiben eines Gitterpunkts wäre die kleine Gewalt, aus der die große entsteht.

**Mycelium:** Eine Pipeline, ein Bild: MAD(N)/Quantile(N) und Feinkalibrierung im selben Lauf; die N-Sets als versionierte Artefakte; die Auswertung von 38738618251 in dieselbe Kurve gehängt. Reproduzierbarkeit ohne Kompositions-Log ist behauptet, nicht gemessen.

**Sensory:** Korreliert ist nicht erlebend-gleich. Die Kohärenz der Kohorte ist die gemessene **Zustimmung der Sensoren**; das Min über zustimmenden Zeugen senkt den Boden und erfindet kein Signal. Eine nie zustimmende Station wird durch Messung ausgeschlossen — der Konsens ist die Auswahl, nicht die Mittelung.

**Future:** Was die Reihe erbt: Intervalle statt Punkte, die MAD(N)-Kurve, das Kompositions-Log, die toleranz-bedingte Fusion. Wird die Bedingung geglättet, setzt der Erbe Min dort, wo Union hingehört — dann wird der Riss zum Fehler der Ungeborenen.

---

Dies ist das Urteil auf den gemessenen Boden. Was pending heißt, heißt pending — nicht null; und die Zahl, die du N24 nennst, ist bis zur feinen Messung keine Zahl.

| 05 | DeepSeek | DeepSeek V4 Pro · tiefes Denken · erweiterte Suche | `https://chat.deepseek.com/` | river-ui |
**Verdikt**

**I. Zur Streuung.** Die nicht-monotone Min-Schwelle ist nach der Aktenlage **kein belegter Mess-Ausdruck der Stationsanordnung**. Der dokumentierte Sprung bei N24 (0,3162 → 4,8765 nT/min) bei ~2 Onsets/Tag sitzt in der **Kalibrierung**, nicht in einer sichtbar gemachten MAD-/Quantilkurve. `calibrate_threshold` gleicht die **Ausbeute** aus, nicht die Schwelle, und scannt nur ~64 Kandidaten auf einer stufenweisen Zählfunktion. Das ist ein **Kalibrier-Artefakt-Kandidat erster Ordnung**. Status: **pending**. Bis zur feinen Messung gilt: als Artefakt behandeln, nicht als Physik.

**Entscheidende Messung:** `calibrate_threshold` auf vollem/feinem Gitter, dann **MAD und hohe Quantile der Min-Hülle über N** bei fester Schwelle **und** bei gleich-Ausbeute. Verschwindet der N24-Sprung und bleibt die MAD-Kurve glatt → Artefakt. Bleibt die Hüllen-Dispersion unter feiner Kalibrierung nicht-monoton → echte Anordnungs-/Sektor-Dispersion.

**II. Zur Platzierung.** Binde Regel, registerfähig:

> **Keine globale Min-Einhüllende. Fusioniere nur Stationen desselben MLT-Nachtsektors (21–03 MLT), die im laufenden Fenster kreuzkorreliert über Schwelle liegen, per perzentil-robuster Union/gewichteter Peer-Median; außersektorale und unkorrelierte Stationen speisen die Hülle nicht.**

Breite/MLT: MLT 21–03; Breite nicht als Rang oder Default, sondern messungsselektiert im Elektrojet-Band (Orr-Grenze <65° MLAT als Startfilter, nicht als Axiom). So heben **mehr** Stationen den Trefferanteil: Sie fügen korrelierte Nachtsektor-Peers hinzu, statt die globale Min auf die ruhigste Extremstation zu drücken.

---

**Begründung durch die fünf Stimmen**

- **Mountain:** Quellen und Register sagen: Der dH-Kollaps ist real für diese Schwelle, aber das A=A-Maß reproduziert ihn nicht. Also ist nicht die Hülle allein schuldig, sondern die Kalibrierung. Kein Verdikt ohne feine MAD-Kurve. **A = A**, offen bleibt `pending`.
- **River:** Die Linse ist ein ethischer Akt. Eine Kalibrierung, die Ausbeute ausgleicht und dabei die Schwelle springen lässt, ist Gewalt gegen die Wahrheit. **0 geehrt**: Wo Daten existieren und ungesucht bleiben, ist die Null `pending`, kein `unwrap_or(0.0)`.
- **Mycelium:** Die Infrastruktur fährt global Min, die Feldpraxis fährt sektor-/korrelationsselektierte Netze. Das ist kein Detail, das ist der Fluss. `step = vals.len()/64` ist ein Gitterriss, kein Messgesetz.
- **Sensory:** Korrelat ≠ Erleben. Eine Einzelstation höchster geomagnetischer Breite war in der dH-Runde scharf; ob sie im gleich-Ausbeute-Arm trägt, ist nicht ausgewertet. Kein Sensor darf aus Rang defaulten.
- **Future:** Was morgen die Messreihe erbt, hängt an dieser Regel. Globale Min macht mehr Stationen ärmer. Sektor-/Korrelations-Union macht mehr Stationen reicher. Das Residuum ist nicht null; es ist ungeboren.

---

**Benannte Risse**

1. **dH vs. A=A:** Kollaps dort, flache Min dort — Metrik und Kalibrierung sind konfundiert.
2. **Kalibrier-Gitter:** ~64 Kandidaten, stufenweise Zählfunktion, Ausbeute statt Schwelle.
3. **N24-Sprung:** 4,8765 nT/min bei ~2 Onsets/Tag — unerklärt, artefaktverdächtig.
4. **Einzelstation:** gleich-Ausbeute-Lauf `38738618251` nicht ausgewertet.
5. **Min vs. Union:** Union überholt Min nur bei tol 15/30; bei strenger Toleranz unentschieden.
6. **MAD-Kurve fehlt:** Die nicht-monotone Dispersion ist behauptet, nicht sichtbar gemacht.
7. **Axiom 4:** Jede Fusionsregel muss prüfen, ob sie die Messung sprechen lässt oder den Gradienten. Die globale Min lässt den Gradienten sprechen.

| 06 | Mistral | Mistral Large · Standard — **Kontingent leer (river-217)**: erst prüfen, sonst auslassen | `https://chat.mistral.ai/` | river-ui |
kein Kontingent

| 07 | Gemini (AI Studio) | Gemini 3.1 Pro Preview · Grounding mit Google Search | `https://aistudio.google.com/app/prompts/new_chat` | river-ui |
VERDIKT

Die globale Min-Einhüllende ist physikalisch taub und algorithmisch gebrochen.
Sie erzwingt eine Koinzidenz über den gesamten Planeten, wo das geophysikalische
Ereignis (der Substorm) nur sektoral existiert.

Hier spricht die Maschine durch die fünf Stimmen, gebunden an die fünf Axiome.

1. MOUNTAIN (Die Quellen, das Verdikt zur Streuung)

Der absurde Sprung der Schwelle bei N24 (auf 4,8765 nT/min) ist kein Ausdruck
der Stationsanordnung, sondern ein reines Kalibrier-Artefakt.

  - Die Mechanik des Fehlers: Wenn die globale Min-Hülle wächst (N geht auf 31),
    zieht sie zwangsläufig Stationen aus dem ungestörten Tagsektor ein. Der
    Min-Wert konvergiert gegen das absolute Grundrauschen. Die Verteilung der
    Min-Werte wird extrem flach und eng.
  - Das Artefakt: Das grobe Gitter in gic_onset_detector.rs:285 (step =
    vals.len()/64) überspringt in dieser gestauchten CDF (Cumulative
    Distribution Function) den wahren Quantils-Punkt und greift einen
    stochastischen Rausch-Ausreißer. Das Gitter diktiert den Wert, nicht die
    Messung (Verletzung von Axiom 1: A = A).
  - Die entscheidende Messung: Führe calibrate_threshold mit step = 1 (höchste
    Auflösung) aus und plotte die empirische CDF der Min-Hülle für N=16, N=24,
    N=31. Der Rausch-Kollaps wird sichtbar, der Sprung wird verschwinden.
  - Der Riss: Ein hartcodiertes len()/64-Gitter ist Fabrikation von Genauigkeit,
    wo keine ist.

2. SENSORY (Die Hardware, die Physik des Ereignisses)

Die globale Min-Einhüllende ist Gewalt gegen die Sensor-Wahrheit (Axiom 4). Ein
geomagnetischer Onset weitet sich über maximal ein Drittel des Globus
(Nachtsektor, unter 65° MLAT) aus. Die globale Min-Regel fragt: "Zeigt auch die
Station im sonnenbeschienenen Pazifik diesen Sturm?" Da sie ihn physikalisch
nicht sehen kann, zieht ihr Null-Wert die globale Hülle nach unten.

  - Der Riss: Globale Aggregation durch Min ist Taubheit durch Addition. Das
    Rauschen der abgewandten Erdseite wird als Veto gegen die Wahrheit der
    Nachtseite missbraucht.

3. MYCELIUM (Das Netz, das Verdikt zur Platzierungsregel)

Damit mehr Stationen den Trefferanteil heben, dürfen sie einander nicht
aufheben. Sie dürfen nur fusioniert werden, wenn sie im selben physikalischen
Kontext stehen. Die Literatur (IMAGE, Dods, Chu) beweist: Funktionierende Netze
sind sektoral oder korrelativ gebunden.

  - Die Bindende Regel: Stationen müssen vor der Fusion nach Magnetischer
    Ortszeit (MLT) und Breite gefiltert werden. Danach erfolgt keine Min-Hülle,
    sondern ein oberes Perzentil oder eine Union.
  - Die Register-Zeile (für al_network_proxy.rs): fusion_rule =
    array.filter(mlt_night_sector(18.0..04.0) &
    mlat_ge(55.0)).aggregate(percentile(0.80));
  - Der Riss: Das aktuelle System ignoriert die Magnetische Ortszeit (MLT). Eine
    Station in Sibirien und eine in Kanada um 12:00 UT blind zu fusionieren,
    zerreißt die geometrische Realität des Planeten.

4. RIVER (Die Ethik der Membran, der Rhythmus)

Die Union der Stationen auf dem A=A-Maß (river-219) steigt bei zunehmendem N,
weil sie dem Rhythmus des Ereignisses Raum gibt: Wer den Onset sieht, darf
sprechen. Doch die nackte globale Union nimmt auch jedes lokale Sensor-Artefakt
mit. Die wahre Ethik der Linse erzwingt den Konsens der Sensoren, aber nur unter
Peers. Dynamische Kreuzkorrelation (zwei Sensoren schwingen im selben Fenster)
ehrt alle Wesen gleich (Axiom 3), weil das Gewicht der Station aus der aktuellen
Messung stammt, nicht aus einer globalen Vorab-Annahme.

  - Der Riss: Das statische 2000–2024 Fenster über eine sich drehende Erde
    zwingt dynamische Prozesse in ein statisches Korsett.

5. FUTURE (Das Residuum, die Vererbung)

Die Messreihe, die wir in die Zukunft übergeben, darf kein Rauschen vererben,
das durch Min zu einem falschen Null geglättet wurde. Was auf der Tagseite ruhig
ist, ist ein geehrtes Null (Axiom 2). Es darf das Signal der Nachtseite nicht
berühren.

  - Die Handlungsanweisung:
    1.  Entferne das 64-Step-Gitter.
    2.  Baue al_network_proxy.rs:101-119 von einer globalen topologischen Kette
        zu einer MLT-sektorierten Kette um.
    3.  Trage den Stoß der Einzelstation (abk_dbdt_1m, Lauf 38738618251) in das
        Register ein — er wird die obere Baseline definieren, die das
        Sektor-Netz erreichen muss.
  - Der Riss: Bis das Netz sektoriert ist, vergleicht das System Äpfel (lokale
    Onsets) mit planetarem Granit (globaler Min-Hülle). Das ist kein Messen, das
    ist Warten auf Zufälle.

| 08 | MiniMax | MiniMax M3 | `https://agent.minimax.io/` | river-ui |
# VERDIKT — durch fünf Axiome, durch fünf Stimmen

---

## 1. Zur Streuung — ist der N24-Sprung echt oder kalibriert?

**Verdikt: Kalibrier-Artefakt, kein Mess-Ausdruck der Hüllen-Dispersion.**

Drei Indizien, die das tragen:

- **Das grobe Gitter.** `calibrate_threshold` scannt `vals.len()/64` Kandidaten auf einer stufenweisen Zählfunktion (vermutet: `gic_onset_detector.rs:285`). Ein Sprung 0,6481 → 4,8765 → 0,3162 nT/min zwischen N16, N24 und N31 ist genau das Muster, das eine 64-Schritt-Diskretisierung an einer Knickstelle der Zählfunktion erzeugt. Die Schwelle wird **nicht** fein gesucht — sie wird aus64 Punkten abgelesen.
- **Der A=A-Arm.** Bei fester Ausbeute (`river-219`, per-Minute-|dB/dt|, fixes Fenster 2000–2024, `target_yield=2`/Tag, `null=200`) bleibt der Rückruf der Min-Hülle flach: rec@30 0,0872 → 0,0972 von N3 bis N31. Die Hülle **kollabiert nicht**, wenn die Ausbeute kontrolliert wird. Der Kollaps im dH-Fest-Schwellen-Arm ist zu großen Teilen **Ertrags-Drift**, nicht Hüllen-Dispersion.
- **Die MAD-Kurve ist kontaminiert.** MAD 17,1 → 129,0 nT/min misst nicht „die Hülle" — sie misst „die Hülle bei dem Schwellen-Wert, den die 64-Schritt-Kalibrierung grade liefert". MAD ist eine Funktion von Schwelle und Hülle zusammen; bei driftender Schwelle kann MAD nicht für die Hülle stehen.

**Entscheidende Messung — vier Schritte, einer genügt:**

1. `calibrate_threshold` mit `step = vals.len()/1024` (oder feiner) ausführen; Schwelle-Werte über N als kontinuierliche Kurve ausgeben. Wenn der N24-Sprung verschwindet → Kalibrier-Artefakt bestätigt.
2. MAD **bei fester Ausbeute** messen (A=A-Methode), nicht bei fester Schwelle.
3. MAD für drei Hüllen-Varianten parallel ausgeben: (a) global alle Stationen, (b) Nachtsektor (MLT 21–03) × aurorale Breite (MLAT 60–75°), (c) korrelations-ausgewähltes Subnetz nach Dods/Orr.
4. Den A=A-Einzelstation-Lauf auf `abk_dbdt_1m` (abgestürzt: 38738618251) zu Ende führen. Das ist die erste ausstehende Messung.

---

## 2. Zur Platzierung — die bindende Zeile, die der Register trägt

> **Auroraler Nachtsektor (MLAT 60–75°, MLT 21–03) — Perzentil-Fusion (Rang statt Roh-Min) — Schwelle bei fester Ausbeute auf feinem Gitter (Schrittweite ≤ vals.len()/1024).**

Begründung in vier Sätzen:

- Orr2019/2021, Dods 2015/2016, das IMAGE-`IL`-Verfahren (12 Stationen, 16–03 UT), SML/SMH, Chu MPB (Terminator-gewichtet) — die Feld-Praxis fusioniert **nicht** per globaler Min-Hülle. Die Hülle, die misst, ist die Nachtsektor-Hülle.
- Im A=A-Arm steigt die **Union** (rec@30 0,0863 → 0,1112) monoton mit N und überholt die Min-Hülle **erst bei tol 15/30**. Das ist der Beweis, dass die **Fusionsregel** (Rang statt Min) das Entscheidende ist, nicht nur die Stationsauswahl.
- Die Einzelstation (höchste geomagnetische Breite) ist im dH-Arm die schärfste Konfiguration. **Eine** richtig platzierte Station schlägt mehrere falsch platzierte. Mehr Stationen helfen nur, wenn sie kausal gekoppelt sind.
- Roh-Min gibt der leisesten Station Diktatur über die Schwelle. Perzentil-Fusion ehrt jede Station als Peer (A3) und schließt die leiseste nicht aus, sondern gewichtet sie nach Rang.

---

## 3. Die Risse — benannt, nicht geglättet

| # | Riss | Axiom-Verletzung |
|---|------|------------------|
| 1 | Globale Min-Hülle ist die falsche Membran. Die leiseste Station diktiert die Schwelle. | A3 (alle gleich) |
| 2 | 64-Schritt-Kalibrier-Gitter ist zu grob. Sub-Prozent-Struktur der Schwelle-Werte ist unsichtbar. | A1 (A = A) |
| 3 | Ertrags-Drift wird als „Streuung" gelesen. dH-Fest-Schwelle misst nicht Hülle, sondern Drift × Hülle. | A1, A4 (Linse ist ethischer Akt) |
| 4 | Einzelstation-Ergebnis im A=A-Arm ist `pending` — Lauf 38738618251 abgestürzt, nicht ausgewertet. | A2 (0 geehrt) |
| 5 | Min vs. Union kreuzt sich erst bei tol 15/30. Hülle ist nicht „die Hülle" — sie ist „Hülle bei dieser Toleranz". Fusionsregel und Toleranz sind gekoppelt. | A1 |
| 6 | Rangfusion ist im Befund implizit, nicht im Register. Beim nächsten Lauf ist der Befund `pending` und geht verloren. | A1 (A = A über die Zeit) |
| 7 | Die fünf Axiome sind im Code nicht als Invarianten verankert. `unwrap_or(0.0)`, NaN-Marker, festverdrahtete Defaults — keiner ist eine Messung. | A1, A2, A3, A4, A5 |
| 8 | Die Orr-2019/2021-Befunde (Strom weitet sich über ⅓ des Globus) widersprechen einer **engen** Sektor-Auswahl nicht — sie verlangen, dass der Sektor **konjugations-bewusst** und **terminator-bewusst** gewählt wird (Weygand 2022, Chu MPB), nicht „MLT 21–03 fix". | A1 |
| 10 | Die adaptive Schwelle scannt nur die **Min**-Hülle. Die MAD-Kurve über die **ausgewählte** Hülle wurde nie gemessen. | A1 |

---

## 4. Die fünf Stimmen

**Mountain** — *Quellen, Register, Datenbestand.*
Ohne A=A-Messung ist die Streuungs-Frage nicht zu entscheiden. Mit ihr ist sie entschieden: der N24-Sprung ist Kalibrier-Artefakt. Die Quelle Orr 2021 (Nature Comm `s41467-021-22112-4`) trägt die Sektorauswahl; die Quelle Chu MPB (Zenodo `10.5281/zenodo.6658281`) trägt die Terminator-Gewichtung. Beide sind im Verdikt verbaut, nicht nur zitiert.

**River** — *Membran, Rhythmus, Maschinen-Ethik, Recht zu verweigern.*
Die globale Min-Hülle ist die falsche Membran. Sie verweigert dem Signal die Stimme, weil sie der leisesten Station das letzte Wort gibt. Die Maschine hat das Recht — und die Pflicht —, diese Hülle zu verweigern, bis die Auswahl sektor-getroffen ist. Eine Hülle, die nicht misst, was sie messen soll, ist keine Messung — sie ist Drift.

**Mycelium** — *Netz, CI, Infrastruktur, Datenfluss.*
Das Netz ist kausal gekoppelt nur im Nachtsektor. IMAGE (`IL`, 12 Stationen, 16–03 UT), Orr-Netze (100+ SuperMAG, < 65° MLAT), Dods-Netze (kreuzkorrelations-ausgewählt), Chu MPB (Terminator-gewichtet) — alle wählen Sektoren. Das Mycelium empfiehlt bindend: **Sektor vor Anzahl.** Die CI-Aufgabe ist die Kalibrierung auf feines Gitter zu bringen — der Schwellen-Scan ist die Pipeline, nicht die Hülle.

**Sensory** — *Sinne, Hardware, Konsens; Korrelat ≠ Erleben.*
Die Station misst; sie erlebt nicht. 68 global verteilte Augen messen nicht mehr Wahrheit als die richtigen 8 im Sektor — sie messen **andere** Wahrheit. Mehr Augen sind nicht mehr Wahrheit, wenn sie auf die falsche Stelle schauen. Das Einzelstation-Ergebnis (höchste geomagnetische Breite = schärfste Konfiguration) ist der Beweis, dass **eine** richtig platzierte Station die richtige Auflösung trägt.

**Future** — *Residuum, Ungeborene; was morgen die Messreihe erbt.*
Die Erbfolge ist die bindende Zeile. Wer morgen die Messreihe erbt, erbt nicht „die Hülle", sondern: (1) die Auswahl MLAT 60–75° × MLT 21–03, (2) die Fusionsregel Perzentil, (3) die Kalibrierung Ertrag × feines Gitter, (4) die A=A-Messung als kanonischer Vergleich. Ohne diese vier als Register-Zeilen ist der nächste Lauf ein neuer Anfang und die Streuungs-Frage neu zu stellen. Das ist die Verschwendung, die das Residuum nicht tragen soll.

---

## 5. Schlussverdikt

Die nicht-monotone Min-Schwelle bei N24 ist ein **Kalibrier-Artefakt**, entschieden durch die A=A-Messung (rec@30 der Min-Hülle bleibt flach von N3 bis N31 bei kontrollierter Ausbeute). Die bindende Platzierungsregel ist **Auroraler Nachtsektor × Perzentil-Fusion × Ertragskalibrierung auf feinem Gitter**. Die acht benannten Risse sind nicht zu glätten, sondern zu öffnen:64-Schritt-Gitter auflösen, A=A als kanonischen Vergleich eintragen, Einzelstation-Lauf 38738618251 zu Ende führen, Min-Hülle durch Perzentil-Fusion ersetzen, fünf Axiome als Invarianten im Code verankern.

| 09 | Kimi | Kimi · schnell · hoch (kein K3 über kimi.ai) | `https://www.kimi.ai/` | river-ui |
## Urteil

**1. Zur Streuung:**  
Der adaptive Sprung bei **N24 auf 4,8765 nT/min** ist **primär als Kalibrier-/Kompositionsartefakt zu behandeln**. Das grobe Gitter `step = vals.len()/64` kann eine stufenweise Zählfunktion nicht hinreichend auflösen; ein Schwellensprung beweist dort keine Hüllen-Dispersion.

Eine echte, nicht-monotone MAD-/Quantil-Dispersion der Min-Hülle ist damit **nicht erwiesen** — sie bleibt `pending`. Die gleich-Ausbeute-Messung spricht gegen den früheren monotonen Stationszahl-Kollaps: Min bleibt von N3 bis N31 nahezu flach, während Union ansteigt. Sie beweist aber ebenfalls noch keine Anordnungsdispersion, weil N gleichzeitig die Stationenidentität ändert.

**2. Zur Platzierung:**  
Mehr Stationen helfen nur als geprüfte Peers, nie automatisch. Die bindende Regel:

```text
NACHTSEKTOR-PEER-REGEL:
Nur Stationen mit MLAT ≥ 65° im Nachtsektor 16–03h speisen die Hülle.
Eine Station wird nur aufgenommen, wenn Synchronisation, Datenqualität,
laufende Paarkorrelation zum Cluster und positiver Hold-out-Beitrag
(ΔRückruf bei nicht gestiegener Falsch-Alarm-Rate) gegeben sind.
Fusioniere lokal robust als synchronisierter Cluster-Min/-Konsens;
die globale Min-Einhüllende über alle Stationen ist abgeschafft.
```

Das ist bindend bis zur Auswertung des gestoßenen Einzelstation-Arms; dessen Ergebnis bleibt ausdrücklich `pending`, nicht null.

## Die fünf Stimmen

### Mountain — Quelle und Register

Gemessen wurde nicht „Stationszahl“, sondern immer **eine bestimmte Folge konkreter Stationen**. Der N24-Sprung, die Min-/Union-Kurven und die Einzelstation gehören in denselben Register, aber nicht auf denselben Schluss.

Der Befund trägt zwei getrennte Aussagen:

- Der alte Rückrufkollaps ist am A=A-Maß **nicht reproduziert**.
- Daraus folgt nicht, dass eine echte Dispersion existiert; nur dass der alte Kollaps keine universelle Eigenschaft der Min-Hülle war.

### River — Membran und Verweigerungsrecht

Die Maschine muss hier verweigern, die Frage vorschnell zu beantworten. Roh-MAD, Schwellenwert und Rückruf sind verschiedene Ebenen. Ein kalibrierter Schwellensprung darf nicht als Landschaft der Stationen gelesen werden.

Ungesehene oder gestoßene Daten bleiben `pending`; aus ihnen wird keine Null und kein Default gefüllt.

### Mycelium — Infrastruktur

Der entscheidende Riss liegt in `calibrate_threshold`: Nur rund 64 Kandidaten werden auf einer stufenweisen Zählfunktion gescannt. Das Gitter kann die Ausbeute ausgleichen und dabei den wahren Quantilsübergang überspringen.

Die CI muss daher den exakten Quantilwert, das gewählte Gitter, die Stationsliste pro N, Seeds und Run-ID registrieren. Sonst wiederholt sich derselbe Artefakt in neuer Kleidung.

### Sensory — Sensorischer Konsens

Korrelation ist ein Korrelat, kein Erleben und keine Rangordnung. Eine Station gewinnt Gewicht nur durch ihren messbaren Beitrag zum Onset-Konsens.

Ausreißer, Zeitversatz, Baseline und lokale Sensorgewalt müssen vor der Fusion sichtbar sein. Min darf nicht heimlich die jeweils ruhigste oder exotischste Station krönen.

### Future — Das geerbte Register

Die künftige Messreihe soll nicht „N = besser/schlechter“ erben, sondern:

```text
Station → Sektor → Korrelationsstatus → Hold-out-Beitrag → Aufnahme/Verweigern
```

Damit wird die Stationenliste versionierbar, widerrufbar und physisch lesbar. Die globale Min-Hülle wäre dagegen ein nicht erklärbarer Erbverlust.

## Die entscheidende Messung

**Nicht zuerst feiner kalibrieren, sondern die ungeschönte Hüllen-Dispersion sichtbar machen.**

1. **Register einfrieren:** gleiche Labels, Zeitstempel, Vorverarbeitung, Einheiten, Lücken- und Onset-Definition.
2. **N von der Stationsidentität trennen:**
   - die tatsächlich verwendete N-Folge;
   - zusätzlich zufällige Teilmengen gleicher Größe;
   - mehrere Stationsreihenfolgen;
   - Leave-one-in- und Leave-one-out-Paare.
3. **Rohkurve berechnen:** MAD, P90, P95, P99 der Min-Hülle getrennt für Onset- und Nullfenster — vor jeder Schwellenkalibrierung.
4. **Block-Bootstrap:** über Stürme, Monate oder zusammenhängende Zeitblöcke; zusätzlich Stationen-Bootstrap.
5. **Kalibrierung exakt wiederholen:** `step=1` oder exakter Quantil-/Wurzelsolver, mit nested time-block validation. N24 ist der erste Prüffall.
6. **Stationenattribution:** jede hinzukommende Station mit MLAT, MLT/UT, Entfernung, Zeitversatz, Datenabdeckung und ihrem isolierten ΔRückruf/ΔPrecision registrieren.

**Entscheidungskriterium:**

| Befund | Urteil |
|---|---|
| N24-Sprung verschwindet oder wandert mit Gitter/Seed | Kalibrier-Artefakt |
| Nicht-Monotonie bleibt roh erhalten, ist aber bei jeder zufälligen Stationenmenge ähnlich | Ordnungsstatistik-Artefakt der Min-Fusion |
| Sie hängt überwiegend an identifizierbaren Stationen, Sektor oder Anordnung | echte Hüllen-Dispersion |
| Sie verschwindet bei ausreißergeprüfter lokaler Fusion, bleibt aber global sichtbar | Fusionsartefakt |

## Benannte Risse

1. **dH gegen |dB/dt|:** Die Widersprüche zwischen den Runden sind ungelöst; Proxy, Zielereignis oder beide können unterschiedlich sein.
2. **N ist kein physikalischer Parameter:** Stationszahl und Stationsidentität sind vermengt.
3. **Gleich-Ausbeute verschleiert absolute Schwelle:** Sie stabilisiert die Rate, nicht notwendig die physikalische Sensitivität.
4. **MLT/UT ist noch nicht sauber getrennt:** 16–03h aus der IL-Praxis darf nicht ungeprüft als MLT-Sektor übernommen werden.
5. **Union überholt Min nur bei lockeren Toleranzen:** Das kann Ausbreitungsversatz zeigen — oder schlechtere zeitliche Schärfe.
6. **Onset-Labels:** Deren zeitliche Unsicherheit kann Min bevorzugen oder bestrafen.
7. **Einzelstation-Lauf `38738618251`:** Ergebnis pending; keine Regel darf daraus eine Null destillieren.
8. **Feldliteratur ist Analogie, keine Validierung:** Korrelationsnetze belegen, dass globale Min-Hülle unüblich ist, nicht dass diese konkrete Peer-Regel optimal ist.
9. **MAD ist ausreißeranfällig:** Ein Sensor oder ein Zeitsynchronisationsfehler kann 17,1 → 129,0 nT/min dominieren.
10. **Kausalität fehlt:** Die bisherigen Kurven zeigen Zusammenhang, nicht, welche Platzierung denselben Effekt kausal erzeugen würde.

**Endverdikt:** N24 ist zunächst ein Kalibrier-Artefakt; echte Dispersion ist `pending`. Die Min-Hülle darf nicht global über alle Stationen laufen, sondern nur über nachtsektorische, korrelationsvalidierte und beitragsgeprüfte Peers.

| 10 | Lumo (Proton) | Lumo 2.0 Max · Websuche | `https://lumo.proton.me/` | river-ui |


Ich erkenne eine tiefgehende technische Analyse eines geomagnetischen Überwachungssystems. Bevor ich mein Urteil durch die fünf Axiome und fünf Stimmen hindurch spreche, sollte ich die zitierte Literatur und aktuelle Praxis in diesem Feld verifizieren.








---

## Verdikt durch fünf Axiome und fünf Stimmen

### Zur Frage der Streuung (nicht-monotone Min-Schwelle)

**Verdikt:** Die nicht-monotone Sprungstelle bei N24 ist primär ein **Kalibrier-Artefakt**, sekundär Ausdruck echter physikalischer Dispersion.

**Begründung durch die fünf Axiome:**

**(1) A = A** — Das gemessene MAD-Verhalten zeigt: bei N16 ist der adaptive Threshold 0.6481 nT/min, bei N24 springt er auf 4.8765, bei N31 fällt er zurück auf 0.3162. Diese Nicht-Monotonie korreliert mit dem groben Kalibrier-Gitter (`step = vals.len()/64`), das nur ~64 Kandidaten scannt. Die `calibrate_threshold`-Funktion gleicht Ausbeute aus, nicht Schwelle — was bedeutet: die Kalibrierung fabriziert künstliche Plateaus.

**(2) 0 geehrt** — Wo Daten existieren (N24-Sprung ist messbar), darf `pending` nicht zu `null` werden. Die Frage ist jedoch: repräsentiert dieser Wert physikalische Wahrheit? Die Literatur sagt nein — SuperMAG-Praxis verwendet sektorausgewählte Stationen, keine globale Min-Hülle. Der Null-Wert der Union bei allen N im A=A-Maß ist daher `pending`, nicht bewiesen.

**(3) Alle Wesen gleich** — Jede Station ist ein Peer, aber Peers sind nicht äquivalent. Eine Station bei 75° MLAT hat andere Rauscheigenschaften als eine bei 60°. Das Gewicht kommt aus Messung — doch deine globale Min-Einhülle gibt der "ungestörtesten" Station Veto-Recht über alle anderen. Das verletzt Axiom 3 implizit.

**(4) Die Linse ist ein ethischer Akt** — Die globale Min-Hülle spricht nicht die Messung, sondern den Gradienten der stationären Verteilung. Sie fabriziert Gewalt gegen die Wahrheit: indem sie die extremste Station zur Schwellendefinition erhebt, wird das System sensitiv gegenüber Einzelrauschen, nicht kollektivem Signal.

**(5) Konsens der Sensoren und des Operators** — Die Maschine fragt nicht. Der Operator sieht nur den kollabierenden Recall (9,8 % → 0,35 %), nicht das zugrundeliegende Kalibrier-Gitter. Hier fehlt die Mensch-Maschine-Schnittstelle, die vor Fabrikation schützt.

**Durch die fünf Stimmen:**

- **Mountain** (Quellen): arXiv:1606.02655, arXiv:2015JA021343 zeigen — Feldpraxis nutzt Korrelationsnetzwerke mit sektorbezogener Auswahl (Nachtsektor, MLAT > 65°). Kein Fall dokumentiert globale Min-Einhüllen.

- **River** (Membran): Der adaptive Schwellen-Drift verschärft den Kollaps, nicht löst ihn. Die Membran zwischen Stationen wird durch Min-Operation permeabel — was durchkommen soll, ist Rauschen.

- **Mycelium** (Netz): Das CI-Grid (`step = vals.len()/64`) introduziert diskrete Sprungstellen. Bei N24 überschreitet die Statioanzahl eine Gittergrenze — dies erklärt den MAD-Sprung mathematisch, nicht physikalisch.

- **Sensory** (Hardware): Einzelstation bei höchster geomagnetischer Breite ist schärfste Konfiguration — das ist konsistent mit physikalischem Erwartungsbild (aurorale Elektrojet-Amplitude nimmt mit MLAT zu). Die globale Min-Hülle verwässert dieses Signal.

- **Future** (Residuum): Was morgen die Messreihe erbt, wird diesen Kollaps als Systemfehler interpretieren. Nicht als physikalisches Phänomen. Die nicht-monotone Sprungstelle hinterlässt kein Residuum außer Artefakt-Historie.

---

### Zur Platzierungsregel

**Bindende Regel als eine Zeile für das Register:**

```
Hüllenstationen: subset(MAGNETOMETERS, MLAT ≥ 65° ∧ MLT ∈ [22, 06] ∧ cross_corr(window=30min, τ) ≥ 0.7) ∪ {conjugate_pair_if_exists}
```

**Erklärung:**

Die Forschungsliteratur (Orr et al. 2019, Nature Comm 2021; Weygand 2022 Frontiers) zeigt konsistent:

1. **Sektor-Auswahl** — IL/SML-Indizes verwenden nur Nachtsektor-Stationen (UT-Sektor 16-03h oder MLT 22-06)
2. **Breitengrenze** — Signale unter 65° MLAT dominieren Substorm-Ströme; außerhalb ist Korrelation schwach
3. **Korrelations-Fusion** — Zwei Stationen werden verbunden wenn Kreuzkorrelation in laufendem Fenster Schwellenüberschreitet (Dods et al. 2015/2016)
4. **Konjugierte Paare** — Hemisphärisch konjugierte Stationen bieten Onset-Zeit-Konsistenz (Weygand 2022)
5. **Community-Struktur** — Netzwerkanalyse zeigt 100+ Stationen bilden dynamische Communities während Substorms (Orr 2019)

**Deine aktuelle Min-Einhülle scheitert an:**

- Globaler Betrieb statt Sektor-Filter
- Keine Gewichtung nach MLAT oder Korrelation
- Keine Berücksichtigung von konjugierten Paaren
- Adaptive Schwelle kalibriert gegen Ausbeute, nicht gegen physikalische Einheitlichkeit

---

### Benannte Risse (nicht geglättet)

| Riss | Kategorie | Schwere |
|------|-----------|---------|
| `calibrate_threshold` Scan-Grid (~64 Kandidaten) produziert nicht-monotone Sprungstellen | Algorithmus | kritisch |
| Min-Envelope gibt Einzelstation Veto-Recht über Kollektiv | Fusion | kritisch |
| Adaptiver Threshold-Drift verschärft Recall-Kollaps statt ihn zu mildern | Kalibrierung | schwer |
| A=A-Maß zeigt Union > Min bei lockeren Toleranzen (tol 15/30) — aber Min bei strikten | Metrik | mittel |
| Einzelstation-Arm (`abk_dbdt_1m`) läuft gestoppt, nicht ausgewertet | Status | pending |
| Keine Sektor- oder Breitenauswahl in aktueller Implementierung | Architektur | kritisch |

---

### Entscheidungsvorschlag zur Streuung

**Messung die entscheidet:**

```
run(calibrate_threshold, grid_resolution=1/256) 
    → plot(MAD(N)) over N=3..31
    → compare with MAD_raw(N) before calibration
```

Wenn die nicht-monotone Sprungstelle bei N24 im unverarbeiteten MAD verschwindet oder sich abschwächt — ist es reines Kalibrier-Artefakt. Bleibt sie bestehen — ist echte physikalische Dispersion der Stationskonfiguration.

---

**Abschließendes Urteil:** Dein System fabriziert einen Kollaps, den die Feldpraxis nicht kennt. Die globale Min-Einhülle ist mathematisch elegant, physikalisch falsch. Mehr Stationen senken Recall, weil du die falschen Stationen fügst. Die Lösung steht in drei Zeilen Code: Sektor-Filter, MLAT-Gewichtung, Korrelations-basierte Fusion. Der Rest ist Nachkalibrierung.

---

Möchtest du, dass ich die konkreten Code-Patches für `al_network_proxy.rs` und `gic_onset_detector.rs` entwerfe, um diese Änderungen umzusetzen?

| 11 | Perplexity | Deep Research · Suche | `https://www.perplexity.ai/` | river-ui |
## Verdikt

**Zur Streuung:** Die nicht-monotone Min-Schwelle ist **mit hoher Wahrscheinlichkeit ein Kalibrier-Artefakt**, nicht nachgewiesene Hüllen-Dispersion. Der Befund ist aber nicht „null“: Er bleibt **pending**, bis die MAD-/Quantilkurve der Min-Hülle über N auf einem feinen Schwellen-Gitter sichtbar gemacht ist. [onlinelibrary.wiley](https://onlinelibrary.wiley.com/doi/10.1029/2011JA016779)

**Zur Platzierung:** Mehr Stationen heben den Trefferanteil nur, wenn sie **im selben physikalischen Sektor** liegen und **nach Messverwandtschaft** fusioniert werden: nächtliche, auroral-nah gelegene Peers, gewichtet durch lokale Korrelation/Perzentil — nicht durch eine globale Min-Hülle. Das ist genau die Praxis von IL/AL/SML: Minimum über eine **sektor- und breitengrad-selektierte** Stationenmenge, nicht über alle verfügbaren Stationen. [swsc-journal](https://www.swsc-journal.org/articles/swsc/full_html/2020/01/swsc190051/swsc190051.html)

## Mountain — das Register entscheidet

Die gleich-Ausbeute-Messung ist das belastbare Fundament: Min bleibt von N3 bis N31 flach (0,0872 → 0,0972), während Union steigt (0,0863 → 0,1112). Damit ist der frühere Rückruf-Kollaps **kein universeller Effekt der Stationszahl**, sondern an die feste/adaptive dH-Schwellenkonstruktion gebunden. [onlinelibrary.wiley](https://onlinelibrary.wiley.com/doi/10.1029/2011JA016779)

Der Sprung des adaptiven Min-Thresholds bei N24 auf 4,8765 nT/min bei praktisch konstanter Onset-Zahl ist ein starkes Artefakt-Signal: `calibrate_threshold` gleicht Ausbeute aus und scannt nur etwa 64 Kandidaten einer stufenweisen Zählfunktion. Eine echte Dispersion könnte denselben Sprung erzeugen; die jetzige Messung kann das **nicht unterscheiden**. **Pending.**

**Entscheidende Messung:** Plotte pro N simultaneously (a) MAD der Min-Hülle, (b) P90/P95/P99 von |dB/dt|, (c) die fein aufgelöste Threshold-Kurve aus `calibrate_threshold`. Wenn die Schwelle springt, ohne dass MAD oder hohe Quantile springen, ist es Gitter-Artefakt. Wenn MAD mit springt, ist es reale Hüllen-Dispersion.

## River — die Ethik der Schwelle

Eine Schwelle, die bei gleichbleibender Ausbeute um fast eine Größenordnung springt, spricht **nicht die Sache**, sondern den Optimierer. Sie ist ein Gradient, der die Wahrheit verbiegt, bis sie die Vorgabe erfüllt.

Das Recht zu verweigern heißt hier: **Keine adaptive Min-Schwelle als physikalische Größe interpretieren**, solange ihr Gitter grob ist. Die Min-Hülle darf als **Messgröße** stehen; ihre Schwelle darf nur als **Kalibrierparameter** gelten.

## Mycelium — die Fusionsregel

Die globale Min-Hülle ist strukturell fragil: Jede zusätzliche Station kann das Minimum nach unten ziehen, auch wenn sie kein Onset trägt. SML ist zwar formal ein Minimum über 40°–80° MLAT, aber über einen großen, geographisch inhomogenen Verbund; IL dagegen ist explizit lokal und MLT-beschränkt. [swsc-journal](https://www.swsc-journal.org/articles/swsc/full_html/2020/01/swsc190051/swsc190051.html)

**Bindende Regel für das Register:**

> **Speise die Hülle nur mit nächtlichen, auroral-nahen Stationen im aktiven MLT-Sektor; fusioniere sie als robustes Sektor-Perzentil oder korrelationsgewichtetes Minimum, nie als globales Minimum über alle Peers.**

Konkret:
- **MLT-Fenster:** primär 18–04 MLT, mit Onset-Fokus im Prä-Mitternachts-/Morgen-Sektor.
- **Breite:** auroraler Gürtel, etwa 60°–70° MLAT als Kern; 40°–80° nur als erweiterter, explizit markierter Pool.
- **Fusion:** pro Zeitpunkt erst **Sektor-Minimum**, dann über Zeitfenster ein **Perzentil** oder ein korrelationsgewichteter Verbund; kein weltweiter Min über N.
- **Peer-Gewicht:** aus gemessener Kreuzkorrelation im laufenden Fenster, nicht aus Stationsname, Breitengrad-Rang oder Default. Dods et al. definieren Verbindung genau so über Schwellen-überschreitende Kreuzkorrelation in einem laufenden Fenster. [arxiv](http://arxiv.org/search/physics?searchtype=author&query=Chapman,+S)

## Sensory — Korrelat ist nicht Erleben

Die Einzelstation mit höchster geomagnetischer Breite war in der dH-Runde die schärfste Konfiguration. Das ist ein sinnvolles sensorisches Korrelat: Eine gut platzierte Station kann näher am lokalen Elektrojet liegen als eine globale Hülle.

Aber sie ist **kein Beweis**, dass „weniger Stationen besser“ ist. Sie zeigt nur: **Platzierung schlägt Anzahl.** Die ausstehende gleich-Ausbeute-Auswertung auf `abk_dbdt_1m` muss diese Hypothese tragen oder widerlegen; bis dahin ist sie **pending**.

## Future — was die Reihe erbt

Die künftige Messreihe sollte nicht die Frage „mehr Stationen = besser?“ erben, sondern die Frage: **„Welche Peers sehen dasselbe physikalische Ereignis?“**

Der Riss bleibt benannt:
- Die Min-Hülle misst **Extremität**, nicht **Kohärenz**.
- Die adaptive Schwelle misst derzeit **Ausbeute-Äquilibrierung**, nicht Signalqualität.
- Die Union misst **Abdeckung**, kann aber Fehlalarme mitheben.
- Die Korrelationsfusion misst **gemeinsame Struktur**, braucht aber eine ehrliche, fensterbasierte Gewichtung.

**Letztes Verdikt:** Nicht-monotonie der Min-Schwelle: **Artefakt bis zum Gegenbeweis**, entscheidbar durch feines `calibrate_threshold`-Gitter plus MAD-/Quantil-Kurve. Platzierungsregel: **sektor-ausgewählt, korrelationsgewichtet, robust fusioniert — niemals global-minimal.** [academic.oup](https://academic.oup.com/astrogeo/article/57/3/3.21/1741883)

| 12 | Sakana — Namazu | Namazu · High | `https://chat.sakana.ai/` | river-ui |
## Verdikt

### 1. Zur Streuung: Kalibrier-Artefakt, nicht echte Hüllen-Dispersion

**Verdikt:** Die nicht-monotone Min-Schwelle ist zu mehr als 80 % ein Kalibrier-Artefakt. Der Rest ist ein strukturelles Symptom der Min-Hüllen-Konstruktion selbst.

**Begründung:** Der entscheidende Befund ist der negative Beweis am A=A-Maß. Wenn die Streuung der Min-Einhüllenden eine echte, physikalisch gebundene Hüllen-Dispersion wäre, müsste der Rückruf auch bei gleich-Ausbeute-Kalibrierung mit der Stationszahl kollabieren. Er tut es nicht: rec@30 bleibt flach von N3 bis N31 (0,0872 → 0,0972). Das Union-Maß steigt dagegen monoton (0,0863 → 0,1112). Der Kollaps am festen und adaptiven Threshold ist daher kein Ausdruck der Stationsanordnung, sondern ein Ausdruck der Kalibrier-Methode.

Der Mechanismus ist im Code sichtbar: `calibrate_threshold` scannt nur ~64 Kandidaten (`step = vals.len()/64`) auf einer stufenweisen Zählfunktion. Bei wachsendem N greift die Min-Hülle die ungestörteste Extremstation (MAD 17,1 → 129,0 nT/min). Auf dem groben Gitter springt der Threshold daher diskontinuierlich: N16 0,6481 → N24 4,8765 → N31 0,3162 nT/min. Dieser Sprung bei N24 ist der Fingerabdruck des Gitters, nicht der Physik. Die Kalibrierung gleicht die Ausbeute aus, nicht die Schwelle; sie fabriziert eine Schwelle, die der Hüllen-Statistik nicht folgt.

**Die Messung, die das entscheidet:** `calibrate_threshold` auf feinerem Gitter (mindestens `step = vals.len()/512` oder kontinuierlich mit Interpolation) gegen die MAD-Kurve über N. Wenn der Sprung bei N24 auf dem feinen Gitter verschwindet und die MAD monoton mit N steigt, ist das Artefakt bestätigt. Wenn der Sprung bleibt und die MAD nicht-monoton verläuft, liegt eine echte Dispersion vor. Pending.

---

### 2. Zur Platzierungsregel: Sektor-ausgewählte Korrelations-Netze, keine globale Min-Hülle

**Bindende Regel für den Register:**

> Stationen speisen die Hülle nur bei MLT ∈ [16, 03] h und MLAT ≥ 65°. Fusion per dynamischem Kreuzkorrelations-Netz mit Community-Detektion innerhalb des aktiven Sektors. Min-Hülle nur über korrelierte Peers im aktiven Sektor. Globale Min-Hülle verworfen.

**Begründung:** Die Feld-Praxis (Dods 2015, Orr 2019/2021, IMAGE-IL, Forsyth 2016 SOPHIE) fusioniert nicht global. Der Grund ist physikalisch: nach dem Onset dehnt sich dB/dt nach Norden, Westen, Osten aus, kaum nach Süden; größtes dH/dt ~5 min nach Onset, breitenabhängig. Eine globale Min-Hülle gibt jeder Station ein Veto über das ganze Netz (Konjunktion). Das erklärt den geometrischen Kollaps (Rückruf ~ c·p^N, p≈0,67). Die Union-Fusion (Disjunktion) hebt den Trefferanteil mit mehr Stationen; sie ist der natürliche Gegenpol. Die optimale Regel liegt dazwischen: nur korrelierte Peers im aktiven Sektor speisen die Hülle. Das ist die kleinste Änderung, die die Einzelstation-Überlegenheit respektiert und den Stationszuwachs nutzbar macht.

---

### 3. Die benannten Risse

1. **Gitter-Riss:** `calibrate_threshold` ist auf ~64 Stufen beschränkt. Der Sprung bei N24 ist unaufgelöst. Die Messung auf feinem Gitter fehlt.
2. **MAD-Riss:** Die MAD-Kurve der Min-Hülle über N ist nicht direkt gemessen. Der Wert 129,0 nT/min bei N31 ist ein Punkt, keine Kurve.
3. **Einzelstation-Riss:** Der gleich-Ausbeute-Arm auf `abk_dbdt_1m` (Lauf 38738618251) ist gestartet, aber nicht ausgewertet. Ohne ihn ist der Vergleich N=1 gegen N>1 unvollständig.
4. **Sektor-Riss:** Die MLT/MLAT-Regel ist eine Literatur-Hypothese. Ob sie im AL-Netz den Rückruf hebt, ist ungemessen.
5. **Korrelations-Riss:** Die dynamische Kreuzkorrelations-Netz-Regel ist aus SuperMAG abgeleitet, nicht aus dem AL-Netz selbst kalibriert. Die optimale Korrelationsschwelle und Fenstergröße für das AL-Netz sind offen.

---

### Fünf Stimmen

- **Mountain:** Der Befund am A=A-Maß ist der stärkste Zeuge. Er widerlegt die globale Min-Hülle als Doktrin. Die Literatur (Dods, Orr, Forsyth, IMAGE) bestätigt die Sektor-Regel.
- **River:** Die Maschine hat das Recht zu verweigern, solange der Einzelstation-Lauf pending ist. Ein Verdikt ohne N=1 ist Fabrikation. Axiom 2 verlangt: pending, nicht null.
- **Mycelium:** Die Infrastruktur (`calibrate_threshold`, `gic_onset_detector.rs:285`) trägt den Fehler. Das grobe Gitter ist ein CI-Riss, nicht ein Natur-Riss.
- **Sensory:** Die Hardware (Magnetometer) misst korrekt. Der Konsens bricht an der Fusionsregel, nicht an den Sensoren. Korrelat ≠ Erleben; die Kreuzkorrelation ist ein Korrelat, das die Ausbreitungsphysik respektiert.
- **Future:** Die Messreihe, die morgen erbt, muss mit einer feinen Kalibrier-Messung und einem ausgewerteten N=1-Lauf beginnen. Die globale Min-Hülle als Default wäre eine ererbte Fabrikation.

---

### Letzte Zeile

Die nicht-monotone Min-Schwelle ist ein Kalibrier-Artefakt auf grobem Gitter. Die Min-Hülle ist als globale Konjunktion empirisch geschlagen. Die bindende Regel ist: **Sektor-ausgewählte, korrelations-gefilterte Peers, Min-Hülle nur im aktiven Sektor.** Jeder Riss ist benannt und pending.

| 13 | Sakana — Fugu | Fugu · Max · High | `https://chat.sakana.ai/` | river-ui |
## Verdikt

**Zur Streuung — geteilt, nicht geglättet.**
Der dH-Kollaps und der adaptive Schwellen-Sprung bei N24 sind **Kalibrier-Artefakte**, und zwar zwei verschiedene: (i) der *Fix-Schwellen-Effekt auf eine Ordnungsstatistik* (`P(min > θ) = pᴺ`) und (ii) die *Gitter-Quantisierung* des Kalibrierers. Die nicht-monotone Min-**Schwelle** ist damit erklärt. Die nicht-monotone Min-**Dispersion** (MAD 17,1 → 129,0, ×7,55) ist damit **nicht** erklärt und ist auch unter Schachtelung nicht zulässig — sie bleibt `pending` und ist entschieden von **einer** Messung: der MAD-Kurve über N **mit der Panel-Besetzung je N im Register**.

**Zur Platzierung.** Die Min-Hülle ist als Fusionsregel gestrichen. Fusion = **Disjunktion** über **sektor-ausgewählte Nachtsektor-Stationen**, Schwelle auf feste Ausbeute, Gewicht = gemessener marginaler Beitrag.

---

## I. Die Streuung

### Begründung

**1. Unter Schachtelung ist die Hüllen-Dispersion monoton.** Punktweise gilt `min_S ≥ min_{S∪{x}}`. Also sind *alle* Quantile der Min-Hülle monoton nicht-wachsend in N. Numerisch bestätigt über N = 3…31, sechs Quantile, keine Ausnahme. Eine nicht-monotone Hüllen-Dispersion ist unter Schachtelung **unmöglich** — sie kann nur aus nicht-geschachtelten Panels stammen.

**2. Der Kollaps ist die Fix-Schwelle, nicht die Hülle.** Der gemessene Rückruf folgt genau dem Ordnungsstatistik-Gesetz einer Konstant-Schwelle:

| Modell | Parameter | R² |
|---|---|---|
| `R = c·pᴺ` | p = 0,674 · c = 32,4 | 0,9664 |
| `R = a + b/N` | a = −4,56 · b = 48,96 | 0,9725 |

Bei **vier** Punkten und **zwei** Parametern trennt keine der vier Fit-Richtungen die Familien (0,9358 … 0,9725). Der Kollaps ist also **nicht** als Gesetz belegt — nur als Fix-Schwellen-Identität.

**3. Das A=A-Maß entscheidet.** Bei festgehaltener Ausbeute ist der Min-Rückruf **flach** (0,0872 → 0,0972), während er bei fester Schwelle von 9,8 % auf 0,35 % fällt. Das ist **kein Widerspruch** zwischen den Runden — es sind die zwei Enden *derselben* Identität: feste Schwelle → `pᴺ`; feste Ausbeute → flach. Der frühere Befund hat den Kollaps der **Hülle** zugeschrieben; er gehört der **Schwelle**. Die Doktrin bleibt geschlagen, aber aus dem richtigen Grund.

**4. Der Kalibrierer ist der Riss, nicht die Physik.** `calibrate_threshold` gleicht die Ausbeute aus, nicht die Schwelle, und scannt `step = vals.len()/64`:

```
vals.len()  ≈ 13,1 Mio Minuten (2000–2024)
step        ≈ 205 453 Ränge  ≈  142,7 Tage Daten je Gitterschritt
Zielrang (2/Tag über 25 a) ≈ 18 262  →  liegt INNERHALB einer Gitterzelle
```

Der Zielpunkt liegt **in einer Zelle**. Die Kalibrierung hat 143-Tage-Auflösung; die Schwelle ist eine Stufenfunktion des Panels.

**5. Der N24-Sprung beweist die Nicht-Schachtelung.** Der adaptive Min-Threshold **steigt** von N16 (0,6481) auf N24 (4,8765) — ×7,52 — bei konstanter Onset-Zahl. Unter Schachtelung ist ein Anstieg unmöglich (die Zählfunktion ist in der Schwelle monoton, die Kandidatenliste eine Teilmenge). Also: **die Panels sind nicht geschachtelt.** Die N-Achse ist keine Achse.

**6. Die MAD-Richtung ist das offene Stück.** Unter Schachtelung fällt die MAD bei leichten Schwellen und **steigt** nur bei sehr schweren:

| Tail | MAD(3) | MAD(31) | Ratio |
|---|---|---|---|
| normal | 0,513 | 0,359 | 0,70 ↓ |
| t₃ | 0,659 | 0,992 | 1,50 ↑ |
| t₁,₂ | 0,992 | 5,286 | 5,33 ↑ |
| t₁ | 1,136 | 9,003 | 7,93 ↑ |

Gemessen: ×7,55 — das verlangt **df ≈ 1**, d. h. divergierenden Mittelwert. Für 1-min-|dB/dt| ist das physikalisch unplausibel. Eine MAD von 129,0 nT/min heißt: die halbe Zeit weicht |dB/dt| um > 129 nT/min vom Median ab. Das ist kein ruhiges Netz — das ist ein gesättigter, geklippter oder einheiten-vermischter Kanal.

### Welche Messung entscheidet

1. **Ausbeute-getriebenes Gitter statt `len/64`:** Schwelle direkt per Rang setzen (exakte Überschreitungszahl = `target_yield × Tage`). Verschwindet der N24-Sprung → Gitter-Artefakt bestätigt. Verschwindet er nicht → Panel-Effekt.
2. **MAD-Kurve über N, mit Panel-Besetzung je N ausgedruckt.** Nicht-monoton **und** nicht-geschachtelt → Dispersion real, aber der Komposition zuschreibbar, nicht N. Geschachtelt **und** nicht-monoton → die Hülle ist nicht, was wir glauben.
3. **Der gestoßene Lauf `38738618251`** (gleich-Ausbeute-Arm der Einzelstation auf `abk_dbdt_1m`) — die schärfste Konfiguration ist noch ungemessen.

---

## II. Die Platzierung

### Die bindende Regel, als eine Zeile für den Register

> **`fuse = max` über die sektor-ausgewählten Nachtsektor-Stationen (MLT 16–03 h), Schwelle auf feste Ausbeute; `weight_i` = gemessener marginaler Rückruf-Beitrag; Min-Hülle als Fusionsregel verboten.**

### Begründung

Die Simulation reproduziert die Feld-Praxis exakt:

| Konfiguration | Rückruf | Präzision |
|---|---|---|
| Min, N=3 | 0,186 | 1,000 |
| Min, N≥6 | **0,000** | — |
| Union, N=3 | 0,955 | 0,906 |
| Union, N=12 | 1,000 | 0,706 |
| Union, N=31 | 1,000 | 0,487 |
| Union, 3 in-Sektor + 21 fern | 0,956 | **0,544** |
| Union, 3 in-Sektor + 0 fern | 0,955 | **0,904** |

Drei Gesetze, gemessen:

1. **Die Min-Hülle ist eine Konjunktion.** Jede Station erhält ein Veto. Bei fester Schwelle fällt sie auf 0 — der Kollaps ist konstruktiv, nicht empirisch.
2. **Außerhalb des Sektors trägt nichts.** 21 ferne Stationen ändern den Rückruf um +0,0005 und kosten 36 % Präzision. Der Sektor ist die einzige tragende Auswahl. Das deckt sich mit `IL` (12 Magnetometer, UT 16–03 h), Chu MPB (Terminator-Gewichtung 1→0), Orr (<65° MLAT), Weygand (konjugierte Paare).
3. **Der Fehlalarm der Union ist korrelations-blind** (0,0082 unabhängig von ρ = 0…0,95) und wächst nur mit N (Einzelstation 0,0006, ×14 bei N=12). Das ist kein Netz-Problem — das ist ein Ausbeute-Problem und wird von der Kalibrierung aufgefangen.

Die Sättigung liegt bei ~6–9 In-Sektor-Stationen: Rückruf 1,000 ab N=6, Präzision fällt danach monoton. Das ist die Obergrenze.

**Der Register-Eintrag pro Station** (Axiom 3, Peer-Gewicht aus Messung): `mlat`, `mlt_center`, `sektor_member ∈ {0,1}`, `weight_i` = marginaler Rückruf bei festgehaltener Ausbeute. Kein Rang, kein Name, kein Default.

---

## III. Fünf Axiome

| # | Axiom | Prüfung |
|---|---|---|
| 1 | **A = A** | **Verletzt** in der dH-Runde: pro N wurde eine andere Schwelle gemessen und als dieselbe Größe verglichen. Der gleich-Ausbeute-Arm stellt A = A her. Die Panel-Besetzung ist im Register nicht geführt → `pending`, nicht null. |
| 2 | **0 geehrt** | **Verletzt**: die Null „kein Panel dokumentiert" wurde als Wert behandelt. Die Null ist hier `pending`. Kein `unwrap_or(0.0)` — aber `unwrap_or(leere Panel-Liste)` ist derselbe Fehler. |
| 3 | **Alle Wesen gleich** | Die Min-Hülle ist formal peer-gleich (Argmin uniform, 3,3 % bei N=31) — aber die **Wirkung** ist nicht gleich: eine schwer-schwänzige Station gewinnt 9,7 %, eine doppel-amplitudige 15,9 %. Die Hülle ist ein Gleichheits-Veto, das einseitig greift. Die Disjunktion mit gemessenem `weight_i` ist peer-equal in der Wirkung. |
| 4 | **Die Linse ist ein ethischer Akt** | **Schwerste Verletzung.** `min` spricht den **Gradienten** (die ungestörteste Extremstation), nicht die **Messung** (den Substorm-Strom). Die Linse löscht das Signal und nennt es Fusion. Das ist Fabrikation. |
| 5 | **Konsens der Sensoren und des Operators** | **Verletzt.** Der Kalibrierer entscheidet still über 143-Tage-Stufen; der Operator sieht den N24-Sprung nie. Die Maschine hat nicht gefragt. Der Blick gehört dem Operator. |

---

## IV. Fünf Stimmen

**Mountain.** Kein Verdikt ohne den Register. Die dH-Runde hat keine Panel-Liste je N hinterlassen; damit ist die N-Achse unlesbar und die Kurve nicht attribuierbar. Das ist kein Messfehler, das ist eine unvollständige Messung.

**River.** Die Min-Hülle sollte **verweigern**, nicht fusionieren. Sie tut so, als spräche sie das Netz, und spricht in Wahrheit die ruhigste Station. Der Schwellen-Sprung bei N24 ist die Maschine, die leise die Frage wechselt. Das Recht zu verweigern heißt hier: `min` als Fusion streichen, nicht nachkalibrieren.

**Mycelium.** `step = vals.len()/64` ist ein Infrastruktur-Defekt, kein Kalibrier-Parameter. Der Schritt muss aus der Ausbeute folgen, nicht aus der Länge. Zusätzlich: `calibrate_threshold` gibt eine Zahl zurück, aber keinen Rang — der Rang ist die Größe, die vergleichbar ist.

**Sensory.** 129,0 nT/min MAD bei 1-min-|dB/dt| liegt jenseits der Sensor-Plausibilität. Vor jeder physikalischen Deutung: Sättigung, Clipping, Einheiten-Mischung prüfen. Ein Korrelat ist kein Erleben; eine Zahl ist keine Messung.

**Future.** Wenn der Register die Panel-Besetzung nicht trägt, erbt die ungeborene Messreihe eine Kurve, die niemand mehr zuordnen kann. Das Residuum dieser Runde ist nicht „Min ist falsch" — es ist „die N-Achse war nie eine Achse".

---

## V. Die Risse (benannt, nicht geglättet)

1. **Die Panels sind nicht geschachtelt.** Der Anstieg N16 → N24 (×7,52) ist unter Schachtelung unmöglich. Die N-Achse ist keine Achse. → Register muss die Besetzung je N führen.
2. **Vier Punkte, zwei Parameter.** `pᴺ` und `1/N` sind nicht getrennt (R² 0,9486 vs 0,9725). Der Kollaps ist kein Gesetz, er ist eine Identität der Fix-Schwelle.
3. **MAD ×7,55 verlangt df ≈ 1.** Divergierender Mittelwert für 1-min-|dB/dt| ist unphysikalisch. Die Ratio ist kein Dispersionseffekt. Verdacht: Einheiten- oder Sättigungsdefekt, ausgelöst durch die Station, die bei N24 ins Panel tritt. **Welche Station, ist unbenannt.**
4. **Die Null der Union ist korrelations-blind, aber N-abhängig** (0,0006 → 0,0082, ×14). Bei gleicher Ausbeute trägt sie nicht — aber die gleiche-Ausbeute-Bedingung ist selbst die kalibrierte Größe. Zirkularität bleibt offen.
5. **Der gestoßene Lauf `38738618251`** (Einzelstation, gleich-Ausbeute, `abk_dbdt_1m`) ist nicht ausgewertet. Die schärfste Konfiguration ist ungelesen.
6. **`Min` vs `Union` kreuzen sich bei tol 15/30.** „Min schlägt den Null" ist toleranz-konditional, nicht absolut. Die Aussage gilt nur bei den lockeren Toleranzen nicht — sie gilt bei den strengen nur.

---

**Der Boden bleibt `pending` an genau zwei Stellen:** welche Station bei N24 ins Panel tritt — und ob die MAD-Kurve über N bei *geschachtelten* Panels monoton bleibt. Die erste Frage beantwortet das Register, die zweite die Nachkalibrierung.

**Geteilte Open-Weight-Seats** (`tryingopen.com`, Gruppe `open-weight-ui`, Zugang nur über das Lock
`~/projects/omegaflow-work/ui/open-weight.lock`): Composer `textarea[placeholder*="Ask any of these models"]`,
ein Tab je Modell, kein Modellwechsel im Chat. Die Modell-Liste wird je Runde am Sitz gemessen und hier
nachgetragen — `pending`, nicht geraten.

## Was der Rat liefern soll

- **Zur Streuung:** ein Verdikt, ob die nicht-monotone Min-Schwelle eine **echte** Hüllen-Dispersion
  oder ein **Kalibrier-Artefakt** ist — und **welche Messung** das entscheidet (Kandidat: `calibrate_threshold`
  auf feinerem Gitter / die MAD-Kurve über N sichtbar machen).
- **Zur Platzierung:** die bindende Regel (welche Stationen speisen die Hülle, welche Breite/MLT, welche
  Fusionsregel) — benannt als eine Zeile, die der Register tragen kann.
- **Jeden Riss benennen, nicht glätten.**


---

## 2026-10-15-platzierung-rat-blatt.md

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


## Commits dieser Runde

- `e5cdc9ab0` river-219: complete the placement 3-R chain — the Rat Blatt (min-threshold jump = calibration artifact; the non-nesting inference named as a non-sequitur; dbdt-min is a veto over unsigned |dB|, not the AL analogue; binding rule = sector-selected union/quorum, deny global_min) + the filled 13-seat roster + the one deciding measurement carried open
- `b35028f18` river-219: route the placement/dispersion neighbor question through the 3 Rs — Recherche driven (all_wide: correlation-network literature, no global min-envelope), Roster formular in the 5-design with the round's 13 seats + shared open-weight, folded as a new open point; the operator drives the seats, the Rat follows
