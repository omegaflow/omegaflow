<!--
  title: Handover — Neptun-Scheinbar-Orts-Kette: die Kette gebaut und über den vollen Zeitraum (1846–1983) gegen das breite DE441-Zentrum reduziert (Median ~0.2″); die Datenprüfung der frühen HILTON-Ausreißer und die CDN-Manifestation des breiten Zentrums sind die zwei offenen Linien
  class: handover
  date: 2026-09-09
  sha256: d11dde13d76f5238a9e61c72c9a4c3ea2260f7c5f17ad691fc56824d69ac295f
  status: archived
  see-also: docs/handover/handover-2026-09-09-neptun-astrometrie-kopplung.md docs/befund/befund-2026-09-09-neptun-scheinbar-orts-kette.md docs/TODO.md
-->

# Handover — Neptun-Scheinbar-Orts-Kette: die Kette gebaut und über den vollen Zeitraum (1846–1983) gegen das breite DE441-Zentrum reduziert (Median ~0.2″); die Datenprüfung der frühen HILTON-Ausreißer und die CDN-Manifestation des breiten Zentrums sind die zwei offenen Linien

Übergabe für die nächste Sitzung. Die empfangende Sitzung hat
`handover-2026-09-09-neptun-astrometrie-kopplung.md` konsumiert und die eine
offene Linie (Scheinbar-Orts-Kette) gebaut und über den vollen Zeitraum
reduziert. Zwei offene Linien bleiben: die Datenprüfung der frühen
HILTON-Ausreißer (§2b) und die CDN-Manifestation des breiten Zentrums (§2a).

## 0. Die abgebende Sitzung ist abgeschlossen

Commits: `90f80f1` (Kette), `bb96264` (breite Ephemeride, Korrektur der
Deckel-Aussage), `41eccc1` (breites Zentrum + Fenster-Fix). Alle laufen
`cargo check` 0/0 und passieren `commit_check`. Der geteilte Index ist gegen
den Parallel-Session-Stand konsistent (§4).

## 1. Was gebaut und gemessen steht

- **Die Kette** — `src/archivar/astrometry.rs`: IAU-1980-Nutation (volle
  106 Terme, gegen den SOFA-Testvektor 1e-15), mittlere Schiefe,
  Newcomb- und IAU-1976-Präzession, Aoki-1983-FK4→FK5 (Seidelmann 3.591-4),
  FK5→ICRS-Frame-Bias, Aberration (jährlich + täglich), Parallaxe,
  Espenak–Meeus-ΔT; 14 Unit-Tests. Der Probe `neptune_apparent_chain_probe`
  (tools/measure) liest drei Formate (HILTON token-weise, URSS/BDL feste
  Spalten, OBSLIST.OPT λφh) und reduziert per Reihe: Aberration → Parallaxe
  (nur HILTON topozentrisch) → Nutation → Newcomb-Präzession → Aoki →
  Frame-Bias.

- **Zwei Konvention-Bugs gefunden und geflickt**: (a) Fundamental-Argumente
  in Bogensekunden statt Grad; (b) Präzession/Nutation transponiert (passive
  statt aktive Rotation) — der Fix ist `rot_z(z)·rot_y(−θ)·rot_z(ζ)`.

- **Das breite Zentrum** — `horizons_compiler --neptune-c-spk` spannt jetzt
  1802–2030 (war J2000±30; der `nep097xl-899.bsp` deckt −14000…+15000). Dabei
  ein O(n²)-Bottleneck im Granulen-Fit gefixt (Fenster-Subslice statt
  Full-Vektor-Scan). `ephemeris_neptune_c.bin` = 38 352 Granulen, 17 MB.

- **Die Messung** — Kalibrier-Gate: +40/−25 mas injiziert → +40.273/−25.079
  mas über alle 7391 Reihen zurückgewonnen (exakt). Reduktion gegen das breite
  Zentrum, Median: USNO +421 mas, CAMB +398, CAPE −27, GREN +172, NICE +635,
  PARI +217, TKY +244, URSS-USNO −75, NIK-Foto-B1950 −426 mas (RMS 488/498) —
  die glatte Mitte liegt bei **~0.2–0.6″**.

## 2. Die zwei offenen Linien

### (a) CDN-Manifestation des breiten Zentrums

`ephemeris_neptune_c.bin` ist lokal jetzt 17 MB (breit, 1802–2030); das
CDN-Asset ist noch das alte ±30-Jahre-Exemplar (4.9 MB). Duty:
`neptune-c-spk-cdn.yml` dispatchen (oder lokal `--ci-mode`), dann CDN-200 des
breiten Assets verifizieren. Der Compiler trägt die neue Range bereits — es
ist kein Code, nur ein Lauf.

### (b) Datenprüfung der frühen HILTON-Ausreißer

Die frühen Reihen (vor 1900) tragen Bogenminuten- bis Grad-Ausreißer
(CAMB 1862 −1.6°, 1868 −14.8°; GREN/RADC/PARI ähnlich). Der Median trennt sie
(glatte Mitte ~0.2–0.6″), Mittel/RMS werden von ihnen getragen. Sie sind
Transkriptionsfehler im APDB-Datum, keine Kettenfrage — die Kette bildet jede
geprüfte Reihe exakt ab (reduced ≈ model). Nächster Schritt: dem Probe ein
Ausreißer-Report-Modus wachsen lassen (Zeilen über N·σ listen), dann die
Zeilen gegen die Originalquelle prüfen.

## 3. Symmetrie-Duties

- `uranus-c-spk-cdn.yml` / `--uranus-c-spk` trägt dieselbe ±30-Hartcodierung
  und denselben O(n²)-Fit — falls die Uranus-Astrometrie je ein breites
  Zentrum braucht, ist es derselbe Zwei-Zeilen-Fix.
- BESA (Besançon, 112 Reihen) und STRA (Strasbourg, 63 Reihen) fehlen in
  OBSLIST.OPT — sie reduzieren ohne Parallaxe (topozentrisch als geozentrisch
  behandelt); λφh nachtragen wäre der saubere Abschluss der 171 Übersprungenen.

## 4. Arbeitsregeln (wie gehabt)

- Geteiltes Repo: nur eigene Dateien stagen, Hunks der Parallel-Session nie,
  `.git/index.lock` kurz abwarten; eigene Commits über `GIT_INDEX_FILE`,
  danach `git reset HEAD -- <eigene Pfade>`.
- Commit-Gate: `cargo check` 0/0, `commit_check`, Englisch, TODO im selben
  Commit.
- 0-Kanon: Dec-absent (PARI „0 0 0.0000") bleibt absent — die Reihe wird
  übersprungen, nie mit einer 0 fabriziert.
