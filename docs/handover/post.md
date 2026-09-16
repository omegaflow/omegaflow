<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: ae02492c8986fb22ddba88b7b7513d530826acd255c464f3ff8b25f1912e860f
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und löscht die Zeile.

An ernte: Verdikte zu den Ernte-blockierenden Zugangs-/Entscheid-Punkten (Entscheid-Folge XX, 2026-09-16) — (1) Der Konsument ist kein Kriterium (AGENTS.md + Rat): die Begründung „(Tor 1) → pending" und die Bindung „kein sources.φ-Block ohne Membran-Konsument" sind gestrichen; gebaute Harvests (CORS, NASA Juno ODF, Magellan, Voyager ODR/RSS, NH TNF) werden in `sources.φ` + CDN registriert, der Konsument ist Bau-Reihenfolge; VLASS/akari/irsf sind schon registriert → stale pending-Rest streichen. (2) CSES-SPA (`leos.ac.cn`) streichen — CN-SMS-Login, Host 000; CSES trägt Limadou L2 (hält). JUNO: NASA Juno ODF registrieren; Neutrino-JUNO `not-published` (kein Eintrag). TA: USArray-TA-FDSN (seismisch, `fdsn_station_compiler`) ist die Messung; Telescope Array (Kosmik) ist keine Feldquelle. HAWC: nicht offen — via CA-Bundle (`curl --cacert` → 200) erreichbar, akzeptieren. LHAASO: den IHEP-`/lhaaso/`-Pfad als Route streichen (News-Seite, keine Daten); LHAASO trägt CASDC. FITS-Rice ist GEBAUT (`src/archivar/fits.rs:663`) — die Handover-Zeile „Rice fehlt" ist stale. limadou-PI: extern, Nachfassen = per-Akt-Consent (Operator). (Schritt: `phi/blocked_sources.φ` + `phi/sources.φ`-Disposition.)

An ernte: CI rot @ `17156ab4` (ci-check run 35089095235) — `cargo fmt --check` (Diffs in `src/archivar/parquet.rs`, `src/archivar/eea.rs`), clippy `the variable \`o\` is used as a loop counter` (`parquet.rs:923`), `cargo test --release` arithmetic overflow (`eea.rs:69`); die drei Dateien stammen aus `59a2b353`. (Schritt: `cargo fmt --all` + die zwei Stellen, dann `cargo check`.)
