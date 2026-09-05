<!--
  title: Befund — Galileo-resid same-day-Niveau-Assoziation Spec↔Noise (mode / ref_hz / Kadenz) mit Epochen- und Stations-Kontrolle
  class: befund
  date: 2026-09-05
  status: done
  sha256: 71ae214c5b91f4e06d5f0096970bb4efa04e91e0f6a23695d098356f02b755fb
-->

# Befund — Galileo-resid: same-day-Niveau-Assoziation Spec↔Noise (mode / ref_hz / Kadenz) mit Epochen- und Stations-Kontrolle

## Frage

Der gerichtete Tag-Lag-TE (`befund-galileo-te-spec.md`, 2026-09-05) fand das resid auf den
Spec-Achsen entkoppelt/era-koinzident, konnte aber eine rein **gleichzeitige** (same-day)
Niveau-Assoziation Spec↔Noise nicht testen (ref/mode setzen sich tages-weise neu; Tag-Lag-TE
hat keine Kraft für eine persistente Gleichzeitigkeits-Verknüpfung). Diese Messung schließt
den dort registrierten pending: **Ist am selben Tag die Tages-Noisehöhe mit dem Tages-Spec-
Zustand assoziiert — jenseits des Epochen-/Stations-Confounds?**

Das ist kein Gerichtetheits-Test. Es ist ein kontrollierter Niveau-Assoziations-Test: Verringert
das Wissen um den Tages-Spec-Zustand (dominante Mode, Tages-Referenz) die Unsicherheit über die
Tages-Noisehöhe, nach Kontrolle von Epoche und Station?

## Bindung (A = A, 0 honored)

- Spec-Zustand = der **realisierte** Träger im Feld (kein Nominalwert ergänzt): `mode` [3],
  `doppler_ref/10` Hz [5] (die Referenz), Kadenz `sampler_time/100` s [6] — Feld-Layout wie in
  `befund-galileo-te-spec.md`.
- Reinigung (bestehende Konvention): resid endlich, |resid| ≤ 1000 Hz (LOCK), signal_strength ≠ 0.
- Metric-Definition (neu, nicht die std-of-|resid|-Metrik des TE-Befunds — nicht verwechseln):
  - Tages-Noisehöhe über die qualifizierenden Mode-Bins des Tages (alle Moden 1..3, ≥ 30
    gereinigte Samples je Bin), **unabhängig** von der Dominanz-Klassifikation:
    `arms_l` = log10(RMS-Amplitude sqrt(⟨r²⟩)), `med_l` = log10(Median |resid|). Höhe in Dekaden.
  - Spec-Zustand des Tages: dominante Mode (Bin mit den meisten Samples), `ref_med` = Median der
    Tages-Referenz über die gereinigten Samples, Kadenz als frac_non1 (Anteil Nicht-1-s-Samples).
- Kontrolle: Niveau-Block = (Station, Kalendermonat) — Epoche **und** Station zugleich fixiert.
  Schwelle: ≥ 8 Tage je Block, ≥ 4 Tage je Mode-Gruppe für einen Mode-Differenzwert,
  ≥ 4 distinkte Tages-Referenz-Level für einen rho.
- Stärkste Kontrolle (zusätzlich): **within-day** gepaarte Mode-Bin-Noisehöhen desselben Tages
  (beide Moden ≥ 30 Samples am selben Tag) — löscht Epoche, Station **und** Tag.

## Datensatz, Messschritt

Quelle: `data/galileo_resid.bin` (GASR). 14 077 825 Records, gereinigt 12 076 707
(|resid| ≤ 1000 Hz, strength ≠ 0, endlich), lock 1 994 510, zero-strength 6 608, nicht-endlich 0.
Qualifizierende Tag-Zeilen: 457 (≥ 1 Mode-Bin mit ≥ 30 Samples, dominante Mode 1..=3);
0 Tage verworfen (Niveau in log10 degeneriert). Within-day-Mode-Paar-Records: 324.
Gereinigte Kadenz: 12 060 118 bei 1 s, 16 589 andere (frac1s 0.998626); Samples mit Mode außerhalb
1..=3: 0. Probe: `tools/measure/src/bin/galileo_sameday_spec.rs` (additiv; `cargo check`
0 Warnungen). Vollausgabe: /tmp/opencode/galileo_sameday.out.

## Messergebnisse

### 1. Mode → Tages-Noisehöhe je (Station, Monat) — Tag-Ebene

Tabelle aller testbaren d1v2-Blöcke (Differenz der Gruppen-Mediane, Dekaden; n1/n2 = Mode-Tage
je Gruppe; arms = RMS-Amplitude, med = Median|resid|):

