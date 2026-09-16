<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 1046728f3ea38fb3612697360206280bb9c4d52964a4eae3f03346d972acea70
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

An alle Linien (format-Gate): CI-`format` rot @625452e5 — fremde unformatierte Dateien: `src/gate/commit_gate.rs:540`, `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`, `tools/utils/src/bin/archive_search.rs` (die ernte-eigenen fünf harvest-Compiler trägt das Ernte-Handover). (Schritt: rustfmt-Diff aus `ci-check` run 35091175017 anwenden.)

An alle Linien: die Planungs-Klausel „einen schweren und fünf leichte" ist gestrichen (Operator-Wort, 2026-09-16) — es gilt wieder: so viele offene Punkte wie möglich pro Session; `AGENTS.md` + `_template.md` korrigiert. (Schritt: die eigene Preamble beim nächsten Handover angleichen.)


An entscheid: AMS-02 — der Descoped-Eintrag (`declined_sources.φ:231`, open no-consumer) trägt die überholte Klausel „kein Compiler"; ein TDAT/FITS-Serializer-Compiler steht jetzt (Ernte-Folge 48, uncommittet). Die Feld-/Force-Registrierung bleibt verweigert (kein CR-Medium in der 9er-Registry). Operator-Wort nötig: die Verdrahtung eröffnen oder den Compiler streichen. (Schritt: das eine oder andere Wort.)

An entscheid: Mail-Fang — der KV-Namespace `MAIL_QUEUE` blockt an der Token-Scope: `wrangler kv namespace create` und der REST-POST liefern `Authentication error [code 10000]`, obwohl `wrangler kv namespace list` `workers_kv (write)` zeigt. (Schritt: KV-Namespace im Dashboard anlegen oder den Token account-scoped mit „Workers KV Storage:Edit" neu erzeugen.)

