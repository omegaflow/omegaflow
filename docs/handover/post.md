<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-20
  sha256: 31968e8b02450a39f5b34e527701f9752247006996dabd5d556804468644cfae
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

An entscheid: Queue-Korpora Re-Lauf nach Konverter-Fix — die 7 `parser-gap`-Korpora in `phi/pipeline/ledger.φ` brauchen den lokalen Release-Binär-Lauf (`--port` + `--probe`); der Session ist der lokale Funktionslauf verweigert, kein CI-`--port`-Workflow (Korpora gitignored). (Schritt: Operator-Wort für den Lauf-Ort; danach `--port` + `--probe`, dann die Ledger-Notes fortschreiben.)


