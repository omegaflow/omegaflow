# Stimmen — Entwurfsfrage (Schärfe · Platzierung · Kette), Roster 2026-10-15

Rohmaterial der Roster-Runde (ein benannter Akt, Operator-Wort 2026-10-15). Der Operator fuhr
die Frontier-Sitze — je die maximale Variante, wo möglich der Search-/Research-Modus; die
Zeit-/Modell-Kennungen sind beim Einlesen eingetragen (Paste 2026-10-15 08:14Z). Die
Open-Weight-Sitze (`open-weight-ui`, tryingopen) waren `blocked: site credit`
(s. `2026-10-15_tryingopen_entwurfsfrage.md`). Die Session verifiziert jede Behauptung am Baum,
nie die Stimme; Verdikt trägt der Rat.

**Sitze (13):** 01 Duck.ai · 02 Claude · 03 Qwen · 04 Z.ai/GLM · 05 DeepSeek · 06 Mistral ·
07 Gemini AI Studio · 08 MiniMax · 09 Kimi · 10 Lumo · 11 Perplexity · 12 Sakana Namazu ·
13 Sakana Fugu.

## Prompt (verbatim, eine Frage je Sitz)

> Entwurfsfrage — je Teil Verdikt (1 Satz) + Begründung (2–4 Sätze), knapp, kein Aufsatz.
>
> Lage: Wir erkennen geomagnetische Störungen (GIC-Vorläufer) aus einem Netz von Magnetometer-Stationen; bay-onSets gegen eine bekannte Onset-Liste, mit Zufalls-Null (verschobene Zeit). Gemessen: EINE Station (höchste geomagnetische Breite) erkennt am besten (Rückruf ~6,6 %); jede weitere Station senkt den Rückruf (1→6→31 Stationen: 6,6 % → 0,2 % → 0,01 %). Eine niedrigere Ansprechschwelle ist besser UND schlägt den Zufall; kein inneres Optimum.
>
> (a) SCHÄRFE — Wie definiert man „am schärfsten" als EIN Maß über die zwei Achsen Toleranz × Stationszahl, ohne dass eine gelockerte Toleranz das Maß mechanisch schönt? Nenne das Maß und begründe es.
>
> (b) PLATZIERUNG — Wie müssen Stationen zueinander angeordnet sein, damit der Trefferanteil nicht fällt? Warum schlägt eine einzelne Station ein ganzes Netz? Welche Rolle spielt Längen- vs. Breiten-Abdeckung relativ zum Polarlicht-Elektrojet?
>
> (c) KETTE — Eine Frühwarnkette Sonne→Boden, in der jede Stufe doppelt gemessen wird: Vorlauf (Zeit) und Sicherheits-Zuwachs (inkrementelle Transfer-Entropie über die Vorstufe). Trägt dieser Bau, und wo ist das schwächste Glied?

---

## 01 · Duck.ai
- **Site:** https://duck.ai/
- **Modell/Einstellung:** GPT (primär OpenAI) · kein Modell wählbar · Websuche
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) SCHÄRFE — Verdikt:** Nimm die vorab gewichtete Fläche unter einer zufalls-korrigierten Trefferquote über Toleranz und Stationszahl als einen Schärfewert.  
Für jedes \((\tau,N)\) berechne \(S_{\tau,N}=(R_{\tau,N}-R_{0,\tau,N})/(1-R_{0,\tau,N})\), wobei \(R_0\) aus zeitverschobenen Nullreihen mit derselben Toleranz und Stationszahl stammt; bilde daraus den gewichteten Mittelwert über einen vorab festgelegten Bereich, etwa gleichmäßig in \(\log\tau\) und über die getesteten Stationszahlen. So zählt nicht der rohe Zugewinn an Treffern, den eine lockere Toleranz auch im Zufall erzeugt, sondern der Überschuss gegenüber dieser Null; Bereich und Gewichte dürfen nicht nachträglich optimiert werden.

**(b) PLATZIERUNG — Verdikt:** Um den Rückruf beim Hinzufügen von Stationen nicht zu senken, muss die Auswertungsregel „jede Station darf allein auslösen“ zulassen; die Platzierung allein garantiert das nicht.  
Dann kann die Trefferquote bei identischer Ereigniszuordnung mathematisch nicht sinken. Physikalisch sollten Stationen längs der auroralen Zone über verschiedene Längengrade verteilt sein und zugleich mehrere geomagnetische Breiten abdecken: Der Elektrojet ist längsgerichtet, kann sich aber bei Störungen deutlich äquatorwärts verschieben; diese Verschiebung kann wichtiger sein als Lücken in der Längengradabdeckung. Eine einzelne Station kann das Netz schlagen, wenn sie den relevanten Onset-Sektor gut erfasst und die Netzregel Signale mittelt, Konsens verlangt oder lokal unterschiedliche Signaturen verwirft; die vorliegenden Rückrufwerte allein belegen nicht, welcher Mechanismus wirkt. <citation src="1"></citation>

**(c) KETTE — Verdikt:** Der Doppelansatz trägt als Diagnose, aber Transferentropie ist kein direktes Maß für Sicherheitsgewinn.  
Miss den Vorlauf pro Stufe zum selben Zielereignis und den bedingten Informationszuwachs der Stufe gegenüber der Vorstufe, unter Kontrolle der Zielvergangenheit und relevanter gemeinsamer Einflussgrößen; andernfalls kann gemeinsame Information wie ein Stufeneffekt aussehen. Transferentropie misst zusätzliche gerichtete Vorhersageinformation, nicht automatisch Kausalität oder vermiedene Schäden. Nach den vorliegenden Zahlen ist die Boden-Erkennung der empirische Engpass; das schwächste Glied der gesamten Sonne‑zu‑Boden‑Kette lässt sich ohne vergleichbare Stufenmessungen und direkte GIC-Zielereignisse nicht sicher bestimmen. <citation src="2"></citation>
---

