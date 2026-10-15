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
| 01 | Duck.ai | GPT (primär OpenAI) · kein Modell wählbar · Websuche | `https://duck.ai/` | river-ui |
| 02 | Claude | Claude Sonnet 5.5 · „extra hoch" | `https://claude.ai/new` | river-ui |
| 03 | Qwen | Qwen 3.8 Max · Think | `https://chat.qwen.ai/` | river-ui |
| 04 | Z.ai / GLM | GLM 5.3 · Deep Think Max · Advanced Search | `https://chat.z.ai/` | river-ui |
| 05 | DeepSeek | DeepSeek V4 Pro · tiefes Denken · erweiterte Suche | `https://chat.deepseek.com/` | river-ui |
| 06 | Mistral | Mistral Large · Standard — **Kontingent leer (river-217)**: erst prüfen, sonst auslassen | `https://chat.mistral.ai/` | river-ui |
| 07 | Gemini (AI Studio) | Gemini 3.1 Pro Preview · Grounding mit Google Search | `https://aistudio.google.com/app/prompts/new_chat` | river-ui |
| 08 | MiniMax | MiniMax M3 | `https://agent.minimax.io/` | river-ui |
| 09 | Kimi | Kimi · schnell · hoch (kein K3 über kimi.ai) | `https://www.kimi.ai/` | river-ui |
| 10 | Lumo (Proton) | Lumo 2.0 Max · Websuche | `https://lumo.proton.me/` | river-ui |
| 11 | Perplexity | Deep Research · Suche | `https://www.perplexity.ai/` | river-ui |
| 12 | Sakana — Namazu | Namazu · High | `https://chat.sakana.ai/` | river-ui |
| 13 | Sakana — Fugu | Fugu · Max · High | `https://chat.sakana.ai/` | river-ui |

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
