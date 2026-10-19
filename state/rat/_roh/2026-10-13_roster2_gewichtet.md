<!-- Gewichtete Auswertung der UI-Runde 2 (Mountain, 2026-10-13). Der Rat hält die Struktur; die starken Stimmen tragen das Urteil. -->
# Gewichtete Auswertung — vier Entscheidungen (2026-10-13)

**Nicht gezählt, gewichtet.** Gewicht = gemessene 4/4-Fähigkeit (`docs/concepts/ui-seats.md`, AGENTS-Roster;
`state/mycelium/voice-capability-*.tsv` für die lokale Schicht) × Modellklasse/Größe × Rolle.
Eine starke Abweichung schlägt viele schwache Zustimmungen — nie umgekehrt.

## Gewichtsstufen (grounded)
- **A — Urteil** (Frontier/größte gemessen-4/4): Z.ai GLM-5.3 Deep Think Max (753B, Pflicht) · Duck.ai GPT-6 Luna · Claude Sonnet 5.5 Extra hoch · GLM 5.3 (753B) · DeepSeek V4 Pro (1.7T) · Inkling (975B).
- **B+ — Bestätigung/Recherche** (Frontier, gemessen 4/4): Gemini 3.1 Pro (Grounding) · Qwen3.8-Max · Mistral Large 4 · Perplexity (15 Quellen).
- **B — Bestätigung** (gemessen 4/4): Lumo 2.0 Max · Sakana Namazu · DeepSeek Chat.
- **ohne Gewicht:** MiniMax M3 (Eingabe gekappt, unbrauchbar).

## Gewichtetes Verdikt je Punkt

1. **Höhenfenster — RISS, nicht entschieden.** A-Stimmen konvergieren: GLM-5.3 (Geoid ≈ −30 m, Toleranz ±10–15 m vorab), Claude (Datum/Toleranz VOR der Prüfung), DeepSeek V4 Pro (mehrere Stichpunkte, Bezugssystem), Z.ai (Vegetationsbias, orthometrisch vs. ellipsoidisch). **Inkling (A) weicht ab:** ein globales Dataset für einen Punkt ist Über-Overhead — lokale Teilprüfung ohne Registereintrag reicht. Gewichtet: Kachel-Registrierung **nicht** einfach durchziehen; erst Datum/Toleranz, und Inklings „kein Registereintrag" als gleichwertige A-Alternative tragen.
2. **Katalog-Verdrahtung — eigenes Modul schlägt block-scoped.** A-Stimmen (DeepSeek V4 Pro, GLM 5.3, Gemini, Inkling) und Qwen: ohne Sperre/CAS bleibt last-writer-wins; block-scoped ist Sofortmaßnahme, **Dauerlösung = eigenes Modul/Datei** (Eigentumsgrenze = Dateigrenze). Gewichtet: Modul bauen; block-scoped nur als Übergang.
3. **Ersatzquellen — konvergiert.** A-Stimmen (GLM-5.3, Z.ai, Inkling): nur bei **identischem Ursprung** streichen, nicht bei gleichem Inhalt anderer URL (Redundanz = Fallback); Lizenz unbekannt = Use-Blocker/Quarantäne, nicht registrieren. Hält.
4. **Ephemeriden — RISS zwischen Rat und starken Stimmen.** Der Rat (Struktur) sagt: Vertrag gehört dem Leser → auf Binär umstellen. Die A-/B+-Stimmen (Inkling, Z.ai, Lumo, Gemini, Duck, Sakana, DeepSeek V4 Pro) sagen mehrheitlich: **Adapter/externer Vertrag statt Zwangs-Umbau** — Rückwärtskompatibilität und Debuggbarkeit. Gewichtet: **nicht einseitig umschreiben**; Vertrag explizit versionieren, Adapter prüfen, dann entscheiden. Der Riss wird getragen, nicht geglättet.

## Konsequenz am Baum
- Punkt 1 und 4 sind **offene Risse** (nicht „bestätigt"); die Handover-Formulierung „Roster bestätigt" war falsch.
- Punkt 2 wird auf „eigenes Modul" geschärft; Punkt 3 hält.