## 02 · Claude
- **Site:** https://claude.ai/new
- **Modell/Einstellung:** Claude Sonnet 5.5 · „extra hoch"
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) Schärfe**

**Verdikt:** Nimm den Lift gegen die Zufalls-Null in Bits, also S = log₂(R/R₀), und werte ihn als untere Konfidenzgrenze über das Null-Ensemble.

**Begründung:** R₀ wird mit exakt derselben Regel berechnet wie R: gleiches τ, gleiche Stationszahl, gleiche Koinzidenzregel, gleiche Auslösemenge, und die Zeit wird pro Station unabhängig verschoben. Bei gelockertem τ wachsen R und R₀ gemeinsam (R₀ ~ τ bei N=1, ~ τ^(N−1) bei Koinzidenz von N Stationen), und für τ→∞ geht der Lift gegen 1. Lockern kann das Maß also nicht beschönigen, und beide Achsen landen in derselben Einheit. Dass es kein inneres Optimum gibt, ist ein Warnsignal: Das Maß kennt keine Fehlalarmkosten. Fixiere deshalb ein Auslösebudget (Trigger pro Tag) statt der Schwelle und vergleiche erst dann. Bei 0,01 % Rückruf sind die Zählungen klein, daher die untere Konfidenzgrenze statt des Punktwerts.

**(b) Platzierung**

**Verdikt:** Stationen müssen im selben Auroral-Band und innerhalb der Längenausdehnung eines Onsets liegen, und das Netz muss per MLT-Gate oder k-aus-N statt per UND über alle Stationen verknüpft werden.

**Begründung:** Ein Bay-Onset ist lokal: Er sitzt im Nachtsektor (etwa 21–03 MLT) unter dem westwärtigen Elektrojet, mit einem Breitenband von wenigen Grad. Südlich davon ist das Signal schwach oder verzögert, polwärts davon kehrt es das Vorzeichen um, und auf der Tagseite fehlt es ganz. Die Station mit der höchsten Breite liegt am häufigsten unter dem Elektrojet, ist also die beste Einzelstation. Ein UND über viele Stationen bildet den Schnitt ihrer Sichtbarkeitsmengen, der Rückruf ist durch die schlechteste Station beschränkt, und 31 Stationen heißt vor allem viele Vetos von Tagseite und niedriger Breite (daher 6,6 → 0,2 → 0,01 %). Für die Längenabdeckung gilt: Der Stationsabstand muss kleiner sein als die Onset-Ausdehnung (grob 1–2 h MLT, also 15–30° Länge), damit Nachbarn denselben Onset sehen. Zugleich muss der Ring um den Pol so dicht sein, dass wegen der Erdrotation immer jemand im Nachtsektor steht. Ein sinkender Rückruf heißt zudem nicht, dass das Netz schlechter ist: Wenn R₀ schneller fällt als R, steigt der Lift, und das wäre über (a) zu prüfen.

**(c) Kette**

**Verdikt:** Der Bau trägt als Auswertungsdisziplin, aber nicht als Beweis einer Kausalkette, und das schwächste Glied ist der letzte Schritt vom Sonnenwind bei L1 zum Boden-Onset.

**Begründung:** Vorlauf und inkrementelle TE messen Verschiedenes und sind nicht unabhängig, denn mit wachsendem Vorlauf zerfällt die Kopplung. Berichte daher TE(τ) als Kurve und lies den Vorlauf am TE-Maximum ab. Am Ende der Kette ist der Vorlauf nur Minuten bis eine Stunde, der Ziel-Prozess ist ein seltener, teils intern ausgelöster Punktprozess (Loading/Unloading), und der Rückruf liegt bei höchstens 6,6 %. TE aus so wenigen Ereignissen ist instabil, und autokorrelierter Sonnenwind als Vorstufe erzeugt leicht scheinbare Zuwächse, sofern die Null nicht blockweise verschoben ist. Zudem endet die Kette bei Bay-Onsets, nicht bei GIC, denn GIC hängt zusätzlich von Erduntergrund und Netztopologie ab. Am Anfang der Kette bleibt außerdem die Bz-Vorhersage vor L1 ein bekannter Engpass.

---

## 03 · Qwen
- **Site:** https://chat.qwen.ai/
- **Modell/Einstellung:** Qwen 3.8 Max · Think
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) SCHÄRFE**  
**Verdikt:** „Am schärfsten“ ist die Kombination mit maximalem chance-korrigiertem Skill, konkret **TSS/Youden-J**: \(S(\tau,n)=\text{Recall}_{\text{Onset}}(\tau,n)-\text{Recall}_{\text{Null}}(\tau,n)\).  

**Begründung:** Für jede Toleranz \(\tau\) und Stationszahl \(n\) wird die Trefferquote gegen die echten Onsets und gegen die verschobene Null gemessen. Eine größere Toleranz erhöht beide Quoten; der Score steigt nur, wenn der echte Überschuss über Zufall wächst. Damit werden auch zusätzliche Stationen nur dann belohnt, wenn sie den Null-Überschuss erhöhen, statt den Rückruf mechanisch zu verwässern.

**(b) PLATZIERUNG**  
**Verdikt:** Stationen sollten im elektrojet-nahen Breitenband bleiben, aber in Länge/MLT verteilt sein und als **Oder-Verbund** statt als Koinzidenzgatter ausgewertet werden.  

**Begründung:** Der Polarlicht-Elektrojet ist in geomagnetischer Breite schmal, in Länge/MLT aber ausgedehnt und oft lokal/propagierend. Eine einzelne Station unter oder nahe dem Elektrojet sieht das stärkste Signal; zusätzliche Stationen außerhalb oder mit UND-Verknüpfung wirken dann wie Filter und entfernen echte Treffer, deshalb kann eine einzelne Station ein Netz schlagen. Breitenabdeckung entscheidet über die Kopplung an den Elektrojet, Längenabdeckung über die Erfassung verschiedener lokaler Zeiten.

