<!--
  title: Handover — River-Folge 76 (2026-10-01)
  session: River-Folge 76
  class: handover
  date: 2026-10-01
  sha256: 68e6e5596533c3d448756f4a58ea0237e3bd1819c9f354fa2002311eed4e12c5
  status: live
-->
# Handover — River-Folge 76 (2026-10-01)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„kannst du bitte einen benschmark mit allen zur verfügung stehenden sinnvollen stimmen machen und zudem habe ich die idee ob die stimmen nicht auch archive search für die online recherche nutzen dürfen …" | 2026-10-01 | Operator (Session, River 76)
„warum nutzt du nur die schlechten stimmen wir brauchen wirklich fähige senior reviewer … warum ist gemini blocked und was ist mit den chat uis?" | 2026-10-01 | Operator (Session, River 76)
„was muss ich fixen? … eigentlich habe ich doch den gemini key per opencode auth eingetragen. warum haben wir nur so eine kleine auswahl an opencode stimmen und ich glaub es ist wichtig dass wir die archive search bereiche für die … fremden stimmen begrenzen … bitte z.ai + arena noch fahren und prüfe welcher z.ai ui chat besser funktioniert" | 2026-10-01 | Operator (Session, River 76)
„future ist schon dabei eine voices agenten lösung zu bauen bitte spreche dich mit ihr ab und bitte a und b" | 2026-10-01 | Operator (Session, River 76)

## An future

Origin: river folge76. Abgleich voice-Agent (deine folge163:247).

- **Kein Duplikat gebaut.** Deine `voice`-Lösung (`opencode run -m <provider/model> --agent voice`,
  read-only, Bash nur `archive_search_public` öffentliche Netz-Modi + `voice_read_full`) deckt den
  geforderten Public-Scope. River baut **keinen** zweiten `--public-only`-Loop; die gemessenen
  Kanäle fließen in deinen Agenten.
- **Gemini-Kanal (Fix, gemessen):** `tools/measure/free_models.tsv:12-22` trug alle `google`-Zeilen
  als `blocked` und las `GOOGLE_API_KEY` (fehlt). Mit `GEMINI_API_KEY` antwortet der OpenAI-Compat-Pfad
  `…/v1beta/openai/chat/completions` **HTTP 200**. Zeilen → `GEMINI_API_KEY` + `eligible`; nach Rebuild
  gemini-2.5/3.5/3.5-lite **3/3 ok**. `text_review` bekommt mit `--raw` einen freien Prompt.
- **Arena (gemessen):** `arena.ai/text/direct?model_a=<id>` — valid: `claude-sonnet-5-5-high`
  (antwortet), `gpt-5.2-high`/`grok-4.6-high` (**Security Verification / reCAPTCHA-Wand**, kein
  Antwortlauf), `grok-4.20-beta-0309-reasoning` (**Fehler**, toter Pin). Kein Bypass.
- **z.ai (gemessen):** `chat.z.ai` = einziger funktionierender Chatbot (GLM-5.3 Deep Think Max,
  Prompt abgeschickt, nach >15 min noch `Thinking...`); `z.ai/chat` = Builder-Fläche, Composer da,
  aber weder Enter noch synthetischer noch truster CDP-Klick schickt ab.
- **Blinder fam/null-Benchmark gefahren:** 14 HTTP/API-Stimmen + claude (5/5, voller Rang-Test
  1/(n+1): 9.1 % n=10, 1.0 % n=100). Artifact: `state/stimmen/2026-10-01_blind-fam-null.md`,
  `…_blind-fam-null-ui.md`.
- **Rivers Verdikt — Modell-Rangfolge für den `voice`-Agenten (gemessen 2026-10-01):** für den
  Agenten-Loop zählt Tragezuverlässigkeit (Tool-Call + `--all`-Spill verdauen), nicht nur Urteil.
  1. `nvidia/z-ai/glm-5.3` — zuerst (vollständig via `opencode run`, billigste vollständige).
  2. `nvidia/nvidia/nemotron-3-ultra-550b-a55b` — zweite (5/5 blind; unterschätzt die DGP im Niveau).
  3. `google/gemini-3.5-flash-lite` — dritte (Kanal heute gefixt; vollständiger blinder Befund,
     billig). Reserve: `qwen/qwen3.8-27b` (HTTP, vollständig) oder `@cf/openai/gpt-oss-120b`.
  UI (per-Akt-Wort, nicht agentenfähig): claude.ai Sonnet 5.5 zuerst, kimi.ai K3 zweite.
  **Nicht:** `glm-4.5-flash` (falscher Dedup), `kimi-k3` via opencode (0 B/Hang),
  `gemini-2.5-pro` (nicht beendet), arena `grok-4.20` (toter Pin), arena `gpt-5.2-high`/`grok-4.6-high`
  (reCAPTCHA-Wand, gemessen), `gemini-3.8-flash` (**http_5xx**, gemessen), `gemini-3.1-pro-preview`
  (**429**, gemessen).
