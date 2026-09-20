<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-20
  sha256: 3022c80727f4055440565ef6655da332bb6b4cc26bc91feef5f1dc39bb741ff2
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

An bau: EPA RadNet ERM_RESULT (accept, em, Bq/L) ist nicht registrierbar — `src/archivar/units.rs` kennt `bq/l`/`bq/m3` nicht (weder `convert_to_si` noch `allowed_units_for_force(em)`); die Quelle ist positionslos (kein lat/lon in ERM_RESULT/ERM_SAMPLE/ERM_LOCATION) und die Einheit datengetragen (BQ/L, BQ/M3, G/L). (Schritt: `bq/l` ×1000 → m⁻³·s⁻¹ in beide Unit-Tabellen; dann Ernte-Registrierung.)

An bau: still fallengelassener Strang „strukturierte Feld-Grammatik" (Kanon-Akt, Operator-Wort) — letzte Nennung `handover-2026-09-18-bau-folge74.md`, seither in keiner Übergabe, `git: none`, nicht in `external-state.md`/entscheid. (Schritt: als offenen Punkt ins bau-Handover zurücktragen, Operator-Entscheid einholen.)

An ernte: still fallengelassener Strang „Rosetta ungelaufene Pfade / Idempotenz-Gate" (`wartend`) — letzte Nennung `handover-2026-09-17-ernte-folge75.md`, `idempotenz` 0 Treffer in rs/φ, kein zentraler Trigger in `external-state.md`. (Schritt: offenen Punkt ins ernte-Handover zurücktragen, Auslöser benennen.)


