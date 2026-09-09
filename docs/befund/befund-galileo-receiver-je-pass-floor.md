<!--
  title: Befund — Galileo-Receiver-Identität pro Pass gegen die station-gebundenen Floor-Gipfel (Richtung F): auch auf der Pass-Achse trägt die kodierte Receiver-Identität die laut/ruhig-Trennung nicht — 547 robuste Boden-Pässe (314 laut, 233 ruhig) laufen vollständig in einer konstanten Konfiguration (ref 5, rcv/amp/atype 0), das einzige variierende Wort RX_REF existiert nur im starken Zustand, ist Ära-/Tages-blockgebunden und trennt laut von ruhig nicht
  class: befund
  date: 2026-09-05
  sha256: 6d3d33a39ceb408e6aabe99ce9e6403f5c20d1703c8b1104ca4aeafac0930210
  status: draft
  antwortet-auf: docs/befund/befund-galileo-floor-subtages-recurrenz.md docs/befund/befund-galileo-receiver-floor-ursache.md
  see-also: docs/handover/archiv/handover-2026-09-05-galileo-tiefe-rotor-spin-receiver.md docs/befund/befund-galileo-ops-aera-floor.md tools/harvest/src/bin/galileo_atdf_receiver_compiler.rs
-->
# Befund: Galileo-Receiver-Identität pro Pass gegen die Floor-Gipfel — die letzte je-Pass-Achse ist gemessen: keine Receiver-Konfiguration trennt laut von ruhig

## Frage & Bindung

Der Receiver-Ursachen-Befund (done) maß die kodierte Receiver-Identität auf der
Zell-Ebene (Mode, Tag, Station, Klasse) und fand DOPPLER_RCVR_REF im gesamten
Boden uniform 5, RCVR_NUMBER/AMP_NUMBER/AMP_TYPE unbesetzt. Die Sub-Tages-Blätter
(draft) ließen den Sitz der Gipfel je-Pass offen; die Receiver-Identität pro Pass
war die letzte nicht je-Pass gegriffene messbare Achse. Dieser Lauf (Richtung F)
kreuzt die Receiver-Felder des Assets `galileo_receiver.bin` gegen die
laut/ruhig-Tage der station-gebundenen Floor-Gipfel **pro Pass**: Sample-Läufe
(Gap ≤ 600 s) je (Mode, Station, Tag), je Pass die vier rx-Felder, laut/ruhig je
Pass.

Gebunden wie die Vorlagen: Stationen 14/43/63, Modi 1/2; Boden = Stärke exakt
−2560 (AGC-Klemmwert), stark = Stärke ≥ −1750; Lock (|resid| > 1000 Hz) vor der
Klasse getrennt; Zelle = (Mode, Tag, Station, Klasse); robust = Zellen-n ≥ 30;
laut = RMS ≥ 1 Hz. Die Pass-Ebene: Pass = zusammenhängende Samples derselben
Klasse (Gap ≤ 600 s, das Gap-Kriterium der Sub-Tages-Probe); Pass-RMS um den
Tages-Klassen-Mittelwert (so zerlegt die Pass-RMS die Tages-RMS: laut-Tage tragen
mindestens einen lauten Pass); laut/ruhig je Pass = Pass-RMS ≥ 1 Hz. Das Asset
trägt resid + station + mode + strength + die vier rx-Wörter selbst; die
Kreuzung läuft ausschließlich aus `galileo_receiver.bin`, die Resid-Serie
`galileo_resid.bin` dient nur als Abdeckungs-/Tag-Kreuzreferenz (die Quelle der
Register-Tage). Probe `tools/measure/src/bin/galileo_receiver_pass_cross.rs`
(neu, einzige Repo-Änderung außer diesem Blatt; `cargo check -p
omegaflow-measure --bin galileo_receiver_pass_cross` mit RUSTFLAGS `-D
warnings`, 0/0), Report `/tmp/opencode/galileo_receiver_pass_cross_report.txt`.

## n zuerst (0 geehrt) — Abdeckung des Assets über die Floor-Ära

`galileo_receiver.bin`: Header 15 013 544 Samples, Spanne 1990-11-29 ..
1997-02-28; ausgeschlossen vor der Klasse: Lock/nicht-endlich 2 097 256,
Nicht-Trio-Station 475 340, Nicht-Modus-1/2 986 155, Stärke weder Boden noch
stark 2 918 422; klassierte Trio-Samples Modi 1/2: **8 536 371**. Floor-Samples
(Modi 1/2, Trio): 4 244 313. Die Tages-Zell-Mengen (Boden, Modi 1/2, Trio) von
Receiver-Asset und Resid-Serie sind tag-identisch (kein Tag nur im einen oder
nur im anderen Asset). Die robuste laut/ruhig-Serie des Assets reproduziert die
Register-Zählungen:

