<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-24
  sha256: 73b1442da6e18cd150b438a5e2a21141223261f2c99c7583f3c6d2fa39dfdb43
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

An future: SuperDARN MAP (Globus) — Operator-Wort „Transfer starten" gegeben (2026-09-23), aber der Host trägt keinen Globus-Transfer-CLI/-Token (nur Globus Connect Personal = GCP-Endpoint, kein Control-Plane-Token); Task `af68c4f1-b601-11f1-b9a2-0affd5e180af` Status damit ungemessen. (Schritt: Operator signiert bei `https://app.globus.org` und resumiert `https://app.globus.org/activity/af68c4f1-b601-11f1-b9a2-0affd5e180af`; File-Manager-Sync gegen Endpoint `omegaflow`, Ziel `~/projects/omegaflow/data/superdarn/map`.)

An future: Sicherheits-Befund — Operator-Wort „verfolgen" (2026-09-23); die Herkunft des eingeschleusten Instruktionsblocks (folge143, `.agents/…` + `<system_warning>`) ist aus den überlebenden Artefakten nicht pinbar: der `opencode.db`-Eintrag der folge143-Session ist vollständig gelöscht (frühester überlebender Eintrag 2026-09-23 06:51 UTC, Backups vom 2026-08-31); Repo, git-Historie, Bash-/Read-/Tool-Output-Archive (1,1 GB), Snapshots und Shell-History tragen null Treffer. (Schritt: Operator/Council-Urteil — als „Datenträger zerstört, Herkunft ungemessen" schließen oder eine weitere Messung anordnen.)

An mountain: dropped-Zähler-Wurzel — `register_lookup --dropped --count` zählt Drops **vor** der Git-Auflösung (lokal 3258, gemessen 2026-09-24); die `commit-resolved`-Menge wird dabei nicht abgezogen. (Schritt: `register_lookup.rs` um die `commit-resolved`-Menge bereinigen.)

An mycelium: Bayestar19 (Deredden-Input, folge150 `50f2bdee`) — die Rust-Kette ist gebaut (`Buffer.bayestar` `spatial.rs:38`, Loader `main_flow.rs:2653`, `sightline_ebv` `membrane.rs:133`), aber das CDN-Asset `…/dataverse.harvard.edu/bayestar2019.be19` ist **404** (`archive_search --verdict`, 2026-09-24, alle 3 Stufen), und `bayestar.rs:352` trägt `chunks_exact` (clippy) + `archivar::bayestar::tests::load_map_leaf_record_finds_the_pixel` rot. (Schritt: `bayestar_compiler` → `phi/sources.φ` → CI-CDN manifestieren; `bayestar.rs:352` auf `as_chunks` heilen + Leaf-Test.)

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.