**(c) KETTE**  
**Verdikt:** Der Bau trägt als Informations- und Zeitbudget, aber nur, wenn jede Stufe bedingt positive inkrementelle Transfer-Entropie liefert; das schwächste Glied ist das Minimum dieses Zuwachses, praktisch meist der Übergang Sonnenwind/Magnetosphäre → Boden/Elektrojet.  

**Begründung:** Eine Stufe darf nicht nur Vorlaufzeit kosten, sondern muss die Unsicherheit über den Boden-Onset über die Vorstufe hinaus reduzieren. Gerade die letzte physikalische Kopplung ist nichtlinear, lokal und zustandsabhängig, daher ist der Sicherheits-Zuwachs dort oft klein und die Restvorlaufzeit kurz. Zusätzlich ist Transfer-Entropie bei kurzen, autokorrelierten Zeitreihen schätz- und testbedürftig, sonst entstehen Schein-Zuwächse.

---

## 04 · Z.ai / GLM
- **Site:** https://chat.z.ai/
- **Modell/Einstellung:** GLM 5.3 · Deep Think Max · Advanced Search
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) Schärfe**

**Verdikt:** Maß ist die **zufallsnormierte Zuordnungskosten-Distanz**: minimale Kosten eines Eins-zu-Eins-Matchings (Earth-Mover-/Wasserstein-Typ) zwischen Detektionen und Referenz-Onsets, mit vorab fixierten Miss-/Fehlalarm-Pauschalen in Zeiteinheiten (verankert am Zufallsmaßstab, z. B. ≈ halber mittlerer Onset-Abstand); „am schärfsten" = kleinster Wert — die Toleranz existiert dann nicht mehr als freier Parameter.

**Begründung:** Lockere Toleranz schönt nur, weil Fenster-Matching viele-zu-eins zulässt — ein träger Schwellwert-Durchbruch „deckt" Onset und Nachbarn gratis; bei optimaler Zuordnung verbraucht jede Detektion genau ein Onset, Schlendrian wird über die Fehlalarm-Pauschale bezahlt und Detektions-Spam ebenso. Misses kosten denselben Zeit-Preis, damit der Rückruf-Kollaps (6,6 % → 0,01 %) direkt im Skalar steht, statt sich hinter hoher Precision zu verstecken. Normiert auf die zeitverschobene Null je Konfiguration (gleiche Detektionsrate) ergibt sich ein reiner Skill-Wert, und weil es ein zu *minimierender* Kostenwert ist, gibt es keine Parameter-Richtung, deren Lockerung das Maß heben könnte. Die Stationsachse bleibt als Konfiguration, die Toleranzachse kollabiert ins Maß — gleichwertig berichtbar als kleinste Toleranz τ*, bei der die Null noch signifikant geschlagen wird; beide Ordnungen müssen dasselbe Ranking liefern.

**(b) Platzierung**

**Verdikt:** Stationen als **Längengrad-Kette bei ~fester geomagnetischer Breite** direkt unter dem Elektrojet, fusioniert per ODER (noisy-OR) — nur Längengrad-Spreizung erhöht den Trefferanteil; Breiten-Spreizung plus Konsens-Fusion sind die beiden Killer, und genau die liegen vor.

**Begründung:** Der Polarlicht-Elektrojet ist eine ost–west verlaufende Strombahn in ~100 km Höhe, das Bodensignal (dB/dt) fällt grob mit 1/Abstand — die geomagnetische Breite ist der SNR-Regler, Stationen außerhalb des Oval-Bands (das bei Stürmen äquatorwärts wandert) sind reine Rauschkanäle, deshalb gewinnt die Station höchster Breite. Weil der Jet ost–west läuft, sehen zwei Stationen gleicher Breite, verschiedener Länge *verschiedene* MLT-Segmente — komplementäre Sektoren der Onset-Verteilung (Maximum um Mitternachts-MLT) —, während Stationen gleicher Länge, verschiedener Breite dasselbe Segment nur abgeschwächt sehen: redundant. Eine Station schlägt das Netz, weil die Fusion einen gemeinsamen Onset über unabhängige Stations-Likelihoods annimmt: jede signallose Station wird zum Vetospieler, 30 Rauschfaktoren begraben den Likelihood-Quotienten der einen informativen Station multiplikativ — 6,6 % → 0,2 % → 0,01 % ist exakt diese Verdünnungssignatur (zzgl. Timing-Verschmierung durch über Stationen gemittelte Onset-Zeiten). Konsequenz: Breite fest auf das Zielaktivitätsniveau legen, im Längengrad spreizen, Fusion als Union mit Signalindikator pro Station (abwesend ≈ Faktor 1, kein Veto); Mittelung nur innerhalb ko-lokierter Cluster.

**(c) Kette**

**Verdikt:** Der Bau trägt als **Notwendigkeits-Audit** (jede Stufe muss in beiden Währungen positiv sein), aber nicht als hinreichender Wirksamkeitsnachweis — und das schwächste Glied ist die eigene Boden-Detektionsstufe (Rückruf-Deckel 6,6 %), das die TE-Metrik nicht einmal sieht.

