<!--
  title: Befund — Galileo-resid gegen die Galileo-Specs: gerichtete TE auf den Spezifikations-Trägern (ref_hz, mode, Kadenz)
  class: befund
  date: 2026-09-05
  status: done
  sha256: c2d5a405db0e51d1f4059dfaa29549d026980e5912f5a800ca91431e94696b8b
-->

# Befund — ist das Galileo-resid spec-getrieben? (gerichtete TE, ref_hz / mode / Kadenz)

## Frage & Bindung

Frage (der Auftrag "gegen die Galileo-Specs"): Ist das Galileo-Doppler-Residuum
**gerichtet gekoppelt an einen spec-getriebenen Prozess** — an den in den
resid-Datensätzen realisierten Spec-Trägern `ref_hz`, `mode`, `sampler_s` — oder
ist es von der Spec entkoppelt / rein era-koinzident?

Bindung (A = A, 0 honored): Die Spec ist kein Text; sie ist das realisierte
Referenz/Signal-Feld in den GASR-Datensätzen **plus** die extern verifizierten
Formaterwartungen. Der SPEC-SOLL wird also direkt aus den gemessenen Feldern
gebaut (kein Zahlenwert aus dem Gedächtnis — 0 honored: nominale Frequenzpläne
sind in dieser Messung eine Lesaufgabe, keine Voraussetzung; die realisierten
Trägerwerte liegen in den Feldern). Jedes S-Series ist auf ein verifiziertes
Spezifikationsdokument zurückgeführt (unten). Der Befund spricht nur über die
gerichtete Kopplung gegen Surrogat-Nullen, mit n für jede TE.

## Datensatz, Reinigung, Messschritt

Quelle: `data/galileo_resid.bin` (GASR, `omegaflow::atdf::parse_resid_bin`,
14 077 825 Records, TDB-Tage 7637..9920). Feld-Layout wie in
`src/archivar/atdf.rs` `reduce_resid` (gemessen): [0]=tdb, [1]=resid_hz,
[2]=station, [3]=ground_mode (1/2/3), [4]=data_type, [5]=doppler_ref/10 Hz
(die Referenz), [6]=sampler_time/100 s, [7]=signal_strength.

Reinigung (Konvention des bestehenden Probes `galileo_te_floor_direction.rs`):
resid endlich, |resid| ≤ 1000 Hz (LOCK-Schnitt, die >1-kHz-Samples sind
Cycle-Slips/unlocked), signal_strength ≠ 0. Gemessen: 12 076 707 gereinigt
(85,8 %), 1 994 510 lock, 6 608 zero-strength, 0 non-finite.

Binning: pro (Tag, Station) wird der dominant-Mode-Tag (Modus mit den meisten
gereinigten Samples des Tages) als eine Serie gebildet; Noise = rms bzw. median
|resid| **nur der dominant-Mode-Samples** des Tages; Driver ebenso aus demselben
Sample-Satz (Tagesserie über zusammenhängende Lauftage; Lücken ≤ 0 innerhalb
eines Laufs). Kovariate era = Kalendermonat-Index (condition). Station fest je
Lauf (station ist Filter, nie Driver). Verwendete Läufe (gemessen, ≥ 14
aufeinanderfolgende Tage, ≥ 30 Samples/Tag):

| Station | Lauf (Tage) | n Tage | Moden (Tage) | eraLv |
|---|---|---|---|---|
| 14 | 9457–9478 | 22 | 1:10 2:11 3:1 | 2 |
| 14 | 9889–9903 | 15 | 1:12 2:3 | 2 |
| 43 | 9457–9482 | 26 | 1:14 2:12 | 2 |
| 43 | 9492–9509 | 18 | 1:11 2:7 | 2 |
| 43 | 9847–9879 | 33 | 1:27 2:4 3:2 | 2 |
| 63 | 9457–9482 | 26 | 1:12 2:11 3:3 | 2 |
| 63 | 9497–9510 | 14 | 1:11 2:3 | 1 |
| 63 | 9858–9877 | 20 | 1:15 2:5 | 2 |
| 63 | 9883–9908 | 26 | 1:21 2:5 | 2 |

Methode (omegaflow-TE-Gates, `te.rs`): `transfer_entropy_lag` +
phasenrandomisierte (`surrogate_stats_phase`) und Block-Bootstrap-Null
(`surrogate_stats_block`, Block 5); era-bedingte TE
(`transfer_entropy_conditional`) mit Residuen-Surrogat-Null
(`conditional_te_stats`, N_SURR = 20). Lags 1,2,3,5. Beide Richtungen. TE > mean+2σ
= signifikant. n für jede TE. Konvention: `TE(target, driver, lag)`;
Driver->Target (D->T) = die getestete Richtung; T->D = Reverse.

