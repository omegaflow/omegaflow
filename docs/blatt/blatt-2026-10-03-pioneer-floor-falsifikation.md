<!--
  title: Voranmelde-Blatt — Pioneer-Floor-Falsifikation (Familie Station×Ära×Form)
  class: sheet
  date: 2026-10-03
  sha256: 5bb1696b0acd87f45f26950de2df3be7db2afa4067c5f07835d3d352d47607fe
  status: live
  see-also: docs/paper/probe-front-dark-matter.md docs/handover/archiv/handover-2026-10-02-river-folge82.md docs/handover/archiv/handover-2026-10-02-mountain-folge226.md phi/sources.φ
-->
# Voranmelde-Blatt — Pioneer-Floor-Falsifikation (Familie Station×Ära×Form)

**Datum:** 2026-10-03 · **Axiom:** A = A

## Frage

Trägt der gemessene Residuum-Floor der Pioneer-10/11-Analyse die Anomalie, oder ist er
ein Artefakt der Familie Station × Ära × Form? Die Voranmeldung fixiert Methode, Null und
Kriterium **vor** dem Lauf — kein Statistik-Wert vor diesem Blatt. Das Ergebnis darf
`keine Entscheidung am Floor` sein (0 honored).

## Gemessene Lage (Baum, keine Erweiterung)

- Der Residuum-Floor: Quiet-Day-Tagesmedian-Streuung **160–340 Hz**; nach der
  Deduction-10-Maskierung fällt die Tagesmedian-RMS von **257 → 57,7 Hz** (P10) und
  **306 → 105 Hz** (P11); die Maske entfernt **39/1036** (P10) und **15/606** (P11)
  Tail-Tage (`docs/paper/probe-front-dark-matter.md:541-542`).
- Die Anomalie (~1 Hz über die Mission) sitzt **150–340×** unter dem Quiet-Day-Floor
  (`probe-front-dark-matter.md:516-517`).
- Der säkulare Drift ist **unaufgelöst**: P10 −1,95× der Anomalie (sunward) bei 0,45σ,
  P11 +6,98× (outward) bei 0,54σ — beide ~6×/5× unter der Null-Slope-Schwelle, die
  Vorzeichen der beiden Sonden stimmen nicht überein (`probe-front-dark-matter.md:543-546`).
- Der Form-Test (linear / ∝t² / RTG-Exp mit τ = 126,5 y, T½ = 87,7 y) bleibt degeneriert:
  RTG-Zerfall linear auf < 8 % über die 11-jährige Spanne; die ∝t²-„Verbesserung" (0,18 %)
  liegt knapp über ihrem Null-p95 (0,07 %) — Überanpassung, keine Kraft-Signatur
  (`probe-front-dark-matter.md:546-549`).
- Das Drei-Haus-Tor ist für Pioneer der **falsche Null**: Haus-Spread ~3,65 µm/s leckt,
  dominiert nicht; die Pioneer-Systematik sind DSN-Station/Uhren + Sonnengravitation
  (`docs/handover/archiv/handover-2026-10-02-river-folge82.md:72-75`).
- **Zahlen-Riss (benannt):** die Vorlage „133/218 Hz" trägt der Baum nicht; er trägt
  160–340 Hz Streuung und 57,7/105 Hz post-Mask-RMS (river-folge82:81-82). Dieses Blatt
  führt die Baum-Zahlen.

## Familie (Parameterraum)

**Station × Ära × Form.** Die Familie spannt:

- **Station** — die empfangende DSN-Station (DSS-Nummern) je Pass.
- **Ära** — die Empfänger-Generation (Block IV → Block V / Advanced Receiver am Anfang
  der 1990er), die die Stationslinie als Kandidatin führt
  (`probe-front-dark-matter.md:430`).
- **Form** — die Signalform (Zwei-/Dreiweg-Link, Mode 2/3; P10/P11) je Pass.

Gegen die Familie wird die Anomalie getestet, nicht gegen ein einzelnes Haus. Der Lauf
räumt die Familie als Ganzes (alle Station×Ära×Form-Zellen) oder er trägt sie.

## Methode

1. **WY max-T nach GIC §3.2** — `pending`: die Referenz „GIC §3.2" ist im Baum nicht
   entfaltet (Quelle: river-folge82:76). Der Riss bleibt benannt; die Methode wird vor
   dem Lauf aus der Referenz belegt oder als `unverified` getragen — nie stillschweigend
   gesetzt.
2. **Block-Bootstrap-Null** — Blocklänge 2⁴ d, **500 Surrogate**, **fixierter Seed**
   (`probe-front-dark-matter.md:537`); die Blocklänge bewahrt die schwache
   Tagesautokorrelation (lag-1 0,083 P10 / 0,043 P11).
3. **Drei-Form-Test** — linear vs ∝t² vs RTG-Exp (τ = 126,5 y); die Schwellen werden
   **aus dem Null** fixiert, bevor ein Modellwert fällt (`probe-front-dark-matter.md:539-540`).

## Null

