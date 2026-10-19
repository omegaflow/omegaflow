# Rat + voller Roster — TE-Joint-Grid bei heterogener Auflösung (River 171, 2026-10-10)

Operator-Wort 2026-10-10: „bitte --all rat und roster" + „das ist kein roster wir haben deutlich
mehr sitze". Vorbereitung: `archive_search --all`
(`state/stimmen/2026-10-10-river-te-joint-grid-research.md`). Frage: (A) globales Bin ·
(B) per-Paar-Bin `max(native Raten)` · (C) Multi-Timescale-TE · (D) Summary-Causal-Graph /
causal temporal mutual information über Sampling-Raten.

## Rat (fünf Verfassungsstimmen)

**Verdikt: (B) — per-Paar-Bin, in diesem Atom** (Substrat; Skala je Zelle; C registriert;
D descoped; A gestrichen). Riss: B macht cross-rate-Paare am größeren τ darstellbar, aber die
schnelle Kopplung am groben Gitter ist eine grobkörnige Messung.

## Roster (UI/Open-Weight)

| Seat | Verdikt | Riss |
|---|---|---|
| **Duck.ai** (lief Haiku 4.5) | (D) mit lokalem (B)-Fallback | Nyquist/Aliasing |
| **DeepSeek V4 Pro** (tryingopen) | (D) als Rückgrat + (C); A falsch, **B nur bedingt** (macht TE-Werte über die Matrix **unvergleichbar**) | Abtastratengrenze |
| **DeepSeek Chat** (chat.deepseek.com) | (C) oder (D); (A) unzulässig, (B) Notlösung | asynchrone lead-lag-Struktur über Skalen |
| **Qwen3.8 27B** (tryingopen) | weder A noch B; **(C)** — (D) ist die formale Schicht, keine Alternative (Summary-Graph = Aggregat, nicht Messung) | echte Zeitlückenskala 6 s ↔ monatlich |
| **GLM 5.3 Flash** (tryingopen) | **(C)+(D)**, nicht A/B; (B) macht Zellen **inkomparabel**, häckselt schnelle Paare; TE je Skalenband (10⁰–10¹ s / 10²–10³ s / >10⁵ s) | Skaleninkommensurabilität |
| **MiniMax M3** (agent.minimax.io) | (A) Artefakt; (B) fabriziert Spurious-TE (Upsampling) **oder** wirft AIA-Bursts weg; **(C) Spektrum TE(τ)**; (D) zwingend (PCMCI, Runge 1702.07077) — per Paar Skalentripel τ=(τ_bin, τ_lead, τ_lag) | das Ratenverhältnis ist die Messung |
| **Lumo 2.0 Max** (Proton, anonym) | (D) vorzuziehen; (C) als Sensitivitätsanalyse; (B) opfert Matrix-Konsistenz | zeitliche Asymmetrie / kausales Vorzeichen |
| **Perplexity** | **(C)** primär; (D) Summary-Graph als Aggregat; **(B) nur als Fallback** je Paar, nur innerhalb gleicher Paar-Klassen vergleichen, nie global; (A) ablehnen | physikalische Prozesszeit vs. Beobachtungs-/Sampling-Zeit |

**Pending (gemessen, nicht stille Null):** Kimi (`kimi.ai`) — „Ihr kostenloses Kontingent ist
aufgebraucht" (**Quota**); Claude — Login-Wall; Qwen (`chat.qwen.ai`) — „Token has expired";
Mistral — `v2.auth.mistral.ai/login`; Gemini/AI Studio — Google-Login; Z.ai-Chat — Nachricht
gesetzt, keine Antwort gerendert. (DeepSeek V4 Flash: Tab offen, nicht gesendet.)

## Runde 3 — die STÄRKSTEN Open-Weight-Seats (Operator-Korrektur 2026-10-10)

**Fehler, benannt:** in Runde 2 liefen die schwachen tryingopen-Varianten (Qwen3.8 **27B**,
GLM 5.3 **Flash** 320B), gewählt nach Bequemlichkeit (sprangen schnell an), nicht nach
Fähigkeit — Verstoß gegen „Architekturfragen im stärksten Modus". Volle Liste gelesen; die
stärksten nachgezogen:

| Seat (Größe) | Verdikt | Riss |
|---|---|---|
| **Qwen3.8 2.4T** (2.4T) | **B als Fundament (block-aggregiert aufs GROBE Gitter) + C als zweite Schicht + D als Synthese**; A verwerfen. „`max(native Raten)` darf nicht ‚feinstes Gitter' heißen" — das schnelle Feld wird blockweise ins Monats-/Wochengitter aggregiert (Block-Mittel, Varianz, Entropie als Kanäle) | Richtungsasymmetrie ist Physik: schnell→langsam schätzbar; langsam→schnell ≈ 0 — **nie die langsamen Reihen aufs schnelle Gitter interpolieren/up-sampeln, nie beide Richtungen zu einem symmetrischen Wert mitteln** |
| **Inkling** (975B, Thinking Machines) | **(D)** — Runge SCM + asynchrone lead-lag; (A) tötet 6-s-AIA, (B) nicht vergleichbare Matrix, (C) skalenfixiert | physikalische Zeitskalen-Lücke |
| **GLM 5.3** (753B, Z.ai) | **(C) oder (D)** (Denk-Stand; final pending): (B) gibt gültige Paar-TE, aber die Matrix mischt inkommensurable Zeitbasen — Zellen nicht vergleichbar; per Paar auf nativer/kompatibler Skala, dann skalen-explizit in den Summary-Graph | Inkommensurabilität / kein gemeinsamer Takt; **Up-Sampling Monat→12 s = 223 200 Pseudo-Replikate/Monat = Schein-Signifikanz** |