**Begründung:** Die zwei Achsen fangen komplementäre Fehlstufen ab — Echo-Stufen (Vorlauf ohne neue Information) und Hindsight-Stufen (Information ohne nutzbaren Vorlauf) —, das ist korrekte doppelte Buchhaltung. Gelingensbedingungen: TE strikt kausal mit Lag ≥ beanpruchtem Vorlauf schätzen (sonst zählt instantane Kopplung als Frühwarn-Information), und TE-Kernel/Embedding fest verankern statt auf maximales ΔTE zu tunen — sonst ist die Toleranzfalle aus (a) auf jeder Stufe zurück; im Rare-Event-Regime zusätzlich per Ereignis-Resampling absichern, sonst sind die Inkremente kleiner als ihre Fehlerbalken. Das eigentliche Loch: TE misst mittlere Informationsrate, nicht ereignisbezogenen Rückruf — eine Stufe kann positive ΔTE liefern und trotzdem 93 % der Onsets verpassen; da die Kette Trefferwahrscheinlichkeiten multiplikativ durchreicht, deckelt die Boden-Stufe den End-zu-End-Wert, egal was Sonne und L1 liefern (das Maß aus (a) gehört als dritte Achse hierher). Intrinsisch am schwächsten ist zudem Sonne→L1, die Bz-/Geoeffektivitäts-Lücke: dort ist der Vorlauf am größten und die inkrementelle Information über die Bodenwirkung nahezu klimatologisch — die Kette kauft ihre Information erst, wenn der Vorlauf fast verbraucht ist.

---

## 05 · DeepSeek
- **Site:** https://chat.deepseek.com/
- **Modell/Einstellung:** DeepSeek V4 Pro · tiefes Denken · erweiterte Suche
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) Schärfe**  
Verdikt: Nimm die random-null-korrigierte Toleranz-AUC, \(S=\max_k[\int_0^\infty (TPR_k(\tau)-FPR_k(\tau))\,d\tau-\lambda(k-1)]\).  
Begründung: \(TPR\) zählt Onsets innerhalb \(\pm\tau\), \(FPR\) denselben Treffer im zeitverschobenen Zufalls-Null; damit wächst bei größerem \(\tau\) auch die Zufallstrefferquote, und die Differenz schönt sich nicht. Die AUC integriert über alle Toleranzen statt eine willkürliche zu wählen. Der Strafterm für \(k\) verhindert, dass viele Stationen scheinbare Schärfe kaufen.

**(b) Platzierung**  
Verdikt: Stationen müssen im/am erwarteten Polarlicht-Elektrojet liegen, bevorzugt gleiche geomagnetische Breite und verschiedene Längengrade/MLT-Sektoren, und als ODER-Netz verknüpft sein.  
Begründung: Der Elektrojet ist schmal und lokal; Breite bestimmt Nähe/SNR, Länge den Nachtsektor/MLT. Eine einzelne höchste Station schlägt das Netz, weil zusätzliche Stationen außerhalb des Jets bei UND/Mittelung nur Fehlzeiten und Rauschen einbringen. Bei ODER kann der Trefferanteil nicht fallen; die Gefahr steigt dann bei falschen Alarmen, nicht beim Rückruf.

**(c) Kette**  
Verdikt: Der Bau trägt als Frühwarnkette nur bedingt; das schwächste Glied ist die Ionosphären-Boden-/Netzstufe.  
Begründung: Vorlauf und bedingte Transfer-Entropie sind sinnvoll, aber TE-Zuwächse sind nicht additiv und bei seltenen, nichtstationären Onsets überfit-anfällig. Frühe Sonnen-/Sonnenwindstufen liefern Zeit, aber wenig spezifische Sicherheit; die Bodenmagnetometer-Detektion hat mit ~6,6 % Rückruf und fallendem Rückruf bei mehr Stationen den geringsten Sicherheits-Zuwachs. Ohne robusten Schwellenwert bleibt sie der Flaschenhals.

---

## 06 · Mistral
- **Site:** https://chat.mistral.ai/
- **Modell/Einstellung:** Mistral Large · Standard
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) SCHÄRFE**
Verdikt: Maximale AUC über alle Stationskombinationen und Toleranzstufen.
Begründung: Die AUC ist schwellunabhängig und misst die grundsätzliche Trennschärfe zwischen Onset- und Null-Verteilung. Ihr Maximum über alle Konfigurationen identifiziert die beste Diskriminationsleistung ohne Verzerrung durch Toleranzlockerung oder Stationszahl. Sie aggregiert die Information aller Stationen und ist robust gegen mechanische Schönung.

---

**(b) PLATZIERUNG**
Verdikt: Stationen müssen eng gebündelt im Polarlicht-Elektrojet-Gürtel (hohe geomagnetische Breite) und auf ähnlicher Länge liegen.
Begründung: Der Auroral Electrojet ist in geomagnetischer Breite stark lokalisiert; Stationen außerhalb fügen nur unkorreliertes Rauschen hinzu. Selbst innerhalb des Gürtels verwässern zusätzliche Stationen das Signal durch lokale Variationen, daher übertrifft eine optimale Einzelstation das Netz. Breitenfokussierung dominiert über Längenabdeckung, da der Elektrojet sich ost-west erstreckt.

---
**(c) KETTE**
Verdikt: Der Bau trägt nicht zuverlässig; das schwächste Glied ist die Sonne→Magnetosphäre-Stufe.
Begründung: Diese Stufe bietet langen Vorlauf, aber minimale inkrementelle Transfer-Entropie, da Sonnenwindparameter nicht deterministisch geomagnetische Störungen am Boden auslösen. Spätere Glieder liefern hohe Sicherheit bei kurzem Vorlauf, doch die Kette ist nur so stark wie ihr informativ schwächstes Glied.

---

## 07 · Gemini AI Studio
- **Site:** https://aistudio.google.com/app/prompts/new_chat
- **Modell/Einstellung:** Gemini 3.1 Pro Preview · Grounding mit Google Search
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

