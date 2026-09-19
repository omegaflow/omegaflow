<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-19
  sha256: 23cd26af35a893e8137c2e331e8b5efe6ceb9728fe6ef24bd2e560daac45e359
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

An entscheid: vC-Permeabilität — Operator-Wort 2026-09-19 („Messakt lokal, kein CDN")
steht; gebunden ist der Vollzug: Release-Bin → `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad>`
→ `perm_target_probe --live`. Der Akt liegt auf der Operator-Maschine (lokaler
Funktionslauf), die Session kann ihn nicht vollziehen. (Schritt: Operator-Wort/Vollzug;
bau liest `perm_target_probe --live`.)

An entscheid: Pipeline-Port force-Gate — binär A/B seit entscheid-folge46, neun Sessions
offen. A: lokal (`--port` + Netz-Fetches, Korpora bleiben gitignoriert); B: CI-Upload der
10 `phi/pipeline/queue/*.φ` + Netz-Proben (gitignore öffnen, Upload-Pfad ungebaut).
(Schritt: Operator-Wort A/B; bau baut den gewählten Pfad.)

An entscheid: register_lookup-Symlink — `~/.local/bin/register_lookup` zeigt auf
`target/release` statt `bin/register_lookup`; Fix ist ein PATH-Eingriff im Operator-Domain,
verschränkt mit dem ausstehenden `release-build`-Run (entscheid-folge55:110-111).
(Schritt: Operator setzt Symlink bzw. Release-Build-Wort; danach register_lookup-Verifikation.)