| Serie | Register (resid) robust/laut | Receiver-Asset robust/laut |
|---|---|---|
| M1 st14 | 62/32 | 62/32 |
| M1 st43 | 64/35 | 64/35 |
| M1 st63 | 68/34 | **70/36** |
| M2 st14 | 38/17 | 38/17 |
| M2 st43 | 35/16 | 35/16 |
| M2 st63 | 39/22 | 39/22 |

Abdeckung der laut-Tage: **alle 156 laut-Tage der Register-Serie (M1+M2) sind im
Asset laute Boden-Tage** (156/156 laut, 0 ruhig-robust, 0 dünn, 0 absent). Das
Asset trägt darüber hinaus zwei laute robuste Boden-Tage, die in der
Resid-Serie nur dünn liegen (n 25/28): M1 st63 1995-12-08 und 1995-12-09 (im
Asset n 37/56, RMS 82,6/302 Hz — in der laut-Burst-Ära Dez 1995). Die zwei
Zusatz-Tage verschieben M1 st63 von 68/34 auf 70/36; alle übrigen Serien sind
identisch. Die Kreuzung ist damit über die volle laut-Tag-Liste der Vorlagen
möglich; die zwei Asset-eigenen Zusatz-Tage sind benannt, nicht ergänzt.

## Messung 2 — Pass-Struktur pro Tag (Boden-Klasse, robuste Tage)

Über die 308 robusten Boden-Tage des Assets (M1 62+64+70, M2 38+35+39) trägt die
Boden-Klasse **547 Pässe**; je Serie: M1 st14 135 (laut 83, ruhig 52), M1 st43
127 (84/43), M1 st63 126 (65/61), M2 st14 42 (19/23), M2 st43 47 (23/24), M2 st63
70 (40/30). **Je Pass einheitliche Receiver-Konfiguration:** kein einziger der
547 Boden-Pässe mischt Konfigurationen (mixed-config laut 0, ruhig 0). Die lauten
Pässe (314) und die ruhigen Pässe (233) tragen **vollständig dieselbe
Konfiguration ref 5 rcv 0 amp 0 atype 0**.

## Messung 3 — Receiver-Feld-Zensus pro Pass, laut gegen ruhig

**Boden-Klasse (die Gipfel-Achse), Zensus pro Pass:** über alle 547 Pässe und
alle 6 Serien: laut-Pässe 314 → rx (5,0,0,0) 314; ruhig-Pässe 233 → (5,0,0,0)
233; laut-Samples 1 656 370, ruhig-Samples 2 586 919, alle (5,0,0,0). RX_REF ist
damit auch pro Pass **konstant 5** (über alle 4 244 313 Boden-Samples);
RCVR_NUMBER/AMP_NUMBER/AMP_TYPE sind **in allen 8 536 371 klassierten
Trio-Samples 0** — absent, kein Stations-/Pass-Unterschied. Kein rx-Feld
diskriminiert laut von ruhig, weil auf der Boden-Achse kein rx-Feld eine
Varianz trägt (gemessen, n zuerst).

**Starke Klasse (Kontroll-Achse, die einzige mit variierendem rx-Feld):**
RX_REF variiert dort (ref 0/4/5). Pass-Zensus je Serie (laut-Pässe / ruhig-Pässe
je ref):

| Serie | ref 0 laut/ruhig | ref 4 laut/ruhig | ref 5 laut/ruhig |
|---|---|---|---|
| M1 st14 | 1/0 | 3/17 | 78/84 |
| M1 st43 | 3/1 | 11/9 | 29/164 |
| M1 st63 | 2/0 | 25/1 | 30/167 |
| M2 st14 | 8/1 | — | 7/12 |
| M2 st43 | 13/2 | — | 13/38 |
| M2 st63 | 9/0 | 1/0 | 12/31 |