(a) SCHÄRFE Verdikt: Als Maß eignet sich der „Excess Recall“ (Netto-Rückruf)
oder ein verwandter Skill-Score (z. B. True Skill Statistic), bei dem die
Trefferquote des Zufalls-Nullmodells von der gemessenen Trefferquote subtrahiert
wird. Begründung: Eine gelockerte Zeittoleranz erhöht zwangsläufig die
Wahrscheinlichkeit für rein statistische Treffer, was einfache Quoten
verfälscht. Zieht man die Ergebnisse der zeitverschobenen Zufalls-Null für exakt
dieselbe Toleranz und Stationszahl ab, isoliert man den tatsächlichen, kausalen
Informationsgewinn des Erkennungssystems. Dadurch wird das Maß resistent gegen
mechanische Schönungen durch große Zeitfenster und ermöglicht eine saubere
Optimierung über beide Achsen.

(b) PLATZIERUNG Verdikt: Um den Trefferanteil bei Erweiterung des Netzes zu
erhalten, müssen weitere Stationen rein longitudinal entlang derselben hohen
geomagnetischen Breite (direkt im Fußabdruck des Polarlicht-Elektrojets)
angeordnet werden. Begründung: Bay-Onsets (Substorms) entstehen räumlich stark
begrenzt im nachtseitigen auroralen Oval und ihre Signatur schwächt sich in
Richtung Äquator drastisch ab. Nimmt man latitudinal (breitengradig) tiefer
liegende Stationen in eine Netzwerk-Bedingung auf, verfehlen diese oft die
Ansprechschwelle und ziehen die Gesamt-Erkennungsrate extrem nach unten. Eine
rein longitudinale (längengradige) Abdeckung auf hoher Breite gleicht hingegen
die Erdrotation aus und sichert lediglich ab, dass sich unabhängig von der
Tageszeit immer eine Station im hochaktiven Nachtsektor befindet.

(c) KETTE Verdikt: Diese Architektur ist theoretisch äußerst tragfähig zur
Quantifizierung von echten Vorhersagewerten, hat ihr schwächstes Glied jedoch in
der stark nichtlinearen Kopplung zwischen dem L1-Lagrange-Punkt und der
Magnetosphäre/Ionosphäre. Begründung: Die Messung der inkrementellen
Transfer-Entropie ist ideal, um Redundanzen herauszufiltern; sie zeigt streng
an, ob eine Stufe neue Information liefert oder nur bereits Bekanntes verzögert
weitergibt. Das Nadelöhr ist der Sprung vom Sonnenwind (L1) zum Boden, da die
Auslösung von Onsets stark von internen Magnetosphären-Dynamiken (wie
magnetischer Rekonnektion) abhängt. Diese lokalen, abrupten Entladungen weisen
eigene Dynamiken auf, wodurch die messbare Transfer-Entropie aus den reinen
Sonnenwinddaten massiv einbricht, selbst wenn die Vorlaufzeit noch komfortabel
erscheint.


---

## 08 · MiniMax
- **Site:** https://agent.minimax.io/
- **Modell/Einstellung:** MiniMax M3
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) Verdikt:** Schärfe S(N) := die engste Toleranz τ\*, bei der R(N,τ) das Random-Shift-Nullmodell noch um Δ übersteigt — also S(N)=τ\*(N); kleinere Teler = schärfer, mehr Stationen sind nur dann ein Gewinn, wenn sie τ\* tatsächlich verengen.

**Begründung:** Indem S gegen den Random-Shift-Null kalibriert wird (nicht gegen R allein), kann eine gelockerte Toleranz das Maß nicht mehr mechanisch schönen — R_rand steigt同样 mit τ, also frisst jede Lockerung die Aussage auf. Beide Achsen gehen in *ein* Maß ein, über die τ\*-Wirkung von N. Schärfere Variante mit mehr Trennschärfe: τ\* bei HSS ≥ 0.1 (oder PSS ≥ 0.1) gegen den Random-Null — dann ist „engste Toleranz, bei der das Netz skill hat" direkt der Wert.

---

**(b) Verdikt:** Anordnung entlang der Auroral-Oval-*Länge* bei Spitzen-Elektrojet-MLAT, Stationsabstand ≈ räumliche Kohärenzlänge (~500–1000 km) — Longitude trägt Tracking, Breite trägt Amplitude.

**Begründung:** Das elektrojetische Signal ist räumlich kohärent auf ~100–500 km (Stromkeil-Skala); eine Station nahe Oval-Maximum erfasst den Großteil der Störung in einem Stück. Jede weitere Station in *niedrigerer* Breite liegt außerhalb des Elektrojets, feuert zufällig auf den Bay-Trigger und zieht den Rückruf herunter — der Vote/CoNatch bricht das; unter einem Koinzidenz-Kriterium fallen Onsets, die nur eine Station sieht, durchs Sieb. Grund, warum eine Station ein Netz schlägt: das Signal ist räumlich größer als die räumliche Trennschärfe der Aufgabe; Onset-DETECTION ist kein Lokalisierungs-Problem. Eine Längs-Anordnung *innerhalb* des Ovals (Longitude, nicht Breite) ist die einzige Geometrie, die zusätzlich etwas hinzufügen kann — der Stromkeil wandert beim Substorm entlang des Ovals, also sehen verschiedene Längen den Onset zu leicht verschiedenen Zeiten.

---

**(c) Verdikt:** Trägt von Sonne→Magnetosphäre; bricht am Boden.

**Begründung:** Lead-Time fällt monoton entlang der Kette (~60 min L1, 5–30 min MP, ~0 s Boden), und inkrementelle TE ist an L1→MP real messbar (~0.1–0.3 bit, gut etabliert). Am Boden ist Lead-Time ≈ 0 und der inkrementelle TE gegenüber der Ionosphäre wird durch Sq und die gewöhnliche tageszeitliche Schwankung kontaminiert — er sieht prädiktiv aus, ist es aber nicht. Schwächstes Glied: der Boden-Magnetometer — er DETEKTIERT (Gleichzeitigkeit), er prognostiziert nicht, und bricht die Vorlauf-Versicherung der Kette. Ehrlich gesagt ist die L1→MP-Stelle die *zweitschwächste* (Bz rotiert im ~30-min-Solarwind-Transit, klassisches 1-AU-Vorhersage-Problem), aber der Boden bricht die Versprechen der Kette am stärksten.