**Der korrigierte Riss:** die schwachen Seats (Runde 2) antworteten uniform „(C)+(D), B nur
Fallback". Die **starken** Seats sind differenzierter: **Qwen3.8 2.4T rehabilitiert (B) als
Fundament** (mit der genauen Kante: Aggregation aufs *grobe* Gitter, nicht `max` = feinstes) —
und konvergiert damit **mit dem Rat** (B-Substrat), während Inkling/GLM (C)/(D) tragen. Der
scheinbare „6× gegen den Rat" aus Runde 2 war ein Artefakt schwacher Modelle. Die getragene
Linie ist damit **B (block-aggregiert) + C (Skalenspektrum) + D (Summary-Graph/PCMCI)**, mit dem
unverhandelbaren Riss: **nie Up-Sampling der langsamen Reihe, nie symmetrische Mittelung der
Richtungen**.

## Der Riss (nicht geglättet)

Rat: **B** als Substrat. Roster (7 antwortende Seats): **6× C+D**, **1× D+B-Fallback** — **kein
Seat stützt B als Substrat**. Konvergenz aller Linien: (A) falsch; die **Abtastratengrenze/
Prozess-vs-Sampling-Zeit** ist der ungeglättete Riss; (C) ist die Messung, (D) die formale
Schicht. Divergenz: der Rat baut B zuerst (Darstellbarkeit), der Roster misst erst C und
formalisiert mit D; B bleibt nur als **dokumentierter Fallback** und dann **nur innerhalb
gleicher Paar-Klassen**, nie global verglichen.

Damit ist das Rat-Verdikt **nicht durch den Roster untermauert** — die UI-Konvergenz rippt es.
Kein Bau von B als Substrat; die gewichtete Richtung ist **C (Multi-Timescale, TE je
Skalenband/lag) + D (Formalisierung)**, B als gekennzeichneter Fallback.

> **Korrektur (Runde 3/4):** der Abschnitt „Der Riss" oben ist der Stand der schwachen Runde 2.
> Die stärksten Seats (Runde 3/4) präzisieren: B bleibt als *Basis* tragfähig — aber nur
> block-aggregiert aufs gröbere Gitter, mit Skalen-/N_eff-Annotation. Siehe unten.

## Runde 4 — die Login-Seats (Operator eingeloggt, 2026-10-10)

| Seat | Verdikt | Riss |
|---|---|---|
| **Claude** (Sonnet 5.5 **Extra hoch**) | **(B) als operative Basis, interpretiert als (D)** (Summary-Graph auf der gröberen Skala); (A) entfällt; (C) nur wo die Skala die Forschungsfrage ist. Per Zelle Bin, N_eff, Surrogat-Null; zwischen Zellen z-Scores/p-Werte, nicht rohe TE-Bits | Zellen über ~5 Größenordnungen sind **Aggregat-Aussagen, keine native Lag-TE**; nie im selben Farbschema wie 1-min×1-min; zu wenig Samples → eigener Status, nie 0/Interpolation. (Vorbehalt: arXiv-Nummern nicht gegengeprüft.) |
| **Mistral** (Vibe) | **(C) Kern, (D) Inferenzrahmen**, B nur als Embedding-Regel *innerhalb* einer Skala; (A) verwerfen | Skalentrennung selbst (physikalisch real) |
| **Qwen3.8-Max** (chat.qwen.ai) | **(D) primär**; (B) nur paarweiser Notbehelf (nicht vergleichbar); (C) als Multi-Scale-Diagnose | Abtast-/Zeitskalen-Riss |
| **Gemini 3.1 Pro** (AI Studio, Thinking **High**) | **(D) beste Wahl**; (A)/(B) → Aliasing oder künstliche Nullen | diskrete Stufen/Abtastlücken der niedrigfrequenten Daten — Glättung (Splines/Interpolation) = **Look-ahead-Bias**, zerstört die zeitliche Asymmetrie der TE |
| **Z.ai** (GLM-5.3) | **(C) Kern + (D) Meta-Layer**; A/B Artefakte (Aufupsampling 2678400→6 s = Pseudo-Samples) | physikalische Skalenlücke (~446 400 : 1) |
| **Sakana** (Namazu) | **(D) einzige fundierte Wahl**; (C) an gemeinsame Achse gebunden | kausale Feinstruktur/lead-lag auf der nativen Skala des **schnelleren** Prozesses — Downsampling der schnellen Felder vernichtet sie |

**Konvergenz der starken Seats (Runde 3 + 4):** (A) ist überall falsch. Der Riss ist überall
derselbe: die **Abtast-/Skalengrenze** — nie die langsame Reihe up-sampeln (QBO-Monat → 6 s =
Pseudo-Samples/Look-ahead), nie die schnelle Struktur down-sampeln (Aliasing). Die tragende
Architektur ist **C + D** (native per-Paar-Skala + Summary-Causal-Graph/PCMCI-async); **B** ist
bei zwei starken Seats die *Basis* (Claude: „als (D) interpretiert"; Qwen3.8 2.4T:
block-aggregiert aufs grobe Gitter), bei den übrigen ein gekennzeichneter Fallback. Kein starkes
Seat stützt A. Die (B)-Frage des Rats ist damit **nicht gerissen, sondern präzisiert**: B als
Substrat nur mit blockweiser Aggregation aufs gröbere Gitter und Skalen-/N_eff-Annotation.