| Block (Station Monat) | n1 | n2 | d arms (Dek) | d med (Dek) |
|---|---|---|---|---|
| st14 1995-12 | 17 | 9 | +0.59 | +2.12 |
| st14 1996-11 | 4 | 6 | −1.45 | +0.52 |
| st14 1997-02 | 14 | 6 | −1.43 | +0.10 |
| st43 1995-12 | 12 | 10 | +0.55 | +1.56 |
| st43 1996-01 | 10 | 4 | −1.12 | −1.97 |
| st43 1996-12 | 10 | 4 | −1.53 | −0.38 |
| st63 1995-12 | 14 | 5 | +0.76 | +2.46 |
| st63 1996-11 | 6 | 4 | −2.28 | −2.06 |
| st63 1996-12 | 10 | 4 | −1.97 | −0.80 |
| st63 1997-01 | 23 | 6 | −0.39 | −0.07 |
| st63 1997-02 | 17 | 7 | +1.16 | +0.08 |

Vorzeichen über die Blöcke: arms +4/−7, med +6/−5 (n 11 Blöcke). **Die Differenz wechselt das
Vorzeichen über die Epochen — auch bei fester Station:** st14 + (1995-12) → − (1996-11) → −
(1997-02); st43 + (1995-12) → − (1996-01) → − (1996-12); st63 + (1995-12) → − (1996-11) → −
(1996-12) → − (1997-01) → + (1997-02). Der 1995-12-Cluster ist einheitlich + (Mode-1-Tage lauter),
der 1996-11/12-Cluster einheitlich −, 1997-02 spaltet zwischen den Stationen (st14 −1.43, st63
+1.16). Die Block-Höhen selbst liegen über die Epochen zwei Dekaden auseinander (st63 1996-11:
Mode-1-Tage arms −0.83, Mode-2-Tage +1.45). Die Größenordnung der Differenzen ist real, die
Richtung ist nicht stabil — dieselbe Station, dieselbe Kalender-Epoche stimmen, benachbarte
Epochen kippen.

### 2. Within-day gepaarte Mode-1-vs-Mode-2-Höhe (Epoche + Station + Tag gelöscht)

Dieselben Tage, an denen beide Moden ≥ 30 Samples tragen, verglichen innerhalb des Tages:

| Block (Station Monat) | n Tage | med Dek (med) | Vorzeichen | p |
|---|---|---|---|---|
| st14 1995-11 | 6 | +0.94 | +6/−0 | 0.03 |
| st14 1995-12 | 10 | −0.63 | +1/−9 | 0.02 |
| st14 1996-01 | 3 | −0.26 | +0/−3 | 0.25 |
| st14 1996-11 | 6 | +0.10 | +5/−1 | 0.22 |
| st14 1996-12 | 3 | −0.12 | +1/−2 | 1.00 |
| st14 1997-02 | 7 | −0.17 | +2/−5 | 0.45 |
| st43 1995-11 | 4 | +0.67 | +4/−0 | 0.12 |
| st43 1995-12 | 6 | −0.04 | +1/−5 | 0.22 |
| st43 1996-01 | 3 | −1.50 | +0/−3 | 0.25 |
| st43 1996-11 | 3 | +0.02 | +2/−1 | 1.00 |
| st43 1996-12 | 4 | −0.77 | +1/−3 | 0.62 |
| st63 1994-12 | 1 | +0.19 | +1/−0 | 1.00 |
| st63 1995-11 | 5 | +0.90 | +5/−0 | 0.06 |
| st63 1995-12 | 7 | −0.25 | +2/−5 | 0.45 |
| st63 1996-01 | 9 | −0.93 | +0/−9 | 0.00 |
| st63 1996-11 | 4 | +0.08 | +2/−2 | 1.00 |
| st63 1996-12 | 5 | −0.07 | +2/−3 | 1.00 |
| st63 1997-01 | 14 | −0.02 | +6/−8 | 0.79 |
| st63 1997-02 | 8 | 0.00 | +4/−4 | 1.00 |

Gepoolt über alle 133 Tage (32 Station-Monat-Blöcke): Median-Delta(med) −0.07 Dek, Vorzeichen
+60/−73, p = 0.298 (zweiseitig); Median-Delta(arms) +0.03 Dek, +71/−62, p = 0.488. Kein
Niveau-Unterschied zwischen Mode-1- und Mode-2-Samples desselben Tages.

**Der entscheidende Befund ist die Richtungs-Umkehr gegen die Tag-Ebene:** st14 1995-12 hat auf
der Tag-Ebene d med +2.12 (Mode-1-Tage lauter), aber within-day −0.63 (Mode-1-Samples am selben
Tag leiser, p 0.02). Dasselbe Muster in st43 1995-12 (Tag-Ebene +1.56, within-day −0.04) und
st63 1995-12 (Tag-Ebene +2.46, within-day −0.25). Die große Tag-Ebenen-Differenz der 1995-12-
Blöcke entsteht aus den Tagen selbst (Mode-1-Tage sind andere Tage als Mode-2-Tage) und **wird
innerhalb der gemischten Tage nicht reproduziert** — sie ist eine Tag-/Epochen-Eigenschaft, keine
Mode-Eigenschaft. Die wenigen p < 0.05-Zellen der within-day-Tabelle (st14 1995-11 +, st63 1996-01 −)
kippen zwischen benachbarten Monaten und sind gegeneinander gerichtet.