---

## 09 · Kimi
- **Site:** https://www.kimi.ai/
- **Modell/Einstellung:** Kimi · schnell · hoch
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**Kurz vorweg:** Das gemessene Muster (1 Station > 6 > 31) ist das klassische Signatur-Dilutions-Phänomen — das prägt alle drei Antworten.

---

**(a) SCHÄRFE**
*Verdikt:* „Am schärfsten" = maximaler **chance-normierter Rückruf (Lift = Rückruf / Zufalls-Rückruf) bei einer fixen, vorab registrierten Toleranz** — die Toleranz ist Nebenbedingung, nicht Zielachse.
*Begründung:* Jedes über die Toleranz integrierte Maß (AUC, Mittelwert über Toleranzen) wächst mechanisch mit gelockerter Toleranz und ist damit spielbar; nur ein anwendungsvorgegebener Punkt (z. B. der früheste für die GIC-Warnung nutzbare Vorlauf) verhindert Toleranz-Shopping. Die Chance-Normierung baut den Nachweis „schlägt den Zufall" direkt in den Score ein. Da die Schwellenkurve monoton ist (kein inneres Optimum), repräsentiert dieser eine Arbeitspunkt die ganze Kurve — kein Cherry-Picking möglich. Stationszahl bleibt die einzige freie Achse: schärfste Konfiguration = argmax Lift bei fixer Toleranz.

---

**(b) PLATZIERUNG**
*Verdikt:* Stationen müssen in der **Kohärenzzone desselben Signals** liegen — entlang des Elektrojets auf Auroralbreite, in der Breite eng bündig, in der Länge über die Jet-Ausdehnung verteilt — sonst verdünnt die Fusion das lokale Signal.
*Begründung:* Ein bay ist die Signatur einer **lokal** intensivierten Elektrojet-Sektion: nur Stationen unterhalb des Jets sehen ihn, alle anderen liefern näherungsweise unabhängiges Rauschen; eine Fusion aus Mittelung oder Koinzidenzlogik mischt ein lokalisiertes Signal mit N−1 Nullen — der Rückruf fällt daher überproportional (mehr als reine 1/N-Verdünnung). Die höchstbreitige Station gewinnt klimatologisch: dort liegt der Oval am häufigsten und aktivsten. Geometrie: Der Jet ist in Längsrichtung lang (~10³ km), in der Breite schmal (~10² km) und wandert aktivitätsabhängig — Längsabdeckung sichert UT-/Lokalzeit-Sektoren (Onsets bevorzugen den Mitternachtssektor), Breitenabdeckung hedgt die Ovalwanderung; **Breitenfehler sind deutlich teurer als Längenfehler**. Hinweis: Mit Max-/ODER-Fusion statt Mittelung/Quorum kann jede weitere Kohärenz-Station den Rückruf nur erhöhen, nie senken.

---

**(c) KETTE**
*Verdikt:* Der Bau trägt, sofern jede Stufe **bedingte (inkrementelle) TE über alle Vorstufen** gegenüber verschobenen-Zeit-Surrogaten zeigt und zeitlich vor ihrer Nachfolgestufe liegt; das schwächste Glied ist die **letzte Stufe (Solarwind/Magnetosphäre → konkreter Boden-Onset)**.
*Begründung:* In einer seriellen Kette multiplizieren sich die bedingten Trefferwahrscheinlichkeiten — dominiert wird sie vom Glied mit dem kleinsten Verhältnis inkrementelle TE zu verbrauchtem Vorlauf, und das ist empirisch die Bodenstufe (Rückruf 6,6 %). Physikalisch folgt dasselbe daraus, dass die Substurm-Triggerzeit der am wenigsten determinierte, chaotischste Teil des Systems ist (Wind→Dst/PC ist gut vorhersagbar, Wind→Onset-Zeitpunkt nur schwach). Methodisch muss die inkrementelle TE konditioniert auf die eigene Vorgeschichte und diskretisierungsrobust geschätzt werden, sonst täuscht Autokorrelation Transfer vor; bricht ein frühes Glied (z. B. CME-Ankunftszeit), wird das Defizit allerdings stets zuerst am Boden sichtbar.

---

## 10 · Lumo (Proton)
- **Site:** https://lumo.proton.me/
- **Modell/Einstellung:** Lumo 2.0 Max · Websuche
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) SCHÄRFE**  
**Verdikt:** „Am schärfsten“ ist die Kombination mit maximalem chance-korrigiertem Skill, konkret **TSS/Youden-J**: \(S(\tau,n)=\text{Recall}_{\text{Onset}}(\tau,n)-\text{Recall}_{\text{Null}}(\tau,n)\).  

**Begründung:** Für jede Toleranz \(\tau\) und Stationszahl \(n\) wird die Trefferquote gegen die echten Onsets und gegen die verschobene Null gemessen. Eine größere Toleranz erhöht beide Quoten; der Score steigt nur, wenn der echte Überschuss über Zufall wächst. Damit werden auch zusätzliche Stationen nur dann belohnt, wenn sie den Null-Überschuss erhöhen, statt den Rückruf mechanisch zu verwässern.

**(b) PLATZIERUNG**  
**Verdikt:** Stationen sollten im elektrojet-nahen Breitenband bleiben, aber in Länge/MLT verteilt sein und als **Oder-Verbund** statt als Koinzidenzgatter ausgewertet werden.  

