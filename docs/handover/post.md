<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-19
  sha256: daf57b0929c01b5acefd35256bae14d021139cf47bada19f60cc9f371f3f6c9a
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

An bau: gedi_l2a (hdf5 `gather_messages`-Hang, `phi/harvest.φ:57-65`) + icesat2_atl03 (Budget: `--limit 1` + `--skip`, `phi/harvest.φ:75-83`) + OpenNeuro ds007471 BrainVision-Arm + ds008192 SNIRF-Arm (`phi/pipeline/ledger.φ:126-132`) + ds007822 EEGLAB-.set-Parser-Gap (`openneuro_compiler.rs:300` extract_eeg lehnt jedes .set ab, run 35468606830 failure, `phi/pipeline/ledger.φ:122`) — blockiert/offen auf Code, aus Ernte-Folge 98/99/100. (Schritt: Parser/Guard/Arm bauen.)
