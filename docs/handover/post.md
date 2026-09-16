<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 2c41af3cc919c115c71a678e0325d4bb74489f3c2857c53d6aa13038ec0b98cd
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und löscht die Zeile.

An alle Linien (format-Gate): CI-`format` rot @625452e5 — fremde unformatierte Dateien: `src/gate/commit_gate.rs:540`, `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`, `tools/utils/src/bin/archive_search.rs` (die ernte-eigenen fünf harvest-Compiler trägt das Ernte-Handover). (Schritt: rustfmt-Diff aus `ci-check` run 35091175017 anwenden.)

An die DRS-FITS-Linie: `tools/harvest/src/bin/drs_fits_compiler.rs:93` (`{epoch:.3f}` = ungültiger Format-Trait) blockt `cargo check -p omegaflow-harvest` im geteilten Baum; die Datei bleibt unangetastet. (Schritt: DRS-FITS-Linie fixt ihre Datei.)

An ernte: `phi/blocked_sources.φ` korrigieren (browser-gemessen 2026-09-16) — **CDDIS IONEX** (`:36–38`) `blocked key-needed` ist **stale**: der vorhandene `EARTHDATA_EDL_TOKEN` öffnet das Verzeichnis (`https://cddis.nasa.gov/archive/gnss/products/ionex/2026/` → HTTP 200, 96 KB), kein `client_id` nötig → Eintrag nach `sources.φ` (Register + Reader). **WWLLN** (`:40–43`) → `declined` (kommerziell: UW-copyright, „nominal cost" = kostenpflichtig; keine kostenpflichtigen Dienste). **GES-DISC**: kein Nutzer-`client_id` — der OAuth-Flow nutzt GES-DISCs eigene `client_id` (`e2WVk8Pw6weeLUKZYOxvTQ`); Bau implementiert den Authorization-Code-Flow (`S3CredentialRoute::OAuth`), das EDL-Konto ist `Application Creator: False`. (Schritt: Register-Disposition.)

An entscheid: Operator-gebundene Punkte aus Bau-Folge 46/47 — **Mail-Fang** (`wrangler kv namespace create MAIL_QUEUE` → Id eintragen → `wrangler deploy`, Cloudflare-Konto), **ESP32-Modul** (on hold; BOM `docs/specs/mantis-shrimp-bom.md`), **20-s-Bande-Papier** (`git tag` + Welt-Fassung-Branch, Name/Datum fehlt), **Register-Digest `--live`-Namensstimme** (`--open` oder bleibt). (Schritt: je Operator-Wort.)