**Begründung:** Der Polarlicht-Elektrojet ist in geomagnetischer Breite schmal, in Länge/MLT aber ausgedehnt und oft lokal/propagierend. Eine einzelne Station unter oder nahe dem Elektrojet sieht das stärkste Signal; zusätzliche Stationen außerhalb oder mit UND-Verknüpfung wirken dann wie Filter und entfernen echte Treffer, deshalb kann eine einzelne Station ein Netz schlagen. Breitenabdeckung entscheidet über die Kopplung an den Elektrojet, Längenabdeckung über die Erfassung verschiedener lokaler Zeiten.

**(c) KETTE**  
**Verdikt:** Der Bau trägt als Informations- und Zeitbudget, aber nur, wenn jede Stufe bedingt positive inkrementelle Transfer-Entropie liefert; das schwächste Glied ist das Minimum dieses Zuwachses, praktisch meist der Übergang Sonnenwind/Magnetosphäre → Boden/Elektrojet.  

**Begründung:** Eine Stufe darf nicht nur Vorlaufzeit kosten, sondern muss die Unsicherheit über den Boden-Onset über die Vorstufe hinaus reduzieren. Gerade die letzte physikalische Kopplung ist nichtlinear, lokal und zustandsabhängig, daher ist der Sicherheits-Zuwachs dort oft klein und die Restvorlaufzeit kurz. Zusätzlich ist Transfer-Entropie bei kurzen, autokorrelierten Zeitreihen schätz- und testbedürftig, sonst entstehen Schein-Zuwächse.

---

