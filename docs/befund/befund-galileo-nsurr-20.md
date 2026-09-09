<!--
  title: Befund — Galileo N_SURR 20-vs-10: 10 bleibt das gemessene Blatt (die Phasen-/Block-Schwellen halten ihre Schlüsse unter 20)
  class: befund
  date: 2026-09-09
  sha256: b1518f50e0bc325416ac739d7778cd600c7d8842d109cd7abe4dffb6779da6e3
  status: done
  antwortet-auf: docs/auftrag/archiv/auftrag-galileo-nsurr-20.md
  see-also: docs/handover/archiv/handover-2026-09-09-te-galileo-nsurr.md docs/handover/archiv/handover-2026-09-09-te-atom-4-folge.md
-->
# Befund — Galileo N_SURR 20-vs-10: 10 bleibt das gemessene Blatt

## Frage & Bindung

Frage (der Auftrag `auftrag-galileo-nsurr-20.md`): Halten die Phasen-/Block-
Schwellen unter N_SURR=20, was unter N_SURR=10 gemessen wurde — oder bleibt 10
das gemessene Blatt? Das Papier dachte 20 für die Phasen-/Block-Null der
Galileo-TE-Blätter; der Baum trug 10. Der Ist-Zustand (10) blieb, bis diese
Erhebung ihn gegen 20 prüft. Kein stiller Shift.

Bindung (A = A): Die fünf Proben tragen die additive `--n-surr`-Parameterisierung
(`surrogate_stats_phase_n` / `surrogate_stats_block_n` in
`src/mathematikerin/te.rs`); die kanonische 10-Null bleibt byte-identisch
(`phase_block_null_stays_byte_identical_at_ten` grün). Die era-bedingte
Residual-Null (Spalte cThr) trägt in allen Proben `N_SURR = 20` — sie hängt
nicht an `--n-surr`; ihr Vergleich 10-vs-20 ist konstruktionsgleich. Der Befund
spricht nur über das gemessene Sheet.

## Das Sheet

CI `galileo-nsurr-20.yml`, 10 Punkte (5 Proben × `--n-surr` 10/20), Release,
gleicher Seed 0x9E3779B97F4A7C15, Commit-SHA `f546019`. `galileo_resid.bin`
(14 077 825 Records, 12 076 707 gereinigt) + `ephemeris_earth.bin` +
`ephemeris_galileo_daily.bin` vom CDN. Spalten je Zelle: TE, thrPh (Phasen-Null,
mean+2σ), thrBl (Block-Null, Block 5, mean+2σ), cTE|era, cThr (Residual-Null,
N_SURR=20). Marken `Ph*`/`Bl*`/`cT*` = TE über der jeweiligen Schwelle. Drei
Null-Familien: Phasen, Block, Residual — nie auf zwei gekürzt.

## Die fünf Proben unter 10 vs 20

### direction (`galileo_te_floor_direction`) — die eine Probe mit Marken

Richtung: S->N = Stärke→Noise (Signal-Stärke als Driver, |resid|-Noise als
Ziel), N->S = die Umkehr. Zwei Insel-Fenster (1995-11-22..1996-01-14,
1996-12-16..1997-02-14), je pooled/st14/st43/st63, je median|r| und rms(r).

Markenzählung: n=10 → 20 Zellen; n=20 → 19 Zellen. Die Differenz ist kein
Netto-Effekt — sechs Zellen ändern sich, drei verlieren, eine gewinnt, drei
wechseln die Null-Familie:

| Zelle | n=10 | n=20 | Art |
|---|---|---|---|
| N->S lag1 st14 1996 rms | Bl* | — | verliert Block (thrBl 3.85e-2 → 8.78e-2) |
| N->S lag3 st63 1996 rms | Bl* | — | verliert Block (thrBl 6.47e-2 → 9.89e-2) |
| S->N lag1 st43 1996 median | — | Bl* | gewinnt Block (thrBl 3.90e-2 → 3.13e-2) |
| N->S lag1 st14 1995 rms | Bl* | Ph* | Null-Familie (thrBl 4.51e-2 → 1.01e-1, thrPh 9.36e-2 → 8.87e-2) |
| S->N lag1 st43 1996 rms | Bl* | Ph* Bl* | gewinnt Phase (thrPh 2.09e-1 → 1.86e-1) |
| N->S lag5 st43 1996 rms | cT* | Bl* cT* | gewinnt Block (thrBl 1.37e-1 → 1.26e-1) |

