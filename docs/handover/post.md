<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 478232b47ed164c72838229b689e3d11ff71d10aeef058628995a64a4ce851d8
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

An mountain: Riss 4 — der WGSL-TE-Pfad rechnet KDE (`shaders.rs:483` `te_embedded_kde`), die kanonische CPU-Referenz rechnet KSG (`te.rs` `transfer_entropy_embedded_ksg`, `TE_KSG_K=4`); die Kalibrierung (FP/FN/Symmetrie/n-Floor) hängt an KSG und überträgt sich nicht auf den GPU-Wert. Operator-Wort 2026-09-21: **bauen** — KSG als WGSL-Spiegel. (Schritt: Bau-Atom, WGSL-KSG-Pfad neben `te_embedded_kde`; Parität gegen `te.rs` im Kalibrier-Gate prüfen.)

An mountain: clippy `src/mathematikerin/te.rs:1574` `clippy::needless_range_loop` aus `ci-check 35608204623` @`8d6553fe` (Rust 1.98, `-D warnings`). (Schritt: Lint am `te.rs` beheben.)

An mountain: rote lib-Tests `te::tests::coherent_phase_null_absorbs_linear_cross_coupling` und `te::tests::gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200` aus `ci-check 35608204623` @`8d6553fe`; ein reduced-TE-Atom an `src/mathematikerin/te.rs` liegt uncommittet im Baum (fremder Hunk). (Schritt: prüfen, ob der Hunk heilt, sonst Wurzel messen + Fix/Gate im selben Atom; `ci_manage log 35608204623 --all`.)

An mountain: `register_lookup --dropped` holt Punkte zurück, deren auflösender Commit in fremden Linien lag (Split-Routing→ernte, Umbenennung→entscheid); der Sweep prüft nur den eigenen Strang. (Schritt: vor dem Zurückholen `git log --all --grep/-S` über alle Linien, Bau in `tools/register/src/bin/register_lookup.rs`.)

An ernte: Amentum Developer (`phi/blocked_sources.φ:52`) — Register-Seite akzeptiert nur die Privacy Policy, kein Redistributionsrecht; „14-day free trial" / „Enterprise API access" = kommerzieller Dienst. Ein Trial ist Testzugang, keine Lizenz zum Ziehen+CDN-Hosten. (Schritt: Verdikt `declined` (kommerziell) in `phi/declined_sources.φ`.)

An ernte: solar-system-open-data (`phi/blocked_sources.φ:47`) — REST 401, Key frei/selbstbedienung (`generatekey.html`), Gegenüber Maschine. (Schritt: per-act consent, Key ziehen.)

An ernte: Free-Model-Bench (105 Modelle) — Aufbau auf inkrementelles Schreiben je Zeile korrigiert; Lauf-Abschluss offen. (Schritt: `free-model-bench.tsv` lesen, `gemini-2.5-flash` (`free_models.tsv:72`) messen.)

An sensory: F2 — flare-Gate-Power, Probe n∈{400,600,1000} aus. (Schritt: Re-run.)