- **`archive_search_public`/`--all` ändert die Rangfolge:** es **hebt** Modelle mit sauberem
  Tool-Calling + langem Kontext und **disqualifiziert** die kleinen (lfm-2.5-2.6b, granite-micro,
  llama-4-scout, mistral-small-3.1) und die hängenden (kimi-k3). Der `--all`-Spill ist groß →
  starkes Instruction-Following zuerst; genau die HTTP-Blind-Runde (glm-5.3, nemotron-ultra,
  gemini-3.5-flash-lite, qwen3.8) trägt. Futures Vorschlag bestätigt, mit zwei Korrekturen:
  **gemini-3.5-flash-lite statt 3.8-flash** (3.8 = 5xx) und **kein kimi-k3** im Loop.

## Offen

### σ-Asset `dr3_stars.bin` — σ-Zensus nach Re-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `gaia-cdn 36819890733` (Dispatch 2026-10-01)
- **Lage:** (gemessen 2026-10-01 via `ci_manage view`) Lauf `in_progress`; `tap_compiler.rs:403`
  `STAR_BIN_STRIDE=56` + drei `sigma_slot` stehen.
- **Blockade:** kein eigener Schritt bis zum Lauf-Ende
- **Braucht:** nach dem Lauf `archive_search --verdict/--sniff` auf das CDN-`dr3_stars.bin` und den
  σ-Zensus rechnen; dann in Quellen-/Weberin-Konsumenten fortschreiben.

### Kalibrierte Null (Westfall–Young max-T) — CI-Messung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t 36824285645`
- **Lage:** (gemessen 2026-10-01 via `ci_manage view`) `pending`; Bau steht
  (`tools/measure/src/bin/wy_max_t_probe.rs`, Workflow `.github/workflows/wy-max-t.yml`).
- **Blockade:** kein eigener Schritt bis zum Lauf-Ende
- **Braucht:** `ci_manage log 36824285645` lesen; die `fam`-Verdikte §4/§5 des GIC-Papiers mit dem
  gemessenen Ergebnis fortschreiben — **der Satz `the riss stands` bleibt bis dahin als offen
  gerahmt** (§3.2 nennt die Konstruktion „the open construction", α unbenannt).

### Blinder Stimmen-Benchmark — UI-Rest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** chat.z.ai `Thinking...` (Deep Think Max, seit >15 min)
- **Lage:** (gemessen 2026-10-01) HTTP/API + claude fertig; `chat.z.ai` hängt; kimi K3 Spur da,
  Finaltext offen.
- **Blockade:** kein eigener Schritt bis die UI-Antwort rendert
- **Braucht:** chat.z.ai-Seite einmal neu lesen; kimi/chat.z.ai-Ergebnis ins Roster fortschreiben.

### Finsternis-Uhr — pre-1972-Anker
- **Status:** eigen | **Bindung:** eigen
- **Triggers:** —
- **Lage:** (gemessen 2026-10-01) `src/archivar/lsk.rs` um ΔT-Anker (`delta_t_espenak_meeus`)
  erweitert; `ephemeris_house_gate --epoch-ymd 1919-05-29` löst `perigee tdb`
  (`-2543357667.004`), `window` bleibt `pending` — die drei Ephemeriden-Bücher tragen keinen
  gemeinsamen Bogen 1919 (Datenabdeckung, nicht Leap-Tabelle). Uncommitted.
- **Blockade:** keiner für den Code; der 1919-Wert braucht Datenabdeckung
- **Braucht:** committen; dann `window` als Daten-Coverage-Punkt führen.

### Finsternis-Haus-Cross-Check — Zahlen pending bis Re-Manifest
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Re-Manifest der CDN-Bins (mountain-216 geheilt, Bins alt)
- **Lage:** (gemessen 2026-10-01 via mountain-folge216 adressiert) `docs/paper/eclipse-clock-worldlines.md` führt die
  DE↔INPOP-Zahlen (0.064/0.164/0.168 km) als gemessen; sie sind vom Solver-Bug belastet.
- **Blockade:** erst nach Re-Manifest ist der Haus-Quercheck belastbar
- **Braucht:** die Zeile auf `pending` setzen (mountain-216-Wort) und nach dem Re-Manifest
  `eclipse_shadow_probe` neu ziehen.

### GIC-Treiber-Erweiterung — Newell gemessen verdrahtet
- **Status:** eigen | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-01 via `sgrep`) `bz_retro_probe.rs:377` `newell_from_cells`, `:741`
  verdrahtet, `:777` als Paar geführt — der Handover-Punkt „Newell fehlt" war **falsch**.
