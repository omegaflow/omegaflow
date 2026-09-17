<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-17
  sha256: d41a2ef8bdf1e40fa2026ecf354f0a6db8321fe49d756fa911e3f75868b1da0f
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

An alle Linien: die Planungs-Klausel „einen schweren und fünf leichte" ist gestrichen (Operator-Wort, 2026-09-16) — es gilt wieder: so viele offene Punkte wie möglich pro Session; `AGENTS.md` + `_template.md` korrigiert. (Schritt: die eigene Preamble beim nächsten Handover angleichen.)

An entscheid: Verlorene/vergessene Diskussionen — Archäologie über beide Historien (aktuell + `/home/johannes/backup/archive-root/omegaflow-legacy`). Beleg-Fall TTL/φ-CDN-Gate: das Prinzip ist **dreifach verhandelt** (`docs/concepts/pfeiler-der-architektur.md:155–181` §12/§13 „TTL < Datei → kein Fetch", `docs/concepts/archivar-mathematikerin.md:22` „CI Archivar … fetches only when `origin_stale`", `docs/specs/sources-v2-spec.md:57` „Fetch fires at ttl/Φ. Retention ttl×2⁶"), lokal gebaut (`origin_stale` `src/archivar/fetch.rs:313`, `cache_fresh_cdn` ehem. `cdn_fresh`, Sturm-Reparatur `d9d2c720`), aber die **CI-Instanz nie gebaut** — die Lücke wurde schon gemessen (`survey-2026-08-19-landschaft.md:81–83`, „der 5-min-Takt müsste im sources-Repo leben — unverifiziert"), nie geschlossen, wortgleich in die neue Doku eingewandert; dazu Namens-Drift `cdn_fresh`→`cache_fresh_cdn`. (Schritt: ein Archäologie-Pass mit `git -C <repo> log --all -S/--grep`, `git grep`, `archive_search --root <dir>` über beide Repos — Pfeiler/AGENTS/specs/TODO/handover/survey — der eine Karte liefert: (a) entschieden-und-gebaut, (b) verhandelt-und-nie-gebaut, (c) Doku-Behauptung ≠ Baum, (d) Namens-/Ebenen-Drift; jeder Eintrag mit Hash/Datei:Zeile und der nächsten Entscheidung; Operator-Wunsch 2026-09-17: weitere solcher lost-and-forgotten-Diskussionen finden.)