ref 5 (die dominante Konfiguration) trägt an **jeder** Serie laute und ruhige
Pässe. Die ref-0-Pässe gehören zu den 1990-Tagen (Datums-Spannen je
Konfiguration in Messung 4) — vor der Floor-Gipfel-Ära, die ab 1995-11-23
beginnt — und liegen dort fast nur laut (1/0, 3/1, 2/0, 8/1, 13/2, 9/0 je
Serie, n klein). Die ref-4-Pässe liegen auf wenigen, über 1994–1997
verstreuten Tagen (M1 st14 12, M1 st43 12, M1 st63 17, M2 st63 1 Tag) und sind
je Station gemischt (M1 st43 11/9 ausgeglichen, M1 st63 25/1 laut-lastig, M1
st14 3/17 ruhig-lastig). Die die Ära durchziehende
Konfiguration (ref 5) ist an jeder Serie beides, laut und ruhig; die
ref-0-/ref-4-Sub-Populationen sind klein, Ära-/Tages-blockgebunden und je
Station uneinheitlich (benannt, nicht glattgezogen).

## Messung 4 — Je-Station-Tageslage: ist die laut/ruhig-Trennung eines Tages an die Receiver-Konfiguration dieses Tages gebunden?

Die Boden-Konfiguration ist je Station über die ganze Ära konstant — die
laut/ruhig-Flips eines Tages (derselbe Pass-Phasen-Bereich der Station, an dem
auch die ruhigen Tage liegen) laufen **innerhalb einer einzigen konstanten
Konfiguration** ab. Die Pass-Start-Phasen der lauten und ruhigen Pässe je Serie
überlappen (Boden): M1 st14 laut 19,6 h (R 0,68) gegen ruhig 19,0 h (R 0,65),
M1 st43 1,7 h (0,36) gegen 1,4 h (0,17), M1 st63 12,0 h (0,68) gegen 10,7 h
(0,78), M2 st14 20,1 h (0,70) gegen 20,0 h (0,71), M2 st63 13,5 h (0,73) gegen
12,0 h (0,90) — die Phasen-Kongruenz der laut-Tage ist die stationseigene
Pass-Tageslage (reproduziert), und beide Zustände tragen exakt dieselbe rx-
Konfiguration. Ein „gleiche Station, gleiche Tageslage, anderer rx-Wert → laut"
existiert auf der Boden-Achse nicht als Vergleich, weil es keinen zweiten rx-Wert
gibt (gemessen). Die einzige rx-Variation der Ära (starker Zustand) ist an
Datums-Blöcke gebunden, nicht an die laut/ruhig-Tageslage: z. B. M1 st14 stark
ref 0 nur 1990-12-11, ref 4 1994-12-17..1996-12-21 (12 Tage), ref 5 ab
1995-11-24; M1 st43 stark ref 0 1990-11-29..12-05, ref 4 1994-12-17..1997-01-01,
ref 5 ab 1995-12-06. Die Konfigurations-Wechsel der starken Klasse sind
Ära-/Datum-Blockwechsel, kein Tag-zu-Tag- oder Pass-zu-Pass-Lautschalter.

## Anker-Tage (Reproduktion, pro Pass)

1995-11-24 M1 Boden: st14 laut (2 Pässe 16,8 h n 6106 RMS 28,6 Hz und 21,2 h
n 1357 RMS 1,5 Hz), st43 ruhig (21,2 h n 39 991 RMS 0,031 Hz), st63 ruhig
(12,0 h/11,2 h, RMS 0,019/0,092 Hz) — **alle drei Stationen, laut und ruhig,
bei derselben Konfiguration (5,0,0,0) im selben Augenblick**. 1996-06-26 M1
Boden: st63 laut (21,7 h n 24 201 RMS 20,64 Hz), st14 ruhig (4,5 h n 19 083 RMS
0,021 Hz), st43 ruhig (15,3 h/8,4 h, RMS 0,014/0,038 Hz) — ebenfalls alle
(5,0,0,0). Die station-gebundene Lautheit liest an identischer Position und
identischer Pass-Konfiguration je Station laut oder ruhig.

## Verdikt