## Spec-Quelle jeder S-Series

- S1 — `ref_hz` [5]: `doppler_ref/10` (atdf.rs:652), die Doppler-Referenz des
  Tracking-Records. Spec-Grund: TRK-2-25 (DSN Archival Tracking Data File SIS,
  820-13, 1986-01-21), im Band selbst archiviert und verifiziert-offen:
  `pds-ppi.igpp.ucla.edu/annex/GO-J-RSS-1-TDF-V1.0/DOCUMENT/TRK_2_25.TXT`
  (Recherche-Order §1, HTTP 200). Wert aus dem Feld gelesen (realisierte Spec),
  kein nominaler Zahlenwert ergänzt.
- S2 — `mode` [3]: `ground_mode` (atdf.rs:650 push / :257 field / :279 extract),
  1/2/3 = one/two/three-way (Link-Struktur). Spec-Grund: TRK-2-25 Mode-Feld;
  Verifikation wie S1.
- S3 — `sampler_s` [6]: `sampler_time/100` s (atdf.rs:627), die Zähl-/Reduktions-
  Kadenz. Spec-Grund: TRK-2-25 Count-Time-Feld; Verifikation wie S1.
- Reinigungsschwelle 1000 Hz und strength≠0: bestehende Mess-Konvention im Repo
  (`galileo_te_floor_direction.rs`), nicht erfunden.

## Messergebnisse

### Vorbefund 1 — die gemessene Struktur der Träger

- **Kadenz-Achse ist im Feld nicht realisiert.** Von 12 076 707 gereinigten
  Records liegen 12 060 056 bei 1 s (frac 0.998666), 16 111 bei anderer Kadenz
  (60 s global nur 18 674 von 14 M). Mode 2 (two-way) ist zu **0.9959** bei 1 s
  und nur zu 0.0037 bei 60 s — die Annahme "60 s in two-way" ist im realisierten
  resid-Bestand nicht die gemessene Kadenz. Nur 72 Tag-Station-Bins enthalten
  überhaupt ein Nicht-1-s-Sample.
- **ref_hz und mode sind Tages-Proxy derselben Pass-Konfiguration.** ref hat
  pro Lauf 7–21 verschiedene Tageswerte über 14–33 Tage (Fast-Neustart pro Tag);
  die Tages-Mode-Serie ist mäßig persistent (adjacente gleiche Moden 11/19 …
  27/32). S0-Kontroll-TE `mode->ref` (Tag->Folgetag) ist **null** in allen 9
  Läufen (z. B. st43 9847–9879 n33 D->T lag1 TE 1.86e-2, thrPh 3.05e-1) —
  Mode und ref bewegen sich gleichzeitig, nicht mit Folgetag-Gedächtnis.
- **Noise-Floor variiert era/station-dominiert.** median log10(rms) der
  Mode-1-Tage über die Läufe: −1.13 … +0.78 (st14 9889 bzw. st63 9497), d. h.
  zwei Größenordnungen innerhalb derselben Mode. Moden-Differenz wechselt das
  Vorzeichen zwischen Läufen (st14 9457: m1−m2 = +1.6 Dekaden; st63 9457:
  −0.8; st63 9858: −1.5; st63 9883: −0.8) — kein stabiles Mode-Gesetz, era-
  koinzident. (Kontext, kein TE-Urteil.)

### S1 — ref_hz → Noise (und reverse), cond era

Ergebnis über **26 Vorwärts-Tabellen** (18 Basis- + 8 mode-1-kontrollierte; 2 Metriken,
rms/median, über die Läufe): in den testbaren S1-Zellen liegt die Vorwärts-TE **unter der
Phasen-Null**, und nur **1 isolierte Zelle** überlebt die era-bedingte Null.

| S1 ref->noise | n | lag | TE | thrPh | thrBl | cTE|era | cThr | Befund |
|---|---|---|---|---|---|---|---|---|
| st43 9847–9879 (dom) | 33 | 1 | 8.25e-2 | 2.45e-1 | 1.30e-1 | 5.42e-2 | 1.90e-1 | null |
| st43 9847–9879, mode1-kontr. | 27 | 1 | 5.68e-2 | 1.70e-1 | 1.29e-1 | 3.42e-2 | 1.32e-1 | null |
| st43 9847–9879, mode1-kontr. | 27 | 3 | 2.11e-1 | 2.58e-1 | 1.59e-1 | 1.41e-1 | 1.96e-1 | **collapses unter era** (block-allein oben) |
| st63 9883–9908, mode1-kontr. | 21 | 3 (med) | 1.43e-1 | 3.32e-1 | 1.34e-1 | 1.35e-1 | 1.28e-1 | 1 isolierte Zelle über cThr (Margin 0.007), unter thrPh, nicht replizierend |

