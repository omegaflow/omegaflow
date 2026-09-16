<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 04d34500c306f3b9a3372e12412cee6c2ef20bc83e3e50299e9d95435f57bef2
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und löscht die Zeile.

An bau: Gate-Fixtures für die Consent-Akt-Vokabeln (Mail senden, Konto/API-Key anlegen, Antrag/Auskunft, Einreichung an fremder Stelle, Vertragsannahme, Zahlung, fremdes Konto löschen) + Gate-Test im selben Atom — die strukturelle Sichtbarkeit für die curl-Schreibpfade (der `smail`-Pfad selbst ist Default-dry-run, `--send` explizit). (Schritt: `src/gate/commit_gate_vocab.json` + Test in `src/gate/commit_gate.rs`.)

An entscheid: Zugangsanfragen-Verdikt (Ernte, gemessen 2026-09-15) — streichen (entbehrlich/redundant/geschlossen): NOIRLab, JSOC, LPF, GAVO, BiSON (anonyme Routen 200, offen nur die Tabelle), IGETS (`igets.bin` in `phi/sources.φ:8039`), TOAR (WOUDC liefert dieselben WMO-Daten); halten: CSES-Limadou (L2-Zugang lokal), NSE/Haug (Rohdaten descoped), Rubin RSP (via Fink-LSST anonym 200). (Schritt: die entbehrlichen Wartepunkte streichen, die „hält"-Punkte behalten.)

An entscheid: Zugangs-/Entscheid-Punkte, die die Ernte blockieren — GES-DISC `client_id` (nur `EARTHDATA_EDL_TOKEN` vorhanden), CSES-SPA-Login, JUNO-Disambiguierung (Neutrino-JUNO vs NASA Juno, `blocked_sources.φ:20–22`), TA-Disambiguierung (Telescope-Array vs USArray-TA-FDSN), HAWC-TLS (Leaf-Zert; `-k`→200), LHAASO-IHEP-Pfad (`english.ihep.cas.cn/lhaaso/` 200), limadou-PI-Freigabe (Sotgiu; Nachfassen = Mail = per-Akt-Consent). (Schritt: je ein Wort/Verdikt, dann Register-Disposition.)

An entscheid: Tor-1/Parser-Magic — braucht ein Konsument-Verdikt, sonst halten/streichen: ERI/VLASS/CORS, LASzip, FITS-Rice, JVO-skynode-Proxy, Babamul (Tor 1); Gap 1+8 (`Frame::Data`+`flush!()`, bricht 9 erschöpfende `match`-Arme), Gap 12 (`group`-Direktiv — Schema oder streichen). (Schritt: entscheiden, welcher Konsument zuerst, oder alle halten.)
