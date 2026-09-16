<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 2a0c3a73972d69b71dd10c4d248df4a1dfe0d9742ce9291ad27a04dae15cacff
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

An ernte: `phi/dead_sources.φ` Domaincheck 2026-09-16 — 275 Domains, 52 DNS-tot (alle mit Dienst-Identität/Nachfolger = gelebt); 6 nie-gelebt-Einträge entfernt (vokal-ausgedünnt `p.ntrlst.rg`→`api.inaturalist.org`, `p.pn-mt.cm`→`api.open-meteo.com` — beide leben; `lightning.gld`; `masie_ice.apps.nsidc.org`). Relevanz-Erstpass (flash): 91 sichere + 35 unsichere „nie Force-Kanal"-Kandidaten → `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md`. (Schritt: Force-Gate-Verdikt nach SOURCE_PORT.md, dann Prune; die zwei lebenden Dienste ggf. als Quellen registrieren.)


An ernte: Ernte-Register-Rest aus dem entscheid-Handover ausquartiert — MPC-Shard UnnObs, GHRC-DAAC, Survey §1 (26 Pendings); Register-Digest-Rest (Fink/ALeRCE, TDAT/FITS, Akteure, Holdings, Orphan-Verdicts). (Schritt: `sources.φ` + CDN / je Punkt messen.)

An bau: `smail_recv` — leerer Body bei verschachteltem MIME; der rekursive MIME-Abstieg (`collect_text` in `tools/service/src/bin/smail_recv.rs`) ist gebaut + getestet, der Root cause (leerer Sotgiu-Reply) unbestätigt. (Schritt: bei erneutem leerem Body die Worker-`message.raw`-Quelle messen — `cloudflare/email_worker.js`.)

An ernte: BiSON p-Moden — die Daten sind offen ladbar (`bison.ph.bham.ac.uk/opendata`, `allsites-alldata-waverage-fill.fits.gz`, 200; Hale et al. 2015, DOI 10.1007/s11207-015-0810-0); die Mail-Anfrage an das BiSON-Team ist damit hinfällig, der Schritt ist ein `bison_compiler` (FITS.gz → Zeitreihen-bin) oder ein ASCII-Dump, GONG `gong2.nso.edu/` die Live-Alternative. (Quelle: `phi/pipeline/research/agent_output/grind_suchliste_a_urteil.φ`, `survey-2026-09-14-warteliste-offene-alternativen.md`.)

An bau: aus dem entscheid-Handover ausquartiert (eigene Domäne, keine Operator-Aufgaben): **Mail-Fang** (`cloudflare/wrangler.toml:14` trägt noch `REPLACE_WITH_WRANGLER_KV_NAMESPACE_ID` → KV-Namespace anlegen, Id eintragen, `wrangler deploy`), **20-s-Bande-Papier** (`git tag` + Welt-Fassung-Branch zu `docs/paper/twenty-second-band-ground-chain.md`), **Register-Digest `--live`** (Rat-Verdikt: das Flag reibt an der Registerklasse `live` → `--open`). (Schritt: bau baut/deployt.)
