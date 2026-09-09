<!--
  title: Handover — Matrix-Fix nachgetragen: Re-Verifikation maß zwei Wurzeln (size-gate + verlorener Fix), beide behoben; Recompile läuft, Galileo-Re-Lauf bereit
  class: handover
  date: 2026-09-09
  sha256: a798ca499b9b535271dab398f46e7abab8e07dca4429d0eec9f625e23432bdc1
  status: archived
  see-also: docs/handover/handover-2026-09-09-body-fixed-matrix-fix.md docs/TODO.md src/archivar/ephemeris.rs src/archivar/motion.rs tools/harvest/src/bin/ephemeris_compiler.rs
-->
# Handover — Matrix-Fix nachgetragen: Re-Verifikation maß zwei Wurzeln, beide behoben

Übergabe für die nächste Sitzung. Die Re-Verifikation der Übergabe
`handover-2026-09-09-body-fixed-matrix-fix.md` maß zwei Wurzeln und behob
beide: (1) der Bin-Recompile war durch eine Größen-Prüfung blockiert,
(2) der Matrix-Fix selbst war aus HEAD verloren gegangen. Der Recompile läuft
(CI Run 34348392827); die Re-Verifikation des `galileo_elevation_match` steht
danach als nächster Schritt bereit.

## 1. Was gemessen wurde (A-Zweig)

Der Kontroll-Lauf `galileo_elevation_match` (full + sanity) gegen das
Vor-Fix-Bin reproduziert `befund-galileo-inpass-elevation.md` exakt (Anchor:
st43 med_diff 1,828 / 20/5 / Ratio 9,688 · st63 18,827 / 9/4 / 35,3 · interior
st43 1,162/19/5, st63 4,757/9/4; Tabelle 0: st14 +1,0/9, st43 −22,5/20, st63
−7,9/9). Der Sanity-Sweep (Matrix-Pfad) zeigte die ~117°-Verschiebung
(DSS43-Peak 12:00 gegen Lehrbuch 04:00) — das Bin trug die alten falschen
Matrizen.

## 2. Wurzel 1 — der Recompile war blockiert (size-gate)

Der `kernel-flatten`-bodies-Job meldete „success", schrieb aber nichts: jeder
Kernel-Download wurde mit `size mismatch` abgelehnt, weil `phi/sources_index.φ`
falsche Größen trägt (0 B, 64512 B für ein 2-GB-SPK, ±191 B bei Textkerneln —
fragiles HTML-Listing-Parsing). Die byte-exakte Prüfung `landed != e.size` in
`download_missing` lehnte deshalb alles ab. Behoben: auf das Repo-Muster
(`epm_compiler::fetch_bsp`) zurückgeführt — akzeptieren bei `landed > 0`,
Teil-Datei bei curl-Fehler entfernen, Frische = existiert und nicht leer; dazu
die vom commit-gate benannten Fabrication-Muster derselben Datei beseitigt
(`numeric_of` → Option, jobs-Floor am Parse-Ort, `landed` → Option,
`--depth`-Default, fk-tk-Anzeige).

## 3. Wurzel 2 — der Matrix-Fix war aus HEAD verloren (der Kern-Befund)

Der erste Recompile (18 MB, 36 020 Matrizen) verlagerte den DSS43-Peak nur
12:00 → 09:00 — Restfehler ~5 h ≈ 75°. Die Nachforschung maß die Ursache: der
in `bf4d334`/`232a0ea` committete Fix war im aktuellen HEAD nicht mehr
vorhanden. `rotation_matrix_from_angles` baute weiterhin `w = pm − ra` mit
vertauschten Zeilen, `rotate_about_pole` las weiterhin die Polachse aus Zeile 2
statt Spalte 2; der Quer-Test
`test_matrix_path_matches_analytic_across_bodies_and_epochs` fehlte ganz. Die
`bf4d334`/`232a0ea`-Änderungen an `ephemeris.rs`/`motion.rs`/`tests.rs` sind in
der Datei-Historie nicht mehr sichtbar — eine spätere History-Änderung hat sie
überschrieben. Nachgetragen: der IAU-Fix Rz(90°+α)·Rx(90°−δ)·Rz(W) als
explizites Produkt, die Spalte-2-Polachse, und beide Regressionstests
wiederhergestellt.

## 4. Nebenbei gemessen

INPOP/EPM-Bins sind tatsächlich neu kompiliert (earth 2340/4875 Rotationen,
CDN last-modified heute) — sie tragen die korrigierten Matrizen schon. Das
neue ssd-earth-bin ist mit 18 MB / 36 020 Granulen / Spanne jd 1600–3000
deutlich kleiner als das alte (183 MB / 346 876 / Spanne 30 000 Jahre) — ein
getrennter Struktur-Unterschied, offen (siehe §5).

## 5. Offene Steine

1. **Re-Verifikation (der eine offene Stein der Ursprungs-Übergabe):** der
   laufende `kernel-flatten` (Run 34348392827) kompiliert das earth-bin jetzt
   mit dem nachgetragenen Matrix-Fix. Danach: lokal auffrischen
   (`data/ssd.jpl.nasa.gov/ephemeris_earth.bin`, Membran-Cache löschen) und
   `galileo_elevation_match` full + sanity re-laufen. Hypothese (nicht
   Behauptung): Haupttabellen unverändert (Lehrbuch-Pfad), Sanity-Sweep
   (Matrix-Pfad) jetzt im Einklang mit dem Lehrbuch (DSS43-Peak 04:00).
2. **Membran-Folgeauftrag** (canSense/canRadiate): Orientierungs-Sonde bauen
   (Sub-Solar, Matrix gegen analytischen Pfad, echte Bins) + Funktions-Check;
   kein visible run. Als Folgeauftrag im Register.
3. **Getrennte CI-Lücken** (vorbestehend, nicht Matrix): TNO-Split fällt auf
   fehlendem `phi/pipeline/catalog/asteroid_gm_sb441.φ`; `--omega-g` liest
   `solar_omega_g.φ` void (beides gitignored, auf CI-Runner absent). Der
   ssd-earth-bin-Struktur-Unterschied (18 MB vs 183 MB, 1400 vs 30 000 Jahre
   Spanne) ist ungemessen — offen.

## 6. Commits

`04d5e86` size-gate-Fix (ephemeris_compiler) · `d41a945` Matrix-Fix
nachgetragen. Beide auf `origin/main`; `main` == `origin/main` == `d41a945`.
Kein Stray dieser Sitzung.

## 7. Zustand des Arbeitsbaums

Fremde uncommittet (Parallel-Sitzung, unangetastet):
`.github/workflows/{gaia-xp-cdn,meteo-cdn}.yml`, `Cargo.{toml,lock}`,
`docs/auftrag/gavo-dc-account-anfrage.md`,
`docs/specs/meteo-korrelations-screening-vorlage.md`,
`tools/harvest/src/bin/{gaia_xp_compiler,tap_compiler}.rs`,
`docs/handover/handover-2026-09-09-te-atome-blocknull-ksg-pcmci.md`,
`tools/vo-tap/`. Nichts davon angefasst. `cargo check -p omegaflow` 0/0;
`cargo test -p omegaflow --lib -- matrix|rotation` grün (20/20 bzw. 4/4).