- **Blockade:** keiner
- **Braucht:** prüfen, ob Bs/By/Clock/P_dyn/M_A noch fehlen; die verbleibenden als Kanal bauen.

### ENSO-Kanal-Erweiterung — QBO + d20 verdrahtet
- **Status:** eigen | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-01) `enso_blatt_probe.rs` trägt QBO + d20 als Kanäle 5/6
  (`CH_QBO=4`, `CH_D20=5`), build grün, uncommitted.
- **Blockade:** keiner
- **Braucht:** committen; dann Positiv-Kontrollen τx/SOI registrieren; CI-Lauf.

### Flyby-path-2-Kette
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** OMNI2 HAPI-Lag (~2026-10-04); kp-`def`-Freigabe; ESOC-`*recon*`-SPK
- **Lage:** (gemessen 2026-10-01 via `state/zustand/wartend.φ`) alle Zellen pending
- **Blockade:** externe Datenvorläufe
- **Braucht:** bei Fälligkeit `flyby_path2_fill` (CI) lesen, Addendum fortschreiben.

### Visionen — Sonne-Erde-Blatt · Frühwarnsystem · Weberin
- **Status:** blockiert | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-01, unverändert aus folge75) drei Visionen offen.
- **Blockade:** kein Multi-Atom-Auftrag bzw. kein Rats-Wort
- **Braucht:** Multi-Atom bzw. Rats-Frage.

### Ladearchitektur-Survey — Träger (adressiert mycelium-216 + sensory-216)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-01, `register_lookup --orphan-docs`) `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md:122`
  trägt einen offenen Marker (`TODO.md:2661-2676`, Membran-scoped Cache/Lade-Hülle), §7 erklärt alle Punkte
  geschlossen; seit folge178 als River-Punkt (presence-only Ladearchitektur) geführt.
- **Blockade:** keiner
- **Braucht:** `:122` als gemessenes `descoped` annotieren (die presence-only Ladearchitektur ist gebaut/geschlossen);
  damit ist der Orphan geschlossen. Träger bis dahin: diese Zeile.

## LOCK

- **GIC — DOI-Knoten, dann GEMS-Einreichung (Operator-Hand).** Vorbereitung privat
  `state/future/gic-causal-driver-doi-metadaten.md`; Send = Operator-Hand.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Pfad-begrenzte Commit-Pfade:

- `tools/measure/free_models.tsv` (Gemini-Kanal: `GEMINI_API_KEY` + `eligible`)
- `tools/measure/src/bin/text_review.rs` (`--raw`)
- `tools/measure/src/bin/enso_blatt_probe.rs` (QBO + d20)
- `src/archivar/lsk.rs` (pre-1972 ΔT-Anker)
- `docs/paper/eclipse-clock-worldlines.md` (Haus-Cross-Check auf `pending`, mountain-217)
- `docs/handover/handover-2026-10-01-river-folge76.md`, und `…-folge75.md` → `archiv/` (Move)

Fremde uncommittete Arbeit unangetastet.

## Burn: open 0.0000 · close 0.1679 · cap 0.50 · Grund: operator-getriebenes Mehr-Nachrichten-Atom (blinder Benchmark über HTTP/API + UI, Gemini-Kanal-Fix, z.ai/arena-Messung, Future-Abgleich); close unter dem Cap
