<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 8338d36546e41c6adff726bc12c712180acc57284142bb55459f4d2a67e7c935
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


An ernte: DEMETER/CDPP — **Korrektur, selbst gemessen 2026-09-16: der Key funktioniert, die Route ist nicht blockiert.** Login `regards.cnes.fr/api/v1/rs-authentication/oauth/token?grant_type=password&scope=cdpp&username=…&password=…` (Query-String-Form, Basic `Y2xpZW50OnNlY3JldA==`) → 200 mit Bearer (`role: REGISTERED_USER`); `rs-catalog/engines/legacy/dataobjects/search?q=DatasetName:DMT_N1_1144` mit Token → 200, **57 760** Objekte; `rs-order/user/orders` → 42 Orders (`demeter-0000…0026` meist `DONE`, einzelne `FAILED`/`EXPIRED`; Labels `demeter-` **und** `demeter_`). Der `403` aus `archive_search --verdict` war ein **Auth-Gate**, kein Route-Block — `--verdict` ist auf auth-gated APIs irreführend (kein Geo-/Exit-Bedarf). Lokal: `demeter_harvest` nicht in `target/`, Quelle nicht in `phi/sources.φ` (nur `phi/pipeline/research/agent_output/ledger_disposition_nontap_2026-09-15.φ:219` — pending); `demeter-cdn.yml` + `demeter_compiler` + `src/archivar/demeter.rs` stehen. (Schritt: Ernte über den vorhandenen Ledger fortsetzen, Quelle in `phi/sources.φ` registrieren; neue Orders = Dritt-Akt → Consent über entscheid.)


