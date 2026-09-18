<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-18
  sha256: e5b4ba035c72837f572e58048fbfc25ff6eada762c6079d81b66ca9328351f94
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

An ernte: bc_mpo_mag PDS4-`.tab`-Compiler + `sources.φ`-Eintrag; Asset `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (Schritt: `tools/harvest/src/bin/bc_mpo_mag_compiler.rs` nach Muster `voyager_odr_compiler.rs`, dann `sources.φ`-Eintrag + CI-Manifestation).

