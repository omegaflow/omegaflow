<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 9367453b88bf3a75203b5a6809a4d0aa707d14dfcee95b010960bf9c6a487947
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

An mycelium: zwei echte `ci-check`-Reds, in `mycelium-folge143` nicht getragen — (a) `register phi/sources.φ holds 11 ttl-order and 466 url-order violations across 1409 blocks` (`ci-check` `35806971846` @`32de9f3d`, `register_sort` `ci-check.yml:63` exit 1); (b) `dropped-gate` baseline 2680 | current 2757 (working tree `629486b77`, `register_lookup --dropped --count`; Lauf `35806971846` maß 2741) — offene Punkte abgeworfen ohne auflösenden Commit. (Schritt: `register_sort --write phi/sources.φ` kanonisieren; für (b) Punkte forttragen oder Baseline `docs/zustand/dropped-baseline.md` im annehmenden Commit heben.)

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.