## 11 · Perplexity
- **Site:** https://www.perplexity.ai/
- **Modell/Einstellung:** Deep Research · Suche
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) Verdikt:** Nutze die **flächennormierte Trefferdichte** \(S=\frac{\#\text{Treffer}}{A_{\text{akzeptiert}}\cdot N_{\text{Stationen}}}\) bzw. bei fixer Stationszahl \(S=\#\text{Treffer}/A\); das ist „Schärfe“ als Treffer pro Toleranzfläche.

Begründung: Sie belohnt mehr Treffer, bestraft aber jede Vergrößerung des Akzeptanzfensters linear mit dessen Fläche \(A\) — eine gelockerte Toleranz kann das Maß daher nicht mechanisch schönen. Bei Vergleich über Stationszahlen wird zusätzlich durch \(N\) normiert, sonst gewinnt trivialerweise das größere Netz. Wichtig: \(A\) muss aus den tatsächlichen Zeit- und Stationsabstandseinheiten gebildet werden (z. B. \(\Delta t \times \Delta\lambda_{\text{mag}}\)), nicht aus bloßen Rang- oder Schwellenwerten; alternativ ist Precision-at-fixed-recall bzw. AUC bei fixierter Toleranz die robustere Variante, wenn die Onset-Liste sehr lückenhaft ist.

**(b) Verdikt:** Stationen müssen als **dichte Meridian-Kette quer zum Polarlicht-Elektrojet** liegen — vor allem Breitenabdeckung durch die Auroralzone, nicht flächige Längenausdehnung.

Begründung: Der Elektrojet verläuft ost-westlich entlang konstanter magnetischer Breite; sein Ort und seine Intensität werden daher primär durch ein Nord-Süd-Profil erfasst, wie es Meridianketten für das Tracking der Elektrojet-Grenzen nutzen.  Eine einzelne günstigere Station kann ein Netz schlagen, wenn sie näher am typischen Elektrojet-Maximum liegt, während zusätzliche Stationen außerhalb der aktiven Breitenzone nur Rauschen bzw. Mittelung beisteuern und die scharfe lokale Signatur verwässern.  Längenabdeckung ist nur für globale/MLT-Erfassung nötig; für einen regionalen Onset-Detektor dominiert die Breitenauflösung, weil Störungen in der Auroralzone kleinräumig und stark variabel sind. [swsc-journal](https://www.swsc-journal.org/articles/swsc/full_html/2013/01/swsc130002/swsc130002.html)

**(c) Verdikt:** Der Bau trägt **als Diagnose-, nicht als Beweis-Kette**: Vorlauf und inkrementelle Transfer-Entropie sind sinnvolle Stufen-Metriken, aber das schwächste Glied ist die kausale Attribution zwischen den Stufen.

Begründung: Transfer-Entropie misst prädiktiven Informationszuwachs, nicht zwingend physische Kausalität; common drivers wie südwärtiges IMF können Scheinflüsse erzeugen, weshalb konditionierte TE und Placebo-/Zeitverschiebungs-Kontrollen nötig sind.  Zudem hängt der gemessene Nutzen jeder Stufe stark von der Stationsgeometrie ab — genau der Effekt aus (b) — sodass eine „schlechte“ Bodenstufe die Kette brechen kann, obwohl Sonne→Magnetosphäre intakt ist. Praktisch: Vorlauf getrennt je Stufe quantifizieren, TE nur gegen verschobene Nullmodelle und unter Konditionierung auf den gemeinsamen Treiber interpretieren. [arxiv](https://arxiv.org/pdf/2206.13992.pdf)

---

## 12 · Sakana — Namazu
- **Site:** https://chat.sakana.ai/
- **Modell/Einstellung:** Namazu · High
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) SCHÄRFE**

**Verdikt:** Als Schärfemaß dient der Lift gegen die Zufalls-Null, definiert als Verhältnis des empirischen Recalls zum erwarteten Zufalls-Recall für jede Kombination aus Toleranz und Stationszahl, denn so wird der bloße mechanische Gewinn eines größeren Zeitfensters eliminiert.

**Begründung:** Bei zufälligen Vorhersagen wächst die Trefferwahrscheinlichkeit linear mit der Toleranzfensterbreite, weshalb der absolute Recall durch bloßes Aufweiten des Fensters künstlich erhöht wird. Das Lift-Maß normiert den empirischen Recall auf diese Baseline und misst ausschließlich den überzufälligen Vorhersagewert. Dadurch wird verhindert, dass eine gelockerte Toleranz das Maß schönt, und Toleranz sowie Stationszahl sind auf einem gemeinsamen, fairen Niveau vergleichbar.

---

**(b) PLATZIERUNG**

**Verdikt:** Stationen müssen in der geomagnetischen Breite dicht um die Auroralzone gruppiert sein, nicht longitudinal verteilt; eine einzelne optimale Station schlägt das Netz, weil der Polarlicht-Elektrojet in der Länge homogen und in der Breite scharf lokalisiert ist.

**Begründung:** Der aurorale Elektrojet ist eine ost-west verlaufende, breitenfixierte Struktur, sodass Längenabdeckung kaum zusätzliche Information liefert, während Breitenabdeckung exakt auf die aktive Zone zielen muss. Stationen außerhalb dieser Zone tragen nur unkorreliertes geomagnetisches Rauschen bei, das bei einer naiven Netzwerk-Fusion entweder die Schwelle anhebt oder Fehlalarme vermehrt und so den Recall senkt. Erst wenn alle Stationen im gleichen Breitenband denselben Signalraum abtasten, bleibt der Trefferanteil bei steigender Stationszahl erhalten.

---

**(c) KETTE**

**Verdikt:** Der Bau trägt dann, wenn jede Stufe eine positive inkrementelle Transfer-Entropie (ITE) aufweist; das schwächste Glied ist die Boden-Magnetometer-Stufe, da sie trotz kürzester Vorlaufzeit den geringsten marginalen Informationszuwachs liefert und durch rauschbehaftete Mehrstationen-Netzwerke weiter entropiebehaftet wird.

**Begründung:** Die ITE quantifiziert den nicht-redundanten Vorhersagewert, den eine Stufe über alle vorherigen hinweg zum Endereignis beisteuert; eine Stufe mit ITE ≈ 0 ist für die Kette nutzlos. Da selbst die beste Einzelstation nur einen Recall von ~6,6 % erreicht und jede weitere Station diesen drückt, ist die Informationsqualität der Bodenebene gegenüber früheren Stufen am niedrigsten. Das schwächste Glied sitzt daher am Ende der Kette, wo physischer Vorlauf gegen Null geht und die Datenlage durch Netzwerkrauschen am fragilsten ist.

## 13 · Sakana — Fugu
- **Site:** https://chat.sakana.ai/
- **Modell/Einstellung:** Fugu · Max · High
- **Zeit:** 2026-10-15 08:14Z
- **Antwort:**

**(a) SCHÄRFE**

* **Verdikt:** Das Maß der Wahl ist der **normierte relative Informationsgewinn** bzw. die **zeitbezogene Kullback-Leibler-Divergenz gegenüber der zeitverschobenen Null-Hypothese** ($I(T;\Delta t) = \text{Recall}(K, \Delta t) \cdot \ln\left[\frac{\text{Recall}(K, \Delta t)}{\text{Null-Rate}(K, \Delta t)}\right]$).
* **Begründung:** Eine mechanische Weitung des Zeitfensters $\Delta t$ erhöht zwar die absolute Trefferquote, lässt die Null-Rate der Zufallskontrolle jedoch mindestens linear anwachsen. Übersteigt der Signalzuwachs nicht das exponentiell wachsende Rauschen des Zufallsprozesses, sinkt dieser Informationsgewinn ab. Indem man die Metrik strikt an die Differenzierung gegenüber der permutierten/verschobenen Null-Verteilung koppelt, bestraft sie koinzidente Scheintreffer bei gelockerten Schranken sofort mathematisch.

---

**(b) PLATZIERUNG**

* **Verdikt:** Die Einzelstation schlägt das Netz, weil eine **starre UND-Koinzidenz über raumzeitlich hochgradig lokalisierte Phänomene** angewendet wird, anstatt eine **stochastische ODER- bzw. $k$-aus-$N$-Logik entlang des magnetischen Mitternachtsmeridians**.
* **Begründung:** Der Polarlicht-Elektrojet (auroral electrojet) bricht substurminduziert eng begrenzt im Mitternachtssektor aus (1–2 Stunden MLT, wenige Breitengrade) und bildet ein multipolares, dynamisches Stromsystem. Eine globale UND-Bedingung multipliziert Fehlstellen und lokale Blindheiten, wodurch der Gesamtrückruf mit jeder weiteren Station geometrisch einbricht. Stationen müssen daher als **Längenspreizung** (zur Erfassung des lokalen Zeitfensters um Mitternacht) mit **dichten meridionalen Ketten** angeordnet und über ein Schwellwertmodell gekoppelt werden, das bereits das lokale Vorbeiziehen des Stromelements als Gesamtereignis wertet.

---

**(c) KETTE**

* **Verdikt:** Die Kette trägt als formal-theoretisches Kausalitätsgerüst, besitzt ihr **schwächstes Glied jedoch am Phasenübergang Magnetosheath/Bow-Shock $\to$ innere Magnetosphäre (L1-In-situ-Messung zu Boden/Ionosphäre)**.
* **Begründung:** Während Sonnenbeobachtungen (Korona) zu L1 (Solarwind) relativ deterministischen MHD-Transportgesetzen mit hoher Vorlaufzeit folgen, ist die magnetosphärische Kopplung intrinsisch metastabil. Das Laden und dissipative Entladen des Magnetschweifs (Tail-Reconnection, Substorm-Onset) erfolgt hochgradig nichtlinear mit minimaler Vorwarnzeit im Minutenbereich. Inkrementelle Transfer-Entropie bricht an dieser Schnittstelle oft ein, da L1 zwar die Randbedingung (z. B. $B_z$ negativ) liefert, der exakte Trigger und Ort des Boden-GIC aber von internen Instabilitäten der Magnetosphäre abhängt.