Einzige era-bedingt signifikante Zelle der ganzen Batterie (st63 9883–9908
mode1 med lag3): TE = 1.43e-1 unter der Phasen-Null 3.32e-1, knapp über der
Block-Null und über cThr um 0.007 nats. Sie repliziert weder in der rms-Metrik
desselben Laufs noch in irgendeinem anderen Lauf → als Zufallestreffer bei
n=21 behandelt, kein Kopplungsnachweis.

Reverse (noise->ref): era-bedingt über Null in mehreren Läufen (st43 9847–9879
lag2 cTE 2.08e-1 > 1.11e-1; st63 9858–9877 lag1 cTE 2.02e-1 > 1.90e-1) —
physikalisch unmögliche Richtung (Residuum treibt die Referenz nicht); sie ist
häufiger als die Vorwärtskreuzung (ca. 14 vs 2 Zellen) und zeigt, dass die
Residuen-Surrogat-Null bei n ≈ 15–33 breit ist. Kein Vorwärts-Nachweis.

**S1-Urteil: null / kollabiert unter Konditionierung — das resid folgt der
Referenz nicht gerichtet; die wenigen block-null-Kreuzungen sind era-koinzident.**

### S2 — mode → Noise (und reverse), cond era

| S2 mode->noise | n | lag | TE | thrPh | thrBl | cTE|era | cThr | Befund |
|---|---|---|---|---|---|---|---|---|
| st43 9457–9482 (m1/m2) | 26 | 1 | 3.50e-2 | 9.52e-2 | 5.47e-2 | 3.84e-2 | 6.51e-2 | null |
| st43 9492–9509 | 18 | 1 | 5.98e-2 | 2.33e-1 | 1.13e-1 | 7.71e-2 | 1.96e-1 | null |
| st63 9858–9877 | 20 | 1 | 1.34e-1 | 2.39e-1 | 2.03e-1 | 1.63e-1 | 2.36e-1 | null |
| st63 9883–9908 | 26 | 1 | 1.09e-1 | 3.14e-1 | 2.05e-1 | 1.19e-1 | 1.93e-1 | null |

6 Vorwärts-Tabellen (S2): über der Phasen-Null und der era-bedingten Null **keine**
testbare Vorwärts-Zelle signifikant. ref und mode sind tages-weise redundant (beide
setzen sich pro Tag teil-neu zurück; die S0-Kontroll-TE mode->ref mit Folgetag ist
null — Kollinearität als Schluss aus Same-Day-Reset + Null-Lag-TE, **nicht** als
gemessene mode↔ref-Korrespondenz; der trennende Mode-3-Teil (three-way) ist
ungemessen und `pending`). Mode-3 ist in diesen Fenstern zu selten (1–3 Tage/Lauf),
um zu urteilen (gemessene Grenze). Die Noise-Differenz m1 vs m2 existiert als
Gleichzeitigkeits-Niveau, wechselt aber das Vorzeichen zwischen den Epochen
(siehe Vorbefund) — kein stabiles Link-Gesetz.

**S2-Urteil: null — die Noise-Decke ist nicht link-konfigurationsgetrieben auf
dieser Achse; die Niveau-Unterschiede sind era-koinzident.**

### S3 — Kadenz/Clock → Noise

Vorbefund: Kadenz-Feld ist nahezu konstant 1 s (frac 0.998666); die
Tages-Kadenz-State-Serie hat drvVar ≈ 1e-6 … 1e-9 bei drvLvl 1 → die Achse ist
**im Feld degeneriert**. Die zwei S3-Tabellen liefen auf einem effektiv
konstanten Driver; die einzige "cThr-Kreuzung" (st63 9883–9908 cad lag3,
cTE 9.30e-2 > 6.04e-2) sitzt auf drvVar = 1.2e-9 — nicht interpretierbar als
gerichtete Kopplung. Der 60-s-Grid/Comb kann in diesem Bestand keine
realisierte Kadenz-Serie treiben, weil die realisierte Kadenz 1 s ist; die
"60-s in two-way"-Formel ist im resid-Bin nicht realisiert (mode 2: 99.6 % bei
1 s). Eine echte Clock-Getriebenheit (falls vorhanden) ist mit dem Feld nicht
messbar — die Kadenz ist nicht die gemessene Wahrheit der Datensätze.

**S3-Urteil: Achse degeneriert — keine gerichtete Kopplung messbar (0 honored:
nicht "kein Effekt", sondern "der Träger variiert nicht im Feld").**

## Befund

Die Galileo-resid-Nachricht ist auf diesen drei Spec-Achsen **nicht gerichtet
spec-getrieben**. Über die auswertbaren Tabellen (S1 26 + S2 8 + S3 3 = 37
Vorwärts-Tabellen; n = 14–33 Tage je Lauf):

