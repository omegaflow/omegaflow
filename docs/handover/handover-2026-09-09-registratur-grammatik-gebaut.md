<!--
  title: Handover — Registratur-Grammatik gebaut (Atom geschlossen: keine Pende offen)
  class: handover
  date: 2026-09-09
  sha256: d48e2fb73f78cad336709b6b996608ac37b80aa241b22b79788119e2cbfa4431
  status: live
  see-also: docs/blatt/blatt-h0-linien-register.md docs/TODO.md src/archivar/parse.rs src/archivar/types.rs src/archivar/sha256.rs src/archivar/port.rs src/archivar/main_flow.rs src/archivar/naming.rs tools/register/src/bin/cdn_reconcile.rs phi/sources.φ tools/measure/src/bin/h0_ladder_weigh.rs
-->
# Handover — Registratur-Grammatik gebaut

Übergabe für die nächste Sitzung. Der Rat hielt den Befund vor der
Implementierung (Spruch b: fetch-only-Klasse `format reference`, kein vierter
Zeugen-Typ, kein descope); die Sitzung baute den Beschluss und schloss auch
die Folge-Pende (cdn_reconcile-Kanonik). Keine Pende bleibt offen.

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
| reference-Zweig in `ci_mode` (fetch_bytes → sha256-Verifikation → `ensure_release` → Upload as-is) | `src/archivar/port.rs` + `src/archivar/cdn.rs` |
| vier statische Beine sitzen mit gemessenem sha256 | `phi/sources.φ` |
| `upload_asset_bytes` + `--ci-mode` gestrichen, Self-curl bleibt | `tools/measure/src/bin/h0_ladder_weigh.rs` |
| gelöscht (die Grammatik + `health-check.yml` tragen die Manifestation) | `.github/workflows/h0-ladder-cdn.yml` |

Der Flush-Gate bleibt für field-lose Blöcke ohne `reference` scharf (die Falle
steht); ein Referenzdatensatz ohne `sha256` sitzt trotzdem (der Pin ist
Provenienz, kein Sitz-Kriterium). Der Probe behält seinen Self-curl — das ist
die Ursprungs-Zertifizierung, kein redundanter Fetch.

## 2. Die cdn_reconcile-Kanonik — geschlossen

Der reference-Zweig lädt die vier Beine unter dem Ursprungs-Dateinamen auf das
CDN; `cdn_reconcile` leitete seine Kanonik vorher über `source_name_from_url`
ab (`e-print-2012.08534` …) und nannte die Abweichung im Report. Geschlossen
durch eine geteilte Kanonik: `reference_name_from_url` (`src/archivar/naming.rs`)
liefert den Ursprungs-Dateinamen, `port.rs` lädt unter ihm hoch und
`cdn_reconcile` erwartet ihn — gemessen 0 `divergence` / 0 `missing` für
`arxiv.org` und `raw.githubusercontent.com`.

## 3. Gemessene Zustände — benannt, nicht geglättet

- **Die vier Pins (gemessen 2026-09-09, curl + sha256sum):** arXiv-Tarball
  `bc86e4e4…85e7`, `Pantheon+SH0ES.dat` `1cb0fc37…cf8`,
  `…_STAT+SYS.cov` `abf806d9…0fdc`, `…_STATONLY.cov` `9f177129…297f`
  (volle Hex-Werte in `phi/sources.φ` und im Blatt).
- **Die reference-Assets sind manifestiert (2026-09-09):** der
  `ci_mode`-reference-Zweig verifizierte die vier sha256 und lud unter
  `arxiv.org` / `raw.githubusercontent.com` auf das CDN (`2012.08534`
  320718 B, `Pantheon+SH0ES.dat` 579283 B, `…_STAT+SYS.cov` 33284960 B,
  `…_STATONLY.cov` 31827416 B). Der Zweig stellt den Release-Tag zuerst
  (`cdn::ensure_release`); der nächste `health-check.yml`-Lauf prüft nur noch
  nach (Mismatch = „Sha256 Drift").
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
- TODO: `docs/TODO.md` (die Registratur-Grammatik-Pende ist geschlossen — keine Zeile mehr offen)
- Eingriffspunkte: `tools/register/src/bin/cdn_reconcile.rs`, `src/archivar/naming.rs`
