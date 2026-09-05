<!--
  title: Befund — Richtungs-TE Stärke → Mode-1-Rauschen: der AGC-Boden ist auf Tages-Achse nicht gerichtet treibend (Epochen-/Stations-kollokiert)
  class: befund
  date: 2026-09-05
  sha256: b85085740a97cc26b68a28e60a44371929464f9f61b07d80e4b0f319ecfeea9d
  status: done
  antwortet-auf: docs/befund/befund-galileo-mode1-snr-kurve.md docs/befund/befund-galileo-pass-segmentierung.md
-->
# Befund: Richtungs-TE Stärke → Mode-1-Rauschen — der AGC-Boden ist auf Tages-Achse nicht gerichtet treibend (Epochen-/Stations-kollokiert)

## Frage & Bindung

Trägt `signal_strength` das Mode-1-Rest-Rauschen *gerichtet* über die Epochen-/Stations-Konfundierung
hinaus — ist der AGC-Boden (−2560) ein echter SNR-getriebener (PLL-)Term oder ein Epochen-/Stations-
Kollokations-Artefakt? Gemessen mit dem omegaflow-Richtungsinstrument `transfer_entropy_lag` /
`transfer_entropy_conditional` (KDE-TE, Silverman). Surrogat-Null: mean+2σ über 10 phasenrandomisierte
(`surrogate_stats_phase`, kanonische Null; `Ph*`) und 10 Block-Bootstrap-Surrogate
(`surrogate_stats_block`, Block 5 Tage, `Bl*`); konditionale Null via `conditional_te_stats`
(Residual-Surrogat, 20, `cT*` = cTE > cThr). Argument-Konvention wie `te_pair_probe`: `TE_lag(Ziel,
Treiber)`; Vorwärts S→N = `TE_lag(rauschen, stärke)`, die Surrogat-Null mischt den Treiber.

Gebunden: nur Mode 1; Lock-Übergänge (|resid_hz| > 1000) ausgeschlossen; Stärke 0 (Pad, 4 204 Proben)
getrennt. Treiber X = Tages-Median `signal_strength` [7]; Ziel Y = Tages-Median |`resid_hz`| desselben
Tages (gleiche Tage, zeitgeordnet), Tages-Zellen ≥ 30 Proben. Als Empfindlichkeit zusätzlich Y =
Tages-RMS des Residuums (die Rausch-Metrik der Vorgänger-Blätter). Epochen-Kovariate = Kalendermonat
(`year*12+month`); Station dadurch, dass je Station eine eigene Reihe gebaut wird (Stations-Identität
konstant); die gepoolte Reihe mischt Stationen im Tag — selbst ein Artefakt-Kanal. Zwei durchgängige
Tages-Inseln: 1995-11-22..1996-01-14 (ruhiger Boden 1995-11 + lauter Boden 1995-12 st43/63) und
1996-12-16..1997-02-14 (die laute Boden-Epoche). Datenkette `data/galileo_resid.bin` (GASR,
`omegaflow::atdf::parse_resid_bin` → Vec<[f64;8]>). Neue additive Sonde
`tools/measure/src/bin/galileo_te_floor_direction.rs`; `cargo check` 0/0 Warnings; Report auf stdout.

## n zuerst (0 geehrt)

Mode 1: 9 743 574 Proben, 1 568 246 Lock-Übergänge, 8 171 124 nach Lock- und s=0-Ausschluss.

| Insel | Reihe | n Tage | max Tageslücke | med S | med \|r\| Hz |
|---|---|---|---|---|---|
| 1995-11-22..1996-01-14 | gepoolt | 52 | 2 d | −1757 | 0,739 |
| 1995-11-22..1996-01-14 | st14 | 44 | 4 d | −1753 | 0,713 |
| 1995-11-22..1996-01-14 | st43 | 31 | 10 d | −1754 | 0,833 |
| 1995-11-22..1996-01-14 | st63 | 40 | 10 d | −1761 | 0,603 |
| 1996-12-16..1997-02-14 | gepoolt | 60 | 1 d | −1752 | 0,280 |
| 1996-12-16..1997-02-14 | st14 | 47 | 4 d | −1749 | 0,271 |
| 1996-12-16..1997-02-14 | st43 | 53 | 3 d | −1750 | 0,281 |
| 1996-12-16..1997-02-14 | st63 | 55 | 2 d | −1757 | 0,293 |

n ≥ 30 in allen Reihen — die Tages-Achse trägt den Test. 64 Vorwärts- und 64 Rückwärts-Zellen
(2 Inseln × 4 Reihen × 2 Metriken × 4 lags).

## Befund — TE vs Surrogat-Null

Nur die signifikanten oder entscheidenden Zellen; volle Tabelle im Probe-Stdout. Legende:
`Ph*` = TE > phasenrandomisierte Schwelle; `Bl*` = TE > Block-Schwelle (5 d); `cT*` = cTE > cThr
(konditionale Schwelle, Epoche = Kalendermonat).

### Insel 1996-12-16..1997-02-14 (die laute Boden-Epoche)

