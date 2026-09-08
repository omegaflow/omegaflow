<!--
  title: Handover — H₀-Crossmatch gewogen (das nächste Atom: die Registratur-Grammatik mit Rat)
  class: handover
  date: 2026-09-09
  sha256: 436d665e1584094f1c21ce609fb63f8fc8c523e42166b5ef6d12bd7e200aad7d
  status: live
  see-also: docs/blatt/blatt-h0-linien-register.md docs/TODO.md tools/measure/src/bin/h0_gaia_crossmatch_probe.rs tools/measure/src/h0.rs src/archivar/parse.rs src/archivar/zeuge.rs
-->
# Handover — H₀-Crossmatch gewogen

Übergabe für die nächste Sitzung. Das abgebende Atom hat die 74 Namen der
SH0ES-Tabelle end-to-end gegen Gaia DR3 aufgeschlagen — das eigene
Gaia-TAP-Crossmatch ist gewogen, die Transkription π_EDR3 ist zertifiziert.
Eine Pende bleibt offen; sie ist das nächste Atom.

## 0. Die abgebende Sitzung ist ein abgeschlossenes Atom

Der Probe `tools/measure/src/bin/h0_gaia_crossmatch_probe.rs` schlägt die 74
Namen über SIMBAD (`sim-tap/sync`, `ident`→`basic`) in Positionen auf und zieht
je Position den nächsten Gaia-DR3-Quellstern im 3″-Konus; `gaiadr3.vari_cepheid`
trägt die Klassifikation als zweite Linie. Der Namen-Parser `parse_cepheids`
und der Namen-Normierer `normalize_name` leben jetzt geteilt in
`tools/measure/src/h0.rs` (beide Bins). Commit: `b068cc0`.

## 1. Was gewogen wurde

| Befund | Wert |
|---|---|
| Namen aufgelöst | 74/74 via SIMBAD `ident` (Hauptname `V* …`, einer `* 12 Sgr`) |
| Identität (Separation < 2″) | 74/74 — Identity-Gate PASS |
| Parallax-Offset π_EDR3 − Gaia-DR3 | Median +21 μas, Spanne [+4, +38] μas |
| vari_cepheid-Klassifikation | 70 DCEP, 4 absent (T Mon, 12 Sgr, U Sgr, V636 Sco) |

- **Der Offset ist der L20b-Nullpunkt:** die Tabelle trägt laut Note d den
  L20b-Parallax-Offset (nicht den Residuen-Offset −14 μas); der gemessene
  Median +21 μas über 67 gefittete Sterne ist genau dieser Offset. Die
  Transkription π_EDR3 ist gegen Gaia-DR3 zertifiziert, nicht kopiert.
- **74 von 74, nicht 75:** die Tabelle trägt 74 volle Datenzeilen; der Probe
  schlägt alle 74 auf — auch die 7 `\nd`-Sterne tragen eine gemessene Identität
  (ihr π bleibt absent, ihre Position ist gemessen, 0 honored).
- **Die MRT-Route wurde gemessen und erübrigt:** SIMBAD (lebend, HTTP 200)
  trägt alle 74 Namen vollständig — kein IOP-MRT-Fetch nötig (IOP sitzt hinter
  Radware-CAPTCHA, siehe Blatt-Anhang). Der Normierer trägt die Tisch-Marker:
  `$`-Fußnoten gestrichen, Konstellationen expandiert, `V0`-Nullen entfernt.

## 2. Das nächste Atom — die Registratur-Grammatik (mit Rat)

`sources.φ` trägt keinen ehrlichen Sitz für field-lose Probe-Eingaben: der
`flush`-Gate in `src/archivar/parse.rs` fordert `kernel_text` oder einen Frame
und verwirft den Rest. Die drei statischen Beine des `h0_ladder_weigh`
(arXiv-Tarball, `Pantheon+SH0ES.dat`, `.cov`) manifestieren über `--ci-mode` +
Workflow `h0-ladder-cdn.yml`; der Crossmatch-Probe materialisiert kein Asset
(reine Messung, stdout — Präzedenz `cepheid_parallax_weigh`).

Der Rat (council) hält den Befund vor der Implementierung — eine neue Kategorie
im 0-Kanon wird nur durch ein Sprachloch verdient. Die drei Sitz-Optionen:
(a) vierter Zeugen-Typ „Referenzdatensatz" (`phi/witnesses.φ` +
`src/archivar/zeuge.rs`), (b) fetch-only-Klasse, (c) gemessener Befund-Release
(descoped mit Befund). Erst nach dem Rat baut die Sitzung den Beschluss.

## 3. Gemessene Zustände — benannt, nicht geglättet

- **`pioneer_link_correction_probe.rs` trägt eine ungeschlossene Klammer** im
  Arbeitsbaum (WIP einer Fremd-Sitzung) — blockiert `cargo check -p
  omegaflow-measure` über *alle* Bins, nicht aber die beiden H₀-Bins. Nicht
  angefasst; die Sitzung, der er gehört, trägt die Pflicht.
- **Die Vorgänger-Handover `handover-2026-09-08-h0-leiter-gewogen.md` steht
  ungetrackt (nicht committet)** im Arbeitsbaum.

## 4. Zeiger

- Probe: `tools/measure/src/bin/h0_gaia_crossmatch_probe.rs`
- Geteilter Parser: `tools/measure/src/h0.rs`
- Blatt: `docs/blatt/blatt-h0-linien-register.md` („Das eigene Gaia-TAP-Crossmatch der 74")
- TODO: `docs/TODO.md` (Registratur-Grammatik-Pende unter „pending")
- Eingriffspunkte Atom 2: `src/archivar/parse.rs` (flush-Gate), `src/archivar/zeuge.rs`, `phi/witnesses.φ`