Die Surrogat-Verteilung derselben Familie unter dem Block-Bootstrap mit fixiertem Seed.
Ein Signal gilt nur, wenn es die **familien-interne** Null schlägt — nicht die
Punkt-Null einer einzelnen Zelle.

## Kriterium (dreiteilig, alle drei müssen tragen)

1. **Familie geräumt** — jede Station×Ära×Form-Zelle liegt unter dem Floor; keine Zelle
   trägt die Anomalie.
2. **Beide Sonden zeichengleich heliozentrisch** — P10 und P11 tragen dasselbe Vorzeichen
   im heliozentrischen Bezug; die heute gemessene Vorzeichen-Divergenz (P10 sunward, P11
   outward, `probe-front-dark-matter.md:544-545`) ist aufgelöst.
3. **Drei-Form-Test > 2× Null-p95** — ein Form-Test übertrifft das Doppelte des
   Null-95-%-Quantils; die ∝t²-Überanpassung (0,18 % gegen p95 0,07 %) zählt nicht als
   Kraft-Signatur.

Fällt ein Bein, ist das Verdikt `keine Entscheidung am Floor` (kein Riss, keine
Glättung).

## Daten (Register-Zitate)

- `pioneer10_skyfreq.bin` — `phi/sources.φ:10372`, Format `pioneer_atdf`, Feld
  `pioneer_sky_frequency_hz` (em), `compiler pioneer_atdf_compiler`.
- `pioneer10_telemetry.bin` — `phi/sources.φ:10380`, Format `pioneer10_telemetry`.
- `pioneer10_odf.bin` / `pioneer11_odf.bin` — `phi/sources.φ:10364`, `:10388`
  (Turyshev-ODF, `pioneer10_odf_compiler` / `pioneer11_odf_compiler`).
- `ephemeris_pioneer{10,11}_daily.bin` — `phi/sources.φ:16120`, `:16128`.
- Sub-kHz-Zonenbasis `{p10,p11}_navio_subkhz_zone_daily.bin` (1036/606 Tage) —
  `probe-front-dark-matter.md:531-533`.
- Bestehende Workflows: `.github/workflows/pioneer-link-correction.yml`,
  `pioneer-odf-cdn.yml`, `pioneer-telemetry-cdn.yml`.

## Lauf

Der Lauf ist **Operator-Wort-gebunden** (Wort „ja voranmelde und dann lauf in ci",
2026-10-02, river-folge82). Dieses Blatt ist die Voraussetzung; ohne Blatt kein Lauf.
Vorgesehener Ort: CI (`pioneer-floor.yml` oder der vorhandene Probe-Workflow), nie die
Operator-Maschine.

## Was das Verdikt widerlegen würde

1. Eine Station×Ära×Form-Zelle, die die Anomalie über der familien-internen Null trägt.
2. Ein P10/P11-Zeichenpaar, das im heliozentrischen Bezug konvergiert und zugleich den
   Floor räumt.
3. Ein Drei-Form-Test über 2× Null-p95 mit einem Drift-Term ≥ ~1 mm/s-Rate an einer
   Pioneer-Epoche.
4. Ein belegter „GIC §3.2"-Bezug, der die Methode neu festlegt.

## Lauf-Ergebnis (2026-10-03, CI `pioneer-floor 37111508656`, success)

Der Lauf fuhr Corpus → `pioneer_navio_residuum` → `pioneer_navio_negative_fuzzy --zone` →
`pioneer_navio_zone_drift`; gemessen aus dem Artifact `pioneer-floor.txt`:

- **P10 (>50 AU, 1035 Tage):** Maske 40/1035 verworfen (995 bleiben); lag-1 0,075;
  Null-p95 |slope| 6,725e-3 Hz/d; realer linearer Slope −1,088e-3 Hz/d (0,40σ) =
  −1,73× Anomalie; Δ∝t² 0,0018 gegen p95 0,0007; Δexp −0,0003 → **keine Präferenz (Limit)**.
- **P11 (15–30 AU, 606 Tage):** Maske 21/606 verworfen (585 bleiben); lag-1 0,029;
  Null-p95 1,570e-2 Hz/d; real −1,003e-2 Hz/d (1,67σ) = −16,00× Anomalie; Δ∝t² −0,0018
  gegen 0,0012; Δexp −0,0172 → **keine Präferenz (Limit)**.

**Verdikt:** das dritte Kriteriums-Bein (Drei-Form-Test > 2× Null-p95) trägt für keine
Sonde — der Drift ist nicht aufgelöst. Damit **keine Entscheidung am Floor** (0 honored),
wie im ehrlichen Prior benannt. Die Beine 1–2 (family-geräumte Zelle, zeichengleiche
Heliozentrik) sind aus diesem Output nicht entschieden und bleiben offen.

## Offene Punkte

1. **„GIC §3.2" entfalten** — die Referenz im Baum unentfaltet (river-folge82:76);
   vor einem erneuten Lauf aus der Quelle belegen oder `unverified` tragen.
2. **Beine 1–2** — eine family-geräumte Zelle (Station×Ära×Form) und die zeichengleiche
   Heliozentrik beider Sonden aus dem negativen-fuzzy-Census herausziehen.