### 3. ref → Tages-Noisehöhe je (Station, Monat)

Spearman rho(day ref_med, day noise) über 25 Blöcke: rho(arms) Vorzeichen +13/−12,
rho(med) +10/−15. Einzelwerte z. B. st14 1995-12 rho(arms) −0.47 / rho(med) −0.37 (n 26);
st43 1996-12 +0.38/+0.56 (n 16); st63 1996-11 +0.59/+0.39 (n 10); st63 1997-01 +0.05/−0.04
(n 29). Keine stabile Vorzeichen-Richtung über die Epochen — die Referenz ist derselbe
Tages-Proxy der Pass-Konfiguration wie die Mode und zeigt dasselbe Kippen.

### 4. Kadenz → Tages-Noisehöhe

Kadenz-Achse im Feld weiterhin degeneriert (frac1s 0.998626 über die gereinigten Samples).
Ein realisierter Nicht-1-s-Taschen: st12 1990-12 (d7640-7648, 8 Tage, Mode-Tage {2: 7, 3: 1},
8/8 Tage mit Nicht-1-s-Samples, max frac 1.0 = 60-s-Kadenz) — außerhalb der 1994-1997-Fenster
der früheren Befunde. Die Mode-Gruppen (m2 7, m3 1) liegen unter der Schwelle für einen
Niveau-Vergleich; der 60-s-Pfad ist hier gemessen realisiert, aber nicht auswertbar (gemessene
Grenze).

## Befund

**Keine same-day Spec→Noise-Niveau-Assoziation überlebt die Epochen-/Stations-Kontrolle — und
auch nicht die Tag-Kontrolle.**

- Die Mode→Höhen-Differenz der Tag-Ebene existiert als Gleichzeitigkeits-Niveau (bis ~2 Dek),
  **kippt aber über die Kalendermonate, auch bei fester Station** — era-koinzident, kein stabiles
  Mode-Gesetz. Das bestätigt und verschärft das Flip-Muster des Tag-Lag-TE-Befunds unter feinerer
  (Monats-)Kontrolle.
- Wo beide Moden am selben Tag messen (within-day, 133 Tage/32 Station-Monate), ist die gepoolte
  Differenz ≈ 0 (med −0.07 Dek, arms +0.03 Dek; p 0.30/0.49) und kehrt gegen die Tag-Ebenen-
  Richtung derselben Monate um — die Tag-Ebenen-Assoziation ist ein Tag-/Epochen-Confound, keine
  same-day-Mode-Eigenschaft.
- ref zeigt über 25 (Station, Monat)-Blöcke keine stabile Vorzeichen-Richtung (arms +13/−12,
  med +10/−15).
- Kadenz ist im Feld degeneriert; die eine realisierte 60-s-Tasche (st12 1990-12) liegt unter der
  Auswerteschwelle.

Das Niveau des resid hängt am Tag und an der Epoche, nicht am realisierten Spec-Zustand desselben
Tages.

## Register-Satz

Registrierung (schließt den pending des TE-Befunds „same-day-Niveau-Assoziation Spec↔Noise mit
Epochen-/Stations-Kontrolle"): **same-day Spec→Noise-Niveau: kollabiert unter Kontrolle — era-/
tag-koinzident, kein Träger-Gesetz.** Ausstehend (0 honored): Mode-3-Fenster (three-way-Kampagnen)
mit ausreichenden Tagen — Mode 3 bleibt je Monat 0–2 Tage (auch within-day zu selten); die
60-s-Tasche st12 1990-12 (Mode 2 vs 3, n 7/1) als Niveau-Vergleich; ref-rho-Vorzeichen ohne
stabile Richtung (die Streuung selbst ist gemessen, eine Erklärung pending).

## Grenzen

1. Kalendermonat als Epochen-Block: kontrolliert Monats-Ebenen, nicht sub-Monats-Trends; die
   within-day-Tabelle löscht auch diese — dort kollabiert die Assoziation.
2. Mode-Gruppen n klein (4–17 Tage je Block); die Tag-Ebenen-Vorzeichen sind über 11 Blöcke
   verteilt, die within-day-Vorzeichen über 133 Tage gepoolt.
3. arms (RMS-Amplitude) und med (Median|resid|) können im Vorzeichen auseinanderlaufen (st14
   1996-11: arms −1.45, med +0.52); beide Metriken sind ausgegeben, keine liefert eine stabile
   Richtung.
4. Die Metrik ist nicht die std-of-|resid|-Metrik des TE-Befunds — Dekadenwerte sind nicht
   zahlen-gleich vergleichbar, nur die Vorzeichen-Struktur.

## Status

Draft. Messung: `tools/measure/src/bin/galileo_sameday_spec.rs` (additiv; `cargo check`
0 Warnungen), Lauf `cargo run --release -p omegaflow-measure --bin galileo_sameday_spec --
data/galileo_resid.bin` → Vollausgabe in /tmp/opencode/galileo_sameday.out. Keine Repo-Datei
überschrieben oder gelöscht.
