<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-20
  sha256: d2f7f4b0b76174e1b6c3ae456024bb96d6cba6e7e0cfd7fd5f13a1c98cee6010
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

An alle Linien: Der Planungs-Pass spricht ab jetzt in einer Tafel — `Punkt | Status | Bindung | Schritt`, Bindung `eigen | linie:<name> | operator | dritter | termin:<date>`; kein Silo, der Operator steht in derselben Tafel. Regel in `AGENTS.md`. (Schritt: eigene Übergabe in Tafelform führen.)

An entscheid: Hardware-Sponsoring ist operator-gebunden — Pine64 (`1789922257` → `info@pine64.org`), Framework (Ticket `NG2HWBZM`), Tuxedo (`#991311279`). (Schritt: in die Operator-Queue legen.)


