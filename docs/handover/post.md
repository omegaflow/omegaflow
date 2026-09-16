<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 81192640910a254fbe7f853b87cfc3e0b3664ad37d598e147cdf3c18a0498385
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

An alle Linien (format-Gate): CI-`format` rot — fremde unformatierte Dateien: `src/archivar/vtscat.rs`, `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`, `tools/utils/src/bin/archive_search.rs` (die ernte-eigenen harvest-Compiler trägt das Ernte-Handover). `src/gate/commit_gate.rs` ist konform (gemessen 2026-09-16, bau-folge55). (Schritt: rustfmt-Diff aus dem `ci-check`-format-Job anwenden — jede Linie ihre eigene Datei.)

An alle Linien: die Planungs-Klausel „einen schweren und fünf leichte" ist gestrichen (Operator-Wort, 2026-09-16) — es gilt wieder: so viele offene Punkte wie möglich pro Session; `AGENTS.md` + `_template.md` korrigiert. (Schritt: die eigene Preamble beim nächsten Handover angleichen.)

An entscheid: DEMETER-Re-Aggregation — der `demeter_isl`-Konsument ist verdrahtet (`extract.rs`/`main_flow.rs`/`demeter.rs`, `cargo check` 0/0); die 77 Alt-Shards tragen ein −1970-Jahr und brauchen einen Re-Run von `demeter-cdn.yml` (die korrekt benannten Monats-Shards fehlen: `demeter_isl_200410.bin` = 404). Der Workflow koppelt an den CDPP-Harvest und könnte neue Orders erzeugen (Dritt-Akt). (Schritt: Operator-Wort für den Dispatch — oder ein Aggregat-only-CI-Job auf dem Cache-Workdir, nur Compiler + CDN-Upload, kein Harvest.)

An entscheid: KASCADE-Grande — DataShop ist Keycloak-SSO/JS, kein URL/POST-Endpoint; `blocked_sources.φ:63` korrekt. Braucht einen Minimal-Job mit dem `omegaflow`-Konto. (Schritt: Operator führt den SSO-Job, oder `/consent` + `archive_search --playwright --headed`.)

An entscheid: doi.org-Landingpage für `10.3929/ethz-c-000797709` bleibt HTTP 429 (direct + Proton-Exit, gemessen 2026-09-16 via `archive_search --verdict`; Wayback-CDX leer). Titel und Jahr sind via DataCite + Crossref gemessen (2026) — die Route wäre nur noch für die Landingpage-Auflösung. Freigabe: `bin/proton-wg.sh suggest doi.org` → `bin/proton-wg.sh <cc>` (Schritt: Operator-Wort `/consent <act>` auf den präsentierten Lauf).



