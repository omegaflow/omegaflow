<!--
  title: Handover — Registratur-Grammatik gebaut (das nächste Atom: die cdn_reconcile-Kanonik der reference-Assets)
  class: handover
  date: 2026-09-09
  sha256: f4fb7c1038312ac59f09674673e269ef29d7e8df24736ebfa79b6999016dd7f9
  status: live
  see-also: docs/blatt/blatt-h0-linien-register.md docs/TODO.md src/archivar/parse.rs src/archivar/types.rs src/archivar/sha256.rs src/archivar/port.rs src/archivar/main_flow.rs phi/sources.φ tools/measure/src/bin/h0_ladder_weigh.rs
-->
# Handover — Registratur-Grammatik gebaut

Übergabe für die nächste Sitzung. Der Rat hielt den Befund vor der
Implementierung (Spruch b: fetch-only-Klasse `format reference`, kein vierter
Zeugen-Typ, kein descope); die Sitzung baute den Beschluss. Eine enge Pende
bleibt offen; sie ist das nächste Atom.

## 0. Die abgebende Sitzung ist ein abgeschlossenes Atom

Die zwei Vorgänger-Atome (Leiter gewogen, Crossmatch gewogen) ließen die
Registratur-Grammatik field-loser Probe-Eingaben offen. Der Rat sprach: ein
Referenzdatensatz ist eine Quelle, kein Zeuge — `Frame::Manifest` benennt den
ehrlichen Zustand „getragen, nicht geortet" bereits; die Lücke ist eine
Bedingung im Flush-Gate, keine neue Kategorie. Gebaut wurde genau das.

## 1. Was gebaut wurde

| Eingriff | Ort |
|---|---|
| `format reference` im Flush-Gate zugelassen (neben `kernel_text`) | `src/archivar/parse.rs` |
| `sha256`-Direktive → `SourceConfig.sha256` | `parse.rs` + `types.rs` |
| `sha256_hex` (reines Rust, Testvektoren) | `src/archivar/sha256.rs` |
| `reference` aus dem Runtime-Fetch-Worker ausgeschlossen | `src/archivar/main_flow.rs` |
| reference-Zweig in `ci_mode` (fetch_bytes → sha256-Verifikation → Upload as-is) | `src/archivar/port.rs` |
| vier statische Beine sitzen mit gemessenem sha256 | `phi/sources.φ` |
| `upload_asset_bytes` + `--ci-mode` gestrichen, Self-curl bleibt | `tools/measure/src/bin/h0_ladder_weigh.rs` |
| gelöscht (die Grammatik + `health-check.yml` tragen die Manifestation) | `.github/workflows/h0-ladder-cdn.yml` |

Der Flush-Gate bleibt für field-lose Blöcke ohne `reference` scharf (die Falle
steht); ein Referenzdatensatz ohne `sha256` sitzt trotzdem (der Pin ist
Provenienz, kein Sitz-Kriterium). Der Probe behält seinen Self-curl — das ist
die Ursprungs-Zertifizierung, kein redundanter Fetch.

## 2. Das nächste Atom — die cdn_reconcile-Kanonik

Der reference-Zweig lädt die vier Beine unter dem Ursprungs-Dateinamen auf das
CDN (`2012.08534`, `Pantheon+SH0ES.dat`, `Pantheon+SH0ES_STAT+SYS.cov`,
`Pantheon+SH0ES_STATONLY.cov`). `cdn_reconcile` leitet seine Kanonik über
`source_name_from_url` ab (`e-print-2012.08534` …) und nennt die Abweichung im
Report. Eine reference-bewusste Kanonik (Ursprungs-Dateiname statt
URL-Flattening) ist das enge Folge-Atom (in TODO registriert).

## 3. Gemessene Zustände — benannt, nicht geglättet

- **Die vier Pins (gemessen 2026-09-09, curl + sha256sum):** arXiv-Tarball
  `bc86e4e4…85e7`, `Pantheon+SH0ES.dat` `1cb0fc37…cf8`,
  `…_STAT+SYS.cov` `abf806d9…0fdc`, `…_STATONLY.cov` `9f177129…297f`
  (volle Hex-Werte in `phi/sources.φ` und im Blatt).
- **Die reference-Assets sind gesetzt, noch nicht manifestiert:** die
  Manifestation trägt der nächste `health-check.yml`-Lauf
  (`cargo run -- --verify phi` → `ci_mode("phi")`); lokal erfordert der Upload
  `GH_TOKEN`.
- **Der volle Lib-Testlauf überschreitet 600 s** in `test_live_sources_extract`
  (Live-Netz-Sweep über alle Quellen) — ein vorbestehender Netztest, keine
  Regression dieses Atoms; die gezielten Tests (sha256 + reference-Sitz) passieren.
- **Parallele Fremd-Sitzung im Arbeitsbaum:** während dieser Sitzung wuchs
  `docs/TODO.md` um einen „Seismische Ortung"-Rat-Konsens-Eintrag — nicht
  angefasst. Der Pionier-„ungeschlossene Klammer"-Zustand ist erledigt
  (`cargo check -p omegaflow-measure` kompiliert sauber bei HEAD); das
  ungetrackte Vorgänger-Handover ist durch `2a0c172` geschlossen.

## 4. Zeiger

- Probe: `tools/measure/src/bin/h0_ladder_weigh.rs` (+ geteilter Parser `tools/measure/src/h0.rs`)
- Blatt: `docs/blatt/blatt-h0-linien-register.md` (Registratur-Grammatik gebaut + Referenz-Pins)
- TODO: `docs/TODO.md` (cdn_reconcile-Kanonik unter „pending")
- Eingriffspunkte Atom 3: `tools/register/src/bin/cdn_reconcile.rs`, `src/archivar/naming.rs`