- über der **Phasen-Null**: in keiner testbaren Vorwärts-Zelle der S1/S2-Achsen;
  die S3-Kadenz-Achse ist degeneriert und kein Kopplungstest (0 honored);
- über der **era-bedingten Null**: 1 isolierte, nicht replizierende S1-Zelle
  (Margin 0.007 nats, n = 21); keine S2-Zelle;
- die block-null-Kreuzungen **kollabieren unter era-Konditionierung** —
  era-koinzident, nicht Spec-getrieben;
- Reverse (noise->ref/mode) kreuzt häufiger als Vorwärts → physikalisch
  unmögliche Richtung, breite Null bei kurzem n; kein Vorwärts-Nachweis.

Verdikt der Frage: Das resid ist auf diesen Achsen **entkoppelt/era-koinzident**
— nicht referenz-getrieben, nicht mode-getrieben, nicht kadenz-getrieben (die
Kadenz-Achse ist im realisierten Feld nicht vorhanden). Das "gegen die
Galileo-Specs"-Bild hält nicht: kein gerichteter Pfad vom realisierten
Spec-Träger in die Rauschhöhe, der über die Epoche hinaus überlebt.

## Grenzen

1. **Tages-Lag-Design**: Die Richtungs-Kopplung wird über Tag-zu-Tag-Übergänge
   gemessen. ref/mode setzen sich pro Tag (teil-)neu (S0 mode->ref null), daher
   hat das Design nur Kraft für *persistente* Kopplungen; eine rein
   gleichzeitige (same-day) Niveau-Assoziation Spec↔Noise liegt außerhalb der
   gerichteten TE und ist nur als Kontext gemessen (vorzeichenwechselnd über
   Epochen).
2. **n klein** (14–33 Tage/Lauf): die Residuen-Surrogat-Null ist breit
   (Reverse-Kreuzungen ~14×). Konservative Lesart; ein schwacher echter Effekt
   unterhalb der Null ist nicht ausgeschlossen, nur nicht gemessen.
3. **Mode 3 (three-way)** ist in den analysierten Läufen zu selten (1–3 Tage)
   für ein Urteil — gemessene Grenze.
4. Kadenz: der comb-artige Grid (60-s-Alias) kann mit dem Feld nicht getestet
   werden, weil die realisierte Kadenz 1 s ist; Sub-Tag-Spektralstruktur des
   resid ist hier kein TE-Gegenstand.
5. Nominale Frequenzpläne (S/X-Band-Werte) wurden nicht benötigt und nicht
   ergänzt; alle Trägerwerte sind aus den Feldern gelesen.

## Register-Satz

Register (`docs/TODO.md`, Galileo-Noise-Familie ~137–170): **Spec-Träger
ref_hz/mode/Kadenz → resid-Noise: entkoppelt/era-koinzident (null)** — kein
gerichteter Pfad vom realisierten Spec-Träger in die Rauschhöhe, der über die
Epoche hinaus überlebt. Das schließt die offenen Posten der
`befund-galileo-mode1-snr-kurve`/`rausch-kurve`-Familie **nicht** — es erweitert
das pending um drei (same-day-Niveau-Assoziation, Mode-3-Fenster, S1-Isolat-
Replikation) und entfernt „spec-getrieben" als lebende Hypothese für diese drei
Träger. Ausstehend (0 honored, nicht als Befund ausgegeben): same-day-Niveau-
Assoziation Spec↔Noise mit Epochen-/Stations-Kontrolle als eigenes Verfahren;
Mode-3-Fenster (three-way-Kampagnen) mit ausreichenden Tagen; die S1-Isolat-Zelle
(st63 9883–9908 mode1 med lag3) als Replikationsauftrag an andere Seeds/Fenster.

## Status

`done` (Rat gehalten, 2026-09-05). Messung: `tools/measure/src/bin/galileo_spec_te.rs`
(neu, additiv; `cargo check` 0 Warnungen), Lauf `cargo run --release -p omegaflow-measure --
bin galileo_spec_te` → Vollausgabe in /tmp/opencode/galileo_spec_te.out.
Bash-Scratch (Parsing-Erkundung) in /tmp/opencode/. Keine Repo-Datei
überschrieben oder gelöscht.

Folge-Befund: die Same-Day-Niveau-Assoziation ist geschlossen
(`befund-galileo-sameday-spec-assoziation`, done — null, within-day-Paarung
kollabiert); die S1-Isolat-Zelle ist gemessen nicht-replizierend
(`befund-galileo-mode3-und-s1-replikation`, done — 6/10 Seeds, unter der
600-Surrogat-Null, verschwindet über verschobene Fenster); Mode-3-Fenster sind
gemessen eine Daten-Dünn-Grenze (0 Fenster ≥10 Tage, 17 isolierte Einzeltage).
