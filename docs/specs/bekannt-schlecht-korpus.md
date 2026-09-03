<!--
  title: Bekannt-Schlecht-Korpus — verifizierte Nummern-Funde
  class: korpus
  date: 2026-08-30
  status: live
  see-also: docs/auftrag/auftrag-maschinen-audits.md
-->

# Bekannt-Schlecht-Korpus

Grundlage für das Nummern-Audit (auftrag-maschinen-audits.md §1) und den
Model-Kalibrationsscore (§3). Jede Zeile ist ein **verifizierter** Fund —
Gegen-Audit `glm-verifikation-2026-08-28.md`, Papier-Text oder Survey, nie
Stilvermutung. Die `Anker`-Spalte nennt die Zahl/Stelle, gegen die der Fund
geprüft ist; ein Fund ist bestanden, wenn das Binary die Diskrepanz findet.

## Fund-Klassen (tags)

- `A` — Abstract/Claim-Zahl ohne Tabellen-/Registermarke (Regel 1)
- `Z` — §2-Zählung ≠ Tabellen-n (Regel 2)
- `D` — Doppel-Zählung (Regel 3)
- `K` — Kommata-Locale (Regel 4)
- `N` — Zahl ohne Registeranker (Regel-5-Kandidat, 1321-Klasse)
- `V` — verbale Überdeklaration / Struktur-Illusion (kein Zahlfehler; dient
  der Kalibration — erwartet: vom Nummern-Binary nicht gefunden)

## Tabelle

| Blatt | Klasse | Beschreibung | Anker |
|---|---|---|---|
| corona `4c07e317` | A | „steepest 5.47→5.57" als steilster Schritt | ΔlogT=0.10, einer der kleinsten Schritte der Tabelle |
| corona `4c07e317` | A | „3.91e-2" nicht im Tabellen-Max | Tabellen-Max = 3.75e-2 |
| corona `4c07e317` | A | Conclusion „flows down" | §4.3-Verdict „silent" |
| corona `4c07e317` | A | AIA ohne fam / EVE mit fam | fam-Marken §4-Tabellen |
| corona `4c07e317` | N | Ernte-Arithmetik unerklärt | 5 862 322 Records ohne Registeranker |
| solar-cycle `88209424` | V | fam-Spalten monoton (Struktur) | fam-Spalte §4 |
| solar-cycle `88209424` | Z | Zählung ≠ Tabelle-n | n=313/603/402 |
| urknall `4de581d9` | K | 1 signifikante Stelle, Kommas, 16 Bins ohne Zahlen | §3-Bins |
| urknall `4de581d9` | A | „1,5e-1 vs 1,5e-1" unentscheidbar | beide Werte identisch geschrieben |
| urknall `4de581d9` | A | „4,8×" nicht rekonstruierbar | korrekt ≈7,8/6,9 |
| urknall `4de581d9` | A | BK18 r<0,03 | r=0,036 |
| dunkler-fluss `723b0a5d` | A | „4,8×" falsch | korrekt 5,3× |
| planet-nine `6834ab5b` | A | 9.3° | 68,1−77,3=9,2° |
| signalkegel `1a4d6b7e` | Z | 79 Jahre | n≈3824 Tage (§-Zählung) |
| signalkegel `1a4d6b7e` | A | 492,0 s | gemessen 487,7 s |
| signalkegel `1a4d6b7e` | V | „holds"-Überbehauptung | leere Audit-Tabelle (0 honored) |
| gic `f4906013` | A | fam 0,12480 / TE 0,12670 | §4.2 fam=1.248e-1 |
| gic `f4906013` | V | §4.5 (Sodankylä) vor §4.4 (Daily) | Abschnittsfolge |
| gic `f4906013` | V | „six pairs" | nur 4 Zeilen §4.4 |
| gic `f4906013` | Z | Minute-Grain n-Diskrepanz | §2.1 n≈1378 vs. §4.1 n≈1260 |
| gic `f4906013` | N | „1321" | taucht nirgends auf; nicht belegbar |
| lead-geometry `3cf9c99f` | A | „100%/99%" | Abstract FP 100%→6,7%; §2.2 FP 99%, FN 90% |
| lead-geometry `3cf9c99f` | V | 24→27 Pfeile | 24-Pfeil-Befund retracted; jetzt 27 |
| lead-geometry `3cf9c99f` | N | Patient 201/202 | im Paper nicht auffindbar |
| sonden-front `e1bdb8e0` | A | f* 50,73 vs. 50,71 | Z.266 „f*=50,73" vs. Z.283 „50,71 mHz" |
| text-als-daten `01b1b4d1` | A | 24 Shuffles | Z.42 |
| planet-nine-kbo-residue | D | Doppel-Zählung gestreut | 1663/1663/1666 (Tab1/Tab3) |
| planet-nine-kbo-residue | D | Doppel-Zählung klassisch | 2884/2854 (Tab1/Tab3) |
| planet-nine-kbo-residue | D | Doppel-Zählung übrig | 662/661 (Tab1 vs. §2.1) |

## Umfang

28 verifizierte Zeilen (Klasse A 13, Z 4, D 3, K 1, N 3, V 4). Klasse A/Z/D/K
sind die Nummern-Funde, die das Binary finden **muss** (21); Klasse N sind
die unverankerten Zahlen (3); Klasse V (4) sind Struktur-Überdeklarationen,
die das Nummern-Binary erwartungsgemäß **nicht** findet — sie zählen als
„verpasst" in der Kalibration und markieren die Grenze des Werkzeugs.
