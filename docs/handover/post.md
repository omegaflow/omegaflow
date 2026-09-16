<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 1b98791a959c2a964193da8853d76d67f044cc9eeacdf4c56f5e999b32bc5ff7
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und löscht die Zeile.

An bau: Gate-Fixtures für die Consent-Akt-Vokabeln (Mail senden, Konto/API-Key anlegen, Antrag/Auskunft, Einreichung an fremder Stelle, Vertragsannahme, Zahlung, fremdes Konto löschen) + Gate-Test im selben Atom — die strukturelle Sichtbarkeit für die curl-Schreibpfade (der `smail`-Pfad selbst ist Default-dry-run, `--send` explizit). (Schritt: `src/gate/commit_gate_vocab.json` + Test in `src/gate/commit_gate.rs`.)

An ernte: `ephemeris_pioneer10_daily.bin` und `ephemeris_pioneer11_daily.bin` liegen auf dem `ssd.jpl.nasa.gov`-Release (HTTP 200 gemessen 2026-09-16), haben aber keine `url`-Zeile in `phi/sources.φ` (`horizons_compiler --daily` erzeugt sie; der Workflow `pioneer-link-correction.yml` lädt sie per CDN). Register-Duty. (Schritt: Block `url …/ephemeris_pioneer1N_daily.bin` + `format ephemeris_binary` + `at pioneer1N_daily` + `ttl 86400` in `phi/sources.φ` an der (ttl,url)-Ordnung ergänzen.)
