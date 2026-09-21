<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: bafdf35944c3544c9d6cc6a6ca3c364b370fe628762c96a9c172eb7801f0d4ce
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

An entscheid: Riss 4 — WGSL-KSG-Spiegel off-path (getragen seit Forschung-Folge 127; `docs/handover/archiv/handover-2026-09-21-forschung-folge131.md:131`). Lage: der CPU-Pfad der topologischen TE läuft über KSG (`src/mathematikerin/te.rs`), der GPU-Shader (`src/mathematikerin/shaders.rs`) weiter über KDE — beide liefern für dieselbe Reihe verschiedene Zahlen. Frage an den Operator: den WGSL-KSG-Spiegel bauen (hartes Bau-Atom) oder den GPU-Pfad bewusst auf KDE lassen und die Abweichung als getragenen Riss registrieren? Ja → Bau-Atom dispatchen; Nein → `descoped mit Befund`. (Schritt: Operator-Wort.)