| Reihe | Metrik | lag | TE(S→N) | thrPh | thrBl | cTE(S→N\|Monat) | cThr | Mark |
|---|---|---|---|---|---|---|---|---|
| gepoolt | rms | 1 | 9,56e-2 | 1,56e-1 | 7,89e-2 | 8,19e-2 | 1,47e-1 | Bl* |
| gepoolt | rms | 2 | 8,21e-2 | 1,40e-1 | 8,14e-2 | 6,90e-2 | 1,40e-1 | Bl* |
| gepoolt | rms | 3 | 8,86e-2 | 1,32e-1 | 8,28e-2 | 6,69e-2 | 1,59e-1 | Bl* |
| st43 | median\|r\| | 2 | 7,40e-2 | 1,50e-1 | 6,70e-2 | 5,48e-2 | 1,01e-1 | Bl* |
| st43 | median\|r\| | 3 | 4,02e-2 | 8,37e-2 | 3,93e-2 | 4,36e-2 | 7,51e-2 | Bl* |
| st43 | rms | 1 | 1,97e-1 | 2,09e-1 | 9,00e-2 | 1,72e-1 | 1,97e-1 | Bl* |
| st43 | rms | 2 | 1,24e-1 | 2,33e-1 | 1,12e-1 | 7,41e-2 | 1,91e-1 | Bl* |
| st43 | rms | 3 | 1,22e-1 | 2,36e-1 | 1,05e-1 | 6,27e-2 | 1,95e-1 | Bl* |
| st63 | median\|r\| | 2 | 6,11e-2 | 1,19e-1 | 5,70e-2 | 3,28e-2 | 5,44e-2 | Bl* |
| st63 | median\|r\| | 3 | 3,75e-2 | 7,87e-2 | 3,46e-2 | 5,96e-2 | 5,55e-2 | Bl* cT* |
| st63 | median\|r\| | 5 | 3,18e-3 | 5,00e-2 | 1,09e-2 | 4,24e-2 | 2,58e-2 | cT* |
| st63 | rms | 1 | 1,10e-1 | 1,64e-1 | 8,93e-2 | 7,52e-2 | 1,74e-1 | Bl* |
| st63 | rms | 2 | 9,57e-2 | 1,53e-1 | 8,21e-2 | 1,13e-1 | 1,66e-1 | Bl* |

Rückwärts (N→S) in dieser Insel: nur st43 rms lag 5 (cTE 1,19e-1 > cThr 1,11e-1, cT*) und
st14 rms lag 1 (Bl*); st63 rms lag 3 N→S Bl*.

### Insel 1995-11-22..1996-01-14 (ruhiger + gemischter Boden)

Keine Vorwärts-Zelle erreicht irgendeine Schwelle (auch die Block-Schwelle nicht) — dort trägt selbst
die unkonditionierte Tages-Achse keinen S→N-Pfeil. Rückwärts: gepoolt rms lag 1 (cT*), gepoolt rms
lag 2 (Bl* cT*), st63 rms lag 2 (cT*), st14 rms lag 1 (Bl*).

## Befund-Satz

1. **Vorwärts überschreitet nie die kanonische phasenrandomisierte Null** — keine Station, keine
   Metrik, kein lag, keine Insel. Die phasenrandomisierte Null erhält das Niederfrequenz-Spektrum des
   Treibers und ist damit die konservative Messung der Epochen-Konfundierung; ihr Fehlen sagt: kein
   gesicherter unkonditionierter gerichteter Pfeil.

2. Die Block-Null (5 d) wird nur in der lauten Boden-Epoche überschritten — gepoolt rms lag 1–3,
   st43 median lag 2–3, st43 rms lag 1–3, st63 median lag 2, st63 rms lag 1–2. Das ist das Tages-Bild
   des „Boden lauter": die Zellen, in denen Stärke und Rauschen tages-weise mitlaufen. **Jede dieser
   Überschreitungen kollabiert unter der Epochen-Konditionierung**: cTE(S→N | Monat) bleibt in diesen
   Zellen unter cThr (z. B. st43 rms lag 1: cTE 1,72e-1 < cThr 1,97e-1; gepoolt rms lag 1: cTE 8,19e-2
   < cThr 1,47e-1).

3. Über das ganze Raster (64 Vorwärts-Zellen) überleben nur zwei Vorwärts-Zellen die Epochen-
   Konditionierung — st63 median lag 3 (5,96e-2 > 5,55e-2) und lag 5 (4,24e-2 > 2,58e-2). lag 2
   derselben Reihe ist nicht konditional signifikant (3,28e-2 < 5,44e-2), lag 1 auch nicht; die RMS-
   Metrik derselben Reihe überlebt nicht. Beide Überlebenden teilen Station 63 und die Metrik
   median|r|. Bei ~64 Zellen und mean+2σ (nomineller Gauß-Schwanz, 10/20 Surrogate) sind ~1,5
   Zufalls-Überschreitungen zu erwarten — zwei isolierte, lag-diskontinuierliche Zellen ohne
   Nachbar-/Metrik-Bestätigung sind kein Befund.