Die Schlagzeile hält — und wird unter 20 schärfer: **S->N** kreuzt die
Block-Null im 1996-Fenster (pooled lag1–3, st43 rms lag1–3, st63 lag1–2) unter
beiden Zählungen, und gewinnt eine Zelle (st43 1996 median lag1) hinzu. **N->S**
(Umkehr) verliert zwei Block-Kreuzungen (st14 1996 rms lag1, st63 1996 rms
lag3) — die Umkehr wird unter 20 nuller. Kein S->N-Kreuz der Schlagzeile fällt
weg.

### spec / external / stair / mode3s1 — null unter 10 und unter 20

- **spec** (`galileo_spec_te`): keine Marke in einer Tabelle (S1/S2/S3,
  S0-Kontrolle) bei n=10 und n=20. Deckungsgleich mit
  `befund-galileo-te-spec.md` (entkoppelt/era-koinzident).
- **external** (`galileo_floor_external_te`): keine Marke — die externen
  Driver (Elongation, step, era) koppeln nicht gerichtet in die Floor-Noise;
  fwd TE liegt überall unter thrPh/thrBl, cTE unter cThr. Identisch bei 10 und 20.
- **stair** (`galileo_floor_stair_te`): keine Marke bei 10 und 20.
- **mode3s1** (`galileo_mode3_s1_repl`): keine Marke bei 10 und 20.

## Der Verdikt-Satz

Die Phasen-/Block-Schwellen sind mean+2σ über 10 bzw. 20 Surrogaten; ihre
Schätzung bewegt sich zwischen den Zählungen innerhalb der eigenen
Kleinstichproben-Streuung (die Block-Schwelle einer Rand-Zelle mehr als
verdoppelt sich, eine andere halbiert sich). Sechs Rand-Zellen ändern dadurch
ihre Zugehörigkeit — aber keine Schlagzeile kippt: der Richtungsbefund
(Stärke→Noise über der Block-Null im 1996-Fenster) hält und wird unter 20
schärfer (die Umkehr verliert zwei Kreuzungen, die Vorwärtsrichtung gewinnt
eine); spec, external, stair und mode3s1 bleiben unter beiden Zählungen null.
Ein gemessener Vorteil der 20-Zählung materialisiert nicht — kein FPR-Vorsprung,
kein Schwellen-Umzug. **10 bleibt das gemessene Blatt.**

## Nebenfund (gemessen, nicht vorhergesehen)

Die externe Probe schrieb ihren Bericht zuerst nur in eine Datei
(`tmp/galileo_floor_external_te.txt`), nicht nach stdout — das erste CI-Sheet
trug nur die Zeile „report written", der externe 10-vs-20-Vergleich war nicht
gemessen (0 honored: `pending`, nicht null). Die Probe druckt den Bericht jetzt
zusätzlich nach stdout (Commit `f546019`), das Sheet ist neu gezogen und trägt
den externen Vergleich vollständig.

## Register-Satz

Register: **Galileo N_SURR 20-vs-10 — 10 bleibt das gemessene Blatt** (kein
Umzug). Die Phasen-/Block-Schwellen halten ihre Schlüsse unter 20, wo unter 10
gemessen wurde; sechs Rand-Zellen ändern die Zugehörigkeit (zwei Umkehr-Verluste,
ein Vorwärts-Gewinn, drei Null-Familien-Wechsel), keine Schlagzeile kippt. Der
Auftrag `auftrag-galileo-nsurr-20.md` ist beantwortet.

## Status

`done`. Messung: CI `galileo-nsurr-20.yml` (10 Punkte, Runs 34400114106 +
34402236206), Sheet `galileo-nsurr-20-sheet.txt` (SHA f546019). Offen an die
nächste Sitzung (im Handover): T-Skalierungs-Blatt (Run 34401566532) und
Bz/LAIC n_surr=100 (Run 34401571351) — in Flug bei Sitzungsende.