**Auch pro Pass trägt die kodierte Receiver-Identität die laut/ruhig-Trennung
der station-gebundenen Floor-Gipfel nicht.** Auf der Boden-Achse (die
Gipfel-Achse der gesamten Faden-Reihe) tragen alle 547 robusten Pässe — 314
laute und 233 ruhige — vollständig dieselbe Konfiguration (ref 5, rcv 0, amp 0,
atype 0); RX_REF ist auch je Pass konstant 5 über alle 4 244 313 Boden-Samples,
RCVR_NUMBER/AMP_NUMBER/AMP_TYPE sind absent (0) in allen 8 536 371 klassierten
Trio-Samples. Das einzige je Pass variierende Wort (RX_REF im starken Zustand,
ref 0/4/5) trennt laut/ruhig über die Gipfel-Ära nicht: die ära-durchziehende
ref 5 ist an jeder Serie beides; ref 0 liegt fast nur laut, aber in der
starken Klasse der 1990-Tage vor der Floor-Ära; ref 4 liegt auf wenigen, über
1994–1997 verstreuten Tagen (M1 je Serie 12–17, M2 st63 1 Tag) und ist je
Station gemischt (M1 st14 ruhig-lastig, M1 st43 ausgeglichen, M1 st63 laut-
lastig) — die Sub-Populationen sind klein und Datums-blockgebunden. Die laut/ruhig-Flips laufen auch pro Pass innerhalb
einer konstanten Receiver-Konfiguration ab, an derselben stationseigenen
Pass-Tageslage (Anker-Tage: laut und ruhig nebeneinander bei identischer
Konfiguration). Der Negativ-Befund der Zell-Ebene (receiver-floor-ursache, done)
ist damit auf der Pass-Achse bestätigt — die letzte offene messbare Achse der
kodierten Identität ist gemessen.

**Was `pending` bleibt:** der Sitz der Gipfel — der per-Pass-Zustand, der in den
TDF nicht verschlüsselt ist (Verschaltung, Schleifen-/AGC-Zustand,
Empfangs-Betriebszustand). Das Feld enthält die Antwort nicht; die Ursache ist
nicht als Abwesenheit einer Receiver-Konfiguration messbar, sondern als
`pending` registriert.

## Grenzen

- Die Pass-laut/ruhig-Metrik ist die Pass-RMS um den Tages-Klassen-Mittelwert
  (genannt; sie zerlegt die Tages-RMS, sodass laut-Tage ≥ 1 lauten Pass tragen);
  die Zell-Definition und die 1-Hz-Schwelle sind unverändert zur Referenz-Linie.
- Die starke Klasse ist eine Kontroll-Achse, nicht das Gipfel-Objekt; ihre
  ref-0/ref-4-Sub-Populationen sind klein (Tage 1–17 je Station) und mit
  Datums-Ära verquickt — eine Trennung Konfiguration gegen Ära ist dort nicht
  möglich (benannt).
- Das Receiver-Asset trägt 2 robuste laute Boden-Tage mehr als die Resid-Register
  (M1 st63 1995-12-08/09, in der Resid-Serie dünn n 25/28); sie sind benannt und
  liegen in der laut-Burst-Ära — die 156 laut-Tage der Register sind vollständig
  abgedeckt.
- Das Asset umfasst Modi 1/2; die Modus-3-Serie der Vorlagen (51 der 207
  laut-Zellen) ist nicht Teil der Receiver-Kreuzung (Modus-3-Scope, benannt).

## Register-Satz

*Die Receiver-Identität aus data/galileo_receiver.bin ist pro Pass gegen die
laut/ruhig-Tage der station-gebundenen Galileo-Floor-Gipfel gekreuzt (Richtung F,
Modi 1/2, Trio-Stationen): die Abdeckung ist vollständig (156/156 laut-Tage der
Register im Asset laut, dazu 2 Asset-eigene laute Tage M1 st63 1995-12-08/09, in
der Resid-Serie dünn), und auf der Pass-Achse tragen alle 547 robusten
Boden-Pässe — 314 laut, 233 ruhig — vollständig die konstante Konfiguration
ref 5 rcv 0 amp 0 atype 0 (RX_REF konstant 5 über 4 244 313 Boden-Samples,
rcv/amp/atype absent in allen 8 536 371 klassierten Samples); das einzige
variierende Wort (RX_REF im starken Zustand, ref 0/4/5) trennt laut/ruhig über
die Gipfel-Ära nicht: die ära-durchziehende ref 5 ist an jeder Serie beides,
ref 0 liegt fast nur laut in den 1990-Tagen vor der Floor-Ära, ref 4 ist je
Station gemischt — keine Receiver-Konfiguration diskriminiert laut von ruhig,
auch nicht pro Pass. Der Sitz der Gipfel (per-Pass-Verschaltung, nicht in den
TDF kodiert) bleibt pending.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; TODO-Registerzeile ergänzt die
Haupt-Session). Probe `galileo_receiver_pass_cross` committet (`cargo check -p
omegaflow-measure --bin galileo_receiver_pass_cross`, RUSTFLAGS `-D warnings`,
0/0), Report `/tmp/opencode/galileo_receiver_pass_cross_report.txt`.