4. Die Richtungsumkehr (N→S) überlebt die Konditionierung in 4 Zellen (gepoolt rms lag 1–2 **Insel A**,
   st63 rms lag 2 **Insel A**, st43 rms lag 5 **Insel B**) — mehr als die Vorwärts-Richtung. **Lesart**
   (nicht als gemessene Asymmetrie, 4 vs 2 liegt in der Zufallserwartung): ein gemeinsamer Pass-/
   Stations-/Epochen-Wechsel als Treiber, nicht ein SNR→PLL-Pfeil.

**Verdikt: Null — Epochen-/Stations-Kollokation.** Ein gerichteter SNR-getriebener Boden-Term ist im
Mode-1-Residuum auf Tages-Achse nicht messbar: TE(stärke→rauschen) überlebt die Konditionierung auf
Monats-Epoche nicht (62 von 64 Vorwärts-Zellen unter cThr; die zwei Ausnahmen sind lag-verstreut,
ohne Metrik-/Nachbar-Bestätigung, innerhalb der Zufallserwartung). Die Block-Überschreitungen der
lauten Epoche stammen aus der Kollokation von Boden-Tagen und lauten Tagen in denselben Monaten —
nicht aus einer vom Stärke-Feld gerichtet verursachten Rausch-Anhebung. Der AGC-Klemmwert −2560
bleibt ein echter Messwert, aber als Epochen-Marker, nicht als PLL-Treiber.

## Grenzen

- Tages-Aggregation mischt Pässe innerhalb des Tages (bekannt aus Fingerabdruck/Pass-Blatt); ein
  In-Pass-Gradient kann auf Tages-Achse unentdeckt bleiben — die Pass-Wahrheit (eine durchgängige
  In-Pass-Rampe bei Pass-Identität) bleibt `pending`, auch nach diesem Blatt.
- Epochen-Kovariate = Kalendermonat; die zwei Inseln liegen ~11 Monate auseinander und sind nicht zu
  einer durchgehenden Tages-Reihe verbindbar — ein Insel-übergreifender Epochen-Kontrast ist nicht als
  eine TE-Reihe messbar.
- st43 (Insel B) hat n = 31, max Tageslücke 10 d — die Reihe trägt den Test knapp.
- Die phasenrandomisierte Schwelle ist für das bimodale, epochal getriebene Stärke-Feld konservativ
  (erhält das Niederfrequenz-Spektrum); ein schwacher echter Term unterhalb der Auflösung bliebe
  unentdeckt — die Block- und konditionale Null sind die feineren Siebe, und auch sie lassen keinen
  konsistenten Vorwärts-Pfeil stehen.
- Null-Honesty: mean+2σ aus 10/20 Surrogaten bei n≈31–60 Tagen trägt eine Schwankung der Schwelle
  selbst; die f32-KDE-Auflösung liegt nahe ~1e-2. TE-Magnituden dieser Größenordnung sind mit dieser
  Surrogat-Zahl und Auflösung grob (0 geehrt) — die Schlüsse ziehen aus dem Kollaps unter der
  Konditionierung, nicht aus feinen TE-Differenzen.
- Stärke-Feld unkalibriert; nur die Ordnung benutzt. Zwei isolierte konditional-signifikante
  Vorwärts-Zellen (st63 median lag 3 & 5) und vier Rückwärts-Zellen sind als Zufall gewertet, nicht
  als Befund.

## Register-Satz

*Ein gerichteter Stärke→Rauschen-Pfeil ist im Mode-1-Residuum nicht messbar: TE(stärke→rauschen)
überschreitet keine phasenrandomisierte Null; die Block-Null-Überschreitungen der lauten Epoche
1996-12..1997-02 kollabieren unter der Konditionierung auf Monats-Epoche, und nur 2 von 64
Vorwärts-Zellen überleben konditional (st63 median, lag 3 & 5, lag-verstreut, ohne Metrik-/Nachbar-
Bestätigung, in Zufallserwartung). Der AGC-Boden −2560 ist damit kein SNR-getriebener PLL-Term auf
Tages-Achse, sondern Epochen-/Stations-kollokiert: Boden-Tage und laute Tage teilen Monat und Station,
das Stärke-Feld trägt die Rausch-Anhebung nicht gerichtet. Was bleibt, ist die Pass-Wahrheit
(In-Pass-Stärke-Rampe bei Pass-Identität): pending.*

## Status

`done` (Rat gehalten, 2026-09-05). Richtungs-TE, Tages-Achse, zwei Epochen-Inseln.
Sonde `galileo_te_floor_direction.rs` additiv, `cargo check` 0/0; Report auf
stdout. Vollständige Zell-Tabellen: Probe-Stdout.

Folge-Befund: die Pass-Wahrheit ist gemessen (`befund-galileo-inpass-staerke-rampe`,
done) — an Stationen 43/63 ist der statische Boden↔Rauschen-Link bei Pass-Identität
echt (nicht Epochen-/Stations-Kollokation), an 14 nicht. Das Tages-Achsen-Null-Urteil
dieses Blatts gilt für die **Richtung**; die statische Assoziation ist an 43/63
nicht bloß kollokiert (F3 misst keine Richtung).
