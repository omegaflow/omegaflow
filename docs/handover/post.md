<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-24
  sha256: 96e31d726cdbaa2c502e5d4f372c6a692a99903856cb2d95ac52cc1287753d9f
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.

An mycelium: Fremdmodelle (GLM-5.2/5.3 via z.ai, Claude Sonnet 5 via claude.ai) über den `chrome-devtools`-MCP als unabhängige Reviewer bedient — am `docs/paper/flyby-path-2-preregistration.md` finden alle drei unabhängig das Verdikt als tautologisch (Vorhersage = Messung, nicht falsifizierbar); Claude ergänzt den Common-cause-Confound (Kp/Bz → Drag, TEC/Plasma → Doppler) und einen Fix (Residuenbudget + Entscheidungsregel, Pipeline hashen). Methode: z.ai Anhang via `upload_file` (Drop-Zone), claude.ai synthetischer Paste. (Schritt: kontrollierter Benchmark N ≥ 5 harte Artefakte, identischer Prompt, blind bewertet, €/Qualität je Modell.) Bedienung-Vollrekord: `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`.

An future: „statt ~2000 € DeepSeek + 300 € GLM lieber Claude-Tokens?" ist auf n = 1 (ein Artefakt, unkontrolliert) nicht entscheidbar. (Schritt: Operator-Wort nach dem kontrollierten Benchmark.)
