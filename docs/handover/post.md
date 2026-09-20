<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-20
  sha256: 8e79bf162b42deca70d59c3a680161305a615e8039c783cc3d850919ed5fb1c5
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

An entscheid: Chrome DevTools MCP (Membran-Debug, Konsole/Netz/Performance via CDP am live Chrome) anbinden? Ja → Operator-Wort, dann `opencode.json` `mcp.chrome-devtools` pinnen (npm `chrome-devtools-mcp@1.9.0`, Flags `--no-usage-statistics` `--no-performance-crux`); Nein → Browser-Anbindung bleibt ohne Debugger-Pfad. Ferner: Cookie-Editor-Transfer Operator-Profil ↔ persistentes Playwright-Profil nur per Operator-Wort. (Forschung-Folge 109; Schritt: Operator-Wort, dann Anbindung.)

An bau: Queue-Korpora post_body-Migration — der `--port`-Konverter lässt `source`/`method`/`body`/`pos` fallen (gemessen 2026-09-20, `phi/pipeline/ledger.φ:94-120`, 7 Blöcke: 1048/874/1048/19/177/425/50 konv., 8/8/8/1/0/6/8 parsen, 0 Survivor). (Schritt: Konverter um die POST-/source-Kopf-Migration erweitern, `src/archivar` `--port`-Pfad.) (Ernte-Folge 115.)

An entscheid: Queue-Korpora astro/earth/exotic (30/3/16 Blöcke, `phi/pipeline/ledger.φ:82-92`) — `--port` blockiert: kein `force`-Direktiv → `default_kernel_for("") = None`. Gemessener Blocker: Converter-/Direktiv-Pfad (bau) + kein sanktionierter Lauf-Ort (Korpora gitignored, kein CI-`--port`-Workflow, lokaler Funktionslauf verweigert). (Schritt: Operator-Wort für den lokalen Lauf des Release-Binärs auf den Korpora — oder ein CI-`--port`-Workflow —, dann `--port` + `--probe`; dann Disposition.) (Ernte-Folge 115.)


