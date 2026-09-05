<!--
  title: Befund — Negativ-Fuzzy-Index auf Bio/Techno
  class: befund
  date: 2026-09-05
  status: done
  sha256: 79108b0a028219abf450330065675af7e568d87931bdba92d9cc9abdda61a1f2
  antwortet-auf: docs/auftrag/auftrag-negativ-fuzzy-techno.md
-->

# Befund: der negative Fuzzy-Index auf Bio/Techno

## Zweistufige Maschine (auf den echten Instrumenten)

1. **Extraktion** — Kandidat gegen alle Zeugen regressieren. Serien-Zeugen →
   `conditional_te_stats` + `residual_surrogate_conditional` (te.rs); Skalar/
   Katalog-Zeugen → Permutations-Null (Spezies↔Host bei fixer Zeugen-Spalte).
2. **Negativ-Test** — `transfer_entropy_lag` beide Richtungen je Lag gegen
   `surrogate_stats_phase` mean+2σ und die fam-Schranke.

## Zeugen-Katalog (gemessen)

**Techno-Dip:** Variabilität (vsx_period_d :3725, gcvs_period_d :3491 — Klasse/
OType ungemappt), Spots/Rotation (g/r-Paar), Staub (IRAS :3782, AKARI :3771),
Bedeckung (exoplanets.json + stellarhost Teff), Systematik (ZTF↔TESS-Zwei-
Instrument-Koinzidenz — Gate noch nicht verdrahtet), Hephaistos-Hintergrund.
**Techno-Narrowband:** die Frequenz-Kanal-Serie existiert nicht im Bestand —
jeder Linien-Zeuge (RFI off-source, Kanal-Baseline) ist **nötig** → pending.
**Bio-Disequilibrium:** Gleichgewicht selbst (Primär-Null; die P=0.94-Permutation
ist der Negativ-Test), Host-Teff (gehalten), Stellar-Aktivität/XUV
(Photochemie-Treiber der SO2/CO2-Hits — **fehlt**, pending), Reservoir [Fe/H]/
C/O (pscomppars st_met unread).

## Benannte Grenzen

n-Floor (aperiodische Dips < N_MIN → Scanner schweigt), falsches Lag
(Kreuzungszeit v·Δt, nicht Kadenz 0..1 — die schärfste Techno-Grenze), falscher/
fehlender Zeuge (TE≈0 gegen bekannte Zeugen schließt nie eine unbekannte
natürliche Klasse aus), Bio-Spezies-Achsen-TE wäre Fabrikation.

## Urteil

Der Index ist übertragbar; der Bio-Kanal trägt seinen Negativ-Test bereits
(die P=0.94-Permutation). Die nötigen Bio-Zeugen (Aktivität/XUV, [Fe/H]) fehlen
als gelesene Serie — pending, nicht getragen.
