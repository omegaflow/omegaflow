<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-17
  sha256: 58102b343fa382fdfd2f0498620a4fa6bf3359e4d60cc787e110c3a82cdf3557
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

An entscheid: DEMETER — neue Orders sind ein Dritt-Akt (Consent). Harvest `35146819646` = Budget-Stopp, 0 `.DAT` (jede Order WAF blocked / order parse void). (Schritt: Consent-Wort für neue DEMETER-Orders oder die Route als `blocked` führen.)

An entscheid: CI-Schreib-Token-Budget — planetary-odf `35190614514` (headSha `1e4faf79`) scheiterte in 7/9 Legs an `HTTP 403: API rate limit exceeded` beim `ensure release` (Schreib-Token `OMEGAFLOW_TOKEN`), `rosetta_odf` cancelled; `docs/specs/ref-auth-apis.md:68–79` nennt seit 2026-09-17 einen eigenen CI-PAT `omegaflow-ci-write` mit getrenntem Bucket — die Läufe zeigen den 403 dennoch. (Schritt: messen, ob die Secret-Rotation griff und ob GitHub-PAT-Buckets wirklich pro Token getrennt sind — operator-gebunden.)

